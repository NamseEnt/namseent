pub use super::diagnostics::{DiagnosticStep, DiagnosticTrace, collect_diagnostic_trace};
use super::encoding::{
    EntityRow, EntitySet, PaddedEntityBatch, TypedObservation, candidate_rows_for_legal_actions,
};
use super::features::observation_features;
use super::model::{DeepSetsActorCritic, tensor_from_flat_rows, tensor_from_repeated_row};
use crate::config::GameConfig;
use crate::simulator::environment::{
    ActionKind, AgentAction, LegalAction, Observation, RewardConfig, StepReason,
};
use crate::simulator::policy_runner::{PolicyRunnerConfig, run_episode_with_step_callback};
use crate::simulator::trajectory::Trajectory;
use anyhow::{Result, bail};
use burn::tensor::activation::softmax;
use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, TensorData};
use indicatif::ProgressBar;
#[cfg(feature = "simulator-wgpu")]
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
#[cfg(feature = "simulator-wgpu")]
use std::sync::mpsc::{Receiver, sync_channel};
#[cfg(feature = "simulator-wgpu")]
use std::sync::{Condvar, Mutex};
#[cfg(feature = "simulator-wgpu")]
use std::thread::JoinHandle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutConfig {
    pub max_decisions_per_episode: usize,
    pub temperature: f32,
    pub gamma: f32,
    pub gae_lambda: f32,
    pub exploration_iteration: u64,
    pub adaptive_exploration: bool,
    pub greedy: bool,
    pub max_stage: Option<usize>,
    pub reward_config: RewardConfig,
}

impl Default for RolloutConfig {
    fn default() -> Self {
        Self {
            max_decisions_per_episode: 10_000,
            temperature: 1.0,
            gamma: 0.99,
            gae_lambda: 0.95,
            exploration_iteration: 0,
            adaptive_exploration: false,
            greedy: false,
            max_stage: None,
            reward_config: RewardConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutStep {
    pub state: Vec<f32>,
    pub typed_observation: TypedObservation,
    pub candidate_rows: Vec<EntityRow>,
    pub legal_candidate_mask: Vec<bool>,
    pub action_index: usize,
    pub old_log_probability: f32,
    pub value_estimate: f32,
    pub next_value_estimate: f32,
    pub reward: f32,
    pub terminal_reward: f32,
    pub shaping_reward: f32,
    pub no_progress_cycle_penalty_reward: f32,
    pub terminated: bool,
    pub truncated: bool,
    pub no_progress_cycle: bool,
    pub bootstrap_allowed: bool,
    pub action_kind: u8,
    pub advantage: f32,
    pub return_value: f32,
    pub policy_entropy: f32,
}

impl RolloutStep {
    pub(crate) fn candidate_count(&self) -> usize {
        self.candidate_rows.len()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpisodeRollout {
    pub seed: u64,
    pub steps: Vec<RolloutStep>,
    pub cleared: bool,
    pub final_clear_rate: f32,
    pub final_stage: usize,
    pub escaped_hp: f32,
    pub player_damage: f32,
    pub tower_damage: f32,
    pub episode_return: f32,
    pub termination_reason: StepReason,
    pub reward_component_sums: BTreeMap<String, f64>,
    pub decision_point_counts: BTreeMap<String, usize>,
    pub forced_action_count: usize,
    pub forced_action_counts: BTreeMap<String, usize>,
}

impl EpisodeRollout {
    pub fn decision_count(&self) -> usize {
        self.steps.len()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutBatch {
    pub episodes: Vec<EpisodeRollout>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolloutDiagnostics {
    pub episode_count: usize,
    pub decision_steps: usize,
    pub mean_reward: f32,
    pub reward_stddev: f32,
    pub mean_episode_return: f32,
    pub mean_action_nll: f32,
    pub mean_policy_entropy: f32,
    pub mean_value_loss: f32,
    pub episode_return_stddev: f32,
    pub mean_final_clear_rate: f32,
    pub mean_escaped_hp: f32,
    pub mean_player_damage: f32,
    pub mean_tower_damage: f32,
    pub nonzero_reward_fraction: f32,
    pub terminal_reward_count: usize,
    pub death_penalty_count: usize,
    pub full_clear_count: usize,
    pub mean_final_stage: f32,
    pub mean_advantage: f32,
    pub advantage_stddev: f32,
    pub terminal_episode_count: usize,
    pub truncated_episode_count: usize,
    pub no_progress_cycle_count: usize,
    pub mean_decisions_per_episode: f32,
    pub action_kind_counts: [usize; ActionKind::COUNT],
    pub decision_point_counts: BTreeMap<String, usize>,
    pub forced_action_count: usize,
    pub forced_action_counts: BTreeMap<String, usize>,
    pub select_count: usize,
    pub deselect_count: usize,
    pub confirm_count: usize,
    pub cancel_count: usize,
    pub immediate_inverse_count: usize,
    pub selected_set_revisit_count: usize,
    pub begin_cancel_repetition_count: usize,
    pub mean_select_count: f32,
    pub mean_deselect_count: f32,
    pub mean_confirm_count: f32,
    pub mean_cancel_count: f32,
    pub mean_immediate_inverse_count: f32,
    pub mean_selected_set_revisit_count: f32,
    pub mean_begin_cancel_repetition_count: f32,
    pub no_progress_cycle_penalty_sum: f64,
    pub bootstrap_allowed_count: usize,
    pub bootstrap_blocked_count: usize,
    pub bootstrap_counts: BTreeMap<String, usize>,
    pub reward_component_sums: BTreeMap<String, f64>,
    #[serde(skip)]
    pub(crate) reward_sum: f64,
    #[serde(skip)]
    pub(crate) reward_sum_squares: f64,
    #[serde(skip)]
    pub(crate) episode_return_sum: f64,
    #[serde(skip)]
    pub(crate) episode_return_sum_squares: f64,
    #[serde(skip)]
    pub(crate) advantage_sum: f64,
    #[serde(skip)]
    pub(crate) advantage_sum_squares: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupervisedNllReport {
    pub sample_count: usize,
    pub mean_nll: f32,
    pub top1_accuracy: f32,
}

pub fn evaluate_supervised_nll<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    trajectory: &Trajectory,
) -> Result<SupervisedNllReport> {
    if trajectory.steps.is_empty() {
        bail!("trajectory contains no supervised steps");
    }
    let mut total_nll = 0.0;
    let mut correct = 0;
    for step in &trajectory.steps {
        let action_index = step
            .legal_actions
            .iter()
            .position(|legal| legal.action == step.action)
            .ok_or_else(|| anyhow::anyhow!("trajectory action is not legal"))?;
        let candidate_rows =
            candidate_rows_for_legal_actions(&step.pre_observation, &step.legal_actions);
        let logits = logits_for(
            model,
            device,
            &step.pre_observation,
            &observation_features(&step.pre_observation),
            &candidate_rows,
        );
        let maximum = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let log_normalizer = maximum
            + logits
                .iter()
                .map(|logit| (*logit - maximum).exp())
                .sum::<f32>()
                .ln();
        total_nll += log_normalizer - logits[action_index];
        let predicted = logits
            .iter()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
            .map(|(index, _)| index)
            .ok_or_else(|| anyhow::anyhow!("model produced no logits"))?;
        correct += usize::from(predicted == action_index);
    }
    let sample_count = trajectory.steps.len();
    Ok(SupervisedNllReport {
        sample_count,
        mean_nll: total_nll / sample_count as f32,
        top1_accuracy: correct as f32 / sample_count as f32,
    })
}

impl RolloutBatch {
    pub fn clear_rate(&self) -> f64 {
        if self.episodes.is_empty() {
            0.0
        } else {
            self.episodes
                .iter()
                .map(|episode| episode.final_clear_rate as f64)
                .sum::<f64>()
                / self.episodes.len() as f64
        }
    }

    pub fn steps(&self) -> impl Iterator<Item = &RolloutStep> {
        self.episodes
            .iter()
            .flat_map(|episode| episode.steps.iter())
    }

    pub fn steps_mut(&mut self) -> impl Iterator<Item = &mut RolloutStep> {
        self.episodes
            .iter_mut()
            .flat_map(|episode| episode.steps.iter_mut())
    }

    pub fn diagnostics(&self) -> RolloutDiagnostics {
        let steps = self.steps().collect::<Vec<_>>();
        let decision_steps = steps.len();
        let rewards = steps.iter().map(|step| step.reward).collect::<Vec<_>>();
        let advantages = steps.iter().map(|step| step.advantage).collect::<Vec<_>>();
        let action_nlls = steps
            .iter()
            .map(|step| -step.old_log_probability)
            .collect::<Vec<_>>();
        let entropies = steps
            .iter()
            .map(|step| step.policy_entropy)
            .collect::<Vec<_>>();
        let value_losses = steps
            .iter()
            .map(|step| (step.return_value - step.value_estimate).powi(2))
            .collect::<Vec<_>>();
        let episode_returns = self
            .episodes
            .iter()
            .map(|episode| episode.steps.iter().map(|step| step.reward).sum::<f32>())
            .collect::<Vec<_>>();
        let mean_reward = mean(&rewards);
        let reward_stddev = stddev(&rewards, mean_reward);
        let mean_advantage = mean(&advantages);
        let advantage_stddev = stddev(&advantages, mean_advantage);
        let mut action_kind_counts = [0; ActionKind::COUNT];
        let mut decision_point_counts = BTreeMap::new();
        let forced_action_count = self
            .episodes
            .iter()
            .map(|episode| episode.forced_action_count)
            .sum();
        let mut forced_action_counts = BTreeMap::new();
        for episode in &self.episodes {
            for (key, value) in &episode.forced_action_counts {
                *forced_action_counts.entry(key.clone()).or_insert(0) += value;
            }
        }
        let mut select_count = 0;
        let mut deselect_count = 0;
        let mut confirm_count = 0;
        let mut cancel_count = 0;
        let mut immediate_inverse_count = 0;
        let mut selected_set_revisit_count = 0;
        let mut begin_cancel_repetition_count = 0;
        let mut bootstrap_allowed_count = 0;
        let mut bootstrap_blocked_count = 0;
        let mut bootstrap_counts = BTreeMap::new();
        let mut reward_component_sums = BTreeMap::new();
        for step in &steps {
            let index = usize::from(step.action_kind);
            assert!(
                index < ActionKind::COUNT,
                "invalid action kind index: {index}"
            );
            action_kind_counts[index] += 1;
            match index {
                value if value == ActionKind::SelectHandCard.index() => select_count += 1,
                value if value == ActionKind::DeselectHandCard.index() => deselect_count += 1,
                value if value == ActionKind::ConfirmCardSelection.index() => confirm_count += 1,
                value if value == ActionKind::CancelCardSelection.index() => cancel_count += 1,
                _ => {}
            }
            if step.bootstrap_allowed {
                bootstrap_allowed_count += 1;
            } else {
                bootstrap_blocked_count += 1;
            }
            *bootstrap_counts
                .entry(format!(
                    "{:?}",
                    if step.no_progress_cycle {
                        StepReason::NoProgressCycle
                    } else if step.terminated {
                        StepReason::Terminal
                    } else if step.truncated {
                        StepReason::MaxDecisions
                    } else {
                        StepReason::DecisionPoint
                    }
                ))
                .or_insert(0) += 1;
            *reward_component_sums
                .entry("total".to_string())
                .or_insert(0.0) += f64::from(step.reward);
            *reward_component_sums
                .entry("terminal".to_string())
                .or_insert(0.0) += f64::from(step.terminal_reward);
            *reward_component_sums
                .entry("shaping".to_string())
                .or_insert(0.0) += f64::from(step.shaping_reward);
            if step.no_progress_cycle {
                *reward_component_sums
                    .entry("no_progress_cycle_penalty".to_string())
                    .or_insert(0.0) += f64::from(step.no_progress_cycle_penalty_reward);
            }
        }
        for episode in &self.episodes {
            for (key, value) in &episode.reward_component_sums {
                *reward_component_sums.entry(key.clone()).or_insert(0.0) += value;
            }
            for (key, value) in &episode.decision_point_counts {
                *decision_point_counts.entry(key.clone()).or_insert(0) += value;
            }
        }
        for episode in &self.episodes {
            let episode_steps = &episode.steps;
            for pair in episode_steps.windows(2) {
                let previous = usize::from(pair[0].action_kind);
                let current = usize::from(pair[1].action_kind);
                if matches!((previous, current), (a, b) if (a == ActionKind::SelectHandCard.index() && b == ActionKind::DeselectHandCard.index()) || (a == ActionKind::DeselectHandCard.index() && b == ActionKind::SelectHandCard.index()))
                {
                    immediate_inverse_count += 1;
                }
                if (previous == ActionKind::BeginRerollSelection.index()
                    || previous == ActionKind::BeginTowerSelection.index())
                    && current == ActionKind::CancelCardSelection.index()
                {
                    begin_cancel_repetition_count += 1;
                }
                if pair[0].typed_observation.sets[super::encoding::observation::HAND_CARDS]
                    == pair[1].typed_observation.sets[super::encoding::observation::HAND_CARDS]
                    && !pair[0].typed_observation.sets[super::encoding::observation::HAND_CARDS]
                        .is_empty()
                {
                    selected_set_revisit_count += 1;
                }
            }
        }
        let episode_count = self.episodes.len().max(1) as f32;
        RolloutDiagnostics {
            episode_count: self.episodes.len(),
            decision_steps,
            mean_reward,
            reward_stddev,
            mean_episode_return: mean(&episode_returns),
            mean_action_nll: mean(&action_nlls),
            mean_policy_entropy: mean(&entropies),
            mean_value_loss: mean(&value_losses),
            episode_return_stddev: stddev(&episode_returns, mean(&episode_returns)),
            mean_final_clear_rate: if self.episodes.is_empty() {
                0.0
            } else {
                self.episodes
                    .iter()
                    .map(|episode| episode.final_clear_rate)
                    .sum::<f32>()
                    / self.episodes.len() as f32
            },
            mean_escaped_hp: mean(
                &self
                    .episodes
                    .iter()
                    .map(|episode| episode.escaped_hp)
                    .collect::<Vec<_>>(),
            ),
            mean_player_damage: mean(
                &self
                    .episodes
                    .iter()
                    .map(|episode| episode.player_damage)
                    .collect::<Vec<_>>(),
            ),
            mean_tower_damage: mean(
                &self
                    .episodes
                    .iter()
                    .map(|episode| episode.tower_damage)
                    .collect::<Vec<_>>(),
            ),
            nonzero_reward_fraction: if decision_steps == 0 {
                0.0
            } else {
                rewards.iter().filter(|reward| **reward != 0.0).count() as f32
                    / decision_steps as f32
            },
            terminal_reward_count: steps
                .iter()
                .filter(|step| step.terminal_reward > 0.0)
                .count(),
            death_penalty_count: steps
                .iter()
                .filter(|step| step.terminal_reward < 0.0)
                .count(),
            full_clear_count: self
                .episodes
                .iter()
                .filter(|episode| episode.cleared)
                .count(),
            mean_final_stage: if self.episodes.is_empty() {
                0.0
            } else {
                self.episodes
                    .iter()
                    .map(|episode| episode.final_stage as f32)
                    .sum::<f32>()
                    / self.episodes.len() as f32
            },
            mean_advantage,
            advantage_stddev,
            terminal_episode_count: self
                .episodes
                .iter()
                .filter(|episode| episode.steps.last().is_some_and(|step| step.terminated))
                .count(),
            truncated_episode_count: self
                .episodes
                .iter()
                .filter(|episode| episode.steps.last().is_some_and(|step| step.truncated))
                .count(),
            no_progress_cycle_count: self
                .episodes
                .iter()
                .filter(|episode| {
                    episode
                        .steps
                        .last()
                        .is_some_and(|step| step.no_progress_cycle)
                })
                .count(),
            mean_decisions_per_episode: if self.episodes.is_empty() {
                0.0
            } else {
                self.episodes
                    .iter()
                    .map(|episode| episode.decision_count() as f32)
                    .sum::<f32>()
                    / self.episodes.len() as f32
            },
            action_kind_counts,
            decision_point_counts,
            forced_action_count,
            forced_action_counts,
            select_count,
            deselect_count,
            confirm_count,
            cancel_count,
            immediate_inverse_count,
            selected_set_revisit_count,
            begin_cancel_repetition_count,
            mean_select_count: select_count as f32 / episode_count,
            mean_deselect_count: deselect_count as f32 / episode_count,
            mean_confirm_count: confirm_count as f32 / episode_count,
            mean_cancel_count: cancel_count as f32 / episode_count,
            mean_immediate_inverse_count: immediate_inverse_count as f32 / episode_count,
            mean_selected_set_revisit_count: selected_set_revisit_count as f32 / episode_count,
            mean_begin_cancel_repetition_count: begin_cancel_repetition_count as f32
                / episode_count,
            no_progress_cycle_penalty_sum: reward_component_sums
                .get("no_progress_cycle_penalty")
                .copied()
                .unwrap_or_default(),
            bootstrap_allowed_count,
            bootstrap_blocked_count,
            bootstrap_counts,
            reward_component_sums,
            reward_sum: rewards.iter().map(|value| *value as f64).sum(),
            reward_sum_squares: rewards.iter().map(|value| (*value as f64).powi(2)).sum(),
            episode_return_sum: episode_returns.iter().map(|value| *value as f64).sum(),
            episode_return_sum_squares: episode_returns
                .iter()
                .map(|value| (*value as f64).powi(2))
                .sum(),
            advantage_sum: advantages.iter().map(|value| *value as f64).sum(),
            advantage_sum_squares: advantages.iter().map(|value| (*value as f64).powi(2)).sum(),
        }
    }
}

fn mean(values: &[f32]) -> f32 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f32>() / values.len() as f32
    }
}

fn stddev(values: &[f32], mean: f32) -> f32 {
    if values.is_empty() {
        0.0
    } else {
        (values
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f32>()
            / values.len() as f32)
            .sqrt()
    }
}

pub fn collect_rollouts<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    config: Arc<GameConfig>,
    seeds: &[u64],
    rollout_config: &RolloutConfig,
    progress: Option<&ProgressBar>,
) -> Result<RolloutBatch> {
    rollout_config
        .reward_config
        .validate_gamma(rollout_config.gamma)
        .map_err(anyhow::Error::msg)?;
    let mut episodes = seeds
        .par_iter()
        .map(|&seed| {
            let episode =
                collect_episode(model, device, Arc::clone(&config), seed, rollout_config)?;
            if let Some(progress) = progress {
                progress.inc(1);
            }
            Ok(episode)
        })
        .collect::<Result<Vec<_>>>()?;
    episodes.sort_by_key(|episode| episode.seed);
    Ok(RolloutBatch { episodes })
}

#[cfg(feature = "simulator-wgpu")]
#[allow(dead_code)]
pub(crate) fn stream_rollout_chunks<B: Backend + Send + Sync + 'static>(
    model: DeepSetsActorCritic<B>,
    device: B::Device,
    config: Arc<GameConfig>,
    seeds: Vec<u64>,
    rollout_config: RolloutConfig,
    chunk_size: usize,
    max_queued_steps: usize,
    progress: Option<ProgressBar>,
) -> (
    Receiver<Result<(usize, RolloutBatch, Arc<StepGate>, usize)>>,
    JoinHandle<()>,
)
where
    B::Device: Send + Sync + 'static,
{
    let (sender, receiver) = sync_channel(2);
    let step_gate = Arc::new(StepGate::new(max_queued_steps));
    let producer_gate = Arc::clone(&step_gate);
    let handle = std::thread::spawn(move || {
        let result = (|| -> Result<()> {
            rollout_config
                .reward_config
                .validate_gamma(rollout_config.gamma)
                .map_err(anyhow::Error::msg)?;
            let episode_chunk_size = (chunk_size / 256).clamp(1, 32);
            let rollout_threads = std::thread::available_parallelism()
                .map(|parallelism| parallelism.get().saturating_sub(1).max(1))
                .unwrap_or(1);
            let rollout_pool = ThreadPoolBuilder::new()
                .num_threads(rollout_threads)
                .thread_name(|index| format!("ml-rollout-{index}"))
                .build()?;
            for (chunk_index, seed_chunk) in seeds.chunks(episode_chunk_size).enumerate() {
                let mut episodes = rollout_pool.install(|| {
                    seed_chunk
                        .par_iter()
                        .map(|&seed| {
                            let episode = collect_episode(
                                &model,
                                &device,
                                Arc::clone(&config),
                                seed,
                                &rollout_config,
                            )?;
                            if let Some(progress) = &progress {
                                progress.inc(1);
                            }
                            Ok(episode)
                        })
                        .collect::<Result<Vec<_>>>()
                })?;
                episodes.sort_by_key(|episode| episode.seed);
                let step_count = episodes.iter().map(EpisodeRollout::decision_count).sum();
                producer_gate.acquire(step_count);
                sender
                    .send(Ok((
                        chunk_index,
                        RolloutBatch { episodes },
                        Arc::clone(&producer_gate),
                        step_count,
                    )))
                    .map_err(|_| anyhow::anyhow!("rollout learner channel closed"))?;
            }
            Ok(())
        })();
        if let Err(error) = result {
            let _ = sender.send(Err(error));
        }
    });
    (receiver, handle)
}

#[cfg(feature = "simulator-wgpu")]
pub(crate) struct StepGate {
    state: Mutex<usize>,
    available: Condvar,
    limit: usize,
}

#[cfg(feature = "simulator-wgpu")]
impl StepGate {
    fn new(limit: usize) -> Self {
        Self {
            state: Mutex::new(0),
            available: Condvar::new(),
            limit: limit.max(1),
        }
    }

    fn acquire(&self, steps: usize) {
        let mut queued = self.state.lock().expect("step gate mutex poisoned");
        while *queued != 0 && queued.saturating_add(steps) > self.limit {
            queued = self
                .available
                .wait(queued)
                .expect("step gate mutex poisoned");
        }
        *queued = queued.saturating_add(steps);
    }

    pub(crate) fn release(&self, steps: usize) {
        let mut queued = self.state.lock().expect("step gate mutex poisoned");
        *queued = queued.saturating_sub(steps);
        self.available.notify_all();
    }
}

fn collect_episode<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    config: Arc<GameConfig>,
    seed: u64,
    rollout_config: &RolloutConfig,
) -> Result<EpisodeRollout> {
    let mut random_state = seed
        ^ 0xD1B5_4A32_D192_ED03
        ^ rollout_config
            .exploration_iteration
            .wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let pending = Rc::new(RefCell::new(Vec::new()));
    let pending_for_policy = Rc::clone(&pending);
    let pending_for_transition = Rc::clone(&pending);
    let nonfinite_error = Rc::new(RefCell::new(None::<String>));
    let nonfinite_error_for_transition = Rc::clone(&nonfinite_error);
    let mut steps = Vec::new();
    let mut reward_component_sums = BTreeMap::<String, f64>::new();
    let mut decision_point_counts = BTreeMap::<String, usize>::new();
    let episode = run_episode_with_step_callback(
        config,
        seed,
        &PolicyRunnerConfig {
            max_decisions_per_episode: rollout_config.max_decisions_per_episode,
            record_steps: false,
            max_stage: rollout_config.max_stage,
            reward_config: rollout_config.reward_config.clone(),
        },
        |observation: &Observation, legal_actions: &[LegalAction]| {
            let state = observation_features(observation);
            let typed_observation = TypedObservation::from_observation(observation);
            let candidate_rows = candidate_rows_for_legal_actions(observation, legal_actions);
            let typed_state = model.encode_typed_sets(&typed_batches(observation), device);
            let candidates = PaddedEntityBatch::from_sets(
                &candidate_rows
                    .iter()
                    .map(|row| EntitySet::new(vec![row.clone()]))
                    .collect::<Vec<_>>(),
            );
            let (logits_tensor, value_tensor) = model.forward_typed_policy_and_value(
                tensor_from_repeated_row(&state, candidate_rows.len(), device),
                &candidates,
                typed_state,
                device,
            );
            let logits = logits_tensor
                .into_data()
                .to_vec::<f32>()
                .expect("Burn logits must be f32");
            if !logits.iter().all(|logit| logit.is_finite()) {
                bail!("non-finite rollout logit for seed {seed}");
            }
            let legal_mask = vec![true; candidate_rows.len()];
            let policy_entropy = entropy_for_logits(&logits, &legal_mask);
            let sampling_temperature = if rollout_config.greedy {
                0.0
            } else {
                rollout_temperature(rollout_config)
            };
            let (action_index, old_log_probability) = sample_action_with_mask(
                &logits,
                &legal_mask,
                sampling_temperature,
                &mut random_state,
            );
            let value_estimate = value_tensor
                .into_data()
                .to_vec::<f32>()
                .expect("Burn values must be f32")[0];
            if !value_estimate.is_finite() {
                bail!("non-finite rollout value for seed {seed}");
            }
            pending_for_policy.borrow_mut().push(RolloutDecision {
                state: state.clone(),
                typed_observation,
                candidate_rows,
                legal_candidate_mask: legal_mask,
                action_index,
                old_log_probability,
                value_estimate,
                policy_entropy,
            });
            Ok::<AgentAction, anyhow::Error>(legal_actions[action_index].action.clone())
        },
        |observation, _legal_actions, action, outcome| {
            let decision = pending_for_transition
                .borrow_mut()
                .pop()
                .expect("callback must follow policy selection");
            let next_state = observation_features(&outcome.observation);
            let action_kind = action.kind().index() as u8;
            let no_progress_cycle = outcome.info.no_progress_cycle;
            *reward_component_sums
                .entry("terminal".to_string())
                .or_insert(0.0) += f64::from(outcome.reward.terminal);
            for (key, value) in &outcome.reward.shaping {
                *reward_component_sums.entry(key.clone()).or_insert(0.0) += f64::from(*value);
            }
            *decision_point_counts
                .entry(format!("{:?}", observation.decision_point))
                .or_insert(0) += 1;
            let bootstrap_allowed = !outcome.terminated;
            let next_value_estimate = if !bootstrap_allowed {
                0.0
            } else {
                value_for_typed(model, device, &outcome.observation, &next_state)
            };
            if !next_value_estimate.is_finite() {
                *nonfinite_error_for_transition.borrow_mut() =
                    Some(format!("non-finite rollout next value for seed {seed}"));
                return;
            }
            steps.push(RolloutStep {
                state: decision.state,
                typed_observation: decision.typed_observation,
                candidate_rows: decision.candidate_rows,
                legal_candidate_mask: decision.legal_candidate_mask,
                action_index: decision.action_index,
                old_log_probability: decision.old_log_probability,
                value_estimate: decision.value_estimate,
                next_value_estimate,
                reward: outcome.reward.total(),
                terminal_reward: outcome.reward.terminal,
                shaping_reward: outcome.reward.shaping.values().sum(),
                no_progress_cycle_penalty_reward: outcome
                    .reward
                    .shaping
                    .get("no_progress_cycle_penalty")
                    .copied()
                    .unwrap_or_default(),
                terminated: outcome.terminated,
                truncated: outcome.truncated,
                no_progress_cycle,
                bootstrap_allowed,
                action_kind,
                advantage: 0.0,
                policy_entropy: decision.policy_entropy,
                return_value: 0.0,
            });
        },
    )?;
    if let Some(error) = nonfinite_error.borrow_mut().take() {
        bail!("{error}");
    }
    let cleared = steps.iter().any(|step| step.terminal_reward > 0.0);
    compute_gae(&mut steps, rollout_config.gamma, rollout_config.gae_lambda);
    Ok(EpisodeRollout {
        seed,
        steps,
        cleared,
        final_clear_rate: episode.clear_rate / 100.0,
        final_stage: episode.final_observation.stage,
        escaped_hp: episode.metrics.total_escaped_hp,
        player_damage: episode.metrics.total_player_damage,
        tower_damage: episode.metrics.total_tower_damage,
        episode_return: episode.episode_return,
        termination_reason: episode.termination_reason,
        reward_component_sums,
        decision_point_counts,
        forced_action_count: episode.forced_actions.total,
        forced_action_counts: episode.forced_actions.by_decision_point,
    })
}

fn rollout_temperature(config: &RolloutConfig) -> f32 {
    if !config.adaptive_exploration {
        return config.temperature;
    }
    let iteration_scale = (config.exploration_iteration.saturating_add(1) as f32).sqrt();
    config.temperature * (1.0 + 0.25 / iteration_scale)
}

fn entropy_for_logits(logits: &[f32], mask: &[bool]) -> f32 {
    let max_logit = logits
        .iter()
        .zip(mask)
        .filter_map(|(logit, valid)| valid.then_some(*logit))
        .fold(f32::NEG_INFINITY, f32::max);
    let normalizer = logits
        .iter()
        .zip(mask)
        .map(|(logit, valid)| {
            if *valid {
                (*logit - max_logit).exp()
            } else {
                0.0
            }
        })
        .sum::<f32>();
    if !normalizer.is_finite() || normalizer <= 0.0 {
        return f32::NAN;
    }
    logits
        .iter()
        .zip(mask)
        .map(|(logit, valid)| {
            if !*valid {
                0.0
            } else {
                let probability = (*logit - max_logit).exp() / normalizer;
                if probability > 0.0 {
                    -probability * probability.ln()
                } else {
                    0.0
                }
            }
        })
        .sum()
}

struct RolloutDecision {
    state: Vec<f32>,
    typed_observation: TypedObservation,
    candidate_rows: Vec<EntityRow>,
    legal_candidate_mask: Vec<bool>,
    action_index: usize,
    old_log_probability: f32,
    value_estimate: f32,
    policy_entropy: f32,
}

pub fn logits_for<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    observation: &Observation,
    state: &[f32],
    candidate_rows: &[EntityRow],
) -> Vec<f32> {
    let typed_state = model.encode_typed_sets(&typed_batches(observation), device);
    let typed_state = typed_state.repeat_dim(0, candidate_rows.len());
    let candidates = PaddedEntityBatch::from_sets(
        &candidate_rows
            .iter()
            .map(|row| EntitySet::new(vec![row.clone()]))
            .collect::<Vec<_>>(),
    );
    let logits = model.forward_typed_logits_with_candidates(
        tensor_from_repeated_row(state, candidate_rows.len(), device),
        &candidates,
        typed_state,
        device,
    );
    logits
        .into_data()
        .to_vec::<f32>()
        .expect("Burn logits must be f32")
}

fn typed_batches(
    observation: &Observation,
) -> [PaddedEntityBatch; crate::simulator::ml::encoding::ENTITY_SET_COUNT] {
    let typed = TypedObservation::from_observation(observation);
    std::array::from_fn(|index| PaddedEntityBatch::from_sets(&[typed.sets[index].clone()]))
}

pub fn value_for<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    state: &[f32],
) -> f32 {
    values_for_rows(model, device, &[state.to_vec()])[0]
}

fn value_for_typed<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    observation: &Observation,
    _state: &[f32],
) -> f32 {
    let typed_state = model.encode_typed_sets(&typed_batches(observation), device);
    model
        .forward_typed_values(typed_state)
        .into_data()
        .to_vec::<f32>()
        .expect("Burn values must be f32")[0]
}

fn values_for_rows<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    states: &[Vec<f32>],
) -> Vec<f32> {
    let columns = states.first().map_or(0, Vec::len);
    let values = model.forward_values(tensor_from_flat_rows(
        states.iter().flatten().copied().collect(),
        states.len(),
        columns,
        device,
    ));
    values
        .into_data()
        .to_vec::<f32>()
        .expect("Burn values must be f32")
}

pub fn compute_gae(steps: &mut [RolloutStep], gamma: f32, gae_lambda: f32) {
    let mut gae = 0.0;
    for step in steps.iter_mut().rev() {
        let continuation = if step.bootstrap_allowed { 1.0 } else { 0.0 };
        let delta =
            step.reward + gamma * continuation * step.next_value_estimate - step.value_estimate;
        gae = delta + gamma * gae_lambda * continuation * gae;
        step.advantage = gae;
        step.return_value = gae + step.value_estimate;
    }
}

pub fn ppo_loss_for_step<B: burn::tensor::backend::AutodiffBackend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    step: &RolloutStep,
    clip_epsilon: f32,
    entropy_coefficient: f32,
    value_coefficient: f32,
) -> Tensor<B, 1> {
    let typed = step.typed_observation.clone();
    let typed_batches =
        std::array::from_fn(|index| PaddedEntityBatch::from_sets(&[typed.sets[index].clone()]));
    let typed_state = model.encode_typed_sets(&typed_batches, device);
    let typed_state = typed_state.repeat_dim(0, 1);
    let candidates = PaddedEntityBatch::from_sets(
        &step
            .candidate_rows
            .iter()
            .map(|row| EntitySet::new(vec![row.clone()]))
            .collect::<Vec<_>>(),
    );
    let logits = model.forward_typed_logits_with_candidates(
        tensor_from_repeated_row(&step.state, 1, device),
        &candidates,
        typed_state.clone(),
        device,
    );
    let logits = logits.reshape([1, step.candidate_count()]);
    assert_eq!(step.legal_candidate_mask.len(), step.candidate_count());
    assert!(step.legal_candidate_mask.iter().any(|legal| *legal));
    let legal_mask = Tensor::from_data(
        TensorData::new(
            step.legal_candidate_mask
                .iter()
                .map(|legal| if *legal { 0.0 } else { -1.0e9 })
                .collect::<Vec<_>>(),
            [1, step.candidate_count()],
        ),
        device,
    );
    let probabilities = softmax(logits + legal_mask, 1);
    let log_probabilities = probabilities.clone().log();
    let mut target = vec![0.0; step.candidate_count()];
    target[step.action_index] = 1.0;
    let target = Tensor::from_data(TensorData::new(target, [1, step.candidate_count()]), device);
    let current_log_probability = (log_probabilities.clone() * target).sum();
    let ratio = (current_log_probability.sub_scalar(step.old_log_probability)).exp();
    let clipped_ratio = ratio.clone().clamp(1.0 - clip_epsilon, 1.0 + clip_epsilon);
    let advantage = Tensor::from_floats([step.advantage], device);
    let surrogate = ratio * advantage.clone();
    let clipped_surrogate = clipped_ratio * advantage;
    let policy_loss = surrogate.min_pair(clipped_surrogate).neg();
    let entropy = (probabilities * log_probabilities).sum().neg();
    let value = model.forward_typed_values(typed_state);
    let expected_value = Tensor::from_floats([step.return_value], device);
    let value_loss = (value.reshape([1]) - expected_value).powf_scalar(2.0).sum();
    policy_loss + value_loss * value_coefficient - entropy * entropy_coefficient
}

#[cfg(test)]
fn sample_action(logits: &[f32], temperature: f32, random_state: &mut u64) -> (usize, f32) {
    sample_action_with_mask(logits, &vec![true; logits.len()], temperature, random_state)
}

fn sample_action_with_mask(
    logits: &[f32],
    legal_mask: &[bool],
    temperature: f32,
    random_state: &mut u64,
) -> (usize, f32) {
    assert_eq!(logits.len(), legal_mask.len());
    assert!(
        legal_mask.iter().any(|legal| *legal),
        "candidate set has no legal action"
    );
    assert!(
        logits.iter().all(|logit| logit.is_finite()),
        "policy produced non-finite logits"
    );
    let masked_logits = logits
        .iter()
        .zip(legal_mask)
        .map(|(logit, legal)| if *legal { *logit } else { f32::NEG_INFINITY })
        .collect::<Vec<_>>();
    let scale = temperature.max(0.0);
    let (probabilities, log_probabilities) = if scale == 0.0 {
        let index = masked_logits
            .iter()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(right.1))
            .map(|(index, _)| index)
            .unwrap_or(0);
        let mut probabilities: Vec<f32> = vec![0.0; logits.len()];
        probabilities[index] = 1.0;
        let log_probabilities = probabilities
            .iter()
            .map(|probability| (*probability).max(f32::MIN_POSITIVE).ln())
            .collect::<Vec<_>>();
        (probabilities, log_probabilities)
    } else {
        let maximum = masked_logits
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        let weights = masked_logits
            .iter()
            .map(|logit| ((logit - maximum) / scale).exp())
            .collect::<Vec<_>>();
        let total = weights.iter().sum::<f32>().max(f32::MIN_POSITIVE);
        let mut probabilities = weights
            .iter()
            .map(|weight| weight / total)
            .collect::<Vec<_>>();
        for (probability, legal) in probabilities.iter_mut().zip(legal_mask) {
            if !legal {
                *probability = 0.0;
            }
        }
        assert!(
            probabilities
                .iter()
                .all(|probability| probability.is_finite())
                && probabilities.iter().sum::<f32>().is_finite(),
            "policy produced non-finite probabilities"
        );
        let log_probabilities = probabilities
            .iter()
            .map(|probability| (*probability).max(f32::MIN_POSITIVE).ln())
            .collect::<Vec<_>>();
        (probabilities, log_probabilities)
    };

    let sample = next_random(random_state);
    let mut cumulative = 0.0;
    for (index, probability) in probabilities.iter().enumerate() {
        cumulative += probability;
        if sample <= cumulative {
            return (index, log_probabilities[index]);
        }
    }
    let index = probabilities.len().saturating_sub(1);
    (index, log_probabilities[index])
}

fn next_random(state: &mut u64) -> f32 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);
    ((*state >> 32) as u32) as f32 / u32::MAX as f32
}

#[cfg(test)]
mod tests {
    use super::super::diagnostics::duplicate_candidate_row_groups;
    use super::*;
    use crate::config::GameConfig;
    use crate::simulator::ml::model::default_policy_device;
    use crate::simulator::policy_runner::run_scripted_oracle_trajectory;
    use std::sync::Arc;

    #[test]
    fn exploration_iteration_changes_sampling_stream() {
        let logits = [0.0, 0.0, 0.0, 0.0];
        let mut first = 7_u64 ^ 0xD1B5_4A32_D192_ED03;
        let mut second = first ^ 0x9E37_79B9_7F4A_7C15;
        let first = sample_action(&logits, 1.0, &mut first);
        let second = sample_action(&logits, 1.0, &mut second);
        assert_ne!(first, second);
    }

    #[test]
    fn adaptive_temperature_is_train_only_and_decays_by_iteration() {
        let base = RolloutConfig::default();
        assert_eq!(rollout_temperature(&base), base.temperature);

        let adaptive = RolloutConfig {
            adaptive_exploration: true,
            exploration_iteration: 0,
            ..base.clone()
        };
        let first_temperature = rollout_temperature(&adaptive);
        let later_temperature = rollout_temperature(&RolloutConfig {
            exploration_iteration: 9,
            ..adaptive.clone()
        });
        assert!(first_temperature > later_temperature);
        assert!(later_temperature > base.temperature);
    }

    #[test]
    fn diagnostics_episode_count_includes_all_rollouts() {
        let batch = RolloutBatch {
            episodes: vec![
                EpisodeRollout {
                    seed: 1,
                    steps: Vec::new(),
                    cleared: false,
                    final_clear_rate: 0.0,
                    final_stage: 1,
                    escaped_hp: 0.0,
                    player_damage: 0.0,
                    tower_damage: 0.0,
                    episode_return: 0.0,
                    termination_reason: StepReason::Terminal,
                    reward_component_sums: BTreeMap::new(),
                    decision_point_counts: BTreeMap::new(),
                    forced_action_count: 0,
                    forced_action_counts: BTreeMap::new(),
                },
                EpisodeRollout {
                    seed: 2,
                    steps: Vec::new(),
                    cleared: false,
                    final_clear_rate: 0.0,
                    final_stage: 1,
                    escaped_hp: 0.0,
                    player_damage: 0.0,
                    tower_damage: 0.0,
                    episode_return: 0.0,
                    termination_reason: StepReason::Terminal,
                    reward_component_sums: BTreeMap::new(),
                    decision_point_counts: BTreeMap::new(),
                    forced_action_count: 0,
                    forced_action_counts: BTreeMap::new(),
                },
            ],
        };
        assert_eq!(batch.diagnostics().episode_count, 2);
    }

    #[test]
    fn non_finite_logits_are_rejected() {
        let result = std::panic::catch_unwind(|| {
            let mut random_state = 1;
            sample_action(&[f32::NAN, 0.0], 1.0, &mut random_state);
        });
        assert!(result.is_err());
    }

    #[test]
    fn illegal_candidates_have_zero_probability_and_are_never_sampled() {
        let logits = [100.0, 0.0, 50.0];
        let mask = [false, true, false];
        let mut random_state = 1;
        let (index, log_probability) =
            sample_action_with_mask(&logits, &mask, 1.0, &mut random_state);
        assert_eq!(index, 1);
        assert_eq!(log_probability, 0.0);
    }

    #[test]
    fn duplicate_candidate_rows_report_only_colliding_indices() {
        let rows = vec![
            EntityRow::new([1, 2, 3, 4], vec![0.0, 1.0]),
            EntityRow::new([1, 2, 3, 4], vec![0.0, 1.0]),
            EntityRow::new([1, 2, 3, 4], vec![1.0, 0.0]),
            EntityRow::new([4, 3, 2, 1], vec![0.0, 1.0]),
        ];

        assert_eq!(duplicate_candidate_row_groups(&rows), vec![vec![0, 1]]);
    }

    #[test]
    fn supervised_nll_uses_raw_logits_from_scripted_trajectory() {
        let config = GameConfig::default_config();
        let trajectory =
            run_scripted_oracle_trajectory(Arc::new(config), 19).expect("scripted trajectory");
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<super::super::model::InferenceBackend>::new(
            super::super::model::ModelConfig::default(),
            &device,
        );

        let report = evaluate_supervised_nll(&model, &device, &trajectory)
            .expect("raw-logit NLL should evaluate");

        assert_eq!(report.sample_count, trajectory.steps.len());
        assert!(report.mean_nll.is_finite());
        assert!(report.top1_accuracy.is_finite());
        assert!((0.0..=1.0).contains(&report.top1_accuracy));
    }

    #[test]
    fn gae_bootstraps_truncation_but_not_terminal_steps() {
        let mut steps = vec![
            RolloutStep {
                typed_observation: TypedObservation::default(),
                legal_candidate_mask: vec![true],
                state: vec![],
                candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0, 1.0, 0.0, 0.0, 0.0])],
                action_index: 0,
                old_log_probability: 0.0,
                value_estimate: 0.0,
                next_value_estimate: 2.0,
                reward: 1.0,
                terminal_reward: 0.0,
                shaping_reward: 1.0,
                no_progress_cycle_penalty_reward: 0.0,
                terminated: false,
                truncated: true,
                no_progress_cycle: false,
                bootstrap_allowed: true,
                action_kind: 0,
                advantage: 0.0,
                return_value: 0.0,
                policy_entropy: 0.0,
            },
            RolloutStep {
                typed_observation: TypedObservation::default(),
                legal_candidate_mask: vec![true],
                state: vec![],
                candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0, 1.0, 0.0, 0.0, 0.0])],
                action_index: 0,
                old_log_probability: 0.0,
                value_estimate: 0.0,
                next_value_estimate: 2.0,
                reward: 1.0,
                terminal_reward: 1.0,
                shaping_reward: 0.0,
                no_progress_cycle_penalty_reward: 0.0,
                terminated: true,
                truncated: false,
                no_progress_cycle: false,
                bootstrap_allowed: false,
                action_kind: 0,
                advantage: 0.0,
                return_value: 0.0,
                policy_entropy: 0.0,
            },
        ];
        compute_gae(&mut steps, 0.5, 0.5);
        assert!(steps[0].advantage > steps[1].advantage);
        assert_eq!(steps[1].advantage, 1.0);
    }

    #[test]
    fn gae_bootstraps_no_progress_cycle_event() {
        let mut steps = vec![RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask: vec![true],
            state: vec![],
            candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0, 1.0, 0.0, 0.0, 0.0])],
            action_index: 0,
            old_log_probability: 0.0,
            value_estimate: 2.0,
            next_value_estimate: 10.0,
            reward: -0.25,
            terminal_reward: 0.0,
            shaping_reward: -0.25,
            no_progress_cycle_penalty_reward: -0.25,
            terminated: false,
            truncated: false,
            no_progress_cycle: true,
            bootstrap_allowed: true,
            action_kind: 0,
            advantage: 0.0,
            return_value: 0.0,
            policy_entropy: 0.0,
        }];
        compute_gae(&mut steps, 0.99, 0.95);
        assert!((steps[0].advantage - 7.65).abs() < 1e-6);
    }

    #[test]
    fn diagnostics_isolates_cycle_penalty_from_other_shaping() {
        let step = RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask: vec![true],
            state: vec![],
            candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
            action_index: 0,
            old_log_probability: 0.0,
            value_estimate: 0.0,
            next_value_estimate: 0.0,
            reward: -0.5,
            terminal_reward: 0.0,
            shaping_reward: -0.5,
            no_progress_cycle_penalty_reward: -0.25,
            terminated: false,
            truncated: true,
            no_progress_cycle: true,
            bootstrap_allowed: false,
            action_kind: 0,
            advantage: -0.5,
            return_value: -0.5,
            policy_entropy: 0.0,
        };
        let batch = RolloutBatch {
            episodes: vec![EpisodeRollout {
                seed: 1,
                steps: vec![step],
                cleared: false,
                final_clear_rate: 0.0,
                final_stage: 0,
                escaped_hp: 0.0,
                player_damage: 0.0,
                tower_damage: 0.0,
                episode_return: -0.5,
                termination_reason: StepReason::NoProgressCycle,
                reward_component_sums: BTreeMap::new(),
                decision_point_counts: BTreeMap::new(),
                forced_action_count: 0,
                forced_action_counts: BTreeMap::new(),
            }],
        };
        let diagnostics = batch.diagnostics();
        assert_eq!(diagnostics.reward_component_sums["shaping"], -0.5);
        assert_eq!(
            diagnostics.reward_component_sums["no_progress_cycle_penalty"],
            -0.25
        );
        assert_eq!(diagnostics.no_progress_cycle_penalty_sum, -0.25);
    }

    #[test]
    fn reward_components_reconstruct_total_reward() {
        let steps = [
            RolloutStep {
                typed_observation: TypedObservation::default(),
                legal_candidate_mask: vec![true],
                state: vec![],
                candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
                action_index: 0,
                old_log_probability: 0.0,
                value_estimate: 0.0,
                next_value_estimate: 0.0,
                reward: 1.25,
                terminal_reward: 1.0,
                shaping_reward: 0.25,
                no_progress_cycle_penalty_reward: 0.0,
                terminated: true,
                truncated: false,
                no_progress_cycle: false,
                bootstrap_allowed: false,
                action_kind: 0,
                advantage: 0.0,
                return_value: 0.0,
                policy_entropy: 0.0,
            },
            RolloutStep {
                typed_observation: TypedObservation::default(),
                legal_candidate_mask: vec![true],
                state: vec![],
                candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
                action_index: 0,
                old_log_probability: 0.0,
                value_estimate: 0.0,
                next_value_estimate: 0.0,
                reward: -0.5,
                terminal_reward: 0.0,
                shaping_reward: -0.5,
                no_progress_cycle_penalty_reward: -0.25,
                terminated: false,
                truncated: true,
                no_progress_cycle: true,
                bootstrap_allowed: false,
                action_kind: 0,
                advantage: 0.0,
                return_value: 0.0,
                policy_entropy: 0.0,
            },
        ];
        let total: f32 = steps.iter().map(|step| step.reward).sum();
        let components = steps
            .iter()
            .map(|step| step.terminal_reward + step.shaping_reward)
            .sum::<f32>();
        assert_eq!(total, components);
    }

    #[test]
    fn gae_single_cycle_penalty_has_negative_advantage_and_return() {
        let mut steps = vec![RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask: vec![true],
            state: vec![],
            candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
            action_index: 0,
            old_log_probability: 0.0,
            value_estimate: 0.0,
            next_value_estimate: 7.0,
            reward: -0.25,
            terminal_reward: 0.0,
            shaping_reward: -0.25,
            no_progress_cycle_penalty_reward: -0.25,
            terminated: false,
            truncated: true,
            no_progress_cycle: true,
            bootstrap_allowed: false,
            action_kind: 0,
            advantage: 0.0,
            return_value: 0.0,
            policy_entropy: 0.0,
        }];
        compute_gae(&mut steps, 0.99, 0.95);
        assert_eq!(steps[0].advantage, -0.25);
        assert_eq!(steps[0].return_value, -0.25);
    }

    #[test]
    fn gae_max_decisions_keeps_bootstrap() {
        let mut steps = vec![RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask: vec![true],
            state: vec![],
            candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
            action_index: 0,
            old_log_probability: 0.0,
            value_estimate: 0.0,
            next_value_estimate: 2.0,
            reward: 0.0,
            terminal_reward: 0.0,
            shaping_reward: 0.0,
            no_progress_cycle_penalty_reward: 0.0,
            terminated: false,
            truncated: true,
            no_progress_cycle: false,
            bootstrap_allowed: true,
            action_kind: 0,
            advantage: 0.0,
            return_value: 0.0,
            policy_entropy: 0.0,
        }];
        compute_gae(&mut steps, 0.5, 0.95);
        assert_eq!(steps[0].advantage, 1.0);
        assert_eq!(steps[0].return_value, 1.0);
    }

    #[test]
    fn gae_does_not_cross_episode_or_chunk_boundaries() {
        let make_step = |reward: f32, bootstrap_allowed: bool| RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask: vec![true],
            state: vec![],
            candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
            action_index: 0,
            old_log_probability: 0.0,
            value_estimate: 0.0,
            next_value_estimate: 0.0,
            reward,
            terminal_reward: 0.0,
            shaping_reward: reward,
            no_progress_cycle_penalty_reward: 0.0,
            terminated: !bootstrap_allowed,
            truncated: false,
            no_progress_cycle: false,
            bootstrap_allowed,
            action_kind: 0,
            advantage: 0.0,
            return_value: 0.0,
            policy_entropy: 0.0,
        };
        let mut first_chunk = vec![make_step(0.0, true), make_step(1.0, false)];
        let mut second_chunk = vec![make_step(2.0, false)];
        compute_gae(&mut first_chunk, 0.5, 1.0);
        compute_gae(&mut second_chunk, 0.5, 1.0);
        assert_eq!(first_chunk[0].advantage, 0.5);
        assert_eq!(second_chunk[0].advantage, 2.0);
    }

    #[test]
    fn cycle_penalty_is_applied_once_on_the_final_transition() {
        let mut steps = vec![
            RolloutStep {
                typed_observation: TypedObservation::default(),
                legal_candidate_mask: vec![true],
                state: vec![],
                candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
                action_index: 0,
                old_log_probability: 0.0,
                value_estimate: 0.0,
                next_value_estimate: 0.0,
                reward: 0.0,
                terminal_reward: 0.0,
                shaping_reward: 0.0,
                no_progress_cycle_penalty_reward: 0.0,
                terminated: false,
                truncated: false,
                no_progress_cycle: false,
                bootstrap_allowed: true,
                action_kind: 0,
                advantage: 0.0,
                return_value: 0.0,
                policy_entropy: 0.0,
            },
            RolloutStep {
                typed_observation: TypedObservation::default(),
                legal_candidate_mask: vec![true],
                state: vec![],
                candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
                action_index: 0,
                old_log_probability: 0.0,
                value_estimate: 0.0,
                next_value_estimate: 100.0,
                reward: -0.25,
                terminal_reward: 0.0,
                shaping_reward: -0.25,
                no_progress_cycle_penalty_reward: -0.25,
                terminated: false,
                truncated: true,
                no_progress_cycle: true,
                bootstrap_allowed: false,
                action_kind: 0,
                advantage: 0.0,
                return_value: 0.0,
                policy_entropy: 0.0,
            },
        ];
        compute_gae(&mut steps, 0.99, 0.95);
        assert_eq!(steps[1].advantage, -0.25);
        assert!((steps[0].advantage + 0.99 * 0.95 * 0.25).abs() < 1e-6);
    }

    #[test]
    fn release_throughput_smoke_benchmark() {
        let device = crate::simulator::ml::model::default_policy_device();
        let model = crate::simulator::ml::model::DeepSetsActorCritic::<
            crate::simulator::ml::model::InferenceBackend,
        >::new(crate::simulator::ml::model::ModelConfig::default(), &device);
        let config = std::sync::Arc::new(crate::config::GameConfig::default_config());
        let rollout_config = RolloutConfig {
            max_decisions_per_episode: 300,
            ..RolloutConfig::default()
        };
        let started = std::time::Instant::now();
        let batch = collect_rollouts(
            &model,
            &device,
            config,
            &[11, 12, 13, 14],
            &rollout_config,
            None,
        )
        .expect("benchmark rollout should succeed");
        let elapsed = started.elapsed().as_secs_f64();
        let decisions = batch.steps().count();
        let episodes_per_second = batch.episodes.len() as f64 / elapsed;
        let decisions_per_second = decisions as f64 / elapsed;
        println!(
            "throughput episodes={} decisions={} elapsed={:.2}s eps={:.2} dps={:.2}",
            batch.episodes.len(),
            decisions,
            elapsed,
            episodes_per_second,
            decisions_per_second
        );
        assert!(episodes_per_second > 0.0);
        assert!(decisions_per_second > 0.0);
    }
}
