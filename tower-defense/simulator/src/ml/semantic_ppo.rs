//! Phase 4B PPO on the Phase 4A semantic action stack.
//!
//! The actor is the unchanged `DeepSetsActorCritic` actor path scored by
//! `semantic_bc::batch_log_probs` over the decisions built by
//! `semantic_bc::semantic_decision` - exactly the candidate set, legal mask,
//! encoding and action-index semantics the BC evaluator uses. The executed
//! environment action is always the sampled candidate's action; nothing
//! rewrites it after sampling.
//!
//! The critic is a separate `DeepSetsActorCritic` instance (its own typed
//! entity encoder plus the global and typed value heads), so value learning
//! never moves the actor's shared encoder: before the first update the actor
//! is bit-identical to its BC initialization.
//!
//! Reward: `r_t = reward_scale * (clear_rate(s_{t+1}) - clear_rate(s_t))`
//! between consecutive decisions. With `gamma = 1` the return telescopes to
//! `reward_scale * (terminal - current clear_rate)`.

use super::encoding::{PaddedEntityBatch, observation::ENTITY_SET_COUNT};
use super::model::{
    DeepSetsActorCritic, InferenceBackend, ModelConfig, PolicyDevice, TrainBackend,
    default_policy_device, model_to_full_precision_bytes, tensor_from_rows,
};
use super::neural_checkpoint::current_git_revision;
use super::phase4_dataset::{
    DatasetProvenance, EpisodeRecord, GAME_RULES_EPOCH, MAX_EPISODE_DECISIONS, Phase4Split,
};
use super::phase4_eval::{EvalPolicy, PairedComparison, PolicySummary, evaluate_policies};
use super::semantic_bc::{
    BcCheckpointMetadata, SemanticDecision, SemanticPolicy, batch_log_probs, load_model_file,
    load_selected_model, sample_masked, semantic_decision,
};
use super::semantic_candidates::{
    EncodedDecision, POLICY_CANDIDATE_SET_VERSION, SEMANTIC_CANDIDATE_ENCODER_VERSION,
    encode_decision,
};
use crate::config::GameConfig;
use crate::environment::{DecisionPoint, GameEnvironment};
use anyhow::{Context, Result, bail};
use burn::module::AutodiffModule;
use burn::optim::adaptor::OptimizerAdaptor;
use burn::optim::grad_clipping::GradientClippingConfig;
use burn::optim::{Adam, AdamConfig, GradientsParams, Optimizer};
use burn::record::{BinFileRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, TensorData};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

pub const SEMANTIC_PPO_CHECKPOINT_SCHEMA_VERSION: u32 = 1;
/// 2: critic inputs pass through `critic_squash`.
pub const CRITIC_CHECKPOINT_SCHEMA_VERSION: u32 = 2;
const INFERENCE_BATCH_SIZE: usize = 256;

/// Action kinds whose sampled share is tracked every iteration (a kind that
/// suddenly vanishes or explodes is a regression signal).
pub const TRACKED_ACTION_KINDS: [&str; 12] = [
    "reroll",
    "build_tower",
    "place_tower",
    "remove_tower",
    "start_defense",
    "continue",
    "use_inventory_item",
    "purchase_shop_item",
    "select_treasure",
    "discard_treasure",
    "select_card_service_card",
    "confirm_card_service_selection",
];

type ActorCriticOptimizer = OptimizerAdaptor<Adam, DeepSetsActorCritic<TrainBackend>, TrainBackend>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PpoConfig {
    pub episodes_per_iteration: usize,
    pub gamma: f32,
    pub gae_lambda: f32,
    /// Reward units per clear_rate percentage point.
    pub reward_scale: f32,
    pub actor_learning_rate: f64,
    pub critic_learning_rate: f64,
    pub update_epochs: usize,
    pub minibatch_size: usize,
    pub clip_epsilon: f32,
    pub entropy_coefficient: f32,
    /// Weight of `KL(pi || pi_init)` against the BC initialization; 0 only
    /// measures it.
    pub kl_to_init_coefficient: f32,
    /// Stop an iteration's remaining epochs once the approximate
    /// old-to-new KL of an epoch exceeds this.
    pub target_kl: Option<f32>,
    pub max_grad_norm: f32,
    pub normalize_advantages: bool,
    /// Actor updates start after this many critic-only iterations.
    pub critic_warmup_iterations: usize,
    pub seed: u64,
    /// Iteration `i` plays `ppo_train` seed block `i + offset`, so a
    /// continuation run never replays another run's training games.
    #[serde(default)]
    pub train_seed_block_offset: usize,
}

impl Default for PpoConfig {
    fn default() -> Self {
        Self {
            episodes_per_iteration: 48,
            gamma: 1.0,
            gae_lambda: 0.95,
            reward_scale: 0.1,
            actor_learning_rate: 1e-5,
            critic_learning_rate: 3e-4,
            update_epochs: 4,
            minibatch_size: 256,
            clip_epsilon: 0.2,
            entropy_coefficient: 0.0,
            kl_to_init_coefficient: 0.0,
            target_kl: Some(0.02),
            max_grad_norm: 0.5,
            normalize_advantages: true,
            critic_warmup_iterations: 0,
            seed: 0,
            train_seed_block_offset: 0,
        }
    }
}

/// Signed `ln(1 + |x|)`. Some shared features are unnormalized (card polish
/// reaches 3,000 after `/1000` scaling), which makes the critic's fresh
/// Adam steps swing its output wildly; the critic squashes its inputs. The
/// actor keeps the raw features its BC initialization was trained on.
pub fn critic_squash(value: f32) -> f32 {
    value.signum() * value.abs().ln_1p()
}

/// Critic value `V(s)`: typed value head over the critic's own typed entity
/// encoding plus the global-feature value head, both on squashed inputs.
/// `[decisions, 1]`.
pub fn critic_values<B: Backend>(
    critic: &DeepSetsActorCritic<B>,
    decisions: &[&EncodedDecision],
    device: &B::Device,
) -> Tensor<B, 2> {
    let typed_batches: [PaddedEntityBatch; ENTITY_SET_COUNT] = std::array::from_fn(|index| {
        PaddedEntityBatch::from_sets(
            &decisions
                .iter()
                .map(|decision| {
                    let mut set = decision.typed.sets[index].clone();
                    for row in &mut set.rows {
                        for value in &mut row.numeric {
                            *value = critic_squash(*value);
                        }
                    }
                    set
                })
                .collect::<Vec<_>>(),
        )
    });
    let typed = critic.encode_typed_sets(&typed_batches, device);
    let global = tensor_from_rows::<B>(
        &decisions
            .iter()
            .map(|decision| {
                decision
                    .global_features
                    .iter()
                    .map(|value| critic_squash(*value))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        device,
    );
    critic.forward_typed_values(typed) + critic.forward_values(global)
}

pub fn critic_value_vector(
    critic: &DeepSetsActorCritic<InferenceBackend>,
    decisions: &[&EncodedDecision],
    device: &PolicyDevice,
) -> Result<Vec<f32>> {
    let mut values = Vec::with_capacity(decisions.len());
    for chunk in decisions.chunks(INFERENCE_BATCH_SIZE) {
        values.extend(
            critic_values(critic, chunk, device)
                .into_data()
                .to_vec::<f32>()?,
        );
    }
    Ok(values)
}

/// Per-decision log-probabilities (unpadded, one vector per decision).
pub fn actor_log_prob_vectors(
    actor: &DeepSetsActorCritic<InferenceBackend>,
    decisions: &[&EncodedDecision],
    device: &PolicyDevice,
) -> Result<Vec<Vec<f32>>> {
    let mut result = Vec::with_capacity(decisions.len());
    for chunk in decisions.chunks(INFERENCE_BATCH_SIZE) {
        let (log_probs, groups) = batch_log_probs(actor, chunk, device);
        let values = log_probs.into_data().to_vec::<f32>()?;
        let width = groups.width();
        for (row, decision) in chunk.iter().enumerate() {
            result.push(values[row * width..row * width + decision.candidates.len()].to_vec());
        }
    }
    Ok(result)
}

fn entropy_of(log_probs: &[f32], legal_mask: &[bool]) -> f32 {
    log_probs
        .iter()
        .zip(legal_mask)
        .filter(|(_, legal)| **legal)
        .map(|(log_prob, _)| {
            let probability = log_prob.exp();
            if probability > 0.0 {
                -probability * log_prob
            } else {
                0.0
            }
        })
        .sum()
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub encoded: EncodedDecision,
    pub action_index: usize,
    pub action_id: String,
    pub action_kind: String,
    pub decision_point: String,
    pub old_log_prob: f32,
    pub behavior_entropy: f32,
    pub clear_rate_before: f32,
    pub raw_delta: f32,
    pub reward: f32,
    pub value: f32,
    pub advantage: f32,
    pub return_value: f32,
    /// Greedy candidate of the behavior policy at this state.
    pub greedy_index: usize,
    /// The canonical scripted action's candidate index.
    pub canonical_index: Option<usize>,
    /// `pi_init` log-probabilities (unpadded).
    pub init_log_probs: Vec<f32>,
}

impl Transition {
    pub fn init_greedy_index(&self) -> Option<usize> {
        greedy_index(&self.init_log_probs, &self.encoded.legal_mask)
    }
}

pub fn greedy_index(log_probs: &[f32], legal_mask: &[bool]) -> Option<usize> {
    (0..legal_mask.len().min(log_probs.len()))
        .filter(|index| legal_mask[*index])
        .max_by(|left, right| {
            log_probs[*left]
                .total_cmp(&log_probs[*right])
                .then_with(|| right.cmp(left))
        })
}

#[derive(Clone, Debug)]
pub struct EpisodeRollout {
    pub seed: u64,
    pub transitions: Vec<Transition>,
    /// Encoding of the state after the last transition when the episode was
    /// truncated (bootstrapped with the critic); `None` at a true terminal.
    pub bootstrap: Option<EncodedDecision>,
    pub initial_clear_rate: f32,
    pub terminal_clear_rate: f32,
    pub final_stage: usize,
    pub victory: bool,
    pub truncated: bool,
    pub illegal_actions: usize,
    /// Decisions whose executed action differs from the sampled candidate.
    pub action_mismatches: usize,
    pub decision_seconds: f64,
    pub forward_seconds: f64,
    pub environment_seconds: f64,
}

enum StepEnd {
    Decision,
    Terminal,
    Truncated,
}

/// Applies `action` and settles forced actions up to the next decision.
fn advance(
    environment: &mut GameEnvironment,
    action: crate::environment::AgentAction,
) -> Result<StepEnd> {
    let mut outcome = environment
        .rollout_step_trusted(action)
        .map_err(|error| anyhow::anyhow!("semantic step failed: {error:?}"))?;
    loop {
        if outcome.terminated || matches!(environment.decision_point(), DecisionPoint::Terminal) {
            return Ok(StepEnd::Terminal);
        }
        if outcome.truncated {
            return Ok(StepEnd::Truncated);
        }
        match environment.forced_action() {
            Some(forced) => {
                outcome = environment
                    .rollout_step_trusted(forced)
                    .map_err(|error| anyhow::anyhow!("forced step failed: {error:?}"))?;
            }
            None => return Ok(StepEnd::Decision),
        }
    }
}

/// Plays one episode with the actor, sampling every decision from its masked
/// distribution (or taking the greedy action when `greedy`), and records the
/// transitions. Values are filled in later in one batched critic pass.
pub fn rollout_episode(
    actor: &DeepSetsActorCritic<InferenceBackend>,
    device: &PolicyDevice,
    config: Arc<GameConfig>,
    game_seed: u64,
    sample_seed: u64,
    reward_scale: f32,
    greedy: bool,
) -> Result<EpisodeRollout> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(sample_seed);
    let mut environment = GameEnvironment::new(config, game_seed);
    let initial_clear_rate = environment.clear_rate();
    let mut transitions = Vec::new();
    let mut illegal_actions = 0usize;
    let mut action_mismatches = 0usize;
    let mut truncated = false;
    let mut bootstrap = None;
    let mut decision_seconds = 0.0;
    let mut forward_seconds = 0.0;
    let mut environment_seconds = 0.0;
    while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
        if transitions.len() >= MAX_EPISODE_DECISIONS {
            truncated = true;
            bootstrap = Some(semantic_decision(&environment)?.encoded);
            break;
        }
        let started = Instant::now();
        let SemanticDecision {
            candidates,
            legal_mask,
            encoded,
        } = semantic_decision(&environment)?;
        decision_seconds += started.elapsed().as_secs_f64();
        let started = Instant::now();
        let (log_probs, _) = batch_log_probs(actor, &[&encoded], device);
        let log_probs = log_probs.into_data().to_vec::<f32>()?[..legal_mask.len()].to_vec();
        forward_seconds += started.elapsed().as_secs_f64();
        if log_probs.iter().any(|value| value.is_nan()) {
            bail!("seed {game_seed}: NaN actor log-probability");
        }
        let u: f64 = rng.r#gen();
        let greedy_choice = greedy_index(&log_probs, &legal_mask).context("no legal candidate")?;
        let action_index = if greedy {
            greedy_choice
        } else {
            sample_masked(&log_probs, &legal_mask, u)
        };
        let candidate = &candidates.candidates[action_index];
        let action = candidate.action.clone();
        if !legal_mask[action_index] || !environment.semantic_action_is_legal(&action) {
            illegal_actions += 1;
        }
        if action.action_id() != candidate.action_id {
            action_mismatches += 1;
        }
        let clear_rate_before = environment.clear_rate();
        let started = Instant::now();
        let end = advance(&mut environment, action)?;
        environment_seconds += started.elapsed().as_secs_f64();
        let raw_delta = environment.clear_rate() - clear_rate_before;
        transitions.push(Transition {
            action_index,
            action_id: candidate.action_id.clone(),
            action_kind: candidate.action.kind().wire_name().to_string(),
            decision_point: format!("{:?}", candidates.observation.decision_point),
            old_log_prob: log_probs[action_index],
            behavior_entropy: entropy_of(&log_probs, &legal_mask),
            clear_rate_before,
            raw_delta,
            reward: raw_delta * reward_scale,
            value: 0.0,
            advantage: 0.0,
            return_value: 0.0,
            greedy_index: greedy_choice,
            canonical_index: candidates.canonical_index(),
            init_log_probs: Vec::new(),
            encoded,
        });
        match end {
            StepEnd::Decision => {}
            StepEnd::Terminal => break,
            StepEnd::Truncated => {
                truncated = true;
                if !matches!(environment.decision_point(), DecisionPoint::Terminal) {
                    let observation = environment.snapshot();
                    let candidates = super::semantic_candidates::policy_candidates(&environment)?;
                    let mask = vec![true; candidates.candidates.len()];
                    bootstrap = Some(encode_decision(&observation, &candidates.candidates, mask));
                }
                break;
            }
        }
    }
    let terminal_clear_rate = environment.clear_rate();
    Ok(EpisodeRollout {
        seed: game_seed,
        transitions,
        bootstrap,
        initial_clear_rate,
        terminal_clear_rate,
        final_stage: environment.snapshot().stage,
        victory: terminal_clear_rate >= 100.0,
        truncated,
        illegal_actions,
        action_mismatches,
        decision_seconds,
        forward_seconds,
        environment_seconds,
    })
}

/// Generalized advantage estimation for one episode. `bootstrap_value` is
/// `V(s_T)` for a truncated episode and 0 at a true terminal.
pub fn compute_gae(
    rewards: &[f32],
    values: &[f32],
    bootstrap_value: f32,
    gamma: f32,
    gae_lambda: f32,
) -> (Vec<f32>, Vec<f32>) {
    assert_eq!(rewards.len(), values.len());
    let mut advantages = vec![0.0; rewards.len()];
    let mut returns = vec![0.0; rewards.len()];
    let mut gae = 0.0f32;
    let mut next_value = bootstrap_value;
    for index in (0..rewards.len()).rev() {
        let delta = rewards[index] + gamma * next_value - values[index];
        gae = delta + gamma * gae_lambda * gae;
        advantages[index] = gae;
        returns[index] = gae + values[index];
        next_value = values[index];
    }
    (advantages, returns)
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateStats {
    pub minibatches: usize,
    pub epochs_completed: usize,
    pub early_stopped: bool,
    pub nonfinite_skips: usize,
    pub policy_loss: f64,
    pub value_loss: f64,
    pub entropy: f64,
    pub approx_kl: f64,
    pub clip_fraction: f64,
    pub kl_to_init_term: f64,
    pub mean_actor_grad_norm: f64,
    pub max_actor_grad_norm: f64,
    pub mean_critic_grad_norm: f64,
    pub max_critic_grad_norm: f64,
    pub first_value_loss: f64,
    pub last_value_loss: f64,
    pub max_value_loss: f64,
}

pub struct PpoLearner {
    pub actor: DeepSetsActorCritic<TrainBackend>,
    pub critic: DeepSetsActorCritic<TrainBackend>,
    pub actor_optimizer: ActorCriticOptimizer,
    pub critic_optimizer: ActorCriticOptimizer,
    pub device: PolicyDevice,
}

fn new_optimizer(max_grad_norm: f32) -> ActorCriticOptimizer {
    AdamConfig::new()
        .with_grad_clipping(Some(GradientClippingConfig::Norm(max_grad_norm)))
        .init()
}

impl PpoLearner {
    pub fn new(
        actor: DeepSetsActorCritic<TrainBackend>,
        critic: DeepSetsActorCritic<TrainBackend>,
        config: &PpoConfig,
    ) -> Self {
        Self {
            actor,
            critic,
            actor_optimizer: new_optimizer(config.max_grad_norm),
            critic_optimizer: new_optimizer(config.max_grad_norm),
            device: default_policy_device(),
        }
    }

    pub fn actor_inference(&self) -> DeepSetsActorCritic<InferenceBackend> {
        self.actor.clone().valid()
    }

    pub fn critic_inference(&self) -> DeepSetsActorCritic<InferenceBackend> {
        self.critic.clone().valid()
    }

    fn critic_step(&mut self, batch: &[&Transition], learning_rate: f64) -> Option<(f64, f32)> {
        let decisions = batch.iter().map(|step| &step.encoded).collect::<Vec<_>>();
        let values = critic_values(&self.critic, &decisions, &self.device);
        let returns = Tensor::<TrainBackend, 2>::from_data(
            TensorData::new(
                batch
                    .iter()
                    .map(|step| step.return_value)
                    .collect::<Vec<_>>(),
                [batch.len(), 1],
            ),
            &self.device,
        );
        let loss = (values - returns).powf_scalar(2.0).mean();
        let loss_value = loss.clone().into_data().to_vec::<f32>().ok()?[0];
        if !loss_value.is_finite() {
            return None;
        }
        let gradients = GradientsParams::from_grads(loss.backward(), &self.critic);
        let norm = super::ppo::gradient_l2_norm(&self.critic, &gradients);
        if !norm.is_finite() {
            return None;
        }
        self.critic = self
            .critic_optimizer
            .step(learning_rate, self.critic.clone(), gradients);
        Some((loss_value as f64, norm))
    }
}

/// Minibatch actor loss terms (autodiff) and their scalar values.
struct ActorTerms {
    loss: Tensor<TrainBackend, 1>,
    policy_loss: f32,
    entropy: f32,
    kl_to_init: f32,
    approx_kl: f32,
    clip_fraction: f32,
}

fn actor_terms(
    actor: &DeepSetsActorCritic<TrainBackend>,
    batch: &[&Transition],
    advantages: &[f32],
    config: &PpoConfig,
    device: &PolicyDevice,
) -> Result<ActorTerms> {
    let decisions = batch.iter().map(|step| &step.encoded).collect::<Vec<_>>();
    let (log_probs, groups) = batch_log_probs(actor, &decisions, device);
    let rows = batch.len();
    let width = groups.width();
    let actions = groups.target_tensor::<TrainBackend>(
        &batch
            .iter()
            .map(|step| step.action_index)
            .collect::<Vec<_>>(),
        device,
    );
    let new_log_prob = log_probs.clone().gather(1, actions);
    let old_log_prob = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(
            batch
                .iter()
                .map(|step| step.old_log_prob)
                .collect::<Vec<_>>(),
            [rows, 1],
        ),
        device,
    );
    let advantage = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(advantages.to_vec(), [rows, 1]),
        device,
    );
    let log_ratio = new_log_prob - old_log_prob;
    let ratio = log_ratio.clone().exp();
    let clipped = ratio
        .clone()
        .clamp(1.0 - config.clip_epsilon, 1.0 + config.clip_epsilon);
    let surrogate = (ratio.clone() * advantage.clone()).min_pair(clipped * advantage);
    let policy_loss = -surrogate.mean();
    let valid = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(
            (0..rows)
                .flat_map(|row| {
                    let groups = &groups;
                    (0..width).map(move |column| groups.is_valid(row, column) as u8 as f32)
                })
                .collect::<Vec<_>>(),
            [rows, width],
        ),
        device,
    );
    let probabilities = log_probs.clone().exp() * valid.clone();
    let entropy = -(probabilities.clone() * log_probs.clone() * valid.clone())
        .sum_dim(1)
        .mean();
    let mut loss = policy_loss.clone() - entropy.clone() * config.entropy_coefficient;
    let mut kl_to_init_value = 0.0;
    if config.kl_to_init_coefficient > 0.0 {
        let init = Tensor::<TrainBackend, 2>::from_data(
            TensorData::new(
                batch
                    .iter()
                    .flat_map(|step| {
                        (0..width).map(|column| {
                            step.init_log_probs
                                .get(column)
                                .copied()
                                .filter(|value| value.is_finite() && *value > -1.0e8)
                                .unwrap_or(0.0)
                        })
                    })
                    .collect::<Vec<_>>(),
                [rows, width],
            ),
            device,
        );
        let kl = (probabilities * (log_probs - init) * valid)
            .sum_dim(1)
            .mean();
        kl_to_init_value = kl.clone().into_data().to_vec::<f32>()?[0];
        loss = loss + kl * config.kl_to_init_coefficient;
    }
    let ratio_values = ratio.into_data().to_vec::<f32>()?;
    let log_ratio_values = log_ratio.into_data().to_vec::<f32>()?;
    let approx_kl = ratio_values
        .iter()
        .zip(&log_ratio_values)
        .map(|(ratio, log_ratio)| (ratio - 1.0) - log_ratio)
        .sum::<f32>()
        / rows as f32;
    let clip_fraction = ratio_values
        .iter()
        .filter(|ratio| (**ratio - 1.0).abs() > config.clip_epsilon)
        .count() as f32
        / rows as f32;
    Ok(ActorTerms {
        policy_loss: policy_loss.into_data().to_vec::<f32>()?[0],
        entropy: entropy.into_data().to_vec::<f32>()?[0],
        kl_to_init: kl_to_init_value,
        approx_kl,
        clip_fraction,
        loss,
    })
}

impl PpoLearner {
    /// PPO update over `transitions` (advantages/returns already filled).
    pub fn update(
        &mut self,
        transitions: &[Transition],
        config: &PpoConfig,
        iteration: usize,
        update_actor: bool,
    ) -> Result<UpdateStats> {
        let mut stats = UpdateStats::default();
        if transitions.is_empty() {
            return Ok(stats);
        }
        let mut advantages = transitions
            .iter()
            .map(|step| step.advantage)
            .collect::<Vec<_>>();
        if config.normalize_advantages && advantages.len() > 1 {
            let mean = advantages.iter().sum::<f32>() / advantages.len() as f32;
            let variance = advantages
                .iter()
                .map(|value| (value - mean).powi(2))
                .sum::<f32>()
                / (advantages.len() - 1) as f32;
            let std = variance.sqrt().max(1e-6);
            for value in &mut advantages {
                *value = (*value - mean) / std;
            }
        }
        let mut sums = [0.0f64; 6];
        let mut actor_steps = 0usize;
        let mut critic_steps = 0usize;
        let mut critic_grad_sum = 0.0f64;
        let mut actor_grad_sum = 0.0f64;
        let mut value_loss_sum = 0.0f64;
        for epoch in 0..config.update_epochs {
            let mut order = (0..transitions.len()).collect::<Vec<_>>();
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(
                config.seed
                    ^ (iteration as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    ^ (epoch as u64).wrapping_mul(0xD1B5_4A32_D192_ED03),
            );
            order.shuffle(&mut rng);
            let mut epoch_kl = 0.0f64;
            let mut epoch_batches = 0usize;
            for chunk in order.chunks(config.minibatch_size) {
                let batch = chunk
                    .iter()
                    .map(|index| &transitions[*index])
                    .collect::<Vec<_>>();
                let batch_advantages = chunk
                    .iter()
                    .map(|index| advantages[*index])
                    .collect::<Vec<_>>();
                stats.minibatches += 1;
                if update_actor {
                    let terms =
                        actor_terms(&self.actor, &batch, &batch_advantages, config, &self.device)?;
                    let finite = [
                        terms.policy_loss,
                        terms.entropy,
                        terms.approx_kl,
                        terms.kl_to_init,
                    ]
                    .iter()
                    .all(|value| value.is_finite());
                    if finite {
                        let gradients =
                            GradientsParams::from_grads(terms.loss.backward(), &self.actor);
                        let norm = super::ppo::gradient_l2_norm(&self.actor, &gradients);
                        if norm.is_finite() {
                            self.actor = self.actor_optimizer.step(
                                config.actor_learning_rate,
                                self.actor.clone(),
                                gradients,
                            );
                            actor_steps += 1;
                            actor_grad_sum += norm as f64;
                            stats.max_actor_grad_norm = stats.max_actor_grad_norm.max(norm as f64);
                            sums[0] += terms.policy_loss as f64;
                            sums[1] += terms.entropy as f64;
                            sums[2] += terms.approx_kl as f64;
                            sums[3] += terms.clip_fraction as f64;
                            sums[4] += terms.kl_to_init as f64;
                            epoch_kl += terms.approx_kl as f64;
                            epoch_batches += 1;
                        } else {
                            stats.nonfinite_skips += 1;
                        }
                    } else {
                        stats.nonfinite_skips += 1;
                    }
                }
                match self.critic_step(&batch, config.critic_learning_rate) {
                    Some((loss, norm)) => {
                        if critic_steps == 0 {
                            stats.first_value_loss = loss;
                        }
                        stats.last_value_loss = loss;
                        stats.max_value_loss = stats.max_value_loss.max(loss);
                        critic_steps += 1;
                        value_loss_sum += loss;
                        critic_grad_sum += norm as f64;
                        stats.max_critic_grad_norm = stats.max_critic_grad_norm.max(norm as f64);
                    }
                    None => stats.nonfinite_skips += 1,
                }
            }
            stats.epochs_completed = epoch + 1;
            if let Some(target) = config.target_kl
                && update_actor
                && epoch_batches > 0
                && epoch_kl / epoch_batches as f64 > target as f64
            {
                stats.early_stopped = true;
                break;
            }
        }
        let actor_steps_f = actor_steps.max(1) as f64;
        stats.policy_loss = sums[0] / actor_steps_f;
        stats.entropy = sums[1] / actor_steps_f;
        stats.approx_kl = sums[2] / actor_steps_f;
        stats.clip_fraction = sums[3] / actor_steps_f;
        stats.kl_to_init_term = sums[4] / actor_steps_f;
        stats.mean_actor_grad_norm = actor_grad_sum / actor_steps_f;
        stats.value_loss = value_loss_sum / critic_steps.max(1) as f64;
        stats.mean_critic_grad_norm = critic_grad_sum / critic_steps.max(1) as f64;
        Ok(stats)
    }
}

// --- critic supervised pretraining --------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CriticPretrainConfig {
    pub hidden_size: usize,
    pub learning_rate: f64,
    pub batch_size: usize,
    pub epochs: usize,
    pub reward_scale: f32,
    pub seed: u64,
}

impl Default for CriticPretrainConfig {
    fn default() -> Self {
        Self {
            hidden_size: 64,
            learning_rate: 1e-3,
            batch_size: 256,
            epochs: 8,
            reward_scale: 0.1,
            seed: 0,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueMetrics {
    pub samples: usize,
    pub mse: f64,
    pub mae: f64,
    pub mean_target: f64,
    pub mean_prediction: f64,
    pub target_std: f64,
    pub explained_variance: f64,
    pub correlation: f64,
    pub nonfinite_predictions: usize,
    /// `stage -> (count, mean target, mean prediction, mae)`.
    pub by_stage: BTreeMap<usize, (usize, f64, f64, f64)>,
}

pub fn value_metrics(predictions: &[f32], targets: &[f32], stages: &[usize]) -> ValueMetrics {
    let n = targets.len().max(1) as f64;
    let mean_target = targets.iter().map(|value| *value as f64).sum::<f64>() / n;
    let mean_prediction = predictions.iter().map(|value| *value as f64).sum::<f64>() / n;
    let mut mse = 0.0;
    let mut mae = 0.0;
    let mut target_var = 0.0;
    let mut prediction_var = 0.0;
    let mut covariance = 0.0;
    let mut nonfinite = 0usize;
    let mut by_stage: BTreeMap<usize, (usize, f64, f64, f64)> = BTreeMap::new();
    for ((prediction, target), stage) in predictions.iter().zip(targets).zip(stages) {
        let prediction = *prediction as f64;
        let target = *target as f64;
        if !prediction.is_finite() {
            nonfinite += 1;
            continue;
        }
        let error = prediction - target;
        mse += error * error;
        mae += error.abs();
        target_var += (target - mean_target).powi(2);
        prediction_var += (prediction - mean_prediction).powi(2);
        covariance += (target - mean_target) * (prediction - mean_prediction);
        let entry = by_stage.entry(*stage).or_default();
        entry.0 += 1;
        entry.1 += target;
        entry.2 += prediction;
        entry.3 += error.abs();
    }
    for entry in by_stage.values_mut() {
        let count = entry.0.max(1) as f64;
        entry.1 /= count;
        entry.2 /= count;
        entry.3 /= count;
    }
    ValueMetrics {
        samples: targets.len(),
        mse: mse / n,
        mae: mae / n,
        mean_target,
        mean_prediction,
        target_std: (target_var / n).sqrt(),
        explained_variance: if target_var > 0.0 {
            1.0 - mse / target_var
        } else {
            0.0
        },
        correlation: if target_var > 0.0 && prediction_var > 0.0 {
            covariance / (target_var.sqrt() * prediction_var.sqrt())
        } else {
            0.0
        },
        nonfinite_predictions: nonfinite,
        by_stage,
    }
}

/// Critic training sample: decision encoding and its gamma=1 return-to-go
/// `reward_scale * (terminal - clear_rate_before)`.
pub struct ValueSample {
    pub encoded: EncodedDecision,
    pub target: f32,
    pub stage: usize,
}

pub fn value_samples(episodes: &[EpisodeRecord], reward_scale: f32) -> Vec<ValueSample> {
    let samples = episodes
        .iter()
        .flat_map(|episode| episode.samples.iter())
        .collect::<Vec<_>>();
    samples
        .par_iter()
        .map(|sample| ValueSample {
            encoded: encode_decision(
                &sample.observation,
                &sample.policy_candidates(),
                sample.legal_mask.clone(),
            ),
            target: reward_scale * (sample.final_terminal_clear_rate - sample.clear_rate_before),
            stage: sample.stage,
        })
        .collect()
}

pub fn evaluate_critic(
    critic: &DeepSetsActorCritic<InferenceBackend>,
    samples: &[ValueSample],
    device: &PolicyDevice,
) -> Result<ValueMetrics> {
    let decisions = samples
        .iter()
        .map(|sample| &sample.encoded)
        .collect::<Vec<_>>();
    let predictions = critic_value_vector(critic, &decisions, device)?;
    Ok(value_metrics(
        &predictions,
        &samples
            .iter()
            .map(|sample| sample.target)
            .collect::<Vec<_>>(),
        &samples
            .iter()
            .map(|sample| sample.stage)
            .collect::<Vec<_>>(),
    ))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CriticEpochRecord {
    pub epoch: usize,
    pub mean_train_batch_loss: f64,
    pub validation: ValueMetrics,
    pub seconds: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CriticCheckpointMetadata {
    pub schema_version: u32,
    pub candidate_encoder_version: u32,
    pub policy_candidate_set_version: u32,
    pub game_rules_epoch: u32,
    pub git_commit: String,
    pub model_config: ModelConfig,
    pub config: CriticPretrainConfig,
    pub train_dataset: String,
    pub validation_dataset: String,
    pub train_provenance: Option<DatasetProvenance>,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub initial_validation: ValueMetrics,
    pub history: Vec<CriticEpochRecord>,
    pub selected_epoch: usize,
}

/// Supervised critic pretraining on canonical trajectories. Writes
/// `critic.bin` (selected = lowest validation MSE) and `critic.json`.
pub fn pretrain_critic(
    run_dir: &Path,
    train: &[ValueSample],
    validation: &[ValueSample],
    mut metadata: CriticCheckpointMetadata,
) -> Result<CriticCheckpointMetadata> {
    let config = metadata.config.clone();
    if train.is_empty() || validation.is_empty() {
        bail!("critic pretraining needs train and validation samples");
    }
    std::fs::create_dir_all(run_dir)?;
    let device = default_policy_device();
    let mut critic =
        super::semantic_bc::seeded_materialized_model(metadata.model_config, config.seed, &device)?;
    let mut optimizer: ActorCriticOptimizer = AdamConfig::new().init();
    metadata.initial_validation = evaluate_critic(&critic.clone().valid(), validation, &device)?;
    eprintln!(
        "critic epoch 0: validation mse {:.4} mae {:.4} ev {:.4}",
        metadata.initial_validation.mse,
        metadata.initial_validation.mae,
        metadata.initial_validation.explained_variance
    );
    let mut best_mse = f64::INFINITY;
    for epoch in 1..=config.epochs {
        let started = Instant::now();
        let mut order = (0..train.len()).collect::<Vec<_>>();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(
            config.seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ epoch as u64,
        );
        order.shuffle(&mut rng);
        let mut loss_sum = 0.0f64;
        let mut batches = 0usize;
        for chunk in order.chunks(config.batch_size) {
            let decisions = chunk
                .iter()
                .map(|index| &train[*index].encoded)
                .collect::<Vec<_>>();
            let targets = Tensor::<TrainBackend, 2>::from_data(
                TensorData::new(
                    chunk
                        .iter()
                        .map(|index| train[*index].target)
                        .collect::<Vec<_>>(),
                    [chunk.len(), 1],
                ),
                &device,
            );
            let loss = (critic_values(&critic, &decisions, &device) - targets)
                .powf_scalar(2.0)
                .mean();
            let value = loss.clone().into_data().to_vec::<f32>()?[0];
            if !value.is_finite() {
                bail!("critic pretraining produced a non-finite loss at epoch {epoch}");
            }
            loss_sum += value as f64;
            batches += 1;
            let gradients = GradientsParams::from_grads(loss.backward(), &critic);
            critic = optimizer.step(config.learning_rate, critic, gradients);
        }
        let inference = critic.clone().valid();
        let validation_metrics = evaluate_critic(&inference, validation, &device)?;
        let record = CriticEpochRecord {
            epoch,
            mean_train_batch_loss: loss_sum / batches.max(1) as f64,
            validation: validation_metrics,
            seconds: started.elapsed().as_secs_f64(),
        };
        eprintln!(
            "critic epoch {epoch}: train mse {:.4}, validation mse {:.4} mae {:.4} ev {:.4} corr {:.4} ({:.1}s)",
            record.mean_train_batch_loss,
            record.validation.mse,
            record.validation.mae,
            record.validation.explained_variance,
            record.validation.correlation,
            record.seconds
        );
        if record.validation.mse < best_mse {
            best_mse = record.validation.mse;
            metadata.selected_epoch = epoch;
            std::fs::write(
                run_dir.join("critic.bin"),
                model_to_full_precision_bytes(inference)?,
            )?;
        }
        metadata.history.push(record);
        std::fs::write(
            run_dir.join("critic.json"),
            serde_json::to_vec_pretty(&metadata)?,
        )?;
    }
    Ok(metadata)
}

pub fn load_critic(
    run_dir: &Path,
    device: &PolicyDevice,
) -> Result<(CriticCheckpointMetadata, DeepSetsActorCritic<TrainBackend>)> {
    let metadata: CriticCheckpointMetadata = serde_json::from_slice(
        &std::fs::read(run_dir.join("critic.json"))
            .with_context(|| format!("read {}", run_dir.join("critic.json").display()))?,
    )?;
    if metadata.schema_version != CRITIC_CHECKPOINT_SCHEMA_VERSION
        || metadata.candidate_encoder_version != SEMANTIC_CANDIDATE_ENCODER_VERSION
        || metadata.game_rules_epoch != GAME_RULES_EPOCH
    {
        bail!("{}: incompatible critic checkpoint", run_dir.display());
    }
    let critic = load_model_file::<TrainBackend>(
        metadata.model_config,
        &run_dir.join("critic.bin"),
        device,
    )?;
    Ok((metadata, critic))
}

// --- training run -------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RolloutStats {
    pub episodes: usize,
    pub transitions: usize,
    pub mean_terminal_clear_rate: f64,
    pub median_terminal_clear_rate: f64,
    pub mean_final_stage: f64,
    pub victories: usize,
    pub mean_decisions: f64,
    pub truncated_episodes: usize,
    pub illegal_actions: usize,
    pub action_mismatches: usize,
    pub fallback_actions: usize,
    pub nonfinite_values: usize,
    pub mean_behavior_entropy: f64,
    pub action_kind_counts: BTreeMap<String, usize>,
    pub action_kind_fractions: BTreeMap<String, f64>,
    pub mean_return: f64,
    pub mean_value: f64,
    pub advantage_mean: f64,
    pub advantage_std: f64,
    pub explained_variance: f64,
    pub max_telescoping_error: f64,
    /// Sampled action != the behavior policy's greedy action.
    pub non_greedy_fraction: f64,
    /// Sampled action != the canonical scripted action.
    pub non_canonical_fraction: f64,
    /// Sampled action != the BC initialization's greedy action.
    pub non_init_greedy_fraction: f64,
    pub by_decision_point: BTreeMap<String, DecisionPointStats>,
    pub timing: RolloutTiming,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DecisionPointStats {
    pub count: usize,
    pub mean_entropy: f64,
    pub non_greedy_fraction: f64,
    pub non_canonical_fraction: f64,
    pub non_init_greedy_fraction: f64,
}

/// CPU seconds summed over episodes for the parallel rollout parts, and wall
/// seconds for the sequential post-processing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RolloutTiming {
    pub wall_seconds: f64,
    pub candidate_encoding_cpu_seconds: f64,
    pub actor_forward_cpu_seconds: f64,
    pub environment_cpu_seconds: f64,
    pub critic_value_seconds: f64,
    pub init_forward_seconds: f64,
    pub gae_seconds: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevEvaluation {
    pub iteration: usize,
    pub split: String,
    pub seeds: usize,
    pub summaries: Vec<PolicySummary>,
    pub comparisons: Vec<PairedComparisonSummary>,
    pub wall_seconds: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedComparisonSummary {
    pub policy: String,
    pub reference: String,
    pub mean: f64,
    pub se: f64,
    pub median: f64,
    pub better: usize,
    pub worse: usize,
    pub tie: usize,
}

impl From<&PairedComparison> for PairedComparisonSummary {
    fn from(value: &PairedComparison) -> Self {
        Self {
            policy: value.policy.clone(),
            reference: value.reference.clone(),
            mean: value.mean,
            se: value.se,
            median: value.median,
            better: value.better,
            worse: value.worse,
            tie: value.tie,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IterationRecord {
    pub iteration: usize,
    pub train_seeds: (u64, u64),
    pub actor_updated: bool,
    pub rollout: RolloutStats,
    pub update: UpdateStats,
    /// Mean `KL(pi_new || pi_init)` over this iteration's states after the
    /// update.
    pub kl_to_init_after: f64,
    /// Mean `KL(pi_old || pi_new)` over this iteration's states.
    pub kl_old_new_after: f64,
    pub rollout_seconds: f64,
    pub update_seconds: f64,
    pub evaluation: Option<DevEvaluation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PpoRunMetadata {
    pub schema_version: u32,
    pub policy_candidate_set_version: u32,
    pub candidate_encoder_version: u32,
    pub game_rules_epoch: u32,
    pub git_commit: String,
    pub model_config: ModelConfig,
    pub config: PpoConfig,
    pub init_bc_run: String,
    pub init_bc_metadata: BcCheckpointMetadata,
    pub init_critic_run: Option<String>,
    #[serde(default)]
    pub init_ppo_iteration: Option<String>,
    pub train_split: Phase4Split,
    pub development_split: Phase4Split,
    pub development_seeds: usize,
    pub evaluate_every: usize,
    pub completed_iterations: usize,
    pub history: Vec<IterationRecord>,
}

fn iteration_dir(run_dir: &Path, iteration: usize) -> PathBuf {
    run_dir.join(format!("iter-{iteration:04}"))
}

fn write_checkpoint(
    run_dir: &Path,
    iteration: usize,
    learner: &PpoLearner,
    metadata: &PpoRunMetadata,
) -> Result<()> {
    let directory = iteration_dir(run_dir, iteration);
    std::fs::create_dir_all(&directory)?;
    std::fs::write(
        directory.join("actor.bin"),
        model_to_full_precision_bytes(learner.actor_inference())?,
    )?;
    std::fs::write(
        directory.join("critic.bin"),
        model_to_full_precision_bytes(learner.critic_inference())?,
    )?;
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    recorder.record(
        learner.actor_optimizer.to_record(),
        directory.join("actor-optimizer.bin"),
    )?;
    recorder.record(
        learner.critic_optimizer.to_record(),
        directory.join("critic-optimizer.bin"),
    )?;
    std::fs::write(
        directory.join("ppo-actor.json"),
        serde_json::to_vec_pretty(&PpoActorFile {
            schema_version: SEMANTIC_PPO_CHECKPOINT_SCHEMA_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            game_rules_epoch: GAME_RULES_EPOCH,
            model_config: metadata.model_config,
            iteration,
        })?,
    )?;
    let temporary = run_dir.join("ppo.json.tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(metadata)?)?;
    std::fs::rename(&temporary, run_dir.join("ppo.json"))?;
    Ok(())
}

/// Identifies a PPO iteration directory's `actor.bin` for policy loading.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PpoActorFile {
    pub schema_version: u32,
    pub policy_candidate_set_version: u32,
    pub candidate_encoder_version: u32,
    pub game_rules_epoch: u32,
    pub model_config: ModelConfig,
    pub iteration: usize,
}

pub fn load_ppo_actor(
    iteration_dir: &Path,
    device: &PolicyDevice,
) -> Result<DeepSetsActorCritic<InferenceBackend>> {
    let file: PpoActorFile = serde_json::from_slice(
        &std::fs::read(iteration_dir.join("ppo-actor.json"))
            .with_context(|| format!("read {}", iteration_dir.join("ppo-actor.json").display()))?,
    )?;
    if file.schema_version != SEMANTIC_PPO_CHECKPOINT_SCHEMA_VERSION
        || file.policy_candidate_set_version != POLICY_CANDIDATE_SET_VERSION
        || file.candidate_encoder_version != SEMANTIC_CANDIDATE_ENCODER_VERSION
        || file.game_rules_epoch != GAME_RULES_EPOCH
    {
        bail!(
            "{}: incompatible PPO actor checkpoint",
            iteration_dir.display()
        );
    }
    load_model_file::<InferenceBackend>(file.model_config, &iteration_dir.join("actor.bin"), device)
}

fn load_learner(
    run_dir: &Path,
    iteration: usize,
    config: &PpoConfig,
    model_config: ModelConfig,
) -> Result<PpoLearner> {
    let device = default_policy_device();
    let directory = iteration_dir(run_dir, iteration);
    let actor =
        load_model_file::<TrainBackend>(model_config, &directory.join("actor.bin"), &device)?;
    let critic =
        load_model_file::<TrainBackend>(model_config, &directory.join("critic.bin"), &device)?;
    let mut learner = PpoLearner::new(actor, critic, config);
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    learner.actor_optimizer = learner
        .actor_optimizer
        .load_record(recorder.load(directory.join("actor-optimizer.bin"), &device)?);
    learner.critic_optimizer = learner
        .critic_optimizer
        .load_record(recorder.load(directory.join("critic-optimizer.bin"), &device)?);
    Ok(learner)
}

pub struct PpoRunInput {
    pub config: PpoConfig,
    pub init_bc_run: PathBuf,
    pub init_critic_run: Option<PathBuf>,
    /// Continue from a PPO iteration directory's actor and critic (fresh
    /// optimizers). The KL reference stays the BC initialization.
    pub init_ppo_iteration: Option<PathBuf>,
    pub iterations: usize,
    pub evaluate_every: usize,
    pub development_seeds: usize,
}

fn train_seed_range(config: &PpoConfig, iteration: usize) -> Result<Vec<u64>> {
    let range = Phase4Split::PpoTrain.range();
    let block = (iteration + config.train_seed_block_offset) as u64;
    let start = *range.start() + block * config.episodes_per_iteration as u64;
    let end = start + config.episodes_per_iteration as u64 - 1;
    if end > *range.end() {
        bail!("PPO training seeds exhausted at iteration {iteration}");
    }
    Ok((start..=end).collect())
}

fn sample_seed(config: &PpoConfig, iteration: usize, game_seed: u64) -> u64 {
    let digest = td_core::derive_seed(
        config.seed,
        td_core::domain::ML_TOWER_TEACHER_SCENARIO,
        &[0x5050_4f00, iteration as u64, game_seed],
    );
    u64::from_le_bytes(digest[..8].try_into().expect("eight bytes"))
}

/// Collects one iteration of rollouts, fills values/advantages/returns.
pub fn collect_iteration(
    learner: &PpoLearner,
    init_actor: &DeepSetsActorCritic<InferenceBackend>,
    config: &PpoConfig,
    game_config: Arc<GameConfig>,
    iteration: usize,
    seeds: &[u64],
) -> Result<(Vec<EpisodeRollout>, RolloutStats)> {
    let wall_started = Instant::now();
    let actor = learner.actor_inference();
    let critic = learner.critic_inference();
    let device = learner.device;
    let mut episodes = seeds
        .par_iter()
        .map(|seed| {
            rollout_episode(
                &actor,
                &device,
                Arc::clone(&game_config),
                *seed,
                sample_seed(config, iteration, *seed),
                config.reward_scale,
                false,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    let mut stats = RolloutStats::default();
    for episode in &episodes {
        stats.timing.candidate_encoding_cpu_seconds += episode.decision_seconds;
        stats.timing.actor_forward_cpu_seconds += episode.forward_seconds;
        stats.timing.environment_cpu_seconds += episode.environment_seconds;
    }
    let started = Instant::now();
    {
        let decisions = episodes
            .iter()
            .flat_map(|episode| episode.transitions.iter().map(|step| &step.encoded))
            .collect::<Vec<_>>();
        let mut init_log_probs =
            actor_log_prob_vectors(init_actor, &decisions, &device)?.into_iter();
        for episode in &mut episodes {
            for step in &mut episode.transitions {
                step.init_log_probs = init_log_probs.next().expect("aligned init log-probs");
            }
        }
    }
    stats.timing.init_forward_seconds = started.elapsed().as_secs_f64();
    let mut returns_all = Vec::new();
    let mut values_all = Vec::new();
    let mut advantages_all = Vec::new();
    for episode in &mut episodes {
        let decisions = episode
            .transitions
            .iter()
            .map(|step| &step.encoded)
            .collect::<Vec<_>>();
        let started = Instant::now();
        let values = critic_value_vector(&critic, &decisions, &device)?;
        let bootstrap_value = match &episode.bootstrap {
            Some(encoded) => critic_value_vector(&critic, &[encoded], &device)?[0],
            None => 0.0,
        };
        stats.timing.critic_value_seconds += started.elapsed().as_secs_f64();
        stats.nonfinite_values += values.iter().filter(|value| !value.is_finite()).count();
        let rewards = episode
            .transitions
            .iter()
            .map(|step| step.reward)
            .collect::<Vec<_>>();
        let started = Instant::now();
        let (advantages, returns) = compute_gae(
            &rewards,
            &values,
            bootstrap_value,
            config.gamma,
            config.gae_lambda,
        );
        stats.timing.gae_seconds += started.elapsed().as_secs_f64();
        let raw_sum = episode
            .transitions
            .iter()
            .map(|step| step.raw_delta as f64)
            .sum::<f64>();
        stats.max_telescoping_error = stats.max_telescoping_error.max(
            (raw_sum - (episode.terminal_clear_rate - episode.initial_clear_rate) as f64).abs(),
        );
        for (index, step) in episode.transitions.iter_mut().enumerate() {
            step.value = values[index];
            step.advantage = advantages[index];
            step.return_value = returns[index];
            *stats
                .action_kind_counts
                .entry(step.action_kind.clone())
                .or_insert(0) += 1;
            stats.mean_behavior_entropy += step.behavior_entropy as f64;
            let non_greedy = step.action_index != step.greedy_index;
            let non_canonical = step.canonical_index != Some(step.action_index);
            let non_init_greedy = step.init_greedy_index() != Some(step.action_index);
            stats.non_greedy_fraction += non_greedy as u8 as f64;
            stats.non_canonical_fraction += non_canonical as u8 as f64;
            stats.non_init_greedy_fraction += non_init_greedy as u8 as f64;
            let point = stats
                .by_decision_point
                .entry(step.decision_point.clone())
                .or_default();
            point.count += 1;
            point.mean_entropy += step.behavior_entropy as f64;
            point.non_greedy_fraction += non_greedy as u8 as f64;
            point.non_canonical_fraction += non_canonical as u8 as f64;
            point.non_init_greedy_fraction += non_init_greedy as u8 as f64;
        }
        returns_all.extend(returns);
        values_all.extend(values);
        advantages_all.extend(advantages);
        stats.illegal_actions += episode.illegal_actions;
        stats.action_mismatches += episode.action_mismatches;
        stats.truncated_episodes += episode.truncated as usize;
    }
    let transitions = returns_all.len();
    stats.episodes = episodes.len();
    stats.transitions = transitions;
    let count = episodes.len().max(1) as f64;
    let mut terminal = episodes
        .iter()
        .map(|episode| episode.terminal_clear_rate as f64)
        .collect::<Vec<_>>();
    stats.mean_terminal_clear_rate = terminal.iter().sum::<f64>() / count;
    terminal.sort_by(f64::total_cmp);
    stats.median_terminal_clear_rate = if terminal.is_empty() {
        0.0
    } else if terminal.len() % 2 == 0 {
        (terminal[terminal.len() / 2 - 1] + terminal[terminal.len() / 2]) / 2.0
    } else {
        terminal[terminal.len() / 2]
    };
    stats.mean_final_stage = episodes
        .iter()
        .map(|episode| episode.final_stage as f64)
        .sum::<f64>()
        / count;
    stats.victories = episodes.iter().filter(|episode| episode.victory).count();
    stats.mean_decisions = transitions as f64 / count;
    let n = transitions.max(1) as f64;
    stats.mean_behavior_entropy /= n;
    stats.non_greedy_fraction /= n;
    stats.non_canonical_fraction /= n;
    stats.non_init_greedy_fraction /= n;
    for point in stats.by_decision_point.values_mut() {
        let count = point.count.max(1) as f64;
        point.mean_entropy /= count;
        point.non_greedy_fraction /= count;
        point.non_canonical_fraction /= count;
        point.non_init_greedy_fraction /= count;
    }
    for kind in TRACKED_ACTION_KINDS {
        stats
            .action_kind_counts
            .entry(kind.to_string())
            .or_insert(0);
    }
    stats.action_kind_fractions = stats
        .action_kind_counts
        .iter()
        .map(|(kind, count)| (kind.clone(), *count as f64 / n))
        .collect();
    stats.mean_return = returns_all.iter().map(|value| *value as f64).sum::<f64>() / n;
    stats.mean_value = values_all.iter().map(|value| *value as f64).sum::<f64>() / n;
    stats.advantage_mean = advantages_all
        .iter()
        .map(|value| *value as f64)
        .sum::<f64>()
        / n;
    stats.advantage_std = (advantages_all
        .iter()
        .map(|value| (*value as f64 - stats.advantage_mean).powi(2))
        .sum::<f64>()
        / n)
        .sqrt();
    let return_mean = stats.mean_return;
    let return_var = returns_all
        .iter()
        .map(|value| (*value as f64 - return_mean).powi(2))
        .sum::<f64>();
    let residual = returns_all
        .iter()
        .zip(&values_all)
        .map(|(ret, value)| (*ret as f64 - *value as f64).powi(2))
        .sum::<f64>();
    stats.explained_variance = if return_var > 0.0 {
        1.0 - residual / return_var
    } else {
        0.0
    };
    stats.timing.wall_seconds = wall_started.elapsed().as_secs_f64();
    Ok((episodes, stats))
}

fn mean_kl(left: &[Vec<f32>], right: &[Vec<f32>], masks: &[&[bool]]) -> f64 {
    let mut total = 0.0f64;
    for ((left, right), mask) in left.iter().zip(right).zip(masks) {
        total += left
            .iter()
            .zip(right)
            .zip(mask.iter())
            .filter(|(_, legal)| **legal)
            .map(|((left, right), _)| {
                let probability = (*left as f64).exp();
                if probability > 0.0 {
                    probability * (*left as f64 - *right as f64)
                } else {
                    0.0
                }
            })
            .sum::<f64>();
    }
    total / left.len().max(1) as f64
}

pub fn development_evaluation(
    game_config: Arc<GameConfig>,
    split: Phase4Split,
    seed_count: usize,
    iteration: usize,
    init: &SemanticPolicy,
    current: DeepSetsActorCritic<InferenceBackend>,
) -> Result<DevEvaluation> {
    let started = Instant::now();
    let seeds = split.seeds(Some(seed_count))?;
    let policies = vec![
        EvalPolicy::Canonical,
        EvalPolicy::Learned {
            name: "bc_init".to_string(),
            policy: Box::new(init.clone()),
        },
        EvalPolicy::Learned {
            name: "ppo".to_string(),
            policy: Box::new(SemanticPolicy::new(current)),
        },
    ];
    let comparisons = vec![
        ("ppo".to_string(), "canonical".to_string()),
        ("ppo".to_string(), "bc_init".to_string()),
        ("bc_init".to_string(), "canonical".to_string()),
    ];
    let report = evaluate_policies(game_config, split.name(), &seeds, &policies, &comparisons)?;
    Ok(DevEvaluation {
        iteration,
        split: split.name().to_string(),
        seeds: seeds.len(),
        summaries: report.summaries,
        comparisons: report
            .comparisons
            .iter()
            .map(PairedComparisonSummary::from)
            .collect(),
        wall_seconds: started.elapsed().as_secs_f64(),
    })
}

/// Trains (or resumes) a PPO run. Iteration `i` checkpoints live in
/// `iter-{i:04}`; `iter-0000` is the BC-initialized actor and the
/// initial critic before any update.
pub fn train_ppo_run(
    game_config: Arc<GameConfig>,
    run_dir: &Path,
    input: PpoRunInput,
) -> Result<PpoRunMetadata> {
    let config = input.config.clone();
    if !(config.gamma > 0.0 && config.gamma <= 1.0)
        || config.episodes_per_iteration == 0
        || config.minibatch_size == 0
    {
        bail!("invalid PPO config");
    }
    std::fs::create_dir_all(run_dir)?;
    let device = default_policy_device();
    let (bc_metadata, init_actor) =
        load_selected_model::<TrainBackend>(&input.init_bc_run, &device)?;
    for provenance in &bc_metadata.train_provenance {
        if provenance.game_rules_epoch != GAME_RULES_EPOCH {
            bail!("BC initialization was trained under a different game rules epoch");
        }
    }
    let init_policy = SemanticPolicy::new(init_actor.clone().valid());
    let model_config = bc_metadata.model_config;
    let (mut metadata, mut learner) = if run_dir.join("ppo.json").exists() {
        let metadata: PpoRunMetadata =
            serde_json::from_slice(&std::fs::read(run_dir.join("ppo.json"))?)?;
        if metadata.config != config
            || metadata.init_bc_run != input.init_bc_run.display().to_string()
        {
            bail!(
                "{}: existing PPO run has a different configuration",
                run_dir.display()
            );
        }
        let learner = load_learner(
            run_dir,
            metadata.completed_iterations,
            &config,
            model_config,
        )?;
        eprintln!(
            "resuming {} after iteration {}",
            run_dir.display(),
            metadata.completed_iterations
        );
        (metadata, learner)
    } else {
        let (actor, critic) = match &input.init_ppo_iteration {
            Some(directory) => (
                load_model_file::<TrainBackend>(
                    model_config,
                    &directory.join("actor.bin"),
                    &device,
                )?,
                Some(load_model_file::<TrainBackend>(
                    model_config,
                    &directory.join("critic.bin"),
                    &device,
                )?),
            ),
            None => (init_actor.clone(), None),
        };
        let critic = match (critic, &input.init_critic_run) {
            (Some(critic), _) => critic,
            (None, Some(path)) => {
                let (critic_metadata, critic) = load_critic(path, &device)?;
                if critic_metadata.model_config != model_config {
                    bail!("critic model config differs from the actor's");
                }
                critic
            }
            (None, None) => {
                super::semantic_bc::seeded_materialized_model(model_config, config.seed, &device)?
            }
        };
        let learner = PpoLearner::new(actor, critic, &config);
        let mut metadata = PpoRunMetadata {
            schema_version: SEMANTIC_PPO_CHECKPOINT_SCHEMA_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            game_rules_epoch: GAME_RULES_EPOCH,
            git_commit: current_git_revision()?,
            model_config,
            config: config.clone(),
            init_bc_run: input.init_bc_run.display().to_string(),
            init_bc_metadata: bc_metadata.clone(),
            init_critic_run: input
                .init_critic_run
                .as_ref()
                .map(|path| path.display().to_string()),
            init_ppo_iteration: input
                .init_ppo_iteration
                .as_ref()
                .map(|path| path.display().to_string()),
            train_split: Phase4Split::PpoTrain,
            development_split: Phase4Split::PpoDevelopment,
            development_seeds: input.development_seeds,
            evaluate_every: input.evaluate_every,
            completed_iterations: 0,
            history: Vec::new(),
        };
        if input.evaluate_every > 0 {
            let evaluation = development_evaluation(
                Arc::clone(&game_config),
                Phase4Split::PpoDevelopment,
                input.development_seeds,
                0,
                &init_policy,
                learner.actor_inference(),
            )?;
            log_evaluation(&evaluation);
            metadata.history.push(IterationRecord {
                iteration: 0,
                train_seeds: (0, 0),
                actor_updated: false,
                rollout: RolloutStats::default(),
                update: UpdateStats::default(),
                kl_to_init_after: 0.0,
                kl_old_new_after: 0.0,
                rollout_seconds: 0.0,
                update_seconds: 0.0,
                evaluation: Some(evaluation),
            });
        }
        write_checkpoint(run_dir, 0, &learner, &metadata)?;
        (metadata, learner)
    };
    let init_inference = init_actor.valid();
    for iteration in metadata.completed_iterations + 1..=input.iterations {
        let seeds = train_seed_range(&config, iteration - 1)?;
        let rollout_started = Instant::now();
        let (episodes, rollout_stats) = collect_iteration(
            &learner,
            &init_inference,
            &config,
            Arc::clone(&game_config),
            iteration,
            &seeds,
        )?;
        let rollout_seconds = rollout_started.elapsed().as_secs_f64();
        let transitions = episodes
            .into_iter()
            .flat_map(|episode| episode.transitions)
            .collect::<Vec<_>>();
        let decisions = transitions
            .iter()
            .map(|step| &step.encoded)
            .collect::<Vec<_>>();
        let old_log_probs =
            actor_log_prob_vectors(&learner.actor_inference(), &decisions, &device)?;
        let init_log_probs = transitions
            .iter()
            .map(|step| step.init_log_probs.clone())
            .collect::<Vec<_>>();
        let update_started = Instant::now();
        let update_actor = iteration > config.critic_warmup_iterations;
        let update = learner.update(&transitions, &config, iteration, update_actor)?;
        let update_seconds = update_started.elapsed().as_secs_f64();
        let decisions = transitions
            .iter()
            .map(|step| &step.encoded)
            .collect::<Vec<_>>();
        let masks = transitions
            .iter()
            .map(|step| step.encoded.legal_mask.as_slice())
            .collect::<Vec<_>>();
        let new_log_probs =
            actor_log_prob_vectors(&learner.actor_inference(), &decisions, &device)?;
        let kl_to_init_after = mean_kl(&new_log_probs, &init_log_probs, &masks);
        let kl_old_new_after = mean_kl(&old_log_probs, &new_log_probs, &masks);
        let evaluation = if input.evaluate_every > 0 && iteration % input.evaluate_every == 0 {
            let evaluation = development_evaluation(
                Arc::clone(&game_config),
                Phase4Split::PpoDevelopment,
                input.development_seeds,
                iteration,
                &init_policy,
                learner.actor_inference(),
            )?;
            log_evaluation(&evaluation);
            Some(evaluation)
        } else {
            None
        };
        let record = IterationRecord {
            iteration,
            train_seeds: (seeds[0], *seeds.last().expect("non-empty seeds")),
            actor_updated: update_actor,
            rollout: rollout_stats,
            update,
            kl_to_init_after,
            kl_old_new_after,
            rollout_seconds,
            update_seconds,
            evaluation,
        };
        log_iteration(&record);
        metadata.history.push(record);
        metadata.completed_iterations = iteration;
        write_checkpoint(run_dir, iteration, &learner, &metadata)?;
    }
    Ok(metadata)
}

fn log_iteration(record: &IterationRecord) {
    let rollout = &record.rollout;
    let update = &record.update;
    let fraction = |kind: &str| {
        rollout
            .action_kind_fractions
            .get(kind)
            .copied()
            .unwrap_or(0.0)
    };
    eprintln!(
        "iter {}: train clear {:.2} (median {:.2}, stage {:.2}, dec {:.1}) ent {:.4} | pl {:+.4} vl {:.4} ent {:.4} kl {:.5} kl_init {:.5} clip {:.3} gn {:.3}/{:.3} ev {:.3} epochs {}{} | illegal {} mismatch {} trunc {} | reroll {:.3} build {:.3} place {:.3} remove {:.3} cont {:.3} item {:.3} shop {:.3} treas {:.3} card {:.3} | {:.1}s+{:.1}s",
        record.iteration,
        rollout.mean_terminal_clear_rate,
        rollout.median_terminal_clear_rate,
        rollout.mean_final_stage,
        rollout.mean_decisions,
        rollout.mean_behavior_entropy,
        update.policy_loss,
        update.value_loss,
        update.entropy,
        update.approx_kl,
        record.kl_to_init_after,
        update.clip_fraction,
        update.mean_actor_grad_norm,
        update.mean_critic_grad_norm,
        rollout.explained_variance,
        update.epochs_completed,
        if update.early_stopped {
            " (kl stop)"
        } else {
            ""
        },
        rollout.illegal_actions,
        rollout.action_mismatches,
        rollout.truncated_episodes,
        fraction("reroll"),
        fraction("build_tower"),
        fraction("place_tower"),
        fraction("remove_tower"),
        fraction("continue"),
        fraction("use_inventory_item"),
        fraction("purchase_shop_item"),
        fraction("select_treasure") + fraction("discard_treasure"),
        fraction("select_card_service_card") + fraction("confirm_card_service_selection"),
        record.rollout_seconds,
        record.update_seconds,
    );
    eprintln!(
        "  value loss first {:.4} last {:.4} max {:.4} | critic grad max {:.1} actor grad max {:.3}",
        update.first_value_loss,
        update.last_value_loss,
        update.max_value_loss,
        update.max_critic_grad_norm,
        update.max_actor_grad_norm,
    );
    let timing = &rollout.timing;
    eprintln!(
        "  explore: non-greedy {:.4} non-canonical {:.4} non-bc-init {:.4} | timing cpu-s: candidates {:.1} forward {:.1} env {:.1} | wall-s: rollout {:.1} init-forward {:.1} critic {:.1} gae {:.3}",
        rollout.non_greedy_fraction,
        rollout.non_canonical_fraction,
        rollout.non_init_greedy_fraction,
        timing.candidate_encoding_cpu_seconds,
        timing.actor_forward_cpu_seconds,
        timing.environment_cpu_seconds,
        timing.wall_seconds,
        timing.init_forward_seconds,
        timing.critic_value_seconds,
        timing.gae_seconds,
    );
    for (point, stats) in &rollout.by_decision_point {
        eprintln!(
            "  {point}: n {} ent {:.4} non-greedy {:.4} non-canonical {:.4} non-bc-init {:.4}",
            stats.count,
            stats.mean_entropy,
            stats.non_greedy_fraction,
            stats.non_canonical_fraction,
            stats.non_init_greedy_fraction,
        );
    }
}

fn log_evaluation(evaluation: &DevEvaluation) {
    for summary in &evaluation.summaries {
        eprintln!(
            "  eval iter {} {}: mean {:.2} median {:.2} stage {:.2} victories {} decisions {:.1} illegal {} fallback {} mutations {}",
            evaluation.iteration,
            summary.policy,
            summary.mean_terminal_clear_rate,
            summary.median_terminal_clear_rate,
            summary.mean_final_stage,
            summary.victories,
            summary.mean_decisions,
            summary.illegal_actions,
            summary.fallback_actions,
            summary.post_sampling_mutations,
        );
    }
    for comparison in &evaluation.comparisons {
        eprintln!(
            "  eval iter {} {} - {}: {:+.2} (SE {:.2}) better/worse/tie {}/{}/{}",
            evaluation.iteration,
            comparison.policy,
            comparison.reference,
            comparison.mean,
            comparison.se,
            comparison.better,
            comparison.worse,
            comparison.tie
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::phase4_dataset::{SourcePolicy, collect_canonical_episode};
    use crate::ml::phase4_eval::run_policy_episode;
    use crate::ml::semantic_bc::{
        BcTrainConfig, BcTrainInput, LabelSource, SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION,
        new_materialized_model, prepare_samples, train_bc_run,
    };

    fn game_config() -> Arc<GameConfig> {
        Arc::new(GameConfig::default_config())
    }

    fn canonical_episodes(seeds: &[u64]) -> Vec<EpisodeRecord> {
        let config = game_config();
        let provenance = DatasetProvenance::new(
            &config,
            SourcePolicy::Canonical,
            Phase4Split::Phase4bCanonicalTrain,
            None,
        )
        .unwrap();
        seeds
            .iter()
            .map(|seed| {
                collect_canonical_episode(Arc::clone(&config), provenance.clone(), *seed).unwrap()
            })
            .collect()
    }

    fn temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        path
    }

    fn random_actor() -> DeepSetsActorCritic<InferenceBackend> {
        let device = default_policy_device();
        crate::ml::semantic_bc::seeded_materialized_model(ModelConfig::default(), 7, &device)
            .unwrap()
            .valid()
    }

    /// A tiny BC run directory usable as a PPO initialization.
    fn tiny_bc_run(name: &str) -> PathBuf {
        let episodes = canonical_episodes(&[0]);
        let mut samples = prepare_samples(&episodes, LabelSource::Canonical, 1.0);
        samples.truncate(64);
        let config = BcTrainConfig {
            epochs: 1,
            batch_size: 32,
            ..BcTrainConfig::default()
        };
        let run_dir = temp_dir(name);
        train_bc_run(
            &run_dir,
            BcTrainInput {
                train: &samples,
                validation: &[],
                metadata: BcCheckpointMetadata {
                    schema_version: SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION,
                    policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
                    candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
                    git_commit: "test".to_string(),
                    model_config: ModelConfig::default(),
                    config,
                    train_datasets: vec!["tiny".to_string()],
                    train_provenance: Vec::new(),
                    validation_dataset: None,
                    train_samples: samples.len(),
                    validation_samples: 0,
                    init_checkpoint: None,
                    completed_epochs: 0,
                    best_epoch: None,
                    history: Vec::new(),
                },
                init_model: None,
            },
        )
        .unwrap();
        run_dir
    }

    #[test]
    fn masked_candidate_is_never_sampled() {
        let log_probs = [-0.01f32, -5.0, -6.0, -1.0e9];
        let mask = [false, true, true, false];
        for step in 0..1000 {
            let index = sample_masked(&log_probs, &mask, step as f64 / 1000.0);
            assert!(mask[index], "sampled masked candidate {index}");
        }
    }

    #[test]
    fn gae_with_gamma_one_and_lambda_one_is_the_undiscounted_return() {
        let rewards = [0.0, 0.5, 0.0, 1.5];
        let values = [0.3, -0.2, 0.7, 0.1];
        let (advantages, returns) = compute_gae(&rewards, &values, 0.0, 1.0, 1.0);
        assert_eq!(returns, vec![2.0, 2.0, 1.5, 1.5]);
        for index in 0..4 {
            assert!((advantages[index] - (returns[index] - values[index])).abs() < 1e-6);
        }
        let (_, truncated) = compute_gae(&[1.0], &[0.0], 4.0, 1.0, 0.95);
        assert_eq!(truncated, vec![5.0]);
        let (advantages, _) = compute_gae(&[0.0, 1.0], &[0.0, 0.0], 0.0, 1.0, 0.5);
        assert_eq!(advantages, vec![0.5, 1.0]);
    }

    #[test]
    fn stochastic_rollout_executes_the_sampled_action_and_telescopes() {
        let actor = random_actor();
        let device = default_policy_device();
        let rollout = rollout_episode(&actor, &device, game_config(), 3, 99, 0.1, false).unwrap();
        assert!(!rollout.truncated);
        assert_eq!(rollout.illegal_actions, 0);
        assert_eq!(rollout.action_mismatches, 0);
        let reward_sum = rollout
            .transitions
            .iter()
            .map(|step| step.reward as f64)
            .sum::<f64>();
        let telescoped = 0.1 * (rollout.terminal_clear_rate - rollout.initial_clear_rate) as f64;
        assert!(
            (reward_sum - telescoped).abs() < 1e-3,
            "{reward_sum} vs {telescoped}"
        );
        assert!(
            rollout
                .transitions
                .iter()
                .any(|step| step.action_index != step.greedy_index)
        );

        let mut environment = GameEnvironment::new(game_config(), 3);
        for step in &rollout.transitions {
            let decision = semantic_decision(&environment).unwrap();
            assert_eq!(decision.encoded, step.encoded);
            let sampled = &decision.candidates.candidates[step.action_index];
            assert_eq!(sampled.action_id, step.action_id);
            assert!(decision.legal_mask[step.action_index]);
            let mut outcome = environment.semantic_step(sampled.action.clone()).unwrap();
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome).unwrap();
        }
        assert!(matches!(
            environment.decision_point(),
            DecisionPoint::Terminal
        ));
        assert_eq!(environment.clear_rate(), rollout.terminal_clear_rate);
    }

    #[test]
    fn bc_evaluator_and_ppo_rollout_see_identical_candidates_and_mask() {
        let actor = random_actor();
        let device = default_policy_device();
        let policy = SemanticPolicy::new(actor.clone());
        let rollout = rollout_episode(&actor, &device, game_config(), 5, 0, 0.1, true).unwrap();
        let mut environment = GameEnvironment::new(game_config(), 5);
        for step in &rollout.transitions {
            let choice = policy.choose(&environment).unwrap();
            let encoded = encode_decision(
                &choice.candidates.observation,
                &choice.candidates.candidates,
                choice.legal_mask.clone(),
            );
            assert_eq!(encoded, step.encoded);
            assert_eq!(choice.index, step.action_index);
            let action = choice.candidates.candidates[choice.index].action.clone();
            let mut outcome = environment.semantic_step(action).unwrap();
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome).unwrap();
        }
        let evaluated = run_policy_episode(
            game_config(),
            5,
            &EvalPolicy::Learned {
                name: "bc".to_string(),
                policy: Box::new(policy),
            },
        )
        .unwrap();
        assert_eq!(evaluated.terminal_clear_rate, rollout.terminal_clear_rate);
        assert_eq!(evaluated.decisions, rollout.transitions.len());
        assert_eq!(evaluated.post_sampling_mutations, 0);
    }

    #[test]
    fn ppo_actor_initialized_from_bc_reproduces_bc_logits() {
        let bc_dir = tiny_bc_run("ppo-init-bc");
        let device = default_policy_device();
        let (_, bc_model) = load_selected_model::<TrainBackend>(&bc_dir, &device).unwrap();
        let critic = new_materialized_model(ModelConfig::default(), &device).unwrap();
        let learner = PpoLearner::new(bc_model.clone(), critic, &PpoConfig::default());
        let bc_policy = SemanticPolicy::from_run_dir(&bc_dir).unwrap();
        let samples = prepare_samples(&canonical_episodes(&[1]), LabelSource::Canonical, 1.0);
        let decisions = samples
            .iter()
            .take(32)
            .map(|sample| &sample.encoded)
            .collect::<Vec<_>>();
        let ppo = actor_log_prob_vectors(&learner.actor_inference(), &decisions, &device).unwrap();
        let bc = actor_log_prob_vectors(bc_policy.model(), &decisions, &device).unwrap();
        assert_eq!(ppo, bc);
        for (log_probs, decision) in ppo.iter().zip(&decisions) {
            assert_eq!(
                greedy_index(log_probs, &decision.legal_mask),
                bc_policy
                    .choose_encoded(decision)
                    .ok()
                    .map(|(index, _)| index)
            );
        }
        std::fs::remove_dir_all(bc_dir).unwrap();
    }

    #[test]
    fn critic_pretraining_is_finite_and_reduces_error() {
        let train = value_samples(&canonical_episodes(&[2, 4]), 0.1);
        let validation = value_samples(&canonical_episodes(&[6]), 0.1);
        let run_dir = temp_dir("ppo-critic-pretrain");
        let metadata = pretrain_critic(
            &run_dir,
            &train,
            &validation,
            CriticCheckpointMetadata {
                schema_version: CRITIC_CHECKPOINT_SCHEMA_VERSION,
                candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
                policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
                game_rules_epoch: GAME_RULES_EPOCH,
                git_commit: "test".to_string(),
                model_config: ModelConfig::default(),
                config: CriticPretrainConfig {
                    epochs: 4,
                    batch_size: 64,
                    ..CriticPretrainConfig::default()
                },
                train_dataset: "tiny".to_string(),
                validation_dataset: "tiny".to_string(),
                train_provenance: None,
                train_samples: train.len(),
                validation_samples: validation.len(),
                initial_validation: ValueMetrics::default(),
                history: Vec::new(),
                selected_epoch: 0,
            },
        )
        .unwrap();
        let last = metadata.history.last().unwrap();
        assert_eq!(last.validation.nonfinite_predictions, 0);
        assert!(last.mean_train_batch_loss.is_finite());
        assert!(last.validation.mse < metadata.initial_validation.mse);
        let (_, critic) = load_critic(&run_dir, &default_policy_device()).unwrap();
        let reloaded =
            evaluate_critic(&critic.valid(), &validation, &default_policy_device()).unwrap();
        let selected = &metadata.history[metadata.selected_epoch - 1].validation;
        assert!((reloaded.mse - selected.mse).abs() < 1e-6);
        std::fs::remove_dir_all(run_dir).unwrap();
    }

    fn tiny_ppo_config() -> PpoConfig {
        PpoConfig {
            episodes_per_iteration: 2,
            update_epochs: 2,
            minibatch_size: 64,
            actor_learning_rate: 1e-4,
            kl_to_init_coefficient: 0.1,
            entropy_coefficient: 0.01,
            ..PpoConfig::default()
        }
    }

    #[test]
    fn ppo_update_is_finite_and_resume_is_deterministic() {
        let bc_dir = tiny_bc_run("ppo-resume-bc");
        let input = |iterations| PpoRunInput {
            config: tiny_ppo_config(),
            init_bc_run: bc_dir.clone(),
            init_critic_run: None,
            iterations,
            evaluate_every: 0,
            development_seeds: 4,
            init_ppo_iteration: None,
        };
        let full_dir = temp_dir("ppo-full");
        let full = train_ppo_run(game_config(), &full_dir, input(2)).unwrap();
        assert_eq!(full.completed_iterations, 2);
        for record in &full.history {
            let update = &record.update;
            for value in [
                update.policy_loss,
                update.value_loss,
                update.entropy,
                update.approx_kl,
                update.kl_to_init_term,
                update.mean_actor_grad_norm,
                record.kl_to_init_after,
            ] {
                assert!(value.is_finite(), "non-finite update metric in {update:?}");
            }
            assert_eq!(update.nonfinite_skips, 0);
            assert!(update.minibatches > 0);
            assert_eq!(record.rollout.illegal_actions, 0);
            assert_eq!(record.rollout.action_mismatches, 0);
            assert_eq!(record.rollout.nonfinite_values, 0);
            assert!(record.rollout.max_telescoping_error < 1e-3);
        }
        assert!(full.history[1].kl_to_init_after > 0.0);

        let resumed_dir = temp_dir("ppo-resumed");
        train_ppo_run(game_config(), &resumed_dir, input(1)).unwrap();
        let probe = prepare_samples(&canonical_episodes(&[1]), LabelSource::Canonical, 1.0);
        let probe = probe
            .iter()
            .take(16)
            .map(|sample| &sample.encoded)
            .collect::<Vec<_>>();
        let outputs = |directory: &Path, iteration: usize| {
            let device = default_policy_device();
            let actor = load_ppo_actor(&iteration_dir(directory, iteration), &device).unwrap();
            let critic = load_model_file::<InferenceBackend>(
                ModelConfig::default(),
                &iteration_dir(directory, iteration).join("critic.bin"),
                &device,
            )
            .unwrap();
            (
                actor_log_prob_vectors(&actor, &probe, &device).unwrap(),
                critic_value_vector(&critic, &probe, &device).unwrap(),
            )
        };
        // Bit-identical when run alone; under parallel test load the CPU
        // backend's parallel reductions may reorder float sums.
        let assert_close =
            |left: (Vec<Vec<f32>>, Vec<f32>), right: (Vec<Vec<f32>>, Vec<f32>), what: &str| {
                let flat = |value: (Vec<Vec<f32>>, Vec<f32>)| {
                    value
                        .0
                        .into_iter()
                        .flatten()
                        .chain(value.1)
                        .collect::<Vec<_>>()
                };
                let (left, right) = (flat(left), flat(right));
                assert_eq!(left.len(), right.len());
                let max_diff = left
                    .iter()
                    .zip(&right)
                    .map(|(left, right)| (left - right).abs())
                    .fold(0.0f32, f32::max);
                assert!(max_diff < 1e-4, "{what}: max difference {max_diff}");
            };
        for iteration in [0, 1] {
            assert_close(
                outputs(&full_dir, iteration),
                outputs(&resumed_dir, iteration),
                &format!("identical runs differ at iteration {iteration}"),
            );
        }
        let resumed = train_ppo_run(game_config(), &resumed_dir, input(2)).unwrap();
        assert_eq!(resumed.completed_iterations, 2);
        assert_close(
            outputs(&full_dir, 2),
            outputs(&resumed_dir, 2),
            "resumed run diverged from the uninterrupted run",
        );
        let actor = load_ppo_actor(&iteration_dir(&full_dir, 0), &default_policy_device()).unwrap();
        let bc = SemanticPolicy::from_run_dir(&bc_dir).unwrap();
        let samples = prepare_samples(&canonical_episodes(&[1]), LabelSource::Canonical, 1.0);
        let decisions = samples
            .iter()
            .take(16)
            .map(|sample| &sample.encoded)
            .collect::<Vec<_>>();
        assert_eq!(
            actor_log_prob_vectors(&actor, &decisions, &default_policy_device()).unwrap(),
            actor_log_prob_vectors(bc.model(), &decisions, &default_policy_device()).unwrap(),
            "iter-0000 actor must equal the BC initialization"
        );
        for directory in [bc_dir, full_dir, resumed_dir] {
            std::fs::remove_dir_all(directory).unwrap();
        }
    }
}
