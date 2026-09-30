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
use super::feature_contract::{InputContract, apply_contract};
use super::model::{
    DeepSetsActorCritic, InferenceBackend, ModelConfig, PolicyDevice, TrainBackend,
    TypedCandidateScorer, default_policy_device, model_to_full_precision_bytes, tensor_from_rows,
};
use super::neural_checkpoint::current_git_revision;
use super::phase4_dataset::{
    DatasetProvenance, EpisodeRecord, GAME_RULES_EPOCH, MAX_EPISODE_DECISIONS, Phase4Split,
};
use super::phase4_eval::{EvalPolicy, PairedComparison, PolicySummary, evaluate_policies};
use super::policy_v2::{
    KindMode, POLICY_REPRESENTATION_VERSION, PolicyNet, SpatialCellHead, factorized_log_probs,
    module_to_bytes,
};
use super::semantic_bc::{
    BcCheckpointMetadata, SemanticDecision, SemanticPolicy, batch_log_probs, load_model_file,
    load_policy_file, load_selected_model, sample_masked, semantic_decision_with,
};
use super::semantic_candidates::{
    BuildOptionInfo, CandidateMode, EncodedDecision, POLICY_CANDIDATE_SET_VERSION,
    SEMANTIC_CANDIDATE_ENCODER_VERSION, encode_decision,
};
use super::spatial::CellSet;
use crate::config::GameConfig;
use crate::environment::{DecisionPoint, GameEnvironment};
use anyhow::{Context, Result, bail};
use burn::module::{AutodiffModule, Module};
use burn::optim::adaptor::OptimizerAdaptor;
use burn::optim::grad_clipping::GradientClippingConfig;
use burn::optim::{Adam, AdamConfig, GradientsParams, Optimizer};
use burn::record::{BinFileRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};
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
type PolicyOptimizer = OptimizerAdaptor<Adam, PolicyNet<TrainBackend>, TrainBackend>;
type SpatialCellOptimizer = OptimizerAdaptor<Adam, SpatialCellHead<TrainBackend>, TrainBackend>;
type BuildOptionOptimizer =
    OptimizerAdaptor<Adam, TypedCandidateScorer<TrainBackend>, TrainBackend>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ActorUpdateMode {
    #[default]
    Full,
    PositionOnly,
    BuildOptionOnly,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum EntropyScheme {
    #[default]
    Joint,
    NormalizedPerHead,
}

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
    /// `Joint`: `entropy_coefficient` times the joint entropy.
    /// `NormalizedPerHead`: `entropy_coefficient` on the normalized family
    /// entropy plus `candidate_entropy_coefficient` on the normalized
    /// candidate entropy.
    #[serde(default)]
    pub entropy_scheme: EntropyScheme,
    #[serde(default)]
    pub candidate_entropy_coefficient: f32,
    pub seed: u64,
    /// Iteration `i` plays `ppo_train` seed block `i + offset`, so a
    /// continuation run never replays another run's training games.
    #[serde(default)]
    pub train_seed_block_offset: usize,
    /// Absolute first game seed for an independently preregistered PPO
    /// training block. When set, iteration `i` uses contiguous seeds starting
    /// at `train_seed_start + i * episodes_per_iteration`; the legacy split
    /// offset must remain zero.
    #[serde(default)]
    pub train_seed_start: Option<u64>,
    #[serde(default)]
    pub actor_update_mode: ActorUpdateMode,
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
            entropy_scheme: EntropyScheme::Joint,
            candidate_entropy_coefficient: 0.0,
            seed: 0,
            train_seed_block_offset: 0,
            train_seed_start: None,
            actor_update_mode: ActorUpdateMode::Full,
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

/// Input transform of a critic.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum CriticInputs {
    /// Every numeric slot squashed (Phase 4B critic).
    #[default]
    SquashAll,
    /// The policy v2 input normalization contract, shared with the actor.
    Contract,
}

/// Critic value `V(s)`: typed value head over the critic's own typed entity
/// encoding plus the global-feature value head. `[decisions, 1]`.
pub fn critic_values<B: Backend>(
    critic: &DeepSetsActorCritic<B>,
    decisions: &[&EncodedDecision],
    inputs: CriticInputs,
    device: &B::Device,
) -> Tensor<B, 2> {
    if inputs == CriticInputs::Contract {
        let contracted = decisions
            .iter()
            .map(|decision| apply_contract(decision, InputContract::Normalized))
            .collect::<Vec<_>>();
        let refs = contracted.iter().collect::<Vec<_>>();
        return critic_forward(critic, &refs, device);
    }
    let squashed = decisions
        .iter()
        .map(|decision| {
            let mut decision = (*decision).clone();
            for set in &mut decision.typed.sets {
                for row in &mut set.rows {
                    for value in &mut row.numeric {
                        *value = critic_squash(*value);
                    }
                }
            }
            for value in &mut decision.global_features {
                *value = critic_squash(*value);
            }
            decision
        })
        .collect::<Vec<_>>();
    let refs = squashed.iter().collect::<Vec<_>>();
    critic_forward(critic, &refs, device)
}

fn critic_forward<B: Backend>(
    critic: &DeepSetsActorCritic<B>,
    decisions: &[&EncodedDecision],
    device: &B::Device,
) -> Tensor<B, 2> {
    let typed_batches: [PaddedEntityBatch; ENTITY_SET_COUNT] = std::array::from_fn(|index| {
        PaddedEntityBatch::from_sets(
            &decisions
                .iter()
                .map(|decision| decision.typed.sets[index].clone())
                .collect::<Vec<_>>(),
        )
    });
    let typed = critic.encode_typed_sets(&typed_batches, device);
    let global = tensor_from_rows::<B>(
        &decisions
            .iter()
            .map(|decision| decision.global_features.clone())
            .collect::<Vec<_>>(),
        device,
    );
    critic.forward_typed_values(typed) + critic.forward_values(global)
}

pub fn critic_value_vector(
    critic: &DeepSetsActorCritic<InferenceBackend>,
    decisions: &[&EncodedDecision],
    inputs: CriticInputs,
    device: &PolicyDevice,
) -> Result<Vec<f32>> {
    let mut values = Vec::with_capacity(decisions.len());
    for chunk in decisions.chunks(INFERENCE_BATCH_SIZE) {
        values.extend(
            critic_values(critic, chunk, inputs, device)
                .into_data()
                .to_vec::<f32>()?,
        );
    }
    Ok(values)
}

/// `log P(cell | option)` over each spatial step's cells (unpadded).
pub fn spatial_cell_log_probs<'a>(
    actor: &PolicyNet<InferenceBackend>,
    steps: impl Iterator<Item = &'a Transition>,
    device: &PolicyDevice,
) -> Result<Vec<Vec<f32>>> {
    let steps = steps.collect::<Vec<_>>();
    if steps.is_empty() {
        return Ok(Vec::new());
    }
    let decisions = steps.iter().map(|step| &step.encoded).collect::<Vec<_>>();
    let options = steps
        .iter()
        .map(|step| step.action_index)
        .collect::<Vec<_>>();
    let cells = steps
        .iter()
        .map(|step| &step.spatial.as_ref().expect("spatial step").cells)
        .collect::<Vec<_>>();
    let (log_probs, lengths) =
        super::policy_v2::cell_log_probs(actor, &decisions, &options, &cells, device);
    let width = log_probs.dims()[1];
    let values = log_probs.into_data().to_vec::<f32>()?;
    Ok(lengths
        .iter()
        .enumerate()
        .map(|(row, length)| values[row * width..row * width + length].to_vec())
        .collect())
}

/// Per-decision log-probabilities (unpadded, one vector per decision).
pub fn actor_log_prob_vectors(
    actor: &PolicyNet<InferenceBackend>,
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

fn policy_log_prob_vectors(
    actor: &PolicyNet<InferenceBackend>,
    option_head: Option<&TypedCandidateScorer<InferenceBackend>>,
    mode: CandidateMode,
    decisions: &[&EncodedDecision],
    device: &PolicyDevice,
) -> Result<Vec<Vec<f32>>> {
    if mode != CandidateMode::BuildOptionA1Marginal {
        return actor_log_prob_vectors(actor, decisions, device);
    }
    let head = option_head.context("option-only policy is missing its option head")?;
    decisions
        .iter()
        .map(|decision| {
            if decision.projection.is_some() {
                super::policy_v2::build_option_log_probs(actor, head, decision, device)
            } else {
                let (log_probs, _) = batch_log_probs(actor, &[*decision], device);
                Ok(log_probs.into_data().to_vec::<f32>()?[..decision.candidates.len()].to_vec())
            }
        })
        .collect()
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
    /// Entropy of the family (action kind) distribution.
    pub family_entropy: f32,
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
    /// The executed action equals the canonical scripted action.
    pub matches_canonical: bool,
    /// Set when the chosen candidate is a spatial option.
    pub spatial: Option<SpatialStep>,
    /// Selected dense BuildTower `(subset, slot)` and its v1 proposal rank.
    pub build_option: Option<BuildOptionChoice>,
    pub build_option_conditional_index: Option<usize>,
    pub old_build_option_log_prob: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildOptionChoice {
    pub subset_index: usize,
    pub card_ids: Vec<usize>,
    pub hand_slot_index: usize,
    pub heuristic_rank: usize,
    pub best_position_index: usize,
    pub outside_v1_top8: bool,
}

impl From<&BuildOptionInfo> for BuildOptionChoice {
    fn from(value: &BuildOptionInfo) -> Self {
        Self {
            subset_index: value.subset_index,
            card_ids: value.card_ids.clone(),
            hand_slot_index: value.hand_slot_index,
            heuristic_rank: value.heuristic_rank,
            best_position_index: value.best_position_index,
            outside_v1_top8: value.outside_v1_top8,
        }
    }
}

/// The cell step below a spatial option.
#[derive(Clone, Debug)]
pub struct SpatialStep {
    pub cells: CellSet,
    pub cell: usize,
    pub greedy_cell: usize,
    /// Behavior policy `log P(cell | option)` over `cells`.
    pub behavior_log_probs: Vec<f32>,
    /// `pi_init` `log P(cell | option)` over `cells`.
    pub init_log_probs: Vec<f32>,
    /// The executed action is not one of the state's v1 top-8 actions.
    pub outside_v1_top8: bool,
}

impl Transition {
    pub fn init_greedy_index(&self) -> Option<usize> {
        greedy_index(&self.init_log_probs, &self.encoded.legal_mask)
    }

    pub fn is_greedy(&self) -> bool {
        self.action_index == self.greedy_index
            && self
                .spatial
                .as_ref()
                .is_none_or(|step| step.cell == step.greedy_cell)
    }

    pub fn matches_init_greedy(&self) -> bool {
        self.init_greedy_index() == Some(self.action_index)
            && self.spatial.as_ref().is_none_or(|step| {
                greedy_index(&step.init_log_probs, &vec![true; step.init_log_probs.len()])
                    == Some(step.cell)
            })
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
#[allow(clippy::too_many_arguments)]
pub fn rollout_episode(
    actor: &PolicyNet<InferenceBackend>,
    device: &PolicyDevice,
    config: Arc<GameConfig>,
    game_seed: u64,
    sample_seed: u64,
    reward_scale: f32,
    greedy: bool,
    mode: CandidateMode,
) -> Result<EpisodeRollout> {
    rollout_episode_with_build_option_head(
        actor,
        None,
        device,
        config,
        game_seed,
        sample_seed,
        reward_scale,
        greedy,
        mode,
    )
}

pub fn rollout_episode_with_build_option_head(
    actor: &PolicyNet<InferenceBackend>,
    build_option_head: Option<&super::model::TypedCandidateScorer<InferenceBackend>>,
    device: &PolicyDevice,
    config: Arc<GameConfig>,
    game_seed: u64,
    sample_seed: u64,
    reward_scale: f32,
    greedy: bool,
    mode: CandidateMode,
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
            bootstrap = Some(semantic_decision_with(&environment, mode)?.encoded);
            break;
        }
        let started = Instant::now();
        let SemanticDecision {
            candidates,
            legal_mask,
            encoded,
        } = semantic_decision_with(&environment, mode)?;
        decision_seconds += started.elapsed().as_secs_f64();
        let started = Instant::now();
        let factorized = factorized_log_probs(actor, &[&encoded], device);
        let family_entropy = factorized
            .kind
            .clone()
            .into_data()
            .to_vec::<f32>()?
            .iter()
            .zip(&factorized.present)
            .filter(|(_, present)| **present)
            .map(|(log_prob, _)| -log_prob.exp() * log_prob)
            .sum::<f32>();
        let log_probs =
            if mode == CandidateMode::BuildOptionA1Marginal && encoded.projection.is_some() {
                let head =
                    build_option_head.context("option-only rollout is missing its option head")?;
                super::policy_v2::build_option_log_probs(actor, head, &encoded, device)?
            } else {
                factorized.joint.clone().into_data().to_vec::<f32>()?[..legal_mask.len()].to_vec()
            };
        forward_seconds += started.elapsed().as_secs_f64();
        if log_probs.iter().any(|value| !value.is_finite()) {
            bail!("seed {game_seed}: non-finite actor log-probability");
        }
        let u: f64 = rng.r#gen();
        let greedy_choice = greedy_index(&log_probs, &legal_mask).context("no legal candidate")?;
        let action_index = if greedy {
            greedy_choice
        } else {
            sample_masked(&log_probs, &legal_mask, u)
        };
        let candidate = &candidates.candidates[action_index];
        let mut old_log_prob = log_probs[action_index];
        let mut behavior_entropy = entropy_of(&log_probs, &legal_mask);
        let spatial = match candidates.cells(&environment, action_index) {
            Some(cells) => {
                let started = Instant::now();
                let (cell_log_probs, _) = super::policy_v2::cell_log_probs(
                    actor,
                    &[&encoded],
                    &[action_index],
                    &[&cells],
                    device,
                );
                let cell_log_probs =
                    cell_log_probs.into_data().to_vec::<f32>()?[..cells.len()].to_vec();
                forward_seconds += started.elapsed().as_secs_f64();
                let all = vec![true; cells.len()];
                let greedy_cell =
                    greedy_index(&cell_log_probs, &all).context("option without cells")?;
                let cell = if greedy {
                    greedy_cell
                } else {
                    sample_masked(&cell_log_probs, &all, rng.r#gen())
                };
                old_log_prob += cell_log_probs[cell];
                behavior_entropy += entropy_of(&cell_log_probs, &all);
                Some(SpatialStep {
                    cells,
                    cell,
                    greedy_cell,
                    behavior_log_probs: cell_log_probs,
                    init_log_probs: Vec::new(),
                    outside_v1_top8: false,
                })
            }
            None => None,
        };
        let action = candidates
            .action(
                action_index,
                spatial
                    .as_ref()
                    .map(|step| (step.cells.clone(), step.cell))
                    .as_ref(),
            )
            .context("sampled candidate has no action")?;
        if !legal_mask[action_index] || !environment.semantic_action_is_legal(&action) {
            illegal_actions += 1;
        }
        if spatial.is_none() && action.action_id() != candidate.action_id {
            action_mismatches += 1;
        }
        let matches_canonical = action == candidates.canonical_action;
        let build_option = candidates
            .build_option_info
            .get(action_index)
            .and_then(Option::as_ref)
            .map(BuildOptionChoice::from);
        let build_option_conditional_index = Some(
            candidates
                .build_option_info
                .iter()
                .take(action_index)
                .filter(|option| option.is_some())
                .count(),
        )
        .filter(|_| build_option.is_some());
        let old_build_option_log_prob = build_option.as_ref().map(|_| {
            let build_kind = crate::environment::ActionKind::BuildTower.index();
            let kind_log_prob = factorized
                .kind
                .clone()
                .into_data()
                .to_vec::<f32>()
                .ok()
                .and_then(|values| values.get(build_kind).copied())
                .unwrap_or(0.0);
            old_log_prob - kind_log_prob
        });
        let mut spatial = spatial;
        if let Some(step) = spatial.as_mut() {
            step.outside_v1_top8 = !candidates.v1_top8.contains(&action);
        }
        let executed_id = action.action_id();
        let clear_rate_before = environment.clear_rate();
        let started = Instant::now();
        let end = advance(&mut environment, action)?;
        environment_seconds += started.elapsed().as_secs_f64();
        let raw_delta = environment.clear_rate() - clear_rate_before;
        transitions.push(Transition {
            action_index,
            action_id: executed_id,
            action_kind: candidate.action.kind().wire_name().to_string(),
            decision_point: format!("{:?}", candidates.observation.decision_point),
            old_log_prob,
            behavior_entropy,
            family_entropy,
            clear_rate_before,
            raw_delta,
            reward: raw_delta * reward_scale,
            value: 0.0,
            advantage: 0.0,
            return_value: 0.0,
            greedy_index: greedy_choice,
            canonical_index: candidates.canonical_index(),
            init_log_probs: Vec::new(),
            matches_canonical,
            spatial,
            build_option,
            build_option_conditional_index,
            old_build_option_log_prob,
            encoded,
        });
        match end {
            StepEnd::Decision => {}
            StepEnd::Terminal => break,
            StepEnd::Truncated => {
                truncated = true;
                if !matches!(environment.decision_point(), DecisionPoint::Terminal) {
                    let observation = environment.snapshot();
                    let candidates =
                        super::semantic_candidates::policy_candidates_with(&environment, mode)?;
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
    #[serde(default)]
    pub normalized_family_entropy: f64,
    #[serde(default)]
    pub normalized_candidate_entropy: f64,
    #[serde(default)]
    pub normalized_cell_entropy: f64,
    #[serde(default)]
    pub position_entropy_contribution: f64,
    pub mean_actor_grad_norm: f64,
    pub max_actor_grad_norm: f64,
    pub mean_critic_grad_norm: f64,
    pub max_critic_grad_norm: f64,
    pub first_value_loss: f64,
    pub last_value_loss: f64,
    pub max_value_loss: f64,
}

pub struct PpoLearner {
    pub actor: PolicyNet<TrainBackend>,
    pub critic: DeepSetsActorCritic<TrainBackend>,
    pub actor_optimizer: PolicyOptimizer,
    pub spatial_cell_optimizer: SpatialCellOptimizer,
    pub build_option_head: Box<TypedCandidateScorer<TrainBackend>>,
    pub build_option_optimizer: BuildOptionOptimizer,
    pub critic_optimizer: ActorCriticOptimizer,
    pub critic_inputs: CriticInputs,
    pub device: PolicyDevice,
}

fn new_optimizer<M: burn::module::AutodiffModule<TrainBackend>>(
    max_grad_norm: f32,
) -> OptimizerAdaptor<Adam, M, TrainBackend> {
    AdamConfig::new()
        .with_grad_clipping(Some(GradientClippingConfig::Norm(max_grad_norm)))
        .init()
}

impl PpoLearner {
    pub fn new(
        actor: PolicyNet<TrainBackend>,
        critic: DeepSetsActorCritic<TrainBackend>,
        critic_inputs: CriticInputs,
        config: &PpoConfig,
    ) -> Self {
        let build_option_head = actor.scorer.typed_candidate_scorer();
        Self {
            actor,
            critic,
            actor_optimizer: new_optimizer(config.max_grad_norm),
            spatial_cell_optimizer: new_optimizer(config.max_grad_norm),
            build_option_head: Box::new(build_option_head),
            build_option_optimizer: new_optimizer(config.max_grad_norm),
            critic_optimizer: new_optimizer(config.max_grad_norm),
            critic_inputs,
            device: default_policy_device(),
        }
    }

    pub fn actor_inference(&self) -> PolicyNet<InferenceBackend> {
        self.actor.clone().valid()
    }

    pub fn critic_inference(&self) -> DeepSetsActorCritic<InferenceBackend> {
        self.critic.clone().valid()
    }

    fn critic_step(&mut self, batch: &[&Transition], learning_rate: f64) -> Option<(f64, f32)> {
        let decisions = batch.iter().map(|step| &step.encoded).collect::<Vec<_>>();
        let values = critic_values(&self.critic, &decisions, self.critic_inputs, &self.device);
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
    normalized_family_entropy: f32,
    normalized_candidate_entropy: f32,
    normalized_cell_entropy: f32,
    position_entropy_contribution: f32,
    kl_to_init: f32,
    approx_kl: f32,
    clip_fraction: f32,
}

/// Per-decision normalized head entropies `[groups, 1]`: the family head's
/// entropy over `ln(families with a legal candidate)`, and the expected
/// candidate-head entropy `sum_f P(f) H(candidate | f) / ln(n_f)`. A head with
/// at most one legal choice contributes 0.
fn normalized_head_entropies<B: Backend>(
    factorized: &super::policy_v2::FactorizedLogProbs<B>,
    valid: Tensor<B, 2>,
    device: &B::Device,
) -> (Tensor<B, 2>, Tensor<B, 2>) {
    use super::policy_v2::FAMILY_COUNT;
    let groups = factorized.present.len() / FAMILY_COUNT;
    let present = Tensor::<B, 2>::from_data(
        TensorData::new(
            factorized
                .present
                .iter()
                .map(|present| *present as u8 as f32)
                .collect::<Vec<_>>(),
            [groups, FAMILY_COUNT],
        ),
        device,
    );
    let inverse_log = |count: usize| {
        if count > 1 {
            1.0 / (count as f32).ln()
        } else {
            0.0
        }
    };
    let family_scale = Tensor::<B, 2>::from_data(
        TensorData::new(
            factorized
                .present
                .chunks(FAMILY_COUNT)
                .map(|row| inverse_log(row.iter().filter(|present| **present).count()))
                .collect::<Vec<_>>(),
            [groups, 1],
        ),
        device,
    );
    let candidate_scale = Tensor::<B, 2>::from_data(
        TensorData::new(
            factorized
                .family_counts
                .iter()
                .map(|count| inverse_log(*count))
                .collect::<Vec<_>>(),
            [groups, FAMILY_COUNT],
        ),
        device,
    );
    let family_probability = factorized.kind.clone().exp() * present.clone();
    let family_entropy =
        -(family_probability.clone() * factorized.kind.clone() * present).sum_dim(1) * family_scale;
    let conditional_probability = factorized.conditional.clone().exp() * valid.clone();
    let candidate_terms = -(conditional_probability * factorized.conditional.clone() * valid);
    let width = candidate_terms.dims()[1];
    let per_family = factorized
        .membership
        .clone()
        .swap_dims(1, 2)
        .matmul(candidate_terms.reshape([groups, width, 1]))
        .reshape([groups, FAMILY_COUNT]);
    let candidate_entropy = (family_probability * per_family * candidate_scale).sum_dim(1);
    (family_entropy, candidate_entropy)
}

/// Cell-head terms of the spatial samples of a minibatch.
struct SpatialCellTerms {
    /// Row of each spatial sample in the minibatch.
    rows: Tensor<TrainBackend, 1, Int>,
    /// `[spatial, 1]` `log P(chosen cell | option)`.
    chosen: Tensor<TrainBackend, 2>,
    /// `[spatial, 1]` cell-head entropy.
    entropy: Tensor<TrainBackend, 2>,
    /// `[spatial, 1]` entropy over `ln(cells)` (0 with one cell).
    normalized_entropy: Tensor<TrainBackend, 2>,
    /// `[spatial, 1]` `KL(pi || pi_init)` of the cell head.
    kl_to_init: Tensor<TrainBackend, 2>,
}

#[derive(Clone, Copy)]
struct FrozenPolicyMetrics {
    entropy: f32,
    normalized_family_entropy: f32,
    normalized_candidate_entropy: f32,
    kl_to_init: f32,
}

/// Per-iteration cache for data that cannot change during position-only PPO.
/// Frozen policy probabilities and scorer contexts are computed once on the
/// inference backend, then reused by all four PPO epochs.
struct PositionOnlyCache {
    metrics: Vec<FrozenPolicyMetrics>,
    context_row_by_transition: Vec<Option<usize>>,
    context_width: usize,
    context_values: Vec<f32>,
}

impl PositionOnlyCache {
    fn new(
        actor: &PolicyNet<InferenceBackend>,
        transitions: &[Transition],
        config: &PpoConfig,
        device: &PolicyDevice,
    ) -> Result<Self> {
        let mut metrics = Vec::with_capacity(transitions.len());
        for chunk in transitions.chunks(INFERENCE_BATCH_SIZE) {
            let decisions = chunk.iter().map(|step| &step.encoded).collect::<Vec<_>>();
            let factorized = factorized_log_probs(actor, &decisions, device);
            let groups = &factorized.groups;
            let rows = chunk.len();
            let width = groups.width();
            let valid_values = (0..rows)
                .flat_map(|row| {
                    (0..width).map(move |column| groups.is_valid(row, column) as u8 as f32)
                })
                .collect::<Vec<_>>();
            let valid = Tensor::<InferenceBackend, 2>::from_data(
                TensorData::new(valid_values.clone(), [rows, width]),
                device,
            );
            let (normalized_family, normalized_candidate) =
                normalized_head_entropies(&factorized, valid, device);
            let normalized_family = normalized_family.into_data().to_vec::<f32>()?;
            let normalized_candidate = normalized_candidate.into_data().to_vec::<f32>()?;
            let joint = factorized.joint.into_data().to_vec::<f32>()?;
            for (row, transition) in chunk.iter().enumerate() {
                let mut entropy = 0.0f32;
                let mut kl_to_init = 0.0f32;
                for column in 0..transition.encoded.candidates.len() {
                    if !transition.encoded.legal_mask[column] {
                        continue;
                    }
                    let log_prob = joint[row * width + column];
                    let probability = log_prob.exp();
                    entropy -= probability * log_prob;
                    if config.kl_to_init_coefficient > 0.0 {
                        let initial = transition
                            .init_log_probs
                            .get(column)
                            .copied()
                            .filter(|value| value.is_finite() && *value > -1.0e8)
                            .unwrap_or(0.0);
                        kl_to_init += probability * (log_prob - initial);
                    }
                }
                metrics.push(FrozenPolicyMetrics {
                    entropy,
                    normalized_family_entropy: normalized_family[row],
                    normalized_candidate_entropy: normalized_candidate[row],
                    kl_to_init,
                });
            }
        }

        let spatial_indices = transitions
            .iter()
            .enumerate()
            .filter_map(|(index, step)| step.spatial.as_ref().map(|_| index))
            .collect::<Vec<_>>();
        let context_row_by_transition = {
            let mut rows = vec![None; transitions.len()];
            for (row, transition_index) in spatial_indices.iter().enumerate() {
                rows[*transition_index] = Some(row);
            }
            rows
        };
        let context_values_and_width = if spatial_indices.is_empty() {
            (Vec::new(), 0)
        } else {
            let decisions = spatial_indices
                .iter()
                .map(|index| &transitions[*index].encoded)
                .collect::<Vec<_>>();
            let options = spatial_indices
                .iter()
                .map(|index| transitions[*index].action_index)
                .collect::<Vec<_>>();
            let contexts = super::policy_v2::cell_contexts(actor, &decisions, &options, device);
            let width = contexts.dims()[1];
            (contexts.into_data().to_vec::<f32>()?, width)
        };

        Ok(Self {
            metrics,
            context_row_by_transition,
            context_width: context_values_and_width.1,
            context_values: context_values_and_width.0,
        })
    }

    fn batch_context(&self, transition_indices: &[usize]) -> Option<Vec<f32>> {
        let rows = transition_indices
            .iter()
            .filter_map(|index| self.context_row_by_transition[*index])
            .collect::<Vec<_>>();
        if rows.is_empty() {
            return None;
        }
        let mut values = Vec::with_capacity(rows.len() * self.context_width);
        for row in rows {
            let start = row * self.context_width;
            values.extend_from_slice(&self.context_values[start..start + self.context_width]);
        }
        Some(values)
    }
}

fn spatial_cell_terms(
    actor: &PolicyNet<TrainBackend>,
    batch: &[&Transition],
    device: &PolicyDevice,
    cached_context: Option<(&[f32], usize)>,
) -> Result<Option<SpatialCellTerms>> {
    let spatial = batch
        .iter()
        .enumerate()
        .filter_map(|(row, step)| step.spatial.as_ref().map(|spatial| (row, *step, spatial)))
        .collect::<Vec<_>>();
    if spatial.is_empty() {
        return Ok(None);
    }
    let decisions = spatial
        .iter()
        .map(|(_, step, _)| &step.encoded)
        .collect::<Vec<_>>();
    let options = spatial
        .iter()
        .map(|(_, step, _)| step.action_index)
        .collect::<Vec<_>>();
    let cell_sets = spatial
        .iter()
        .map(|(_, _, spatial)| &spatial.cells)
        .collect::<Vec<_>>();
    let (log_probs, lengths) = if let Some((values, context_width)) = cached_context {
        let context = Tensor::<TrainBackend, 2>::from_data(
            TensorData::new(values.to_vec(), [spatial.len(), context_width]),
            device,
        );
        super::policy_v2::cell_log_probs_from_context(actor, &context, &cell_sets, device)
    } else {
        super::policy_v2::cell_log_probs(actor, &decisions, &options, &cell_sets, device)
    };
    let count = spatial.len();
    let width = log_probs.dims()[1];
    let valid = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(
            lengths
                .iter()
                .flat_map(|length| (0..width).map(move |column| (column < *length) as u8 as f32))
                .collect::<Vec<_>>(),
            [count, width],
        ),
        device,
    );
    let chosen_index = Tensor::<TrainBackend, 2, Int>::from_data(
        TensorData::new(
            spatial
                .iter()
                .map(|(_, _, spatial)| spatial.cell as i64)
                .collect::<Vec<_>>(),
            [count, 1],
        ),
        device,
    );
    let probabilities = log_probs.clone().exp() * valid.clone();
    let entropy = -(probabilities.clone() * log_probs.clone() * valid.clone()).sum_dim(1);
    let scale = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(
            lengths
                .iter()
                .map(|length| {
                    if *length > 1 {
                        1.0 / (*length as f32).ln()
                    } else {
                        0.0
                    }
                })
                .collect::<Vec<_>>(),
            [count, 1],
        ),
        device,
    );
    let init = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(
            spatial
                .iter()
                .flat_map(|(_, _, spatial)| {
                    (0..width)
                        .map(|column| spatial.init_log_probs.get(column).copied().unwrap_or(0.0))
                })
                .collect::<Vec<_>>(),
            [count, width],
        ),
        device,
    );
    let kl_to_init = (probabilities * (log_probs.clone() - init) * valid).sum_dim(1);
    Ok(Some(SpatialCellTerms {
        rows: Tensor::<TrainBackend, 1, Int>::from_data(
            TensorData::new(
                spatial
                    .iter()
                    .map(|(row, _, _)| *row as i64)
                    .collect::<Vec<_>>(),
                [count],
            ),
            device,
        ),
        chosen: log_probs.gather(1, chosen_index),
        normalized_entropy: entropy.clone() * scale,
        entropy,
        kl_to_init,
    }))
}

fn actor_terms(
    actor: &PolicyNet<TrainBackend>,
    batch: &[&Transition],
    advantages: &[f32],
    config: &PpoConfig,
    device: &PolicyDevice,
) -> Result<ActorTerms> {
    let decisions = batch.iter().map(|step| &step.encoded).collect::<Vec<_>>();
    let factorized = factorized_log_probs(actor, &decisions, device);
    let log_probs = factorized.joint.clone();
    let groups = &factorized.groups;
    let rows = batch.len();
    let width = groups.width();
    let actions = groups.target_tensor::<TrainBackend>(
        &batch
            .iter()
            .map(|step| step.action_index)
            .collect::<Vec<_>>(),
        device,
    );
    let mut new_log_prob = log_probs.clone().gather(1, actions);
    let cells = spatial_cell_terms(actor, batch, device, None)?;
    if let Some(cells) = &cells {
        new_log_prob = new_log_prob.select_assign(
            0,
            cells.rows.clone(),
            cells.chosen.clone(),
            burn::tensor::IndexingUpdateOp::Add,
        );
    }
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
    let (normalized_family, normalized_candidate) =
        normalized_head_entropies(&factorized, valid.clone(), device);
    let rows_f = rows as f32;
    let (entropy, normalized_cell) = match &cells {
        Some(cells) => (
            entropy + cells.entropy.clone().sum() / rows_f,
            cells.normalized_entropy.clone().sum() / rows_f,
        ),
        None => (entropy, Tensor::<TrainBackend, 1>::zeros([1], device)),
    };
    let position_entropy_contribution = cells.as_ref().map_or_else(
        || Tensor::<TrainBackend, 1>::zeros([1], device),
        |cells| cells.entropy.clone().sum() / rows_f,
    );
    let entropy_bonus = match config.actor_update_mode {
        ActorUpdateMode::PositionOnly => {
            position_entropy_contribution.clone() * config.entropy_coefficient
        }
        ActorUpdateMode::BuildOptionOnly => Tensor::<TrainBackend, 1>::zeros([1], device),
        ActorUpdateMode::Full => match config.entropy_scheme {
            EntropyScheme::Joint => entropy.clone() * config.entropy_coefficient,
            EntropyScheme::NormalizedPerHead => {
                normalized_family.clone().mean() * config.entropy_coefficient
                    + (normalized_candidate.clone().mean() + normalized_cell.clone())
                        * config.candidate_entropy_coefficient
            }
        },
    };
    let normalized_cell_entropy = normalized_cell.into_data().to_vec::<f32>()?[0];
    let normalized_family_entropy = normalized_family.mean().into_data().to_vec::<f32>()?[0];
    let normalized_candidate_entropy = normalized_candidate.mean().into_data().to_vec::<f32>()?[0];
    let mut loss = policy_loss.clone() - entropy_bonus;
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
        let mut kl = (probabilities * (log_probs - init) * valid)
            .sum_dim(1)
            .mean();
        if let Some(cells) = &cells {
            kl = kl + cells.kl_to_init.clone().sum() / rows_f;
        }
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
        normalized_family_entropy,
        normalized_candidate_entropy,
        normalized_cell_entropy,
        position_entropy_contribution: position_entropy_contribution.into_data().to_vec::<f32>()?
            [0],
        kl_to_init: kl_to_init_value,
        approx_kl,
        clip_fraction,
        loss,
    })
}

fn build_option_actor_terms(
    actor: &PolicyNet<TrainBackend>,
    head: &TypedCandidateScorer<TrainBackend>,
    batch: &[&Transition],
    advantages: &[f32],
    config: &PpoConfig,
    device: &PolicyDevice,
) -> Result<Option<ActorTerms>> {
    let build_kind = crate::environment::ActionKind::BuildTower.index() as u8;
    let mut selected_log_probs = Vec::new();
    let mut distributions = Vec::new();
    let mut old_values = Vec::new();
    let mut selected_advantages = Vec::new();
    for (transition, advantage) in batch.iter().zip(advantages) {
        let (Some(option_index), Some(old_log_prob)) = (
            transition.build_option_conditional_index,
            transition.old_build_option_log_prob,
        ) else {
            continue;
        };
        let option_indices = transition
            .encoded
            .families
            .iter()
            .enumerate()
            .filter_map(|(index, family)| (*family == build_kind).then_some(index))
            .collect::<Vec<_>>();
        if option_index >= option_indices.len() {
            bail!("BuildTower option index is outside the legal option list");
        }
        let conditional = super::policy_v2::build_option_conditional_log_probs(
            actor,
            head,
            &transition.encoded,
            &option_indices,
            device,
        )?;
        let target = Tensor::<TrainBackend, 2, Int>::from_data(
            TensorData::new(vec![option_index as i64], [1, 1]),
            device,
        );
        selected_log_probs.push(conditional.clone().gather(1, target).reshape([1]));
        distributions.push(conditional);
        old_values.push(old_log_prob);
        selected_advantages.push(*advantage);
    }
    if selected_log_probs.is_empty() {
        return Ok(None);
    }
    let rows = selected_log_probs.len();
    let new_log_prob = Tensor::cat(selected_log_probs, 0).reshape([rows, 1]);
    let old_log_prob =
        Tensor::<TrainBackend, 2>::from_data(TensorData::new(old_values, [rows, 1]), device);
    let advantage = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(selected_advantages, [rows, 1]),
        device,
    );
    let log_ratio = new_log_prob - old_log_prob;
    let ratio = log_ratio.clone().exp();
    let clipped = ratio
        .clone()
        .clamp(1.0 - config.clip_epsilon, 1.0 + config.clip_epsilon);
    let policy_loss = -((ratio.clone() * advantage.clone()).min_pair(clipped * advantage)).mean();
    let entropies = distributions
        .iter()
        .map(|distribution| {
            let probabilities = distribution.clone().exp();
            -(probabilities * distribution.clone()).sum_dim(1)
        })
        .collect::<Vec<_>>();
    let entropy = Tensor::cat(entropies, 0).mean();
    let loss = policy_loss.clone() - entropy.clone() * config.entropy_coefficient;
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
    Ok(Some(ActorTerms {
        loss,
        policy_loss: policy_loss.into_data().to_vec::<f32>()?[0],
        entropy: entropy.into_data().to_vec::<f32>()?[0],
        normalized_family_entropy: 0.0,
        normalized_candidate_entropy: 0.0,
        normalized_cell_entropy: 0.0,
        position_entropy_contribution: 0.0,
        kl_to_init: 0.0,
        approx_kl,
        clip_fraction,
    }))
}

fn position_only_actor_terms(
    actor: &PolicyNet<TrainBackend>,
    batch: &[&Transition],
    advantages: &[f32],
    config: &PpoConfig,
    device: &PolicyDevice,
    frozen_metrics: &[FrozenPolicyMetrics],
    cached_context: &[f32],
    context_width: usize,
) -> Result<ActorTerms> {
    let rows = batch.len();
    let cells = spatial_cell_terms(actor, batch, device, Some((cached_context, context_width)))?
        .context("position-only minibatch had no spatial decisions")?;
    let old_cell = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(
            batch
                .iter()
                .filter_map(|step| {
                    step.spatial
                        .as_ref()
                        .map(|spatial| spatial.behavior_log_probs[spatial.cell])
                })
                .collect::<Vec<_>>(),
            [cells.rows.dims()[0], 1],
        ),
        device,
    );
    let cell_log_ratio = cells.chosen.clone() - old_cell;
    let mut log_ratio = Tensor::<TrainBackend, 2>::zeros([rows, 1], device);
    log_ratio = log_ratio.select_assign(
        0,
        cells.rows.clone(),
        cell_log_ratio,
        burn::tensor::IndexingUpdateOp::Add,
    );
    let ratio = log_ratio.clone().exp();
    let clipped = ratio
        .clone()
        .clamp(1.0 - config.clip_epsilon, 1.0 + config.clip_epsilon);
    let advantage = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(advantages.to_vec(), [rows, 1]),
        device,
    );
    let policy_loss = -((ratio.clone() * advantage.clone()).min_pair(clipped * advantage)).mean();
    let position_entropy = cells.entropy.clone().sum() / rows as f32;
    let normalized_cell = cells.normalized_entropy.clone().sum() / rows as f32;
    let frozen_entropy = frozen_metrics
        .iter()
        .map(|metric| metric.entropy)
        .sum::<f32>()
        / rows as f32;
    let entropy_value = frozen_entropy + position_entropy.clone().into_data().to_vec::<f32>()?[0];
    let normalized_family_entropy = frozen_metrics
        .iter()
        .map(|metric| metric.normalized_family_entropy)
        .sum::<f32>()
        / rows as f32;
    let normalized_candidate_entropy = frozen_metrics
        .iter()
        .map(|metric| metric.normalized_candidate_entropy)
        .sum::<f32>()
        / rows as f32;
    let kl_to_init_value = frozen_metrics
        .iter()
        .map(|metric| metric.kl_to_init)
        .sum::<f32>()
        / rows as f32
        + cells.kl_to_init.clone().sum().into_data().to_vec::<f32>()?[0] / rows as f32;
    let loss = policy_loss.clone() - position_entropy.clone() * config.entropy_coefficient
        + (Tensor::<TrainBackend, 1>::from_data(
            TensorData::new(
                [frozen_metrics
                    .iter()
                    .map(|metric| metric.kl_to_init)
                    .sum::<f32>()
                    / rows as f32]
                .to_vec(),
                [1],
            ),
            device,
        ) + cells.kl_to_init.clone().sum() / rows as f32)
            * config.kl_to_init_coefficient;
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
        loss,
        policy_loss: policy_loss.into_data().to_vec::<f32>()?[0],
        entropy: entropy_value,
        normalized_family_entropy,
        normalized_candidate_entropy,
        normalized_cell_entropy: normalized_cell.into_data().to_vec::<f32>()?[0],
        position_entropy_contribution: position_entropy.into_data().to_vec::<f32>()?[0],
        kl_to_init: kl_to_init_value,
        approx_kl,
        clip_fraction,
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
        // In position-only mode the non-position actor is invariant. Compute
        // its metrics and the scorer context once on the inference backend;
        // differentiating or recomputing these branches in every PPO epoch
        // cannot affect the update.
        let position_cache =
            if update_actor && config.actor_update_mode == ActorUpdateMode::PositionOnly {
                Some(PositionOnlyCache::new(
                    &self.actor_inference(),
                    transitions,
                    config,
                    &self.device,
                )?)
            } else {
                None
            };
        let mut sums = [0.0f64; 8];
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
                    let terms = match config.actor_update_mode {
                        ActorUpdateMode::Full => Some(actor_terms(
                            &self.actor,
                            &batch,
                            &batch_advantages,
                            config,
                            &self.device,
                        )?),
                        ActorUpdateMode::PositionOnly => {
                            let cache = position_cache
                                .as_ref()
                                .context("position-only cache was not initialized")?;
                            if batch.iter().all(|step| step.spatial.is_none()) {
                                None
                            } else {
                                let contexts = cache
                                    .batch_context(chunk)
                                    .context("spatial minibatch has no cached context")?;
                                let frozen_metrics = chunk
                                    .iter()
                                    .map(|index| cache.metrics[*index])
                                    .collect::<Vec<_>>();
                                Some(position_only_actor_terms(
                                    &self.actor,
                                    &batch,
                                    &batch_advantages,
                                    config,
                                    &self.device,
                                    &frozen_metrics,
                                    &contexts,
                                    cache.context_width,
                                )?)
                            }
                        }
                        ActorUpdateMode::BuildOptionOnly => build_option_actor_terms(
                            &self.actor,
                            &self.build_option_head,
                            &batch,
                            &batch_advantages,
                            config,
                            &self.device,
                        )?,
                    };
                    if let Some(terms) = terms {
                        let finite = [
                            terms.policy_loss,
                            terms.entropy,
                            terms.approx_kl,
                            terms.kl_to_init,
                        ]
                        .iter()
                        .all(|value| value.is_finite());
                        if finite {
                            let gradients = match config.actor_update_mode {
                                ActorUpdateMode::Full => {
                                    GradientsParams::from_grads(terms.loss.backward(), &self.actor)
                                }
                                ActorUpdateMode::PositionOnly => GradientsParams::from_grads(
                                    terms.loss.backward(),
                                    &self.actor.spatial_cell_head(),
                                ),
                                ActorUpdateMode::BuildOptionOnly => GradientsParams::from_grads(
                                    terms.loss.backward(),
                                    &*self.build_option_head,
                                ),
                            };
                            let norm = match config.actor_update_mode {
                                ActorUpdateMode::Full => {
                                    super::ppo::gradient_l2_norm(&self.actor, &gradients)
                                }
                                ActorUpdateMode::PositionOnly => super::ppo::gradient_l2_norm(
                                    &self.actor.spatial_cell_head(),
                                    &gradients,
                                ),
                                ActorUpdateMode::BuildOptionOnly => super::ppo::gradient_l2_norm(
                                    &*self.build_option_head,
                                    &gradients,
                                ),
                            };
                            if norm.is_finite() {
                                match config.actor_update_mode {
                                    ActorUpdateMode::Full => {
                                        self.actor = self.actor_optimizer.step(
                                            config.actor_learning_rate,
                                            self.actor.clone(),
                                            gradients,
                                        );
                                    }
                                    ActorUpdateMode::PositionOnly => {
                                        let head = self.actor.spatial_cell_head();
                                        let head = self.spatial_cell_optimizer.step(
                                            config.actor_learning_rate,
                                            head,
                                            gradients,
                                        );
                                        self.actor =
                                            self.actor.clone().with_spatial_cell_head(head);
                                    }
                                    ActorUpdateMode::BuildOptionOnly => {
                                        self.build_option_head = self
                                            .build_option_optimizer
                                            .step(
                                                config.actor_learning_rate,
                                                (*self.build_option_head).clone(),
                                                gradients,
                                            )
                                            .into();
                                    }
                                }
                                actor_steps += 1;
                                actor_grad_sum += norm as f64;
                                stats.max_actor_grad_norm =
                                    stats.max_actor_grad_norm.max(norm as f64);
                                sums[0] += terms.policy_loss as f64;
                                sums[1] += terms.entropy as f64;
                                sums[2] += terms.approx_kl as f64;
                                sums[3] += terms.clip_fraction as f64;
                                sums[4] += terms.kl_to_init as f64;
                                sums[5] += terms.normalized_family_entropy as f64;
                                sums[6] += terms.normalized_candidate_entropy as f64;
                                sums[7] += terms.normalized_cell_entropy as f64;
                                stats.position_entropy_contribution +=
                                    terms.position_entropy_contribution as f64;
                                epoch_kl += terms.approx_kl as f64;
                                epoch_batches += 1;
                            } else {
                                stats.nonfinite_skips += 1;
                            }
                        } else {
                            stats.nonfinite_skips += 1;
                        }
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
        stats.normalized_family_entropy = sums[5] / actor_steps_f;
        stats.normalized_candidate_entropy = sums[6] / actor_steps_f;
        stats.normalized_cell_entropy = sums[7] / actor_steps_f;
        stats.position_entropy_contribution /= actor_steps_f;
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
    #[serde(default)]
    pub inputs: CriticInputs,
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
            inputs: CriticInputs::SquashAll,
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
    inputs: CriticInputs,
    device: &PolicyDevice,
) -> Result<ValueMetrics> {
    let decisions = samples
        .iter()
        .map(|sample| &sample.encoded)
        .collect::<Vec<_>>();
    let predictions = critic_value_vector(critic, &decisions, inputs, device)?;
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
    metadata.initial_validation =
        evaluate_critic(&critic.clone().valid(), validation, config.inputs, &device)?;
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
            let loss = (critic_values(&critic, &decisions, config.inputs, &device) - targets)
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
        let validation_metrics = evaluate_critic(&inference, validation, config.inputs, &device)?;
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
    #[serde(default)]
    pub mean_family_entropy: f64,
    /// Decisions resolved through a spatial option's cell head.
    #[serde(default)]
    pub spatial_decisions: usize,
    /// Of those, cells outside the heuristic top 8 of their option.
    #[serde(default)]
    pub spatial_outside_heuristic_top_k: usize,
    /// Of those, actions differing from the canonical action.
    #[serde(default)]
    pub spatial_non_canonical: usize,
    /// Spatial statistics per action kind (`build_tower`, `place_tower`).
    #[serde(default)]
    pub spatial_by_kind: BTreeMap<String, SpatialKindStats>,
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
pub struct SpatialKindStats {
    pub decisions: usize,
    /// Executed positions outside the state's v1 top-8 actions.
    pub outside_v1_top8: usize,
    pub non_canonical: usize,
    /// Mean heuristic rank percentile of the chosen cell within its option.
    pub mean_rank_percentile: f64,
    /// Mean Manhattan distance from the option's heuristic-best cell.
    pub mean_distance_from_best: f64,
    pub mean_cell_entropy: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DecisionPointStats {
    pub count: usize,
    pub mean_entropy: f64,
    #[serde(default)]
    pub mean_family_entropy: f64,
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
    #[serde(default)]
    pub greedy_build_option_decisions: usize,
    #[serde(default)]
    pub greedy_build_option_outside_v1_top8: usize,
    #[serde(default)]
    pub greedy_build_option_choices: Vec<BuildOptionInfo>,
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
    #[serde(default)]
    pub position_episodes: Vec<PositionEpisodeRecord>,
    #[serde(default)]
    pub build_option_episodes: Vec<BuildOptionEpisodeRecord>,
    #[serde(default)]
    pub position_kl_to_init_after: f64,
    #[serde(default)]
    pub frozen_actor_invariant_passed: bool,
    #[serde(default)]
    pub position_head_changed: bool,
    #[serde(default)]
    pub build_option_head_changed: bool,
    #[serde(default)]
    pub sampled_outside_build_option_rate: f64,
    #[serde(default)]
    pub greedy_outside_build_option_rate: f64,
    #[serde(default)]
    pub option_entropy: f64,
    #[serde(default)]
    pub option_kl_to_init: f64,
    #[serde(default)]
    pub paired_dev_delta_vs_a1_prime: Option<f64>,
    /// Training budget consumed up to and including this iteration,
    /// including the parent run when continued from a PPO checkpoint.
    #[serde(default)]
    pub cumulative: TrainingBudget,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildOptionEpisodeRecord {
    pub seed: u64,
    pub terminal_clear_rate: f32,
    pub build_decisions: usize,
    pub outside_v1_top8_count: usize,
    pub used_outside_v1_top8: bool,
    pub selected_options: Vec<BuildOptionChoice>,
}

fn build_option_episode_record(episode: &EpisodeRollout) -> BuildOptionEpisodeRecord {
    let selected_options = episode
        .transitions
        .iter()
        .filter_map(|step| step.build_option.clone())
        .collect::<Vec<_>>();
    let outside_v1_top8_count = selected_options
        .iter()
        .filter(|option| option.outside_v1_top8)
        .count();
    BuildOptionEpisodeRecord {
        seed: episode.seed,
        terminal_clear_rate: episode.terminal_clear_rate,
        build_decisions: selected_options.len(),
        outside_v1_top8_count,
        used_outside_v1_top8: outside_v1_top8_count > 0,
        selected_options,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionEpisodeRecord {
    pub seed: u64,
    pub terminal_clear_rate: f32,
    pub build_position_decisions: usize,
    pub place_position_decisions: usize,
    pub outside_top8_build_count: usize,
    pub outside_top8_place_count: usize,
    pub non_heuristic_best_count: usize,
    pub used_outside_top8: bool,
    pub selected_positions: Vec<PositionChoiceRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionChoiceRecord {
    pub kind: String,
    pub left: u16,
    pub top: u16,
    pub heuristic_rank: usize,
    pub outside_top8: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TrainingBudget {
    pub episodes: usize,
    /// Semantic decisions (policy samples) - the primary sample-budget axis.
    pub decisions: usize,
    /// Rollout, update and development-evaluation wall time.
    pub seconds: f64,
}

impl TrainingBudget {
    fn after(self, record: &IterationRecord) -> Self {
        Self {
            episodes: self.episodes + record.rollout.episodes,
            decisions: self.decisions + record.rollout.transitions,
            seconds: self.seconds
                + record.rollout_seconds
                + record.update_seconds
                + record
                    .evaluation
                    .as_ref()
                    .map_or(0.0, |evaluation| evaluation.wall_seconds),
        }
    }
}

/// Budget consumed by a run up to `iteration` (inclusive).
fn budget_until(history: &[IterationRecord], iteration: usize) -> TrainingBudget {
    let mut budget = TrainingBudget::default();
    for record in history
        .iter()
        .filter(|record| record.iteration <= iteration)
    {
        if record.cumulative != TrainingBudget::default() {
            budget = record.cumulative;
        } else {
            budget = budget.after(record);
        }
    }
    budget
}

/// Budget of the run a PPO iteration directory belongs to, up to that
/// iteration; zero when the parent run's metadata is unavailable.
fn parent_budget(iteration_dir: &Path) -> Result<TrainingBudget> {
    let actor: PpoActorFile =
        serde_json::from_slice(&std::fs::read(iteration_dir.join("ppo-actor.json"))?)?;
    let Some(run_dir) = iteration_dir.parent() else {
        return Ok(TrainingBudget::default());
    };
    let Ok(bytes) = std::fs::read(run_dir.join("ppo.json")) else {
        return Ok(TrainingBudget::default());
    };
    let parent: PpoRunMetadata = serde_json::from_slice(&bytes)?;
    Ok(budget_until(&parent.history, actor.iteration))
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
    #[serde(default)]
    pub critic_inputs: CriticInputs,
    #[serde(default)]
    pub candidate_mode: CandidateMode,
    pub train_split: Phase4Split,
    pub development_split: Phase4Split,
    pub development_seeds: usize,
    pub evaluate_every: usize,
    pub completed_iterations: usize,
    pub history: Vec<IterationRecord>,
    #[serde(default)]
    pub position_reference_actor: Option<String>,
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
        module_to_bytes(learner.actor_inference())?,
    )?;
    std::fs::write(
        directory.join("critic.bin"),
        model_to_full_precision_bytes(learner.critic_inference())?,
    )?;
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    if metadata.config.actor_update_mode == ActorUpdateMode::PositionOnly {
        recorder.record(
            learner.spatial_cell_optimizer.to_record(),
            directory.join("actor-optimizer.bin"),
        )?;
    } else if metadata.config.actor_update_mode == ActorUpdateMode::BuildOptionOnly {
        recorder.record(
            learner.build_option_optimizer.to_record(),
            directory.join("actor-optimizer.bin"),
        )?;
    } else {
        recorder.record(
            learner.actor_optimizer.to_record(),
            directory.join("actor-optimizer.bin"),
        )?;
    }
    recorder.record(
        learner.critic_optimizer.to_record(),
        directory.join("critic-optimizer.bin"),
    )?;
    if metadata.config.actor_update_mode == ActorUpdateMode::BuildOptionOnly {
        recorder.record(
            (*learner.build_option_head).clone().into_record(),
            directory.join("build-option-head.bin"),
        )?;
    }
    std::fs::write(
        directory.join("ppo-actor.json"),
        serde_json::to_vec_pretty(&PpoActorFile {
            schema_version: SEMANTIC_PPO_CHECKPOINT_SCHEMA_VERSION,
            policy_representation_version: POLICY_REPRESENTATION_VERSION,
            kind_mode: learner.actor.mode,
            input_contract: learner.actor.inputs,
            candidate_mode: metadata.candidate_mode,
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
    /// Absent (1, flat) in checkpoints written before policy v2.
    #[serde(default = "flat_policy_representation")]
    pub policy_representation_version: u32,
    #[serde(default)]
    pub kind_mode: KindMode,
    #[serde(default)]
    pub input_contract: InputContract,
    #[serde(default)]
    pub candidate_mode: CandidateMode,
    pub policy_candidate_set_version: u32,
    pub candidate_encoder_version: u32,
    pub game_rules_epoch: u32,
    pub model_config: ModelConfig,
    pub iteration: usize,
}

fn flat_policy_representation() -> u32 {
    1
}

pub fn load_ppo_actor(
    iteration_dir: &Path,
    device: &PolicyDevice,
) -> Result<PolicyNet<InferenceBackend>> {
    load_ppo_actor_as::<InferenceBackend>(iteration_dir, device)
}

/// The actor of a PPO iteration directory and the candidate mode it plays.
pub fn load_ppo_policy(
    iteration_dir: &Path,
    device: &PolicyDevice,
) -> Result<(PolicyNet<InferenceBackend>, CandidateMode)> {
    let file: PpoActorFile = serde_json::from_slice(
        &std::fs::read(iteration_dir.join("ppo-actor.json"))
            .with_context(|| format!("read {}", iteration_dir.join("ppo-actor.json").display()))?,
    )?;
    Ok((load_ppo_actor(iteration_dir, device)?, file.candidate_mode))
}

pub fn load_ppo_actor_as<B: Backend>(
    iteration_dir: &Path,
    device: &B::Device,
) -> Result<PolicyNet<B>> {
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
    load_policy_file::<B>(
        file.policy_representation_version,
        file.model_config,
        file.kind_mode,
        &iteration_dir.join("actor.bin"),
        device,
    )
    .map(|policy| policy.with_inputs(file.input_contract))
}

fn load_learner(
    run_dir: &Path,
    iteration: usize,
    config: &PpoConfig,
    model_config: ModelConfig,
    critic_inputs: CriticInputs,
) -> Result<PpoLearner> {
    let device = default_policy_device();
    let directory = iteration_dir(run_dir, iteration);
    let actor = load_ppo_actor_as::<TrainBackend>(&directory, &device)?;
    let critic =
        load_model_file::<TrainBackend>(model_config, &directory.join("critic.bin"), &device)?;
    let mut learner = PpoLearner::new(actor, critic, critic_inputs, config);
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    if config.actor_update_mode == ActorUpdateMode::PositionOnly {
        learner.spatial_cell_optimizer = learner
            .spatial_cell_optimizer
            .load_record(recorder.load(directory.join("actor-optimizer.bin"), &device)?);
    } else if config.actor_update_mode == ActorUpdateMode::BuildOptionOnly {
        learner.build_option_head = Box::new(
            learner
                .build_option_head
                .load_record(recorder.load(directory.join("build-option-head.bin"), &device)?),
        );
        learner.build_option_optimizer = learner
            .build_option_optimizer
            .load_record(recorder.load(directory.join("actor-optimizer.bin"), &device)?);
    } else {
        learner.actor_optimizer = learner
            .actor_optimizer
            .load_record(recorder.load(directory.join("actor-optimizer.bin"), &device)?);
    }
    learner.critic_optimizer = learner
        .critic_optimizer
        .load_record(recorder.load(directory.join("critic-optimizer.bin"), &device)?);
    Ok(learner)
}

pub fn load_build_option_head(
    iteration_dir: &Path,
    device: &PolicyDevice,
) -> Result<TypedCandidateScorer<InferenceBackend>> {
    load_build_option_head_as::<InferenceBackend>(iteration_dir, device)
}

pub fn load_build_option_head_as<B: Backend>(
    iteration_dir: &Path,
    device: &B::Device,
) -> Result<TypedCandidateScorer<B>> {
    let actor = load_ppo_actor_as::<B>(iteration_dir, device)?;
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    Ok(actor
        .scorer
        .typed_candidate_scorer()
        .load_record(recorder.load(iteration_dir.join("build-option-head.bin"), device)?))
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
    pub position_reference_actor: Option<PathBuf>,
}

fn train_seed_range(config: &PpoConfig, iteration: usize) -> Result<Vec<u64>> {
    let start = if let Some(seed_start) = config.train_seed_start {
        if config.train_seed_block_offset != 0 {
            bail!("--train-seed-start cannot be combined with a nonzero --train-seed-block-offset");
        }
        seed_start + iteration as u64 * config.episodes_per_iteration as u64
    } else {
        let range = Phase4Split::PpoTrain.range();
        let block = (iteration + config.train_seed_block_offset) as u64;
        *range.start() + block * config.episodes_per_iteration as u64
    };
    let end = start + config.episodes_per_iteration as u64 - 1;
    if config.train_seed_start.is_none() && end > *Phase4Split::PpoTrain.range().end() {
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
#[allow(clippy::too_many_arguments)]
pub fn collect_iteration(
    learner: &PpoLearner,
    init_actor: &PolicyNet<InferenceBackend>,
    init_build_option_head: Option<&TypedCandidateScorer<InferenceBackend>>,
    config: &PpoConfig,
    game_config: Arc<GameConfig>,
    iteration: usize,
    seeds: &[u64],
    mode: CandidateMode,
) -> Result<(Vec<EpisodeRollout>, RolloutStats)> {
    let wall_started = Instant::now();
    let actor = learner.actor_inference();
    let build_option_head = (mode == CandidateMode::BuildOptionA1Marginal)
        .then(|| (*learner.build_option_head).clone().valid());
    let critic = learner.critic_inference();
    let device = learner.device;
    let mut episodes = seeds
        .par_iter()
        .map(|seed| {
            rollout_episode_with_build_option_head(
                &actor,
                build_option_head.as_ref(),
                &device,
                Arc::clone(&game_config),
                *seed,
                sample_seed(config, iteration, *seed),
                config.reward_scale,
                false,
                mode,
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
        let mut init_log_probs = policy_log_prob_vectors(
            init_actor,
            init_build_option_head,
            mode,
            &decisions,
            &device,
        )?
        .into_iter();
        for episode in &mut episodes {
            for step in &mut episode.transitions {
                step.init_log_probs = init_log_probs.next().expect("aligned init log-probs");
            }
        }
        let mut spatial_steps = episodes
            .iter_mut()
            .flat_map(|episode| episode.transitions.iter_mut())
            .filter(|step| step.spatial.is_some())
            .collect::<Vec<_>>();
        for chunk in spatial_steps.chunks_mut(64) {
            let init =
                spatial_cell_log_probs(init_actor, chunk.iter().map(|step| &**step), &device)?;
            for (step, init) in chunk.iter_mut().zip(init) {
                step.spatial.as_mut().expect("spatial step").init_log_probs = init;
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
        let values = critic_value_vector(&critic, &decisions, learner.critic_inputs, &device)?;
        let bootstrap_value = match &episode.bootstrap {
            Some(encoded) => {
                critic_value_vector(&critic, &[encoded], learner.critic_inputs, &device)?[0]
            }
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
            stats.mean_family_entropy += step.family_entropy as f64;
            let non_greedy = !step.is_greedy();
            let non_canonical = !step.matches_canonical;
            let non_init_greedy = !step.matches_init_greedy();
            if let Some(spatial) = &step.spatial {
                stats.spatial_decisions += 1;
                stats.spatial_outside_heuristic_top_k +=
                    spatial.cells.outside_heuristic_top_k(spatial.cell) as usize;
                stats.spatial_non_canonical += non_canonical as usize;
                let family = stats
                    .spatial_by_kind
                    .entry(step.action_kind.clone())
                    .or_default();
                family.decisions += 1;
                family.outside_v1_top8 += spatial.outside_v1_top8 as usize;
                family.non_canonical += non_canonical as usize;
                family.mean_rank_percentile += spatial.cells.rank_percentile(spatial.cell) as f64;
                family.mean_distance_from_best +=
                    spatial.cells.distance_from_best(spatial.cell) as f64;
                family.mean_cell_entropy += entropy_of(
                    &spatial.behavior_log_probs,
                    &vec![true; spatial.behavior_log_probs.len()],
                ) as f64;
            }
            stats.non_greedy_fraction += non_greedy as u8 as f64;
            stats.non_canonical_fraction += non_canonical as u8 as f64;
            stats.non_init_greedy_fraction += non_init_greedy as u8 as f64;
            let point = stats
                .by_decision_point
                .entry(step.decision_point.clone())
                .or_default();
            point.count += 1;
            point.mean_entropy += step.behavior_entropy as f64;
            point.mean_family_entropy += step.family_entropy as f64;
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
    stats.mean_family_entropy /= n;
    stats.non_greedy_fraction /= n;
    stats.non_canonical_fraction /= n;
    stats.non_init_greedy_fraction /= n;
    for family in stats.spatial_by_kind.values_mut() {
        let count = family.decisions.max(1) as f64;
        family.mean_rank_percentile /= count;
        family.mean_distance_from_best /= count;
        family.mean_cell_entropy /= count;
    }
    for point in stats.by_decision_point.values_mut() {
        let count = point.count.max(1) as f64;
        point.mean_entropy /= count;
        point.mean_family_entropy /= count;
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

fn position_episode_record(episode: &EpisodeRollout) -> PositionEpisodeRecord {
    let mut build_position_decisions = 0;
    let mut place_position_decisions = 0;
    let mut outside_top8_build_count = 0;
    let mut outside_top8_place_count = 0;
    let mut non_heuristic_best_count = 0;
    let mut selected_positions = Vec::new();
    for transition in &episode.transitions {
        let Some(spatial) = &transition.spatial else {
            continue;
        };
        let (left, top) = spatial.cells.positions[spatial.cell];
        let kind = transition.action_kind.clone();
        let outside_top8 = spatial.outside_v1_top8;
        match kind.as_str() {
            "build_tower" => {
                build_position_decisions += 1;
                outside_top8_build_count += outside_top8 as usize;
            }
            "place_tower" => {
                place_position_decisions += 1;
                outside_top8_place_count += outside_top8 as usize;
            }
            _ => {}
        }
        non_heuristic_best_count += (spatial.cell != 0) as usize;
        selected_positions.push(PositionChoiceRecord {
            kind,
            left,
            top,
            heuristic_rank: spatial.cell,
            outside_top8,
        });
    }
    PositionEpisodeRecord {
        seed: episode.seed,
        terminal_clear_rate: episode.terminal_clear_rate,
        build_position_decisions,
        place_position_decisions,
        outside_top8_build_count,
        outside_top8_place_count,
        non_heuristic_best_count,
        used_outside_top8: outside_top8_build_count + outside_top8_place_count > 0,
        selected_positions,
    }
}

fn position_kl_to_init(
    actor: &PolicyNet<InferenceBackend>,
    transitions: &[Transition],
    device: &PolicyDevice,
) -> Result<f64> {
    let spatial = transitions
        .iter()
        .filter(|step| step.spatial.is_some())
        .collect::<Vec<_>>();
    if spatial.is_empty() {
        return Ok(0.0);
    }
    let new_log_probs = spatial_cell_log_probs(actor, spatial.iter().copied(), device)?;
    let total = spatial
        .iter()
        .zip(new_log_probs)
        .map(|(step, new)| {
            let spatial = step.spatial.as_ref().expect("spatial transition");
            new.iter()
                .zip(&spatial.init_log_probs)
                .map(|(new, init)| new.exp() as f64 * (*new as f64 - *init as f64))
                .sum::<f64>()
        })
        .sum::<f64>();
    Ok(total / spatial.len() as f64)
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
    current: PolicyNet<InferenceBackend>,
    current_build_option_head: Option<TypedCandidateScorer<InferenceBackend>>,
    position_reference: Option<&SemanticPolicy>,
) -> Result<DevEvaluation> {
    let started = Instant::now();
    let seeds = split.seeds(Some(seed_count))?;
    let current_policy = {
        let policy = SemanticPolicy::new(current).with_candidate_mode(init.candidate_mode());
        match current_build_option_head {
            Some(head) => policy.with_build_option_head(head),
            None => policy,
        }
    };
    let current_policy_stats = current_policy.clone();
    let policies = vec![
        EvalPolicy::Canonical,
        EvalPolicy::Learned {
            name: "bc_init".to_string(),
            policy: Box::new(init.clone()),
        },
        EvalPolicy::Learned {
            name: "ppo".to_string(),
            policy: Box::new(current_policy),
        },
    ];
    let mut policies = policies;
    if let Some(reference) = position_reference {
        policies.push(EvalPolicy::Learned {
            name: "a1_prime".to_string(),
            policy: Box::new(reference.clone()),
        });
    }
    let mut comparisons = vec![
        ("ppo".to_string(), "canonical".to_string()),
        ("ppo".to_string(), "bc_init".to_string()),
        ("bc_init".to_string(), "canonical".to_string()),
    ];
    if position_reference.is_some() {
        comparisons.push(("ppo".to_string(), "a1_prime".to_string()));
        comparisons.push(("a1_prime".to_string(), "canonical".to_string()));
    }
    let report = evaluate_policies(game_config, split.name(), &seeds, &policies, &comparisons)?;
    let (greedy_build_option_decisions, greedy_build_option_outside_v1_top8) =
        current_policy_stats.greedy_build_option_usage();
    let greedy_build_option_choices = current_policy_stats.greedy_build_option_choices();
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
        greedy_build_option_decisions,
        greedy_build_option_outside_v1_top8,
        greedy_build_option_choices,
        wall_seconds: started.elapsed().as_secs_f64(),
    })
}

fn audit_build_option_greedy_alignment(
    game_config: Arc<GameConfig>,
    seeds: &[u64],
    a1: &SemanticPolicy,
    option_policy: &SemanticPolicy,
) -> Result<(usize, usize)> {
    let mut decisions = 0usize;
    let mut mismatches = 0usize;
    for seed in seeds {
        let mut environment = GameEnvironment::new(Arc::clone(&game_config), *seed);
        while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
            let baseline = a1.choose(&environment)?;
            let candidate = option_policy.choose(&environment)?;
            let is_build = baseline.action.kind() == crate::environment::ActionKind::BuildTower;
            let matches = if is_build {
                super::spatial::SpatialAction::of(&baseline.action).map(|(option, _, _)| option)
                    == super::spatial::SpatialAction::of(&candidate.action)
                        .map(|(option, _, _)| option)
            } else {
                baseline.action == candidate.action
            };
            decisions += 1;
            mismatches += (!matches) as usize;
            let mut outcome = environment
                .semantic_step(baseline.action)
                .map_err(|error| anyhow::anyhow!("A1' audit action failed: {error:?}"))?;
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome)?;
        }
    }
    Ok((decisions, mismatches))
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
    let option_only = config.actor_update_mode == ActorUpdateMode::BuildOptionOnly;
    if option_only
        && (bc_metadata.config.candidate_mode != CandidateMode::Top8
            || input.position_reference_actor.is_none()
            || input.init_ppo_iteration.is_none())
    {
        bail!(
            "BuildTower option-only PPO requires Top8 BC, an A1' PPO initialization and its paired reference actor"
        );
    }
    let parent_option_head = input
        .init_ppo_iteration
        .as_ref()
        .is_some_and(|path| path.join("build-option-head.bin").exists());
    let frozen_reference_actor = if option_only {
        let path = input.position_reference_actor.as_ref().unwrap();
        let actor_file: PpoActorFile =
            serde_json::from_slice(&std::fs::read(path.join("ppo-actor.json"))?)?;
        if actor_file.model_config != bc_metadata.model_config
            || actor_file.kind_mode != bc_metadata.config.kind_mode
            || actor_file.input_contract != bc_metadata.config.input_contract
            || actor_file.policy_representation_version != POLICY_REPRESENTATION_VERSION
        {
            bail!("A1' BC and PPO actor parameter correspondence check failed");
        }
        if actor_file.candidate_mode != CandidateMode::Top8 {
            bail!("BuildTower option reference must use Top8 candidates");
        }
        Some(load_ppo_actor_as::<TrainBackend>(path, &device)?)
    } else {
        None
    };
    let option_head_initial = if option_only && parent_option_head {
        load_build_option_head_as::<TrainBackend>(
            input.init_ppo_iteration.as_ref().unwrap(),
            &device,
        )?
    } else if let Some(reference) = &frozen_reference_actor {
        reference.scorer.typed_candidate_scorer()
    } else {
        init_actor.scorer.typed_candidate_scorer()
    };
    let init_policy = if let Some(reference) = &frozen_reference_actor {
        SemanticPolicy::new(reference.clone().valid())
            .with_candidate_mode(CandidateMode::BuildOptionA1Marginal)
            .with_build_option_head(option_head_initial.clone().valid())
            .with_build_option_greedy_from_a1(option_only)
    } else {
        SemanticPolicy::new(init_actor.clone().valid())
            .with_candidate_mode(bc_metadata.config.candidate_mode)
    };
    if config.actor_update_mode == ActorUpdateMode::PositionOnly
        && bc_metadata.config.candidate_mode != CandidateMode::FullPositionA1Marginal
    {
        bail!("position-only PPO requires the A1' marginal full-position candidate mode");
    }
    if config.actor_update_mode == ActorUpdateMode::PositionOnly
        && input.position_reference_actor.is_none()
    {
        bail!("position-only PPO requires the frozen A1' reference actor");
    }
    let invariant_reference_actor = if option_only {
        frozen_reference_actor
            .as_ref()
            .expect("validated option reference")
            .clone()
    } else {
        init_actor.clone()
    };
    let init_actor_bytes = module_to_bytes(invariant_reference_actor.clone())?;
    let frozen_non_position_matches = |actor: &PolicyNet<TrainBackend>| -> Result<bool> {
        if option_only {
            Ok(module_to_bytes(actor.clone())? == init_actor_bytes)
        } else {
            Ok(module_to_bytes(
                actor
                    .clone()
                    .with_spatial_cell_head(init_actor.spatial_cell_head()),
            )? == init_actor_bytes)
        }
    };
    let position_reference = input
        .position_reference_actor
        .as_ref()
        .map(|path| {
            let (actor, mode) = load_ppo_policy(path, &device)?;
            if mode != CandidateMode::Top8 {
                bail!("position reference actor must use the original Top8 candidates");
            }
            Ok(SemanticPolicy::new(actor).with_candidate_mode(mode))
        })
        .transpose()?;
    let (mut greedy_decisions, mut greedy_mismatches) = (0usize, 0usize);
    if option_only && !run_dir.join("ppo.json").exists() {
        let reference = position_reference
            .as_ref()
            .context("option-only PPO needs an A1' comparison policy")?;
        if !parent_option_head {
            let dev_seeds = Phase4Split::PpoDevelopment.seeds(None)?;
            (greedy_decisions, greedy_mismatches) = audit_build_option_greedy_alignment(
                Arc::clone(&game_config),
                &dev_seeds,
                reference,
                &init_policy,
            )?;
            if greedy_mismatches != 0 {
                bail!(
                    "option-only BC initialization changed {greedy_mismatches}/{greedy_decisions} A1' greedy decisions"
                );
            }
        }
        init_policy.reset_greedy_build_option_usage();
        let smoke_seeds = train_seed_range(&config, 0)?;
        let mut outside = 0usize;
        let mut build_decisions = 0usize;
        let mut execution_errors = 0usize;
        let head = option_head_initial.clone().valid();
        let reference_actor_inference = invariant_reference_actor.clone().valid();
        for seed in smoke_seeds.iter().take(8) {
            let episode = rollout_episode_with_build_option_head(
                &reference_actor_inference,
                Some(&head),
                &device,
                Arc::clone(&game_config),
                *seed,
                sample_seed(&config, 1, *seed),
                config.reward_scale,
                false,
                CandidateMode::BuildOptionA1Marginal,
            )?;
            execution_errors += episode.illegal_actions + episode.action_mismatches;
            let report = build_option_episode_record(&episode);
            outside += report.outside_v1_top8_count;
            build_decisions += report.build_decisions;
            if outside > 0 {
                break;
            }
        }
        if execution_errors != 0 || outside == 0 {
            bail!(
                "option-only sampled smoke failed: outside {outside}/{build_decisions}, illegal/mismatch {execution_errors}"
            );
        }
        eprintln!(
            "option-only init audit: A1' greedy mismatches {greedy_mismatches}/{greedy_decisions}, sampled outside top-8 {outside}/{build_decisions}, illegal/mismatch {execution_errors}"
        );
    }
    let prior_budget = match &input.init_ppo_iteration {
        Some(directory) => parent_budget(directory)?,
        None => TrainingBudget::default(),
    };
    let model_config = bc_metadata.model_config;
    let (mut metadata, mut learner) = if run_dir.join("ppo.json").exists() {
        let metadata: PpoRunMetadata =
            serde_json::from_slice(&std::fs::read(run_dir.join("ppo.json"))?)?;
        if metadata.config != config
            || metadata.init_bc_run != input.init_bc_run.display().to_string()
            || metadata.position_reference_actor
                != input
                    .position_reference_actor
                    .as_ref()
                    .map(|path| path.display().to_string())
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
            metadata.critic_inputs,
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
                load_ppo_actor_as::<TrainBackend>(directory, &device)?,
                Some(load_model_file::<TrainBackend>(
                    model_config,
                    &directory.join("critic.bin"),
                    &device,
                )?),
            ),
            None => (init_actor.clone(), None),
        };
        if option_only && module_to_bytes(actor.clone())? != init_actor_bytes {
            bail!(
                "A1' option-only actor initialization differs from the paired reference checkpoint"
            );
        }
        let (critic, critic_inputs) = match (critic, &input.init_critic_run) {
            (Some(critic), _) => {
                let parent = input
                    .init_ppo_iteration
                    .as_ref()
                    .and_then(|directory| directory.parent())
                    .and_then(|run| std::fs::read(run.join("ppo.json")).ok())
                    .map(|bytes| serde_json::from_slice::<PpoRunMetadata>(&bytes))
                    .transpose()?;
                (
                    critic,
                    parent.map_or(CriticInputs::SquashAll, |parent| parent.critic_inputs),
                )
            }
            (None, Some(path)) => {
                let (critic_metadata, critic) = load_critic(path, &device)?;
                if critic_metadata.model_config != model_config {
                    bail!("critic model config differs from the actor's");
                }
                (critic, critic_metadata.config.inputs)
            }
            (None, None) => (
                super::semantic_bc::seeded_materialized_model(model_config, config.seed, &device)?,
                CriticInputs::SquashAll,
            ),
        };
        let mut learner = PpoLearner::new(actor, critic, critic_inputs, &config);
        if option_only {
            learner.build_option_head = Box::new(option_head_initial.clone());
        }
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
            position_reference_actor: input
                .position_reference_actor
                .as_ref()
                .map(|path| path.display().to_string()),
            critic_inputs,
            candidate_mode: if option_only {
                CandidateMode::BuildOptionA1Marginal
            } else {
                bc_metadata.config.candidate_mode
            },
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
                option_only.then(|| (*learner.build_option_head).clone().valid()),
                position_reference.as_ref(),
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
                position_episodes: Vec::new(),
                build_option_episodes: Vec::new(),
                position_kl_to_init_after: 0.0,
                frozen_actor_invariant_passed: true,
                position_head_changed: false,
                build_option_head_changed: false,
                sampled_outside_build_option_rate: 0.0,
                greedy_outside_build_option_rate: 0.0,
                option_entropy: 0.0,
                option_kl_to_init: 0.0,
                paired_dev_delta_vs_a1_prime: None,
                cumulative: prior_budget,
            });
        }
        write_checkpoint(run_dir, 0, &learner, &metadata)?;
        (metadata, learner)
    };
    let init_inference = invariant_reference_actor.valid();
    let init_build_option_head = option_only.then(|| option_head_initial.clone().valid());
    for iteration in metadata.completed_iterations + 1..=input.iterations {
        let seeds = train_seed_range(&config, iteration - 1)?;
        let rollout_started = Instant::now();
        let (episodes, rollout_stats) = collect_iteration(
            &learner,
            &init_inference,
            init_build_option_head.as_ref(),
            &config,
            Arc::clone(&game_config),
            iteration,
            &seeds,
            metadata.candidate_mode,
        )?;
        if matches!(
            config.actor_update_mode,
            ActorUpdateMode::PositionOnly | ActorUpdateMode::BuildOptionOnly
        ) && (rollout_stats.illegal_actions != 0
            || rollout_stats.action_mismatches != 0
            || rollout_stats.nonfinite_values != 0)
        {
            bail!("frozen-head rollout validity gate failed at iteration {iteration}");
        }
        let position_episodes = if config.actor_update_mode == ActorUpdateMode::PositionOnly {
            episodes.iter().map(position_episode_record).collect()
        } else {
            Vec::new()
        };
        let build_option_episodes: Vec<BuildOptionEpisodeRecord> =
            episodes.iter().map(build_option_episode_record).collect();
        let rollout_seconds = rollout_started.elapsed().as_secs_f64();
        let transitions = episodes
            .into_iter()
            .flat_map(|episode| episode.transitions)
            .collect::<Vec<_>>();
        let decisions = transitions
            .iter()
            .map(|step| &step.encoded)
            .collect::<Vec<_>>();
        let old_log_probs = policy_log_prob_vectors(
            &learner.actor_inference(),
            option_only
                .then(|| (*learner.build_option_head).clone().valid())
                .as_ref(),
            metadata.candidate_mode,
            &decisions,
            &device,
        )?;
        let init_log_probs = transitions
            .iter()
            .map(|step| step.init_log_probs.clone())
            .collect::<Vec<_>>();
        let update_started = Instant::now();
        let update_actor = iteration > config.critic_warmup_iterations;
        let update = learner.update(&transitions, &config, iteration, update_actor)?;
        let update_seconds = update_started.elapsed().as_secs_f64();
        let frozen_actor_invariant_passed = matches!(
            config.actor_update_mode,
            ActorUpdateMode::PositionOnly | ActorUpdateMode::BuildOptionOnly
        ) && frozen_non_position_matches(&learner.actor)?;
        if matches!(
            config.actor_update_mode,
            ActorUpdateMode::PositionOnly | ActorUpdateMode::BuildOptionOnly
        ) && !frozen_actor_invariant_passed
        {
            bail!("frozen actor invariant failed at iteration {iteration}");
        }
        let decisions = transitions
            .iter()
            .map(|step| &step.encoded)
            .collect::<Vec<_>>();
        let masks = transitions
            .iter()
            .map(|step| step.encoded.legal_mask.as_slice())
            .collect::<Vec<_>>();
        let new_log_probs = policy_log_prob_vectors(
            &learner.actor_inference(),
            option_only
                .then(|| (*learner.build_option_head).clone().valid())
                .as_ref(),
            metadata.candidate_mode,
            &decisions,
            &device,
        )?;
        let mut kl_to_init_after = mean_kl(&new_log_probs, &init_log_probs, &masks);
        let mut kl_old_new_after = mean_kl(&old_log_probs, &new_log_probs, &masks);
        let spatial_steps = transitions
            .iter()
            .filter(|step| step.spatial.is_some())
            .collect::<Vec<_>>();
        if !spatial_steps.is_empty() {
            let new_cells = spatial_cell_log_probs(
                &learner.actor_inference(),
                spatial_steps.iter().copied(),
                &device,
            )?;
            let count = transitions.len().max(1) as f64;
            for (step, new) in spatial_steps.iter().zip(&new_cells) {
                let spatial = step.spatial.as_ref().expect("spatial step");
                let all = vec![true; new.len()];
                let masks = [all.as_slice()];
                kl_to_init_after += mean_kl(
                    std::slice::from_ref(new),
                    std::slice::from_ref(&spatial.init_log_probs),
                    &masks,
                ) / count;
                kl_old_new_after += mean_kl(
                    std::slice::from_ref(&spatial.behavior_log_probs),
                    std::slice::from_ref(new),
                    &masks,
                ) / count;
            }
        }
        let evaluation = if input.evaluate_every > 0 && iteration % input.evaluate_every == 0 {
            let evaluation = development_evaluation(
                Arc::clone(&game_config),
                Phase4Split::PpoDevelopment,
                input.development_seeds,
                iteration,
                &init_policy,
                learner.actor_inference(),
                option_only.then(|| (*learner.build_option_head).clone().valid()),
                position_reference.as_ref(),
            )?;
            log_evaluation(&evaluation);
            Some(evaluation)
        } else {
            None
        };
        let mut record = IterationRecord {
            iteration,
            train_seeds: (seeds[0], *seeds.last().expect("non-empty seeds")),
            actor_updated: update_actor,
            rollout: rollout_stats,
            update: update.clone(),
            kl_to_init_after,
            kl_old_new_after,
            rollout_seconds,
            update_seconds,
            evaluation: evaluation.clone(),
            position_episodes,
            build_option_episodes: build_option_episodes.clone(),
            position_kl_to_init_after: if config.actor_update_mode == ActorUpdateMode::PositionOnly
            {
                position_kl_to_init(&learner.actor_inference(), &transitions, &device)?
            } else {
                0.0
            },
            frozen_actor_invariant_passed,
            position_head_changed: config.actor_update_mode == ActorUpdateMode::PositionOnly
                && module_to_bytes(learner.actor.spatial_cell_head())?
                    != module_to_bytes(init_actor.spatial_cell_head())?,
            build_option_head_changed: option_only
                && module_to_bytes((*learner.build_option_head).clone())?
                    != module_to_bytes(option_head_initial.clone())?,
            sampled_outside_build_option_rate: {
                let outside = build_option_episodes
                    .iter()
                    .map(|episode| episode.outside_v1_top8_count)
                    .sum::<usize>();
                let decisions = build_option_episodes
                    .iter()
                    .map(|episode| episode.build_decisions)
                    .sum::<usize>();
                outside as f64 / decisions.max(1) as f64
            },
            greedy_outside_build_option_rate: evaluation.as_ref().map_or(0.0, |evaluation| {
                evaluation.greedy_build_option_outside_v1_top8 as f64
                    / evaluation.greedy_build_option_decisions.max(1) as f64
            }),
            option_entropy: if option_only { update.entropy } else { 0.0 },
            option_kl_to_init: if option_only { kl_to_init_after } else { 0.0 },
            paired_dev_delta_vs_a1_prime: evaluation.as_ref().and_then(|evaluation| {
                evaluation
                    .comparisons
                    .iter()
                    .find(|comparison| {
                        comparison.policy == "ppo" && comparison.reference == "a1_prime"
                    })
                    .map(|comparison| comparison.mean)
            }),
            cumulative: TrainingBudget::default(),
        };
        let previous = metadata.history.last().map_or(prior_budget, |last| {
            if last.cumulative == TrainingBudget::default() {
                budget_until(&metadata.history, last.iteration)
            } else {
                last.cumulative
            }
        });
        record.cumulative = previous.after(&record);
        if config.actor_update_mode == ActorUpdateMode::PositionOnly && iteration == 50 {
            let mut build_decisions = 0usize;
            let mut place_decisions = 0usize;
            let mut build_outside = 0usize;
            let mut place_outside = 0usize;
            for episode in metadata
                .history
                .iter()
                .flat_map(|record| record.position_episodes.iter())
                .chain(record.position_episodes.iter())
            {
                build_decisions += episode.build_position_decisions;
                place_decisions += episode.place_position_decisions;
                build_outside += episode.outside_top8_build_count;
                place_outside += episode.outside_top8_place_count;
            }
            let build_rate = build_outside as f64 / build_decisions.max(1) as f64;
            let place_rate = place_outside as f64 / place_decisions.max(1) as f64;
            if build_rate < 0.01 || place_rate < 0.01 || !record.position_head_changed {
                bail!(
                    "position-only pilot gate failed at iteration 50: build outside {:.4}, place outside {:.4}, cell head changed {}",
                    build_rate,
                    place_rate,
                    record.position_head_changed
                );
            }
        }
        if option_only && iteration == 50 {
            let (outside, decisions) = metadata
                .history
                .iter()
                .flat_map(|record| record.build_option_episodes.iter())
                .chain(record.build_option_episodes.iter())
                .fold((0usize, 0usize), |(outside, decisions), episode| {
                    (
                        outside + episode.outside_v1_top8_count,
                        decisions + episode.build_decisions,
                    )
                });
            let rate = outside as f64 / decisions.max(1) as f64;
            let head_changed = record.build_option_head_changed;
            if rate < 0.01 || !head_changed || !record.frozen_actor_invariant_passed {
                bail!(
                    "BuildTower option-only pilot gate failed at iteration 50: outside rate {:.4}, option head changed {}, frozen actor invariant {}",
                    rate,
                    head_changed,
                    record.frozen_actor_invariant_passed
                );
            }
        }
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
    if !record.build_option_episodes.is_empty() {
        eprintln!(
            "  BuildTower options sampled outside {:.3}% | greedy outside {:.3}% | entropy {:.4} | KL-init {:.5} | paired A1' delta {:?}",
            record.sampled_outside_build_option_rate * 100.0,
            record.greedy_outside_build_option_rate * 100.0,
            record.option_entropy,
            record.option_kl_to_init,
            record.paired_dev_delta_vs_a1_prime,
        );
    }
    eprintln!(
        "  budget: episodes {} decisions {} hours {:.2} | family entropy {:.4} | normalized entropy family {:.4} candidate {:.4} cell {:.4} | spatial {} outside-top8 {:.4} non-canonical {:.4}",
        record.cumulative.episodes,
        record.cumulative.decisions,
        record.cumulative.seconds / 3600.0,
        rollout.mean_family_entropy,
        update.normalized_family_entropy,
        update.normalized_candidate_entropy,
        update.normalized_cell_entropy,
        rollout.spatial_decisions,
        rollout.spatial_outside_heuristic_top_k as f64 / rollout.spatial_decisions.max(1) as f64,
        rollout.spatial_non_canonical as f64 / rollout.spatial_decisions.max(1) as f64,
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
    for (kind, family) in &rollout.spatial_by_kind {
        eprintln!(
            "  spatial {kind}: n {} outside-v1-top8 {:.4} non-canonical {:.4} rank-pct {:.4} distance {:.2} cell-entropy {:.4}",
            family.decisions,
            family.outside_v1_top8 as f64 / family.decisions.max(1) as f64,
            family.non_canonical as f64 / family.decisions.max(1) as f64,
            family.mean_rank_percentile,
            family.mean_distance_from_best,
            family.mean_cell_entropy,
        );
    }
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

    #[test]
    fn custom_training_seed_blocks_are_contiguous_and_phaseable() {
        let config = PpoConfig {
            episodes_per_iteration: 48,
            train_seed_start: Some(4_300_000),
            ..PpoConfig::default()
        };
        assert_eq!(
            train_seed_range(&config, 0).unwrap(),
            (4_300_000..=4_300_047).collect::<Vec<_>>()
        );
        assert_eq!(
            train_seed_range(&config, 74).unwrap(),
            (4_303_552..=4_303_599).collect::<Vec<_>>()
        );
        let phase_two = PpoConfig {
            train_seed_start: Some(4_303_600),
            ..config.clone()
        };
        assert_eq!(
            train_seed_range(&phase_two, 0).unwrap(),
            (4_303_600..=4_303_647).collect::<Vec<_>>()
        );
        assert_eq!(
            train_seed_range(&phase_two, 199).unwrap(),
            (4_313_152..=4_313_199).collect::<Vec<_>>()
        );
        let invalid = PpoConfig {
            train_seed_block_offset: 1,
            ..config
        };
        assert!(train_seed_range(&invalid, 0).is_err());
    }

    fn random_actor() -> PolicyNet<InferenceBackend> {
        crate::ml::semantic_bc::seeded_policy_net::<InferenceBackend>(
            ModelConfig::default(),
            KindMode::LogSumExp,
            7,
            &default_policy_device(),
        )
        .unwrap()
    }

    /// A tiny BC run directory usable as a PPO initialization.
    fn tiny_bc_run(name: &str) -> PathBuf {
        tiny_bc_run_with(name, KindMode::LogSumExp, InputContract::Raw)
    }

    fn tiny_bc_run_with(name: &str, kind_mode: KindMode, input_contract: InputContract) -> PathBuf {
        tiny_bc_run_spatial(name, kind_mode, input_contract, CandidateMode::Top8)
    }

    fn tiny_bc_run_spatial(
        name: &str,
        kind_mode: KindMode,
        input_contract: InputContract,
        candidate_mode: CandidateMode,
    ) -> PathBuf {
        let episodes = canonical_episodes(&[0]);
        let mut samples = if candidate_mode == CandidateMode::Top8 {
            prepare_samples(&episodes, LabelSource::Canonical, 1.0)
        } else {
            crate::ml::semantic_bc::prepare_replayed_samples(
                game_config(),
                &episodes,
                candidate_mode,
            )
            .unwrap()
        };
        samples.truncate(64);
        let config = BcTrainConfig {
            epochs: 1,
            batch_size: 32,
            kind_mode,
            input_contract,
            candidate_mode,
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
                    policy_representation_version: POLICY_REPRESENTATION_VERSION,
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
        let rollout = rollout_episode(
            &actor,
            &device,
            game_config(),
            3,
            99,
            0.1,
            false,
            CandidateMode::Top8,
        )
        .unwrap();
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
            let decision = crate::ml::semantic_bc::semantic_decision(&environment).unwrap();
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
        let rollout = rollout_episode(
            &actor,
            &device,
            game_config(),
            5,
            0,
            0.1,
            true,
            CandidateMode::Top8,
        )
        .unwrap();
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
        let learner = PpoLearner::new(
            bc_model.clone(),
            critic,
            CriticInputs::SquashAll,
            &PpoConfig::default(),
        );
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
    fn position_only_update_freezes_actor_and_ratio_is_cell_conditional() {
        let device = default_policy_device();
        let actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            ModelConfig::default(),
            KindMode::Learned,
            41,
            &device,
        )
        .unwrap()
        .with_inputs(InputContract::Normalized);
        let initial = actor.clone();
        let initial_actor_bytes = module_to_bytes(initial.clone()).unwrap();
        let initial_cell_bytes = module_to_bytes(initial.spatial_cell_head()).unwrap();
        let critic = super::super::semantic_bc::seeded_materialized_model(
            ModelConfig::default(),
            43,
            &device,
        )
        .unwrap();
        let mut learner = PpoLearner::new(
            actor,
            critic,
            CriticInputs::SquashAll,
            &PpoConfig::default(),
        );
        let config = PpoConfig {
            actor_update_mode: ActorUpdateMode::PositionOnly,
            episodes_per_iteration: 2,
            update_epochs: 1,
            minibatch_size: 512,
            actor_learning_rate: 3e-4,
            entropy_coefficient: 0.01,
            kl_to_init_coefficient: 0.0,
            ..PpoConfig::default()
        };
        let (episodes, rollout) = collect_iteration(
            &learner,
            &initial.clone().valid(),
            None,
            &config,
            game_config(),
            1,
            &[301, 302],
            CandidateMode::FullPositionA1Marginal,
        )
        .unwrap();
        assert_eq!(rollout.illegal_actions, 0);
        assert_eq!(rollout.action_mismatches, 0);
        let transitions = episodes
            .into_iter()
            .flat_map(|episode| episode.transitions)
            .collect::<Vec<_>>();
        assert!(
            transitions
                .iter()
                .any(|transition| transition.spatial.is_some())
        );
        let update = learner.update(&transitions, &config, 1, true).unwrap();
        assert_eq!(update.nonfinite_skips, 0);
        assert!(update.mean_actor_grad_norm.is_finite());
        assert!(update.mean_actor_grad_norm > 0.0);
        assert_ne!(
            module_to_bytes(learner.actor.spatial_cell_head()).unwrap(),
            initial_cell_bytes,
            "position head did not update"
        );
        let restored = learner
            .actor
            .clone()
            .with_spatial_cell_head(initial.spatial_cell_head());
        assert_eq!(module_to_bytes(restored).unwrap(), initial_actor_bytes);

        let inference = learner.actor_inference();
        let decision_refs = transitions
            .iter()
            .map(|transition| &transition.encoded)
            .collect::<Vec<_>>();
        let new_joint = actor_log_prob_vectors(&inference, &decision_refs, &device).unwrap();
        let spatial_transitions = transitions
            .iter()
            .filter(|transition| transition.spatial.is_some())
            .collect::<Vec<_>>();
        let new_cells =
            spatial_cell_log_probs(&inference, spatial_transitions.iter().copied(), &device)
                .unwrap();
        let mut next_cell = 0;
        for (index, transition) in transitions.iter().enumerate() {
            if let Some(spatial) = &transition.spatial {
                let new_cell = &new_cells[next_cell];
                let joint_log_ratio = new_joint[index][transition.action_index]
                    + new_cell[spatial.cell]
                    - transition.old_log_prob;
                let cell_log_ratio =
                    new_cell[spatial.cell] - spatial.behavior_log_probs[spatial.cell];
                assert!((joint_log_ratio - cell_log_ratio).abs() < 2e-5);
                next_cell += 1;
            } else {
                assert!(
                    (new_joint[index][transition.action_index] - transition.old_log_prob).abs()
                        < 2e-5
                );
            }
        }
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
        let reloaded = evaluate_critic(
            &critic.valid(),
            &validation,
            CriticInputs::SquashAll,
            &default_policy_device(),
        )
        .unwrap();
        let selected = &metadata.history[metadata.selected_epoch - 1].validation;
        assert!((reloaded.mse - selected.mse).abs() < 1e-6);
        std::fs::remove_dir_all(run_dir).unwrap();
    }

    #[test]
    fn normalized_head_entropies_are_bounded_and_zero_without_choice() {
        let device = default_policy_device();
        let actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            ModelConfig::default(),
            KindMode::Learned,
            5,
            &device,
        )
        .unwrap()
        .with_inputs(InputContract::Normalized);
        let samples = prepare_samples(&canonical_episodes(&[1]), LabelSource::Canonical, 1.0);
        let decisions = samples
            .iter()
            .take(64)
            .map(|sample| &sample.encoded)
            .collect::<Vec<_>>();
        let factorized = factorized_log_probs(&actor, &decisions, &device);
        let width = factorized.groups.width();
        let valid = Tensor::<TrainBackend, 2>::from_data(
            TensorData::new(
                (0..decisions.len())
                    .flat_map(|row| {
                        let groups = &factorized.groups;
                        (0..width).map(move |column| groups.is_valid(row, column) as u8 as f32)
                    })
                    .collect::<Vec<_>>(),
                [decisions.len(), width],
            ),
            &device,
        );
        let (family, candidate) = normalized_head_entropies(&factorized, valid, &device);
        let family = family.into_data().to_vec::<f32>().unwrap();
        let candidate = candidate.into_data().to_vec::<f32>().unwrap();
        let mut single = 0;
        for (index, decision) in decisions.iter().enumerate() {
            for value in [family[index], candidate[index]] {
                assert!(
                    value.is_finite() && (-1e-5..=1.0 + 1e-4).contains(&value),
                    "{value}"
                );
            }
            if decision.legal_mask.iter().filter(|legal| **legal).count() == 1 {
                assert!(family[index].abs() < 1e-6 && candidate[index].abs() < 1e-6);
                single += 1;
            }
        }
        assert!(
            single > 0,
            "fixture should contain forced single-candidate decisions"
        );
        assert!(family.iter().any(|value| *value > 0.01));
    }

    #[test]
    fn ppo_update_with_policy_v2_options_is_finite() {
        std::thread::Builder::new()
            .name("ppo-policy-v2-test".to_string())
            .stack_size(8 * 1024 * 1024)
            .spawn(ppo_update_with_policy_v2_options_is_finite_inner)
            .unwrap()
            .join()
            .unwrap();
    }

    fn ppo_update_with_policy_v2_options_is_finite_inner() {
        let bc_dir = tiny_bc_run_with("ppo-v2-bc", KindMode::Learned, InputContract::Normalized);
        let critic_samples = value_samples(&canonical_episodes(&[2]), 0.1);
        let critic_dir = temp_dir("ppo-v2-critic");
        pretrain_critic(
            &critic_dir,
            &critic_samples,
            &critic_samples[..64],
            CriticCheckpointMetadata {
                schema_version: CRITIC_CHECKPOINT_SCHEMA_VERSION,
                candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
                policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
                game_rules_epoch: GAME_RULES_EPOCH,
                git_commit: "test".to_string(),
                model_config: ModelConfig::default(),
                config: CriticPretrainConfig {
                    epochs: 1,
                    batch_size: 64,
                    inputs: CriticInputs::Contract,
                    ..CriticPretrainConfig::default()
                },
                train_dataset: "tiny".to_string(),
                validation_dataset: "tiny".to_string(),
                train_provenance: None,
                train_samples: critic_samples.len(),
                validation_samples: 64,
                initial_validation: ValueMetrics::default(),
                history: Vec::new(),
                selected_epoch: 0,
            },
        )
        .unwrap();
        let run_dir = temp_dir("ppo-v2-run");
        let metadata = train_ppo_run(
            game_config(),
            &run_dir,
            PpoRunInput {
                config: PpoConfig {
                    entropy_scheme: EntropyScheme::NormalizedPerHead,
                    entropy_coefficient: 0.02,
                    candidate_entropy_coefficient: 0.02,
                    ..tiny_ppo_config()
                },
                init_bc_run: bc_dir.clone(),
                init_critic_run: Some(critic_dir.clone()),
                init_ppo_iteration: None,
                iterations: 1,
                evaluate_every: 0,
                development_seeds: 4,
                position_reference_actor: None,
            },
        )
        .unwrap();
        assert_eq!(metadata.critic_inputs, CriticInputs::Contract);
        let update = &metadata.history.last().unwrap().update;
        for value in [
            update.policy_loss,
            update.value_loss,
            update.normalized_family_entropy,
            update.normalized_candidate_entropy,
        ] {
            assert!(value.is_finite(), "{update:?}");
        }
        assert!(update.normalized_candidate_entropy > 0.0);
        let actor = load_ppo_actor(&iteration_dir(&run_dir, 1), &default_policy_device()).unwrap();
        assert_eq!(actor.mode, KindMode::Learned);
        assert_eq!(actor.inputs, InputContract::Normalized);
        for directory in [bc_dir, critic_dir, run_dir] {
            std::fs::remove_dir_all(directory).unwrap();
        }
    }

    #[test]
    fn spatial_rollout_executes_sampled_cells_and_decomposes_log_probs() {
        let actor = crate::ml::semantic_bc::seeded_policy_net::<InferenceBackend>(
            ModelConfig::default(),
            KindMode::Learned,
            11,
            &default_policy_device(),
        )
        .unwrap();
        let device = default_policy_device();
        let mode = CandidateMode::FullPosition;
        let rollout =
            rollout_episode(&actor, &device, game_config(), 7, 3, 0.1, false, mode).unwrap();
        assert_eq!(rollout.illegal_actions, 0);
        assert_eq!(rollout.action_mismatches, 0);
        let reward_sum = rollout
            .transitions
            .iter()
            .map(|step| step.reward as f64)
            .sum::<f64>();
        let telescoped = 0.1 * (rollout.terminal_clear_rate - rollout.initial_clear_rate) as f64;
        assert!((reward_sum - telescoped).abs() < 1e-3);
        let spatial = rollout
            .transitions
            .iter()
            .filter(|step| step.spatial.is_some())
            .count();
        assert!(spatial > 5, "{spatial}");
        assert!(
            rollout
                .transitions
                .iter()
                .filter_map(|step| step.spatial.as_ref())
                .any(|step| step.cells.outside_heuristic_top_k(step.cell))
        );

        let mut environment = GameEnvironment::new(game_config(), 7);
        for step in &rollout.transitions {
            let decision = semantic_decision_with(&environment, mode).unwrap();
            assert_eq!(decision.encoded, step.encoded);
            let cells = decision.candidates.cells(&environment, step.action_index);
            let spatial = cells
                .as_ref()
                .map(|cells| (cells.clone(), step.spatial.as_ref().unwrap().cell));
            let action = decision
                .candidates
                .action(step.action_index, spatial.as_ref())
                .unwrap();
            assert_eq!(action.action_id(), step.action_id);
            assert!(environment.semantic_action_is_legal(&action));
            if let Some(recorded) = &step.spatial {
                assert_eq!(cells.as_ref(), Some(&recorded.cells));
                let space = crate::policy_action::PolicyActionSpace::compute(&environment);
                let index = space.action_to_index(&action).expect("indexed");
                assert!(space.legal_mask()[index]);
                let joint = actor_log_prob_vectors(&actor, &[&step.encoded], &device).unwrap();
                let (cell_log_probs, _) = crate::ml::policy_v2::cell_log_probs(
                    &actor,
                    &[&step.encoded],
                    &[step.action_index],
                    &[&recorded.cells],
                    &device,
                );
                let cell_log_probs = cell_log_probs.into_data().to_vec::<f32>().unwrap();
                let expected = joint[0][step.action_index] + cell_log_probs[recorded.cell];
                assert!(
                    (expected - step.old_log_prob).abs() < 1e-4,
                    "{expected} vs {}",
                    step.old_log_prob
                );
            }
            let mut outcome = environment.semantic_step(action).unwrap();
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome).unwrap();
        }
        assert_eq!(environment.clear_rate(), rollout.terminal_clear_rate);
    }

    #[test]
    fn option_only_update_changes_only_option_head_and_uses_conditional_ratio() {
        let device = default_policy_device();
        let actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            ModelConfig::default(),
            KindMode::Learned,
            81,
            &device,
        )
        .unwrap();
        let actor_before = module_to_bytes(actor.clone()).unwrap();
        let critic =
            crate::ml::semantic_bc::seeded_materialized_model(ModelConfig::default(), 82, &device)
                .unwrap();
        let config = PpoConfig {
            actor_update_mode: ActorUpdateMode::BuildOptionOnly,
            update_epochs: 1,
            minibatch_size: 1,
            target_kl: None,
            entropy_coefficient: 0.01,
            normalize_advantages: false,
            ..PpoConfig::default()
        };
        let mut learner = PpoLearner::new(actor, critic, CriticInputs::SquashAll, &config);

        let mut environment = GameEnvironment::new(game_config(), 17);
        let encoded = loop {
            let decision =
                semantic_decision_with(&environment, CandidateMode::BuildOptionA1Marginal).unwrap();
            if decision.encoded.projection.is_some() {
                break decision.encoded;
            }
            let mut outcome = environment
                .semantic_step(decision.candidates.canonical_action)
                .unwrap();
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome).unwrap();
        };
        let option_indices = encoded
            .families
            .iter()
            .enumerate()
            .filter_map(|(index, family)| {
                (*family == crate::environment::ActionKind::BuildTower as u8).then_some(index)
            })
            .collect::<Vec<_>>();
        assert!(option_indices.len() > 8);
        let action_index = option_indices[0];
        let conditional_index = 0;
        let actor_inference = learner.actor_inference();
        let head_inference = (*learner.build_option_head).clone().valid();
        let joint = super::super::policy_v2::build_option_log_probs(
            &actor_inference,
            &head_inference,
            &encoded,
            &device,
        )
        .unwrap();
        let family = super::super::policy_v2::factorized_log_probs(
            &actor_inference,
            &[&EncodedDecision {
                projection: None,
                candidates: encoded
                    .projection
                    .as_ref()
                    .unwrap()
                    .source_candidates
                    .clone(),
                legal_mask: encoded
                    .projection
                    .as_ref()
                    .unwrap()
                    .source_legal_mask
                    .clone(),
                families: encoded.projection.as_ref().unwrap().source_families.clone(),
                ..encoded.clone()
            }],
            &device,
        );
        let family_log_prob = family.kind.into_data().to_vec::<f32>().unwrap()
            [crate::environment::ActionKind::BuildTower.index()];
        let old_conditional = joint[action_index] - family_log_prob;
        let option = BuildOptionChoice::from(&BuildOptionInfo {
            subset_index: 0,
            card_ids: vec![0],
            hand_slot_index: 0,
            heuristic_rank: 0,
            best_position_index: 0,
            outside_v1_top8: false,
        });
        let transition = Transition {
            encoded,
            action_index,
            action_id: "build-option-smoke".to_string(),
            action_kind: "build_tower".to_string(),
            decision_point: "CardSelection".to_string(),
            old_log_prob: joint[action_index],
            behavior_entropy: 0.0,
            family_entropy: 0.0,
            clear_rate_before: 0.0,
            raw_delta: 0.0,
            reward: 0.0,
            value: 0.0,
            advantage: 1.0,
            return_value: 0.0,
            greedy_index: action_index,
            canonical_index: None,
            init_log_probs: Vec::new(),
            matches_canonical: false,
            spatial: None,
            build_option: Some(option),
            build_option_conditional_index: Some(conditional_index),
            old_build_option_log_prob: Some(old_conditional),
        };
        let head_before = module_to_bytes((*learner.build_option_head).clone()).unwrap();
        let update = learner.update(&[transition], &config, 1, true).unwrap();
        assert_eq!(update.nonfinite_skips, 0);
        assert!(
            update.approx_kl.abs() < 1e-5,
            "initial PPO ratio: {update:?}"
        );
        assert!(update.mean_actor_grad_norm > 0.0);
        assert_ne!(
            module_to_bytes((*learner.build_option_head).clone()).unwrap(),
            head_before,
            "BuildTower option head did not update"
        );
        assert_eq!(
            module_to_bytes(learner.actor.clone()).unwrap(),
            actor_before
        );
    }

    #[test]
    fn option_only_rollout_executes_legal_dense_option_representatives() {
        let device = default_policy_device();
        let actor = random_actor();
        let head = actor.scorer.typed_candidate_scorer();
        let rollout = rollout_episode_with_build_option_head(
            &actor,
            Some(&head),
            &device,
            game_config(),
            19,
            1019,
            0.1,
            false,
            CandidateMode::BuildOptionA1Marginal,
        )
        .unwrap();
        assert_eq!(rollout.illegal_actions, 0);
        assert_eq!(rollout.action_mismatches, 0);
        assert!(
            rollout
                .transitions
                .iter()
                .any(|step| step.build_option.is_some())
        );
        for step in rollout
            .transitions
            .iter()
            .filter(|step| step.build_option.is_some())
        {
            assert!(step.old_log_prob.is_finite());
            assert!(step.old_build_option_log_prob.unwrap().is_finite());
            assert!(step.spatial.is_none());
            assert!(step.build_option_conditional_index.is_some());
        }
    }

    #[test]
    fn replayed_spatial_bc_samples_target_the_canonical_cell() {
        let episodes = canonical_episodes(&[0]);
        let samples = crate::ml::semantic_bc::prepare_replayed_samples(
            game_config(),
            &episodes,
            CandidateMode::FullPosition,
        )
        .unwrap();
        assert_eq!(samples.len(), episodes[0].samples.len());
        let spatial = samples
            .iter()
            .filter_map(|sample| sample.target_cell.as_ref())
            .collect::<Vec<_>>();
        assert!(spatial.len() > 5);
        for (cells, cell) in spatial {
            assert_eq!(*cell, 0, "canonical placements are the heuristic-best cell");
            assert!(cells.len() > 8);
        }
        for sample in &samples {
            assert_eq!(sample.target, sample.canonical_index);
        }
    }

    #[test]
    fn cell_label_smoothing_mixes_the_target_and_the_uniform_cell_cross_entropy() {
        let episodes = canonical_episodes(&[0]);
        let samples = crate::ml::semantic_bc::prepare_replayed_samples(
            game_config(),
            &episodes,
            CandidateMode::FullPosition,
        )
        .unwrap();
        let batch = samples.iter().take(64).collect::<Vec<_>>();
        let device = PolicyDevice::default();
        let actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            ModelConfig { hidden_size: 16 },
            KindMode::Learned,
            3,
            &device,
        )
        .unwrap();
        let loss = |smoothing: f32| {
            crate::ml::semantic_bc::batch_loss(&actor, &batch, smoothing, &device)
                .into_data()
                .to_vec::<f32>()
                .unwrap()[0]
        };
        let (target, spatial, all) =
            crate::ml::semantic_bc::target_cell_log_probs(&actor, &batch, &device).unwrap();
        let target = target.into_data().to_vec::<f32>().unwrap();
        let all = all.into_data().to_vec::<f32>().unwrap();
        let width = all.len() / spatial.len();
        let gap = spatial
            .iter()
            .enumerate()
            .map(|(row, index)| {
                let count = batch[*index].target_cell.as_ref().unwrap().0.len();
                let mean = all[row * width..row * width + count].iter().sum::<f32>() / count as f32;
                target[row] - mean
            })
            .sum::<f32>()
            / batch.len() as f32;
        assert!(!spatial.is_empty());
        let expected = loss(0.0) + 0.02 * gap;
        assert!(
            (loss(0.02) - expected).abs() < 1e-4,
            "{} vs {expected}",
            loss(0.02)
        );
    }

    #[test]
    fn spatial_ppo_update_is_finite() {
        std::thread::Builder::new()
            .name("spatial-ppo-test".to_string())
            .stack_size(8 * 1024 * 1024)
            .spawn(spatial_ppo_update_is_finite_inner)
            .unwrap()
            .join()
            .unwrap();
    }

    fn spatial_ppo_update_is_finite_inner() {
        let bc_dir = tiny_bc_run_spatial(
            "ppo-spatial-bc",
            KindMode::Learned,
            InputContract::Normalized,
            CandidateMode::FullPosition,
        );
        let run_dir = temp_dir("ppo-spatial-run");
        let metadata = train_ppo_run(
            game_config(),
            &run_dir,
            PpoRunInput {
                config: PpoConfig {
                    entropy_scheme: EntropyScheme::NormalizedPerHead,
                    entropy_coefficient: 0.02,
                    candidate_entropy_coefficient: 0.02,
                    ..tiny_ppo_config()
                },
                init_bc_run: bc_dir.clone(),
                init_critic_run: None,
                init_ppo_iteration: None,
                iterations: 1,
                evaluate_every: 0,
                development_seeds: 4,
                position_reference_actor: None,
            },
        )
        .unwrap();
        assert_eq!(metadata.candidate_mode, CandidateMode::FullPosition);
        let record = metadata.history.last().unwrap();
        assert!(record.rollout.spatial_decisions > 0);
        assert_eq!(record.rollout.illegal_actions, 0);
        for value in [
            record.update.policy_loss,
            record.update.normalized_cell_entropy,
            record.update.kl_to_init_term,
            record.kl_to_init_after,
            record.kl_old_new_after,
        ] {
            assert!(value.is_finite(), "{record:?}");
        }
        assert!(record.update.normalized_cell_entropy > 0.0);
        let (_, mode) =
            load_ppo_policy(&iteration_dir(&run_dir, 1), &default_policy_device()).unwrap();
        assert_eq!(mode, CandidateMode::FullPosition);
        for directory in [bc_dir, run_dir] {
            std::fs::remove_dir_all(directory).unwrap();
        }
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
        std::thread::Builder::new()
            .name("ppo-resume-test".to_string())
            .stack_size(8 * 1024 * 1024)
            .spawn(ppo_update_is_finite_and_resume_is_deterministic_inner)
            .unwrap()
            .join()
            .unwrap();
    }

    fn ppo_update_is_finite_and_resume_is_deterministic_inner() {
        let bc_dir = tiny_bc_run("ppo-resume-bc");
        let input = |iterations| PpoRunInput {
            config: tiny_ppo_config(),
            init_bc_run: bc_dir.clone(),
            init_critic_run: None,
            iterations,
            evaluate_every: 0,
            development_seeds: 4,
            init_ppo_iteration: None,
            position_reference_actor: None,
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
                critic_value_vector(&critic, &probe, CriticInputs::SquashAll, &device).unwrap(),
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
