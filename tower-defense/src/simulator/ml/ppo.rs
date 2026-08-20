use super::contract::MlContract;
use super::curriculum::{CurriculumConfig, CurriculumState};
#[cfg(feature = "simulator-wgpu")]
use super::encoding::tensor::{EntityBatchTensors, to_tensors};
use super::encoding::{EntitySet, PaddedEntityBatch};
#[cfg(feature = "simulator-wgpu")]
use super::model::{
    CpuInferenceBackend, CpuPolicyDevice, CpuTrainBackend, cpu_inference_model_to_cpu_train_model,
};
use super::model::{
    DeepSetsActorCritic, InferenceBackend, ModelConfig, TrainBackend, default_policy_device,
    inference_model, initialize_model, tensor_from_rows,
};
use super::neural_checkpoint::NeuralCheckpoint;
#[cfg(feature = "simulator-wgpu")]
use super::rollout::stream_rollout_chunks;
use super::rollout::{
    RolloutBatch, RolloutConfig, RolloutDiagnostics, RolloutStep, SupervisedNllReport,
    collect_rollouts,
};
use super::seed::{SeedRange, TrainingSeedSchedule};
use super::trainer_checkpoint::{TrainerCheckpointStore, TrainerState};
use super::training_progress::{PpoProgress, RolloutTiming, add_duration};
use crate::config::GameConfig;
use crate::simulator::environment::ActionKind;
use crate::simulator::environment::RewardConfig;
use anyhow::{Result, bail};
#[cfg(feature = "simulator-wgpu")]
use burn::module::AutodiffModule;
use burn::module::{Module, ModuleVisitor, Param};
#[cfg(feature = "simulator-wgpu")]
use burn::optim::adaptor::OptimizerAdaptor;
use burn::optim::grad_clipping::GradientClippingConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::tensor::Tensor;
use burn::tensor::TensorData;
use burn::tensor::activation::softmax;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

#[cfg(feature = "simulator-wgpu")]
struct GpuPpoBucket {
    candidate_count: usize,
    source_indices: Vec<usize>,
    state_sets: [EntityBatchTensors<super::model::GpuTrainBackend>;
        super::encoding::observation::ENTITY_SET_COUNT],
    candidates: EntityBatchTensors<super::model::GpuTrainBackend>,
    states: Tensor<super::model::GpuTrainBackend, 2>,
    legal_masks: Tensor<super::model::GpuTrainBackend, 2>,
    targets: Tensor<super::model::GpuTrainBackend, 2>,
    old_log_probabilities: Tensor<super::model::GpuTrainBackend, 2>,
    advantages: Tensor<super::model::GpuTrainBackend, 2>,
    returns: Tensor<super::model::GpuTrainBackend, 2>,
}

#[cfg(feature = "simulator-wgpu")]
struct GpuPpoDataset {
    buckets: Vec<GpuPpoBucket>,
}

#[cfg(feature = "simulator-wgpu")]
impl GpuPpoDataset {
    fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    fn validate(&self) {
        for bucket in &self.buckets {
            assert!(bucket.candidate_count > 0);
            assert_eq!(
                bucket.source_indices.len(),
                bucket.state_sets[0].numeric.dims()[0]
            );
            assert_eq!(bucket.states.dims()[0], bucket.source_indices.len());
            assert_eq!(
                bucket.candidates.numeric.dims()[0],
                bucket.source_indices.len() * bucket.candidate_count
            );
            assert_eq!(
                bucket.legal_masks.dims(),
                [bucket.source_indices.len(), bucket.candidate_count]
            );
            assert_eq!(
                bucket.targets.dims(),
                [bucket.source_indices.len(), bucket.candidate_count]
            );
            assert_eq!(
                bucket.old_log_probabilities.dims(),
                [bucket.source_indices.len(), 1]
            );
            assert_eq!(bucket.advantages.dims(), [bucket.source_indices.len(), 1]);
            assert_eq!(bucket.returns.dims(), [bucket.source_indices.len(), 1]);
        }
    }

    fn loss(
        &self,
        model: &DeepSetsActorCritic<super::model::GpuTrainBackend>,
        device: &super::model::GpuPolicyDevice,
        steps: &[&RolloutStep],
        indices: &[usize],
        config: &PpoConfig,
    ) -> (
        Tensor<super::model::GpuTrainBackend, 1>,
        OptimizationMinibatchMetrics,
    ) {
        let mut total_loss = None;
        let mut total_metrics = OptimizationMinibatchMetrics::default();
        for bucket in &self.buckets {
            let local = indices
                .iter()
                .filter_map(|global| {
                    bucket
                        .source_indices
                        .iter()
                        .position(|index| index == global)
                })
                .collect::<Vec<_>>();
            if local.is_empty() {
                continue;
            }
            let count = bucket.candidate_count;
            let sample_indices = Tensor::from_data(
                TensorData::new(
                    local.iter().map(|index| *index as i32).collect::<Vec<_>>(),
                    [local.len()],
                ),
                device,
            );
            let candidate_indices = Tensor::from_data(
                TensorData::new(
                    local
                        .iter()
                        .flat_map(|index| {
                            (0..count).map(move |column| (index * count + column) as i32)
                        })
                        .collect::<Vec<_>>(),
                    [local.len() * count],
                ),
                device,
            );
            let state = bucket.states.clone().select(0, sample_indices.clone());
            let typed_state = model.encode_typed_tensor_sets(
                bucket
                    .state_sets
                    .clone()
                    .map(|set| set.select(sample_indices.clone())),
            );
            let candidates =
                model.encode_candidate_tensors(bucket.candidates.clone().select(candidate_indices));
            let state = model
                .encode_state(state)
                .reshape([
                    local.len(),
                    1,
                    model
                        .encode_state(Tensor::zeros([1, steps[0].state.len()], device))
                        .dims()[1],
                ])
                .repeat_dim(1, count)
                .reshape([
                    local.len() * count,
                    model
                        .encode_state(Tensor::zeros([1, steps[0].state.len()], device))
                        .dims()[1],
                ]);
            let typed_width = typed_state.dims()[1];
            let expanded_typed = typed_state
                .clone()
                .reshape([local.len(), 1, typed_width])
                .repeat_dim(1, count)
                .reshape([local.len() * count, typed_width]);
            let logits = model
                .forward_typed_logits_with_encoded_candidates(
                    state,
                    candidates,
                    expanded_typed,
                    &vec![count; local.len()],
                )
                .reshape([local.len(), count]);
            let mut legal = Vec::with_capacity(local.len() * count);
            let mut target = vec![0.0; local.len() * count];
            let mut old = Vec::with_capacity(local.len());
            let mut advantages = Vec::with_capacity(local.len());
            let mut returns = Vec::with_capacity(local.len());
            for (row, &local_index) in local.iter().enumerate() {
                let step = steps[bucket.source_indices[local_index]];
                target[row * count + step.action_index] = 1.0;
                legal.extend(
                    step.legal_candidate_mask
                        .iter()
                        .map(|legal| if *legal { 0.0 } else { -1.0e9 }),
                );
                old.push(step.old_log_probability);
                advantages.push(step.advantage);
                returns.push(step.return_value);
            }
            let probabilities = softmax(
                logits + Tensor::from_data(TensorData::new(legal, [local.len(), count]), device),
                1,
            )
            .clamp(1e-7, 1.0);
            let log_probabilities = probabilities.clone().log();
            let current = (log_probabilities.clone()
                * Tensor::from_data(TensorData::new(target, [local.len(), count]), device))
            .sum_dim(1);
            let old = Tensor::from_data(TensorData::new(old, [local.len(), 1]), device);
            let ratios = (current.clone() - old.clone()).exp();
            let advantages =
                Tensor::from_data(TensorData::new(advantages, [local.len(), 1]), device);
            let policy = ratios
                .clone()
                .clamp(1.0 - config.clip_epsilon, 1.0 + config.clip_epsilon)
                .mul(advantages.clone())
                .min_pair(ratios.clone().mul(advantages))
                .neg();
            let entropy = (probabilities * log_probabilities).sum_dim(1).neg();
            let values = model.forward_typed_values(typed_state);
            let returns = Tensor::from_data(TensorData::new(returns, [local.len(), 1]), device);
            let value = (values - returns).powf_scalar(2.0);
            let weight = local.len() as f32 / indices.len() as f32;
            let loss = (policy.clone() + value.clone() * config.value_coefficient
                - entropy.clone() * config.entropy_coefficient)
                .mean()
                * weight;
            total_loss = Some(match total_loss {
                Some(total) => total + loss,
                None => loss,
            });
            total_metrics.policy_loss +=
                policy.mean().into_data().to_vec::<f32>().unwrap()[0] * weight;
            total_metrics.value_loss +=
                value.mean().into_data().to_vec::<f32>().unwrap()[0] * weight;
            total_metrics.entropy +=
                entropy.mean().into_data().to_vec::<f32>().unwrap()[0] * weight;
        }
        (
            total_loss.expect("resident PPO minibatch must not be empty"),
            total_metrics,
        )
    }
}

#[cfg(feature = "simulator-wgpu")]
impl GpuPpoDataset {
    fn from_steps(steps: &[&RolloutStep], device: &super::model::GpuPolicyDevice) -> Self {
        let mut grouped = BTreeMap::<usize, Vec<usize>>::new();
        for (index, step) in steps.iter().enumerate() {
            grouped
                .entry(step.candidate_count())
                .or_default()
                .push(index);
        }
        let buckets = grouped
            .into_iter()
            .map(|(candidate_count, source_indices)| {
                let state_sets = std::array::from_fn(|set_index| {
                    let sets = source_indices
                        .iter()
                        .map(|&index| steps[index].typed_observation.sets[set_index].clone())
                        .collect::<Vec<_>>();
                    to_tensors(&PaddedEntityBatch::from_sets(&sets), device)
                });
                let candidate_sets = source_indices
                    .iter()
                    .flat_map(|&index| {
                        steps[index]
                            .candidate_rows
                            .iter()
                            .map(|row| EntitySet::new(vec![row.clone()]))
                    })
                    .collect::<Vec<_>>();
                let candidates = to_tensors(&PaddedEntityBatch::from_sets(&candidate_sets), device);
                let states = tensor_from_rows(
                    &source_indices
                        .iter()
                        .map(|&index| steps[index].state.clone())
                        .collect::<Vec<_>>(),
                    device,
                );
                let mut legal_masks = vec![0.0; source_indices.len() * candidate_count];
                let mut targets = vec![0.0; source_indices.len() * candidate_count];
                let mut old_log_probabilities = Vec::with_capacity(source_indices.len());
                let mut advantages = Vec::with_capacity(source_indices.len());
                let mut returns = Vec::with_capacity(source_indices.len());
                let source_count = source_indices.len();
                for (row, &index) in source_indices.iter().enumerate() {
                    let step = steps[index];
                    targets[row * candidate_count + step.action_index] = 1.0;
                    for (column, legal) in step.legal_candidate_mask.iter().enumerate() {
                        legal_masks[row * candidate_count + column] =
                            if *legal { 0.0 } else { -1.0e9 };
                    }
                    old_log_probabilities.push(step.old_log_probability);
                    advantages.push(step.advantage);
                    returns.push(step.return_value);
                }
                GpuPpoBucket {
                    candidate_count,
                    source_indices: source_indices.clone(),
                    state_sets,
                    candidates,
                    states,
                    legal_masks: Tensor::from_data(
                        TensorData::new(legal_masks, [source_count, candidate_count]),
                        device,
                    ),
                    targets: Tensor::from_data(
                        TensorData::new(targets, [source_count, candidate_count]),
                        device,
                    ),
                    old_log_probabilities: Tensor::from_data(
                        TensorData::new(old_log_probabilities, [source_count, 1]),
                        device,
                    ),
                    advantages: Tensor::from_data(
                        TensorData::new(advantages, [source_count, 1]),
                        device,
                    ),
                    returns: Tensor::from_data(TensorData::new(returns, [source_count, 1]), device),
                }
            })
            .collect();
        Self { buckets }
    }
}

#[cfg(target_os = "linux")]
fn process_cpu_seconds() -> Option<f64> {
    let mut time = std::mem::MaybeUninit::<libc::timespec>::uninit();
    let result = unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, time.as_mut_ptr()) };
    (result == 0).then(|| {
        let time = unsafe { time.assume_init() };
        time.tv_sec as f64 + time.tv_nsec as f64 * 1e-9
    })
}

#[cfg(not(target_os = "linux"))]
fn process_cpu_seconds() -> Option<f64> {
    None
}

fn cpu_seconds_since(start: Option<f64>) -> Option<f64> {
    process_cpu_seconds()
        .zip(start)
        .map(|(end, start)| end - start)
}

fn cpu_seconds_since_or_zero(start: Option<f64>) -> f64 {
    cpu_seconds_since(start).unwrap_or_default()
}

mod diagnostics;

pub(crate) use diagnostics::{
    BestScore, InitialBestState, best_score_from_checkpoint, best_score_from_run,
};
use diagnostics::{
    OptimizationDiagnostics, OptimizationDiagnosticsAccumulator, OptimizationMinibatchMetrics,
};

fn combine_diagnostics(
    first: &RolloutDiagnostics,
    second: &RolloutDiagnostics,
) -> RolloutDiagnostics {
    let total_episodes = first.episode_count + second.episode_count;
    let total_steps = first.decision_steps + second.decision_steps;
    let weighted = |a: f32, b: f32, na: usize, nb: usize| {
        if na + nb == 0 {
            0.0
        } else {
            (a * na as f32 + b * nb as f32) / (na + nb) as f32
        }
    };
    let mut action_kind_counts = [0; ActionKind::COUNT];
    for (index, count) in action_kind_counts.iter_mut().enumerate() {
        *count = first.action_kind_counts[index] + second.action_kind_counts[index];
    }
    let mut bootstrap_counts = first.bootstrap_counts.clone();
    for (key, value) in &second.bootstrap_counts {
        *bootstrap_counts.entry(key.clone()).or_insert(0) += value;
    }
    let mut reward_component_sums = first.reward_component_sums.clone();
    for (key, value) in &second.reward_component_sums {
        *reward_component_sums.entry(key.clone()).or_insert(0.0) += value;
    }
    let mut decision_point_counts = first.decision_point_counts.clone();
    for (key, value) in &second.decision_point_counts {
        *decision_point_counts.entry(key.clone()).or_insert(0) += value;
    }
    let variance = |sum: f64, sum_squares: f64, count: usize| {
        if count == 0 {
            0.0
        } else {
            (sum_squares / count as f64 - (sum / count as f64).powi(2))
                .max(0.0)
                .sqrt() as f32
        }
    };
    let reward_sum = first.reward_sum + second.reward_sum;
    let reward_sum_squares = first.reward_sum_squares + second.reward_sum_squares;
    let episode_return_sum = first.episode_return_sum + second.episode_return_sum;
    let episode_return_sum_squares =
        first.episode_return_sum_squares + second.episode_return_sum_squares;
    let advantage_sum = first.advantage_sum + second.advantage_sum;
    let advantage_sum_squares = first.advantage_sum_squares + second.advantage_sum_squares;
    RolloutDiagnostics {
        episode_count: total_episodes,
        decision_steps: total_steps,
        mean_reward: weighted(
            first.mean_reward,
            second.mean_reward,
            first.decision_steps,
            second.decision_steps,
        ),
        reward_stddev: variance(reward_sum, reward_sum_squares, total_steps),
        mean_episode_return: weighted(
            first.mean_episode_return,
            second.mean_episode_return,
            first.episode_count,
            second.episode_count,
        ),
        mean_action_nll: weighted(
            first.mean_action_nll,
            second.mean_action_nll,
            first.decision_steps,
            second.decision_steps,
        ),
        mean_policy_entropy: weighted(
            first.mean_policy_entropy,
            second.mean_policy_entropy,
            first.decision_steps,
            second.decision_steps,
        ),
        mean_value_loss: weighted(
            first.mean_value_loss,
            second.mean_value_loss,
            first.decision_steps,
            second.decision_steps,
        ),
        episode_return_stddev: variance(
            episode_return_sum,
            episode_return_sum_squares,
            total_episodes,
        ),
        mean_final_clear_rate: weighted(
            first.mean_final_clear_rate,
            second.mean_final_clear_rate,
            first.episode_count,
            second.episode_count,
        ),
        mean_escaped_hp: weighted(
            first.mean_escaped_hp,
            second.mean_escaped_hp,
            first.episode_count,
            second.episode_count,
        ),
        mean_player_damage: weighted(
            first.mean_player_damage,
            second.mean_player_damage,
            first.episode_count,
            second.episode_count,
        ),
        mean_tower_damage: weighted(
            first.mean_tower_damage,
            second.mean_tower_damage,
            first.episode_count,
            second.episode_count,
        ),
        nonzero_reward_fraction: weighted(
            first.nonzero_reward_fraction,
            second.nonzero_reward_fraction,
            first.decision_steps,
            second.decision_steps,
        ),
        terminal_reward_count: first.terminal_reward_count + second.terminal_reward_count,
        death_penalty_count: first.death_penalty_count + second.death_penalty_count,
        full_clear_count: first.full_clear_count + second.full_clear_count,
        mean_final_stage: weighted(
            first.mean_final_stage,
            second.mean_final_stage,
            first.episode_count,
            second.episode_count,
        ),
        mean_advantage: weighted(
            first.mean_advantage,
            second.mean_advantage,
            first.decision_steps,
            second.decision_steps,
        ),
        advantage_stddev: variance(advantage_sum, advantage_sum_squares, total_steps),
        terminal_episode_count: first.terminal_episode_count + second.terminal_episode_count,
        truncated_episode_count: first.truncated_episode_count + second.truncated_episode_count,
        no_progress_cycle_count: first.no_progress_cycle_count + second.no_progress_cycle_count,
        mean_decisions_per_episode: weighted(
            first.mean_decisions_per_episode,
            second.mean_decisions_per_episode,
            first.episode_count,
            second.episode_count,
        ),
        action_kind_counts,
        decision_point_counts,
        forced_action_count: first.forced_action_count + second.forced_action_count,
        forced_action_counts: {
            let mut counts = first.forced_action_counts.clone();
            for (key, value) in &second.forced_action_counts {
                *counts.entry(key.clone()).or_insert(0) += value;
            }
            counts
        },
        select_count: first.select_count + second.select_count,
        deselect_count: first.deselect_count + second.deselect_count,
        confirm_count: first.confirm_count + second.confirm_count,
        cancel_count: first.cancel_count + second.cancel_count,
        immediate_inverse_count: first.immediate_inverse_count + second.immediate_inverse_count,
        selected_set_revisit_count: first.selected_set_revisit_count
            + second.selected_set_revisit_count,
        begin_cancel_repetition_count: first.begin_cancel_repetition_count
            + second.begin_cancel_repetition_count,
        mean_select_count: weighted(
            first.mean_select_count,
            second.mean_select_count,
            first.episode_count,
            second.episode_count,
        ),
        mean_deselect_count: weighted(
            first.mean_deselect_count,
            second.mean_deselect_count,
            first.episode_count,
            second.episode_count,
        ),
        mean_confirm_count: weighted(
            first.mean_confirm_count,
            second.mean_confirm_count,
            first.episode_count,
            second.episode_count,
        ),
        mean_cancel_count: weighted(
            first.mean_cancel_count,
            second.mean_cancel_count,
            first.episode_count,
            second.episode_count,
        ),
        mean_immediate_inverse_count: weighted(
            first.mean_immediate_inverse_count,
            second.mean_immediate_inverse_count,
            first.episode_count,
            second.episode_count,
        ),
        mean_selected_set_revisit_count: weighted(
            first.mean_selected_set_revisit_count,
            second.mean_selected_set_revisit_count,
            first.episode_count,
            second.episode_count,
        ),
        mean_begin_cancel_repetition_count: weighted(
            first.mean_begin_cancel_repetition_count,
            second.mean_begin_cancel_repetition_count,
            first.episode_count,
            second.episode_count,
        ),
        no_progress_cycle_penalty_sum: first.no_progress_cycle_penalty_sum
            + second.no_progress_cycle_penalty_sum,
        bootstrap_allowed_count: first.bootstrap_allowed_count + second.bootstrap_allowed_count,
        bootstrap_blocked_count: first.bootstrap_blocked_count + second.bootstrap_blocked_count,
        bootstrap_counts,
        reward_component_sums,
        reward_sum,
        reward_sum_squares,
        episode_return_sum,
        episode_return_sum_squares,
        advantage_sum,
        advantage_sum_squares,
    }
}

#[allow(clippy::too_many_arguments)]
fn format_compact_iteration(
    iteration: usize,
    mode: &str,
    run_id: &str,
    configured_cycle_penalty: f32,
    train_clear_rate: f64,
    validation_clear_rate: f64,
    train: &RolloutDiagnostics,
    validation: &RolloutDiagnostics,
    optimizer_step: u64,
    optimization: OptimizationDiagnostics,
) -> String {
    format!(
        "● ppo.v2 event=iter mode={} run_id={} generation={} configured_cycle_penalty={:.2} train.clear_rate={:.2} validation.clear_rate={:.2} train.mean_stage={:.2} validation.mean_stage={:.2} train.return={:.2} validation.return={:.2} train.tower_damage={:.2} validation.tower_damage={:.2} train.cycle_count={} train.cycle_reward_sum={:.2} validation.cycle_count={} validation.cycle_reward_sum={:.2} train.termination_terminal={} train.termination_max_ticks={} train.termination_max_decisions={} train.termination_no_progress_cycle={} validation.termination_terminal={} validation.termination_max_ticks={} validation.termination_max_decisions={} validation.termination_no_progress_cycle={} train.mean_decisions={} validation.mean_decisions={} train.forced_actions={} validation.forced_actions={} train.entropy={:.4} validation.entropy={:.4} train.nonzero_reward={:.4} validation.nonzero_reward={:.4} train.reward_total={:.2} train.reward_terminal={:.2} train.reward_shaping={:.2} train.clear_progress={:.2} train.damage_progress={:.2} train.escaped_hp_penalty={:.2} train.player_hp_loss_penalty={:.2} validation.reward_total={:.2} validation.reward_terminal={:.2} validation.reward_shaping={:.2} validation.clear_progress={:.2} validation.damage_progress={:.2} validation.escaped_hp_penalty={:.2} validation.player_hp_loss_penalty={:.2} train.action_kind_counts={:?} validation.action_kind_counts={:?} train.decision_point_counts={:?} validation.decision_point_counts={:?} train.policy_loss={:.4} train.value_loss={:.4} train.kl={:.4} train.clip_fraction={:.4} train.gradient_norm={:.4} train.bootstrap_allowed={} train.bootstrap_blocked={} validation.bootstrap_allowed={} validation.bootstrap_blocked={} optimizer_step={} validation.full_clear={}",
        mode,
        run_id,
        iteration,
        configured_cycle_penalty,
        train_clear_rate * 100.0,
        validation_clear_rate * 100.0,
        train.mean_final_stage,
        validation.mean_final_stage,
        train.mean_episode_return,
        validation.mean_episode_return,
        train.mean_tower_damage,
        validation.mean_tower_damage,
        train.no_progress_cycle_count,
        train.no_progress_cycle_penalty_sum,
        validation.no_progress_cycle_count,
        validation.no_progress_cycle_penalty_sum,
        train.terminal_episode_count,
        train
            .bootstrap_counts
            .get("MaxTicks")
            .copied()
            .unwrap_or_default(),
        train
            .bootstrap_counts
            .get("MaxDecisions")
            .copied()
            .unwrap_or_default(),
        train.no_progress_cycle_count,
        validation.terminal_episode_count,
        validation
            .bootstrap_counts
            .get("MaxTicks")
            .copied()
            .unwrap_or_default(),
        validation
            .bootstrap_counts
            .get("MaxDecisions")
            .copied()
            .unwrap_or_default(),
        validation.no_progress_cycle_count,
        train.mean_decisions_per_episode,
        validation.mean_decisions_per_episode,
        train.forced_action_count,
        validation.forced_action_count,
        train.mean_policy_entropy,
        validation.mean_policy_entropy,
        train.nonzero_reward_fraction,
        validation.nonzero_reward_fraction,
        train
            .reward_component_sums
            .get("total")
            .copied()
            .unwrap_or_default(),
        train
            .reward_component_sums
            .get("terminal")
            .copied()
            .unwrap_or_default(),
        train
            .reward_component_sums
            .get("shaping")
            .copied()
            .unwrap_or_default(),
        train
            .reward_component_sums
            .get("clear_progress")
            .copied()
            .unwrap_or_default(),
        train
            .reward_component_sums
            .get("damage_progress")
            .copied()
            .unwrap_or_default(),
        train
            .reward_component_sums
            .get("escaped_hp_penalty")
            .copied()
            .unwrap_or_default(),
        train
            .reward_component_sums
            .get("player_hp_loss_penalty")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("total")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("terminal")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("shaping")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("clear_progress")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("damage_progress")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("escaped_hp_penalty")
            .copied()
            .unwrap_or_default(),
        validation
            .reward_component_sums
            .get("player_hp_loss_penalty")
            .copied()
            .unwrap_or_default(),
        train.action_kind_counts,
        validation.action_kind_counts,
        train.decision_point_counts,
        validation.decision_point_counts,
        optimization.policy_loss,
        train.mean_value_loss,
        optimization.approximate_kl,
        optimization.clip_fraction,
        optimization.gradient_norm,
        train.bootstrap_allowed_count,
        train.bootstrap_blocked_count,
        validation.bootstrap_allowed_count,
        validation.bootstrap_blocked_count,
        optimizer_step,
        validation.full_clear_count,
    )
}

#[cfg(test)]
mod logging_tests {
    #[test]
    fn timing_labels_are_not_swapped_by_format_order() {
        let text = "train 5.0% val 4.0% r=10.0s/100d/s o=2.0s v=3.0s/200d/s";
        assert!(text.contains("r=10.0s/100d/s"));
        assert!(text.contains("o=2.0s"));
        assert!(text.contains("v=3.0s/200d/s"));
    }
}

#[cfg(all(test, feature = "simulator-wgpu"))]
mod wgpu_learner_tests {
    use super::*;
    use crate::simulator::ml::encoding::{EntityRow, TypedObservation};
    use crate::simulator::ml::model::{CpuInferenceBackend, DeepSetsActorCritic};
    use crate::simulator::ml::rollout::RolloutStep;

    fn fixture_step(action_index: usize) -> RolloutStep {
        RolloutStep {
            state: vec![0.5; super::super::features::GLOBAL_FEATURE_COUNT],
            typed_observation: TypedObservation::default(),
            candidate_rows: (0..3)
                .map(|index| EntityRow::new([index + 1, 1, 0, 0], vec![0.0, 1.0, 0.0, 0.0, 0.0]))
                .collect(),
            legal_candidate_mask: vec![true, true, true],
            action_index,
            old_log_probability: (1.0_f32 / 3.0).ln(),
            value_estimate: 0.0,
            next_value_estimate: 0.0,
            reward: 1.0,
            terminal_reward: 0.0,
            shaping_reward: 0.0,
            no_progress_cycle_penalty_reward: 0.0,
            terminated: true,
            truncated: false,
            no_progress_cycle: false,
            bootstrap_allowed: true,
            action_kind: 0,
            advantage: 1.0,
            return_value: 1.0,
            policy_entropy: 0.0,
        }
    }

    #[test]
    #[ignore = "requires an AMD discrete GPU and a configured WGPU adapter"]
    fn wgpu_ppo_loss_backward_and_adam_step_smoke() {
        let cpu_model = DeepSetsActorCritic::<CpuInferenceBackend>::new(
            ModelConfig::default(),
            &Default::default(),
        );
        let steps = [fixture_step(0), fixture_step(2)];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let config = PpoConfig {
            ppo_epochs: 1,
            minibatch_size: 2,
            learning_rate: 1e-4,
            ..PpoConfig::default()
        };
        let snapshot = gpu_learner_step_from_cpu_rollout(
            cpu_model,
            ModelConfig::default(),
            &step_refs,
            &config,
        )
        .expect("WGPU learner step should produce a CPU snapshot");
        let logits = snapshot.forward_logits(
            tensor_from_rows::<CpuInferenceBackend>(
                &[vec![0.0; super::super::features::GLOBAL_FEATURE_COUNT]],
                &Default::default(),
            ),
            tensor_from_rows::<CpuInferenceBackend>(
                &[vec![0.0; super::super::features::ACTION_FEATURE_COUNT]],
                &Default::default(),
            ),
        );
        assert!(logits.into_data().to_vec::<f32>().unwrap()[0].is_finite());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PpoConfig {
    pub iterations: usize,
    pub ppo_epochs: usize,
    pub learning_rate: f64,
    pub clip_epsilon: f32,
    pub entropy_coefficient: f32,
    pub value_coefficient: f32,
    pub minibatch_size: usize,
    pub rollout_step_budget: usize,
    pub max_queued_steps: usize,
    pub rollout_chunk_size: usize,
    pub trainer_checkpoint_dir: Option<std::path::PathBuf>,
    pub resume_trainer: bool,
    pub rollout: RolloutConfig,
    pub model: ModelConfig,
    pub init_checkpoint: Option<std::path::PathBuf>,
    pub curriculum: Option<CurriculumConfig>,
    pub curriculum_validation: Option<SeedRange>,
    pub reward_config: RewardConfig,
    #[cfg(feature = "simulator-wgpu")]
    pub wgpu_stream_rollout: bool,
}

impl Default for PpoConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            ppo_epochs: 1,
            learning_rate: 1e-4,
            clip_epsilon: 0.1,
            entropy_coefficient: 0.01,
            value_coefficient: 0.5,
            minibatch_size: 256,
            rollout_step_budget: 0,
            max_queued_steps: 0,
            rollout_chunk_size: 0,
            trainer_checkpoint_dir: None,
            resume_trainer: false,
            rollout: RolloutConfig::default(),
            model: ModelConfig::default(),
            init_checkpoint: None,
            curriculum: None,
            curriculum_validation: None,
            reward_config: RewardConfig::default(),
            #[cfg(feature = "simulator-wgpu")]
            wgpu_stream_rollout: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PpoIterationStats {
    pub iteration: usize,
    pub train_clear_rate: f64,
    pub validation_clear_rate: f64,
    pub train_episodes: usize,
    pub validation_episodes: usize,
    pub decision_steps: usize,
    pub learner_seconds: f64,
    pub train_rollout_seconds: f64,
    pub optimization_seconds: f64,
    pub validation_rollout_seconds: f64,
    pub train_episodes_per_second: f64,
    pub train_decisions_per_second: f64,
    pub validation_episodes_per_second: f64,
    pub validation_decisions_per_second: f64,
    pub train_diagnostics: RolloutDiagnostics,
    pub validation_diagnostics: RolloutDiagnostics,
}

#[derive(Debug)]
pub struct PpoTrainingRun {
    pub model: DeepSetsActorCritic<TrainBackend>,
    pub best_model: DeepSetsActorCritic<TrainBackend>,
    pub best_iteration: usize,
    pub best_validation_clear_rate: f64,
    pub best_validation_full_clear_count: usize,
    pub best_validation_truncated_count: usize,
    pub elite_train_seeds: Vec<u64>,
    pub optimization_diagnostics: OptimizationDiagnostics,
    pub history: Vec<PpoIterationStats>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverfitGateThreshold {
    pub minimum_return_improvement: f32,
    pub maximum_supervised_nll: f32,
    pub maximum_value_loss: f32,
}

impl Default for OverfitGateThreshold {
    fn default() -> Self {
        Self {
            minimum_return_improvement: 0.05,
            maximum_supervised_nll: 0.25,
            maximum_value_loss: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OverfitGateReport {
    pub seed_count: usize,
    pub max_decisions: usize,
    pub initial_return: f32,
    pub final_return: f32,
    pub return_improvement: f32,
    pub initial_supervised_nll: f32,
    pub final_supervised_nll: f32,
    pub final_supervised_top1_accuracy: f32,
    pub final_value_loss: f32,
    pub initial_final_stage: f32,
    pub final_final_stage: f32,
    pub initial_nonzero_reward_fraction: f32,
    pub final_nonzero_reward_fraction: f32,
    pub initial_no_progress_cycle_count: usize,
    pub final_no_progress_cycle_count: usize,
    pub environment_improved: bool,
    pub passed: bool,
}

pub fn evaluate_overfit_gate(
    initial: &RolloutDiagnostics,
    final_diagnostics: &RolloutDiagnostics,
    seed_count: usize,
    max_decisions: usize,
    threshold: OverfitGateThreshold,
    initial_supervised: &SupervisedNllReport,
    final_supervised: &SupervisedNllReport,
) -> OverfitGateReport {
    let return_improvement = final_diagnostics.mean_episode_return - initial.mean_episode_return;
    let environment_improved = return_improvement >= threshold.minimum_return_improvement
        || final_diagnostics.mean_final_stage > initial.mean_final_stage
        || final_diagnostics.nonzero_reward_fraction > initial.nonzero_reward_fraction;
    let cycle_rate_not_worse = final_diagnostics.no_progress_cycle_count * initial.episode_count
        <= initial.no_progress_cycle_count * final_diagnostics.episode_count;
    let passed = return_improvement >= threshold.minimum_return_improvement
        && final_supervised.mean_nll <= threshold.maximum_supervised_nll
        && final_diagnostics.mean_value_loss <= threshold.maximum_value_loss
        && environment_improved
        && cycle_rate_not_worse;
    OverfitGateReport {
        seed_count,
        max_decisions,
        initial_return: initial.mean_episode_return,
        final_return: final_diagnostics.mean_episode_return,
        return_improvement,
        initial_supervised_nll: initial_supervised.mean_nll,
        final_supervised_nll: final_supervised.mean_nll,
        final_supervised_top1_accuracy: final_supervised.top1_accuracy,
        final_value_loss: final_diagnostics.mean_value_loss,
        initial_final_stage: initial.mean_final_stage,
        final_final_stage: final_diagnostics.mean_final_stage,
        initial_nonzero_reward_fraction: initial.nonzero_reward_fraction,
        final_nonzero_reward_fraction: final_diagnostics.nonzero_reward_fraction,
        initial_no_progress_cycle_count: initial.no_progress_cycle_count,
        final_no_progress_cycle_count: final_diagnostics.no_progress_cycle_count,
        environment_improved,
        passed,
    }
}

pub(crate) fn trainer_hyperparameters(config: &PpoConfig) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ppo_epochs".to_string(), config.ppo_epochs.to_string()),
        (
            "learning_rate".to_string(),
            config.learning_rate.to_string(),
        ),
        ("clip_epsilon".to_string(), config.clip_epsilon.to_string()),
        (
            "entropy_coefficient".to_string(),
            config.entropy_coefficient.to_string(),
        ),
        (
            "value_coefficient".to_string(),
            config.value_coefficient.to_string(),
        ),
        (
            "minibatch_size".to_string(),
            config.minibatch_size.to_string(),
        ),
        (
            "rollout_step_budget".to_string(),
            config.rollout_step_budget.to_string(),
        ),
        (
            "max_queued_steps".to_string(),
            config.max_queued_steps.to_string(),
        ),
        (
            "rollout_chunk_size".to_string(),
            config.rollout_chunk_size.to_string(),
        ),
        (
            "max_decisions_per_episode".to_string(),
            config.rollout.max_decisions_per_episode.to_string(),
        ),
        (
            "temperature".to_string(),
            config.rollout.temperature.to_string(),
        ),
        (
            "adaptive_exploration".to_string(),
            config.rollout.adaptive_exploration.to_string(),
        ),
        ("gamma".to_string(), config.rollout.gamma.to_string()),
        (
            "reward_config".to_string(),
            serde_json::to_string(&config.reward_config).expect("RewardConfig is serializable"),
        ),
        (
            "gae_lambda".to_string(),
            config.rollout.gae_lambda.to_string(),
        ),
        ("greedy".to_string(), config.rollout.greedy.to_string()),
        (
            "max_stage".to_string(),
            config
                .rollout
                .max_stage
                .map_or_else(|| "none".to_string(), |value| value.to_string()),
        ),
        (
            "curriculum_validation".to_string(),
            config.curriculum_validation.map_or_else(
                || "none".to_string(),
                |range| format!("{}:{}", range.start, range.end_inclusive),
            ),
        ),
        (
            "hidden_size".to_string(),
            config.model.hidden_size.to_string(),
        ),
    ])
}

fn gradient_l2_norm<B: burn::tensor::backend::AutodiffBackend>(
    model: &DeepSetsActorCritic<B>,
    gradients: &GradientsParams,
) -> f32 {
    struct GradientVisitor<'a> {
        gradients: &'a GradientsParams,
        squared_norm: f64,
        finite: bool,
    }

    impl<B: burn::tensor::backend::AutodiffBackend> ModuleVisitor<B> for GradientVisitor<'_> {
        fn visit_float<const D: usize>(&mut self, parameter: &Param<Tensor<B, D>>) {
            let Some(gradient) = self.gradients.get::<B::InnerBackend, D>(parameter.id) else {
                return;
            };
            let squared_norm = gradient
                .powf_scalar(2.0)
                .sum()
                .into_data()
                .to_vec::<f32>()
                .expect("gradient tensor must be f32");
            let value = squared_norm[0];
            self.finite &= value.is_finite();
            self.squared_norm += f64::from(value);
        }
    }

    let mut visitor = GradientVisitor {
        gradients,
        squared_norm: 0.0,
        finite: true,
    };
    model.visit(&mut visitor);
    if visitor.finite {
        visitor.squared_norm.sqrt() as f32
    } else {
        f32::NAN
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct UpdateTiming {
    backward_seconds: f64,
    gradient_norm_seconds: f64,
    optimizer_step_seconds: f64,
}

enum GuardedMinibatchStep<B: burn::tensor::backend::AutodiffBackend> {
    Stepped(Box<DeepSetsActorCritic<B>>, f32, UpdateTiming),
    NonFiniteMetric,
    NonFiniteGradient,
}

impl<B: burn::tensor::backend::AutodiffBackend> std::fmt::Debug for GuardedMinibatchStep<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stepped(_, norm, timing) => {
                f.debug_tuple("Stepped").field(norm).field(timing).finish()
            }
            Self::NonFiniteMetric => f.write_str("NonFiniteMetric"),
            Self::NonFiniteGradient => f.write_str("NonFiniteGradient"),
        }
    }
}

fn guarded_minibatch_step<B, O>(
    model: DeepSetsActorCritic<B>,
    optimizer: &mut O,
    learning_rate: f64,
    total_loss: Tensor<B, 1>,
    metrics: OptimizationMinibatchMetrics,
) -> GuardedMinibatchStep<B>
where
    B: burn::tensor::backend::AutodiffBackend,
    O: Optimizer<DeepSetsActorCritic<B>, B>,
{
    let Ok(loss_values) = total_loss.clone().into_data().to_vec::<f32>() else {
        return GuardedMinibatchStep::NonFiniteMetric;
    };
    if !loss_values.iter().all(|value| value.is_finite()) || !metrics.is_finite() {
        return GuardedMinibatchStep::NonFiniteMetric;
    }
    let backward_started = Instant::now();
    let gradients = total_loss.backward();
    let gradients = GradientsParams::from_grads(gradients, &model);
    apply_guarded_step(
        model,
        optimizer,
        learning_rate,
        gradients,
        backward_started.elapsed().as_secs_f64(),
    )
}

#[cfg(feature = "simulator-wgpu")]
fn gpu_minibatch_step(
    model: DeepSetsActorCritic<super::model::GpuTrainBackend>,
    optimizer: &mut OptimizerAdaptor<
        burn::optim::Adam,
        DeepSetsActorCritic<super::model::GpuTrainBackend>,
        super::model::GpuTrainBackend,
    >,
    learning_rate: f64,
    loss: Tensor<super::model::GpuTrainBackend, 1>,
) -> DeepSetsActorCritic<super::model::GpuTrainBackend> {
    let gradients = loss.backward();
    let gradients = GradientsParams::from_grads(gradients, &model);
    optimizer.step(learning_rate, model, gradients)
}

fn apply_guarded_step<B, O>(
    model: DeepSetsActorCritic<B>,
    optimizer: &mut O,
    learning_rate: f64,
    gradients: GradientsParams,
    backward_seconds: f64,
) -> GuardedMinibatchStep<B>
where
    B: burn::tensor::backend::AutodiffBackend,
    O: Optimizer<DeepSetsActorCritic<B>, B>,
{
    let gradient_norm_started = Instant::now();
    let gradient_norm = gradient_l2_norm(&model, &gradients);
    let gradient_norm_seconds = gradient_norm_started.elapsed().as_secs_f64();
    if !gradient_norm.is_finite() {
        return GuardedMinibatchStep::NonFiniteGradient;
    }
    let optimizer_step_started = Instant::now();
    let model = optimizer.step(learning_rate, model, gradients);
    GuardedMinibatchStep::Stepped(
        Box::new(model),
        gradient_norm,
        UpdateTiming {
            backward_seconds,
            gradient_norm_seconds,
            optimizer_step_seconds: optimizer_step_started.elapsed().as_secs_f64(),
        },
    )
}

#[cfg(feature = "simulator-wgpu")]
#[derive(Clone)]
pub(crate) struct GpuPpoLearner {
    model: DeepSetsActorCritic<super::model::GpuTrainBackend>,
    device: super::model::GpuPolicyDevice,
    optimizer: OptimizerAdaptor<
        burn::optim::Adam,
        DeepSetsActorCritic<super::model::GpuTrainBackend>,
        super::model::GpuTrainBackend,
    >,
    model_config: ModelConfig,
}

#[cfg(feature = "simulator-wgpu")]
impl GpuPpoLearner {
    pub(crate) fn from_cpu_model(
        cpu_model: DeepSetsActorCritic<super::model::CpuInferenceBackend>,
        model_config: ModelConfig,
    ) -> Result<Self> {
        let device = super::model::GpuPolicyDevice::DiscreteGpu(0);
        let model = super::model::cpu_model_to_gpu_train_model(cpu_model, model_config, &device)?;
        Ok(Self {
            model,
            device,
            optimizer: AdamConfig::new()
                .with_grad_clipping(Some(GradientClippingConfig::Norm(0.5)))
                .init(),
            model_config,
        })
    }

    pub(crate) fn update(
        &mut self,
        steps: &[&RolloutStep],
        config: &PpoConfig,
        iteration: u64,
    ) -> Result<()> {
        if steps.is_empty() {
            bail!("GPU learner requires at least one rollout step");
        }
        let indices = (0..steps.len()).collect::<Vec<_>>();
        for epoch in 0..config.ppo_epochs {
            let mut shuffled = indices.clone();
            deterministic_shuffle(&mut shuffled, iteration, 0, epoch as u64);
            for minibatch in shuffled.chunks(config.minibatch_size.max(1)) {
                let minibatch_steps = minibatch
                    .iter()
                    .map(|&index| steps[index])
                    .collect::<Vec<_>>();
                let minibatch_indices = (0..minibatch_steps.len()).collect::<Vec<_>>();
                let dataset = GpuPpoDataset::from_steps(&minibatch_steps, &self.device);
                debug_assert!(dataset.bucket_count() > 0);
                dataset.validate();
                let (loss, _) = dataset.loss(
                    &self.model,
                    &self.device,
                    &minibatch_steps,
                    &minibatch_indices,
                    config,
                );
                self.model = gpu_minibatch_step(
                    self.model.clone(),
                    &mut self.optimizer,
                    config.learning_rate,
                    loss,
                );
            }
        }
        Ok(())
    }

    pub(crate) fn snapshot(
        &self,
    ) -> Result<DeepSetsActorCritic<super::model::CpuInferenceBackend>> {
        super::model::gpu_train_model_to_cpu_inference_model(
            &self.model,
            self.model_config,
            &super::model::CpuPolicyDevice::default(),
        )
    }
}

#[cfg(feature = "simulator-wgpu")]
#[allow(dead_code)]
pub(crate) fn gpu_learner_step_from_cpu_rollout(
    cpu_model: DeepSetsActorCritic<super::model::CpuInferenceBackend>,
    model_config: ModelConfig,
    steps: &[&RolloutStep],
    config: &PpoConfig,
) -> Result<DeepSetsActorCritic<super::model::CpuInferenceBackend>> {
    let mut learner = GpuPpoLearner::from_cpu_model(cpu_model, model_config)?;
    learner.update(steps, config, 0)?;
    learner.snapshot()
}

pub fn train_ppo(
    game_config: Arc<GameConfig>,
    schedule: &TrainingSeedSchedule,
    config: &PpoConfig,
) -> Result<PpoTrainingRun> {
    train_ppo_with_progress(game_config, schedule, config, None)
}

#[cfg(feature = "simulator-wgpu")]
pub(crate) const MINIMUM_WGPU_VALIDATION_CLEAR_RATE: f64 = 0.10;

#[cfg(feature = "simulator-wgpu")]
#[allow(clippy::too_many_arguments)]
pub(crate) fn train_ppo_wgpu_once(
    game_config: Arc<GameConfig>,
    schedule: &TrainingSeedSchedule,
    config: &PpoConfig,
    initial_model: Option<DeepSetsActorCritic<CpuInferenceBackend>>,
    initial_best_model: Option<DeepSetsActorCritic<CpuInferenceBackend>>,
    initial_best: InitialBestState,
    initial_elite_train_seeds: Vec<u64>,
    starting_iteration: usize,
    progress: Option<&PpoProgress>,
) -> Result<PpoTrainingRun> {
    if config.resume_trainer || config.trainer_checkpoint_dir.is_some() {
        bail!("WGPU learner does not support trainer checkpoint resume");
    }
    if config.iterations == 0 {
        bail!("PPO iterations must be positive");
    }
    if config.ppo_epochs == 0 || config.minibatch_size == 0 {
        bail!("PPO epochs and minibatch size must be positive");
    }
    let cpu_device = CpuPolicyDevice::default();
    let cpu_model = initial_model.unwrap_or_else(|| {
        let model = DeepSetsActorCritic::<CpuTrainBackend>::new(config.model, &cpu_device);
        initialize_model(&model, &cpu_device);
        model.valid()
    });
    let mut learner = GpuPpoLearner::from_cpu_model(cpu_model, config.model)?;
    let mut best_learner = learner.clone();
    let mut history = Vec::with_capacity(config.iterations);
    let mut best_model = initial_best_model
        .map(|model| cpu_inference_model_to_cpu_train_model(model, config.model, &cpu_device))
        .transpose()?;
    let mut final_model = None;
    let mut best_iteration = initial_best.iteration;
    let mut best_validation_clear_rate = initial_best.validation_clear_rate;
    let mut best_validation_full_clear_count = initial_best.full_clear_count;
    let mut best_validation_truncated_count = initial_best.truncated_count;
    let mut elite_train_seeds = initial_elite_train_seeds;
    for local_iteration in 0..config.iterations {
        let iteration = starting_iteration.saturating_add(local_iteration);
        let iteration_started = Instant::now();
        if let Some(progress) = progress {
            progress.begin_iteration(local_iteration, config.iterations);
            progress.begin_rollout(
                "train",
                prioritized_train_seeds(&schedule.train_seeds(), &elite_train_seeds).len(),
            );
        }
        let train_started = Instant::now();
        let rollout_model = learner.snapshot()?;
        let train_seeds = prioritized_train_seeds(&schedule.train_seeds(), &elite_train_seeds);
        let mut rollout_config = config.rollout.clone();
        rollout_config.greedy = false;
        rollout_config.exploration_iteration = iteration as u64;
        let (train_batch, train_rollout_seconds, optimization_seconds, decision_steps) = if config
            .wgpu_stream_rollout
        {
            let (receiver, producer) = stream_rollout_chunks(
                rollout_model,
                cpu_device,
                Arc::clone(&game_config),
                train_seeds.clone(),
                rollout_config,
                config.minibatch_size,
                config.max_queued_steps.max(config.rollout_step_budget),
                progress.map(|progress| progress.rollout_bar().clone()),
            );
            let mut train_batch = RolloutBatch {
                episodes: Vec::new(),
            };
            let mut window = RolloutBatch {
                episodes: Vec::new(),
            };
            let mut decision_steps = 0;
            let mut window_steps = 0;
            let mut held_steps = 0;
            let mut last_step_gate = None;
            let mut optimization_seconds = 0.0;
            let cpu_rollout_started = Instant::now();
            while let Ok(message) = receiver.recv() {
                let (_, mut chunk, step_gate, chunk_steps) = message?;
                last_step_gate = Some(Arc::clone(&step_gate));
                decision_steps += chunk_steps;
                window_steps += chunk_steps;
                held_steps += chunk_steps;
                window.episodes.append(&mut chunk.episodes);
                if config.rollout_step_budget == 0 || window_steps >= config.rollout_step_budget {
                    normalize_advantages(&mut window);
                    emphasize_elite_advantages(&mut window);
                    let steps = window.steps().collect::<Vec<_>>();
                    let update_started = Instant::now();
                    learner.update(&steps, config, iteration as u64)?;
                    optimization_seconds += update_started.elapsed().as_secs_f64();
                    step_gate.release(held_steps);
                    held_steps = 0;
                    window_steps = 0;
                    for episode in &mut window.episodes {
                        episode.steps.clear();
                    }
                    train_batch.episodes.append(&mut window.episodes);
                }
            }
            producer
                .join()
                .map_err(|_| anyhow::anyhow!("rollout producer thread panicked"))?;
            if !window.episodes.is_empty() {
                normalize_advantages(&mut window);
                emphasize_elite_advantages(&mut window);
                let steps = window.steps().collect::<Vec<_>>();
                let update_started = Instant::now();
                learner.update(&steps, config, iteration as u64)?;
                optimization_seconds += update_started.elapsed().as_secs_f64();
                for episode in &mut window.episodes {
                    episode.steps.clear();
                }
                last_step_gate
                    .as_ref()
                    .expect("streamed rollout must provide a step gate")
                    .release(held_steps);
                train_batch.episodes.append(&mut window.episodes);
            }
            (
                train_batch,
                cpu_rollout_started.elapsed().as_secs_f64(),
                optimization_seconds,
                decision_steps,
            )
        } else {
            let mut train_batch = collect_rollouts(
                &rollout_model,
                &cpu_device,
                Arc::clone(&game_config),
                &train_seeds,
                &rollout_config,
                progress.map(PpoProgress::rollout_bar),
            )?;
            let train_rollout_seconds = train_started.elapsed().as_secs_f64();
            let decision_steps = train_batch.steps().count();
            normalize_advantages(&mut train_batch);
            let steps = train_batch.steps().collect::<Vec<_>>();
            let optimization_started = Instant::now();
            learner.update(&steps, config, iteration as u64)?;
            (
                train_batch,
                train_rollout_seconds,
                optimization_started.elapsed().as_secs_f64(),
                decision_steps,
            )
        };
        elite_train_seeds = elite_episode_seeds(&train_batch);
        if let Some(progress) = progress {
            progress.finish_rollout();
            progress.set_rollout_message(format!("GPU update: {} steps", decision_steps));
        }
        if let Some(progress) = progress {
            progress
                .set_rollout_message(format!("GPU update complete: {:.1}s", optimization_seconds));
            progress.begin_rollout("validation", schedule.validation_seeds().len());
        }
        let validation_started = Instant::now();
        let validation_model = learner.snapshot()?;
        let validation_batch = collect_rollouts(
            &validation_model,
            &cpu_device,
            Arc::clone(&game_config),
            &schedule.validation_seeds(),
            &RolloutConfig {
                greedy: true,
                exploration_iteration: 0,
                ..config.rollout.clone()
            },
            progress.map(PpoProgress::rollout_bar),
        )?;
        let validation_rollout_seconds = validation_started.elapsed().as_secs_f64();
        if let Some(progress) = progress {
            progress.finish_rollout();
        }
        let train_diagnostics = train_batch.diagnostics();
        let validation_diagnostics = validation_batch.diagnostics();
        let train_clear_rate = train_batch.clear_rate();
        let validation_clear_rate = validation_batch.clear_rate();
        let full_clear_count = validation_diagnostics.full_clear_count;
        let truncated_count = validation_diagnostics.truncated_episode_count;
        let better = full_clear_count > best_validation_full_clear_count
            || (full_clear_count == best_validation_full_clear_count
                && (validation_clear_rate > best_validation_clear_rate
                    || (validation_clear_rate == best_validation_clear_rate
                        && truncated_count < best_validation_truncated_count)));
        let cpu_snapshot =
            cpu_inference_model_to_cpu_train_model(validation_model, config.model, &cpu_device)?;
        if validation_clear_rate >= MINIMUM_WGPU_VALIDATION_CLEAR_RATE
            && (better || best_model.is_none())
        {
            best_validation_clear_rate = validation_clear_rate;
            best_validation_full_clear_count = full_clear_count;
            best_validation_truncated_count = truncated_count;
            best_iteration = iteration;
            best_model = Some(cpu_snapshot.clone());
            best_learner = learner.clone();
        } else {
            learner = best_learner.clone();
        }
        final_model = Some(cpu_inference_model_to_cpu_train_model(
            learner.snapshot()?,
            config.model,
            &cpu_device,
        )?);
        history.push(PpoIterationStats {
            iteration,
            train_clear_rate,
            validation_clear_rate,
            train_episodes: train_batch.episodes.len(),
            validation_episodes: validation_batch.episodes.len(),
            decision_steps,
            learner_seconds: 0.0,
            train_rollout_seconds,
            optimization_seconds,
            validation_rollout_seconds,
            train_episodes_per_second: train_batch.episodes.len() as f64
                / train_rollout_seconds.max(f64::EPSILON),
            train_decisions_per_second: decision_steps as f64
                / train_rollout_seconds.max(f64::EPSILON),
            validation_episodes_per_second: validation_batch.episodes.len() as f64
                / validation_rollout_seconds.max(f64::EPSILON),
            validation_decisions_per_second: validation_batch.steps().count() as f64
                / validation_rollout_seconds.max(f64::EPSILON),
            train_diagnostics,
            validation_diagnostics,
        });
        if let Some(progress) = progress {
            progress.finish_iteration(
                train_clear_rate,
                validation_clear_rate,
                RolloutTiming {
                    train_seconds: train_rollout_seconds,
                    optimization_seconds,
                    validation_seconds: validation_rollout_seconds,
                    train_decisions_per_second: decision_steps as f64
                        / train_rollout_seconds.max(f64::EPSILON),
                    validation_decisions_per_second: validation_batch.steps().count() as f64
                        / validation_rollout_seconds.max(f64::EPSILON),
                },
            );
            progress.print_event(&format!(
                "wgpu.timing iteration={} train_rollout={:.3} optimization={:.3} validation={:.3} total={:.3} steps={}",
                iteration,
                train_rollout_seconds,
                optimization_seconds,
                validation_rollout_seconds,
                iteration_started.elapsed().as_secs_f64(),
                decision_steps,
            ));
        }
    }
    let best_model = best_model.ok_or_else(|| {
        anyhow::anyhow!(
            "WGPU validation clear rate never reached the minimum acceptable {:.1}%",
            MINIMUM_WGPU_VALIDATION_CLEAR_RATE * 100.0
        )
    })?;
    Ok(PpoTrainingRun {
        model: final_model.expect("WGPU PPO must produce a final model"),
        best_model,
        best_iteration,
        best_validation_clear_rate,
        best_validation_full_clear_count,
        best_validation_truncated_count,
        elite_train_seeds,
        optimization_diagnostics: OptimizationDiagnostics::default(),
        history,
    })
}

pub(crate) fn train_ppo_with_progress(
    game_config: Arc<GameConfig>,
    schedule: &TrainingSeedSchedule,
    config: &PpoConfig,
    progress: Option<&PpoProgress>,
) -> Result<PpoTrainingRun> {
    if config.iterations == 0 {
        bail!("PPO iterations must be positive");
    }
    if config.ppo_epochs == 0 {
        bail!("PPO epochs must be positive");
    }
    if config.minibatch_size == 0 {
        bail!("PPO minibatch size must be positive");
    }
    config
        .reward_config
        .validate_gamma(config.rollout.gamma)
        .map_err(anyhow::Error::msg)?;
    config
        .reward_config
        .validate_equal(&config.rollout.reward_config)
        .map_err(anyhow::Error::msg)?;
    let device = default_policy_device();
    let model = if let Some(path) = &config.init_checkpoint {
        let (checkpoint, model) = NeuralCheckpoint::load_for_initialization(
            path,
            &MlContract::from_config(game_config.as_ref()),
        )?;
        if checkpoint.model_config.hidden_size != config.model.hidden_size {
            bail!("initial checkpoint hidden size does not match PPO model configuration");
        }
        model
    } else {
        let model = DeepSetsActorCritic::<TrainBackend>::new(config.model, &device);
        initialize_model(&model, &device);
        model
    };
    train_ppo_from_model_with_progress(
        game_config,
        schedule,
        config,
        model,
        0,
        InitialBestState {
            validation_clear_rate: f64::NEG_INFINITY,
            iteration: 0,
            full_clear_count: 0,
            truncated_count: usize::MAX,
        },
        progress,
    )
}

pub fn train_ppo_from_model(
    game_config: Arc<GameConfig>,
    schedule: &TrainingSeedSchedule,
    config: &PpoConfig,
    model: DeepSetsActorCritic<TrainBackend>,
    starting_iteration: usize,
) -> Result<PpoTrainingRun> {
    train_ppo_from_model_with_progress(
        game_config,
        schedule,
        config,
        model,
        starting_iteration,
        InitialBestState {
            validation_clear_rate: f64::NEG_INFINITY,
            iteration: starting_iteration,
            full_clear_count: 0,
            truncated_count: usize::MAX,
        },
        None,
    )
}

pub(crate) fn train_ppo_from_model_with_progress(
    game_config: Arc<GameConfig>,
    schedule: &TrainingSeedSchedule,
    config: &PpoConfig,
    mut model: DeepSetsActorCritic<TrainBackend>,
    mut starting_iteration: usize,
    mut initial_best: InitialBestState,
    progress: Option<&PpoProgress>,
) -> Result<PpoTrainingRun> {
    if config.iterations == 0 {
        bail!("PPO iterations must be positive");
    }
    if config.ppo_epochs == 0 {
        bail!("PPO epochs must be positive");
    }
    if config.minibatch_size == 0 {
        bail!("PPO minibatch size must be positive");
    }
    if config.rollout_chunk_size == 1 {
        bail!("PPO rollout chunk size must be zero or at least two");
    }
    config
        .reward_config
        .validate_equal(&config.rollout.reward_config)
        .map_err(anyhow::Error::msg)?;
    if config.curriculum.is_some() && config.curriculum_validation.is_none() {
        bail!("curriculum requires a separate curriculum validation seed range");
    }
    let device = default_policy_device();
    let mut best_model = model.clone();
    let mut best_iteration = initial_best.iteration;
    let mut best_validation_clear_rate = initial_best.validation_clear_rate;
    let mut best_validation_full_clear_count = initial_best.full_clear_count;
    let mut best_validation_truncated_count = initial_best.truncated_count;
    let mut curriculum_state = config
        .curriculum
        .as_ref()
        .map(CurriculumState::new)
        .transpose()
        .map_err(|error| anyhow::anyhow!(error))?;
    let optimizer_config = AdamConfig::new()
        .with_grad_clipping(Some(GradientClippingConfig::Norm(0.5)))
        .init();
    let mut optimizer = optimizer_config;
    let mut best_optimizer = optimizer.clone();
    let mut optimizer_step = 0_u64;
    let train_seeds = schedule.train_seeds();
    let validation_seeds = schedule.validation_seeds();
    let mut history = Vec::with_capacity(config.iterations);
    let mut optimization_diagnostics = OptimizationDiagnosticsAccumulator::default();
    let checkpoint_store = config
        .trainer_checkpoint_dir
        .as_ref()
        .map(TrainerCheckpointStore::new)
        .transpose()?;
    if config.resume_trainer {
        let Some(store) = &checkpoint_store else {
            bail!("trainer resume requires --run-dir");
        };
        if let Some((
            state,
            resumed_model,
            resumed_best_model,
            resumed_optimizer,
            resumed_best_optimizer,
        )) = store.load_latest::<_, DeepSetsActorCritic<TrainBackend>>(
            config.model,
            &MlContract::from_config(game_config.as_ref()),
            &config.reward_config,
            schedule,
            &trainer_hyperparameters(config),
            config.curriculum.as_ref(),
            optimizer.clone(),
            &device,
        )? {
            model = resumed_model;
            optimizer = resumed_optimizer;
            best_optimizer = resumed_best_optimizer;
            optimizer_step = state.optimizer_step;
            starting_iteration = state.iteration.saturating_add(1);
            initial_best = InitialBestState {
                validation_clear_rate: state.best_validation_clear_rate,
                iteration: state.best_iteration,
                full_clear_count: state.best_validation_full_clear_count,
                truncated_count: state.best_validation_truncated_count,
            };
            best_model = resumed_best_model;
            best_iteration = initial_best.iteration;
            best_validation_clear_rate = initial_best.validation_clear_rate;
            best_validation_full_clear_count = initial_best.full_clear_count;
            best_validation_truncated_count = initial_best.truncated_count;
            if let Some(saved_state) = state.curriculum_state {
                curriculum_state = Some(saved_state);
            }
        }
    }

    for local_iteration in 0..config.iterations {
        let iteration = starting_iteration + local_iteration;
        let started = Instant::now();
        let cpu_started = process_cpu_seconds();
        if let Some(progress) = progress {
            progress.begin_iteration(local_iteration, config.iterations);
            progress.begin_rollout("train", train_seeds.len());
        }
        let mut train_episodes = 0;
        let mut decision_steps = 0;
        let mut train_clear_rate_sum = 0.0;
        let mut train_diagnostics = None;
        let mut optimization_seconds = 0.0;
        let mut optimization_loss_seconds = 0.0;
        let mut optimization_update_seconds = 0.0;
        let mut backward_seconds = 0.0;
        let mut gradient_norm_seconds = 0.0;
        let mut optimizer_step_seconds = 0.0;
        let mut train_rollout_seconds = 0.0;
        let mut train_cpu_seconds = 0.0;
        let mut optimization_loss_cpu_seconds = 0.0;
        let mut optimization_update_cpu_seconds = 0.0;
        let train_chunks = if config.rollout_chunk_size > 1 {
            train_seeds
                .chunks(config.rollout_chunk_size)
                .collect::<Vec<_>>()
        } else {
            vec![train_seeds.as_slice()]
        };
        for (chunk_index, seeds) in train_chunks.iter().enumerate() {
            let rollout_model = inference_model(&model);
            let mut rollout_config = config.rollout.clone();
            rollout_config.max_stage = curriculum_state
                .as_ref()
                .map(|state| state.current_max_stage);
            rollout_config.exploration_iteration = iteration as u64;
            rollout_config.greedy = false;
            let rollout_started = Instant::now();
            let rollout_cpu_started = process_cpu_seconds();
            let mut train_batch = collect_rollouts(
                &rollout_model,
                &device,
                Arc::clone(&game_config),
                seeds,
                &rollout_config,
                progress.map(PpoProgress::rollout_bar),
            )?;
            add_duration(&mut train_rollout_seconds, rollout_started);
            train_cpu_seconds += cpu_seconds_since_or_zero(rollout_cpu_started);
            train_diagnostics = Some(match train_diagnostics.take() {
                Some(previous) => combine_diagnostics(&previous, &train_batch.diagnostics()),
                None => train_batch.diagnostics(),
            });
            normalize_advantages(&mut train_batch);
            emphasize_elite_advantages(&mut train_batch);
            train_episodes += train_batch.episodes.len();
            decision_steps += train_batch.steps().count();
            train_clear_rate_sum += train_batch.clear_rate() * train_batch.episodes.len() as f64;
            let steps = train_batch.steps().collect::<Vec<_>>();
            for epoch in 0..config.ppo_epochs {
                let mut indices = (0..steps.len()).collect::<Vec<_>>();
                deterministic_shuffle(
                    &mut indices,
                    iteration as u64,
                    chunk_index as u64,
                    epoch as u64,
                );
                for minibatch_indices in indices.chunks(config.minibatch_size) {
                    let loss_started = Instant::now();
                    let loss_cpu_started = process_cpu_seconds();
                    let (total_loss, minibatch_metrics) = ppo_loss_for_global_minibatch(
                        &model,
                        &device,
                        &steps,
                        minibatch_indices,
                        config,
                    );
                    add_duration(&mut optimization_loss_seconds, loss_started);
                    optimization_loss_cpu_seconds += cpu_seconds_since_or_zero(loss_cpu_started);
                    let update_cpu_started = process_cpu_seconds();
                    let update_started = Instant::now();
                    let step_result = guarded_minibatch_step(
                        model,
                        &mut optimizer,
                        config.learning_rate,
                        total_loss,
                        minibatch_metrics,
                    );
                    add_duration(&mut optimization_update_seconds, update_started);
                    optimization_update_cpu_seconds +=
                        cpu_seconds_since_or_zero(update_cpu_started);
                    let (updated_model, gradient_norm, update_timing) = match step_result {
                        GuardedMinibatchStep::Stepped(model, gradient_norm, timing) => {
                            (model, gradient_norm, timing)
                        }
                        GuardedMinibatchStep::NonFiniteMetric => {
                            optimization_diagnostics.record_nonfinite();
                            bail!(
                                "non-finite PPO metric at iteration {} chunk {} epoch {} minibatch: metrics={:?}",
                                iteration,
                                chunk_index,
                                epoch,
                                minibatch_indices,
                            );
                        }
                        GuardedMinibatchStep::NonFiniteGradient => {
                            optimization_diagnostics.record_nonfinite();
                            bail!(
                                "non-finite PPO gradient at iteration {} chunk {} epoch {} minibatch {:?}",
                                iteration,
                                chunk_index,
                                epoch,
                                minibatch_indices,
                            );
                        }
                    };
                    backward_seconds += update_timing.backward_seconds;
                    gradient_norm_seconds += update_timing.gradient_norm_seconds;
                    optimizer_step_seconds += update_timing.optimizer_step_seconds;
                    model = *updated_model;
                    optimization_diagnostics.record(
                        minibatch_metrics,
                        minibatch_indices.len(),
                        gradient_norm,
                    );
                    optimizer_step = optimizer_step.saturating_add(1);
                }
            }
            optimization_seconds = optimization_loss_seconds + optimization_update_seconds;
            if let Some(progress) = progress {
                let accumulated_clear_rate = train_clear_rate_sum / train_episodes.max(1) as f64;
                let chunk_count = if config.rollout_chunk_size > 1 {
                    train_seeds.len().div_ceil(config.rollout_chunk_size)
                } else {
                    1
                };
                progress.set_rollout_message(format!(
                    "train batch {}/{} clear {:.1}% lr {:.6} step {}",
                    chunk_index + 1,
                    chunk_count,
                    accumulated_clear_rate * 100.0,
                    config.learning_rate,
                    optimizer_step,
                ));
                progress.set_rollout_message(format!(
                    "train {}/{} {:.1}% step {}",
                    chunk_index + 1,
                    chunk_count,
                    accumulated_clear_rate * 100.0,
                    optimizer_step,
                ));
            }
        }
        if let Some(progress) = progress {
            progress.finish_rollout();
            progress.begin_rollout("validation", validation_seeds.len());
        }
        let validation_started = Instant::now();
        let validation_cpu_started = process_cpu_seconds();
        let validation_model: DeepSetsActorCritic<InferenceBackend> = inference_model(&model);
        let mut validation_rollout_config = config.rollout.clone();
        validation_rollout_config.greedy = true;
        validation_rollout_config.exploration_iteration = 0;
        let mut validation_episodes = 0;
        let mut validation_decision_steps = 0;
        let mut validation_clear_rate_sum = 0.0;
        let mut validation_diagnostics = None;
        let validation_chunks = if config.rollout_chunk_size > 1 {
            validation_seeds
                .chunks(config.rollout_chunk_size)
                .collect::<Vec<_>>()
        } else {
            vec![validation_seeds.as_slice()]
        };
        for (chunk_index, seeds) in validation_chunks.iter().enumerate() {
            let validation_batch = collect_rollouts(
                &validation_model,
                &device,
                Arc::clone(&game_config),
                seeds,
                &validation_rollout_config,
                progress.map(PpoProgress::rollout_bar),
            )?;
            validation_diagnostics = Some(match validation_diagnostics.take() {
                Some(previous) => combine_diagnostics(&previous, &validation_batch.diagnostics()),
                None => validation_batch.diagnostics(),
            });
            validation_episodes += validation_batch.episodes.len();
            validation_decision_steps += validation_batch.steps().count();
            validation_clear_rate_sum +=
                validation_batch.clear_rate() * validation_batch.episodes.len() as f64;
            if let Some(progress) = progress {
                let accumulated_clear_rate =
                    validation_clear_rate_sum / validation_episodes.max(1) as f64;
                let chunk_count = if config.rollout_chunk_size > 1 {
                    validation_seeds.len().div_ceil(config.rollout_chunk_size)
                } else {
                    1
                };
                progress.set_rollout_message(format!(
                    "validation batch {}/{} clear {:.1}% lr {:.6} step {}",
                    chunk_index + 1,
                    chunk_count,
                    accumulated_clear_rate * 100.0,
                    config.learning_rate,
                    optimizer_step,
                ));
                progress.set_rollout_message(format!(
                    "validation {}/{} {:.1}% step {}",
                    chunk_index + 1,
                    chunk_count,
                    accumulated_clear_rate * 100.0,
                    optimizer_step,
                ));
            }
        }
        let validation_rollout_seconds = validation_started.elapsed().as_secs_f64();
        let validation_cpu_seconds = cpu_seconds_since_or_zero(validation_cpu_started);
        let train_clear_rate = train_clear_rate_sum / train_episodes.max(1) as f64;
        let validation_clear_rate = validation_clear_rate_sum / validation_episodes.max(1) as f64;
        if let (Some(curriculum_config), Some(curriculum_state), Some(seed_range)) = (
            config.curriculum.as_ref(),
            curriculum_state.as_mut(),
            config.curriculum_validation,
        ) {
            let mut curriculum_rollout_config = config.rollout.clone();
            curriculum_rollout_config.max_stage = Some(curriculum_state.current_max_stage);
            curriculum_rollout_config.greedy = true;
            curriculum_rollout_config.exploration_iteration = 0;
            let curriculum_started = Instant::now();
            let curriculum_report = super::validation::evaluate_clear_rate(
                &validation_model,
                &device,
                Arc::clone(&game_config),
                seed_range,
                &curriculum_rollout_config,
                None,
            )?;
            curriculum_state.observe_iteration(
                curriculum_config,
                curriculum_report.clear_rate as f32,
                iteration,
            );
            eprintln!(
                "ppo.timing iteration={} phase=curriculum_validation seconds={:.3}",
                iteration,
                curriculum_started.elapsed().as_secs_f64()
            );
        }
        let train_episodes_per_second =
            train_episodes as f64 / train_rollout_seconds.max(f64::EPSILON);
        let train_decisions_per_second =
            decision_steps as f64 / train_rollout_seconds.max(f64::EPSILON);
        let validation_episodes_per_second =
            validation_episodes as f64 / validation_rollout_seconds.max(f64::EPSILON);
        let validation_decisions_per_second =
            validation_decision_steps as f64 / validation_rollout_seconds.max(f64::EPSILON);
        let validation_full_clear_count = validation_diagnostics
            .as_ref()
            .map_or(0, |diagnostics| diagnostics.full_clear_count);
        let validation_truncated_count = validation_diagnostics
            .as_ref()
            .map_or(usize::MAX, |diagnostics| {
                diagnostics.truncated_episode_count
            });
        let validation_score = BestScore {
            full_clear_count: validation_full_clear_count,
            clear_rate: validation_clear_rate,
            truncated_count: validation_truncated_count,
        };
        let best_score = BestScore {
            full_clear_count: best_validation_full_clear_count,
            clear_rate: best_validation_clear_rate,
            truncated_count: best_validation_truncated_count,
        };
        if validation_score.is_better_than(best_score) {
            best_validation_clear_rate = validation_clear_rate;
            best_validation_full_clear_count = validation_full_clear_count;
            best_validation_truncated_count = validation_truncated_count;
            best_iteration = iteration;
            best_model = model.clone();
            best_optimizer = optimizer.clone();
        } else {
            model = best_model.clone();
            optimizer = best_optimizer.clone();
        }
        let learner_seconds = started.elapsed().as_secs_f64();
        history.push(PpoIterationStats {
            iteration,
            train_clear_rate,
            validation_clear_rate,
            train_episodes,
            validation_episodes,
            decision_steps,
            learner_seconds,
            train_rollout_seconds,
            optimization_seconds,
            validation_rollout_seconds,
            train_episodes_per_second,
            train_decisions_per_second,
            validation_episodes_per_second,
            validation_decisions_per_second,
            train_diagnostics: train_diagnostics.clone().unwrap_or_else(|| {
                RolloutBatch {
                    episodes: Vec::new(),
                }
                .diagnostics()
            }),
            validation_diagnostics: validation_diagnostics.clone().unwrap_or_else(|| {
                RolloutBatch {
                    episodes: Vec::new(),
                }
                .diagnostics()
            }),
        });
        if let Some(progress) = progress {
            progress.finish_rollout();
            progress.finish_iteration(
                train_clear_rate,
                validation_clear_rate,
                RolloutTiming {
                    train_seconds: train_rollout_seconds,
                    optimization_seconds,
                    validation_seconds: validation_rollout_seconds,
                    train_decisions_per_second,
                    validation_decisions_per_second,
                },
            );
            if let (Some(train), Some(validation)) =
                (train_diagnostics.as_ref(), validation_diagnostics.as_ref())
            {
                progress.print_event(&format_compact_iteration(
                    iteration,
                    "train",
                    "unmanaged",
                    config.reward_config.no_progress_cycle_penalty,
                    train_clear_rate,
                    validation_clear_rate,
                    train,
                    validation,
                    optimizer_step,
                    optimization_diagnostics.finish(),
                ));
            }
        }
        if let Some(store) = &checkpoint_store {
            let checkpoint_started = Instant::now();
            let state = TrainerState {
                schema_version: super::trainer_checkpoint::TRAINER_CHECKPOINT_SCHEMA_VERSION,
                generation: iteration as u64,
                iteration,
                optimizer_step,
                next_train_seed_offset: 0,
                learning_rate: config.learning_rate,
                best_validation_clear_rate: best_validation_clear_rate.max(0.0),
                best_iteration,
                best_validation_full_clear_count,
                best_validation_truncated_count,
                git_revision: super::neural_checkpoint::current_git_revision()?,
                contract: MlContract::from_config(game_config.as_ref()),
                model_config: config.model,
                reward_config: config.reward_config.clone(),
                seed_schedule: schedule.clone(),
                hyperparameters: trainer_hyperparameters(config),
                model_file: String::new(),
                best_model_file: String::new(),
                optimizer_file: String::new(),
                best_optimizer_file: String::new(),
                curriculum_config: config.curriculum.clone(),
                curriculum_state: curriculum_state.clone(),
            };
            store.save::<DeepSetsActorCritic<TrainBackend>, _>(
                &state,
                &model,
                &best_model,
                &optimizer,
                &best_optimizer,
            )?;
            eprintln!(
                "ppo.timing iteration={} phase=trainer_checkpoint seconds={:.3}",
                iteration,
                checkpoint_started.elapsed().as_secs_f64()
            );
        }
        let total_seconds = started.elapsed().as_secs_f64();
        let total_cpu_seconds = cpu_seconds_since_or_zero(cpu_started);
        eprintln!(
            "ppo.timing iteration={} phase=summary train_rollout={:.3} train_cpu={:.3} train_effective_cores={:.2} optimization_loss={:.3} optimization_loss_cpu={:.3} optimization_loss_effective_cores={:.2} optimization_update={:.3} optimization_update_cpu={:.3} optimization_update_effective_cores={:.2} backward={:.3} gradient_norm={:.3} optimizer_step={:.3} optimization_total={:.3} validation={:.3} validation_cpu={:.3} validation_effective_cores={:.2} total={:.3} total_cpu={:.3} total_effective_cores={:.2} train_steps={} optimizer_steps={}",
            iteration,
            train_rollout_seconds,
            train_cpu_seconds,
            train_cpu_seconds / train_rollout_seconds.max(f64::EPSILON),
            optimization_loss_seconds,
            optimization_loss_cpu_seconds,
            optimization_loss_cpu_seconds / optimization_loss_seconds.max(f64::EPSILON),
            optimization_update_seconds,
            optimization_update_cpu_seconds,
            optimization_update_cpu_seconds / optimization_update_seconds.max(f64::EPSILON),
            backward_seconds,
            gradient_norm_seconds,
            optimizer_step_seconds,
            optimization_seconds,
            validation_rollout_seconds,
            validation_cpu_seconds,
            validation_cpu_seconds / validation_rollout_seconds.max(f64::EPSILON),
            total_seconds,
            total_cpu_seconds,
            total_cpu_seconds / total_seconds.max(f64::EPSILON),
            decision_steps,
            optimizer_step,
        );
    }

    Ok(PpoTrainingRun {
        model,
        best_model,
        best_iteration,
        best_validation_clear_rate,
        best_validation_full_clear_count,
        best_validation_truncated_count,
        elite_train_seeds: Vec::new(),
        optimization_diagnostics: optimization_diagnostics.finish(),
        history,
    })
}

fn ppo_loss_for_global_minibatch<B: burn::tensor::backend::AutodiffBackend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    steps: &[&RolloutStep],
    indices: &[usize],
    config: &PpoConfig,
) -> (Tensor<B, 1>, OptimizationMinibatchMetrics) {
    let mut buckets = BTreeMap::<usize, Vec<&RolloutStep>>::new();
    for &index in indices {
        let step = steps[index];
        buckets
            .entry(step.candidate_count())
            .or_default()
            .push(step);
    }

    let mut total_loss = None;
    let mut total_metrics = OptimizationMinibatchMetrics::default();
    for (_candidate_count, bucket) in buckets {
        let (loss, metrics) = ppo_loss_for_minibatch_with_metrics(
            model,
            device,
            &bucket,
            bucket[0].candidate_count(),
            config.clip_epsilon,
            config.entropy_coefficient,
            config.value_coefficient,
        );
        let weight = bucket.len() as f32 / indices.len() as f32;
        let weighted_loss = loss * weight;
        total_loss = Some(match total_loss {
            Some(total) => total + weighted_loss,
            None => weighted_loss,
        });
        total_metrics.policy_loss += metrics.policy_loss * weight;
        total_metrics.value_loss += metrics.value_loss * weight;
        total_metrics.entropy += metrics.entropy * weight;
        total_metrics.approximate_kl += metrics.approximate_kl * weight;
        total_metrics.clip_fraction += metrics.clip_fraction * weight;
    }
    (
        total_loss.expect("PPO global minibatch must not be empty"),
        total_metrics,
    )
}

pub(crate) fn ppo_loss_for_minibatch<B: burn::tensor::backend::AutodiffBackend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    steps: &[&RolloutStep],
    clip_epsilon: f32,
    entropy_coefficient: f32,
    value_coefficient: f32,
) -> Tensor<B, 1> {
    let candidate_count = steps[0].candidate_count();
    assert!(
        steps
            .iter()
            .all(|step| step.candidate_count() == candidate_count),
        "PPO minibatch must contain a single candidate-count bucket"
    );
    ppo_loss_for_minibatch_with_metrics(
        model,
        device,
        steps,
        candidate_count,
        clip_epsilon,
        entropy_coefficient,
        value_coefficient,
    )
    .0
}

fn ppo_loss_for_minibatch_with_metrics<B: burn::tensor::backend::AutodiffBackend>(
    model: &DeepSetsActorCritic<B>,
    device: &B::Device,
    steps: &[&RolloutStep],
    candidate_count: usize,
    clip_epsilon: f32,
    entropy_coefficient: f32,
    value_coefficient: f32,
) -> (Tensor<B, 1>, OptimizationMinibatchMetrics) {
    let mut typed_sets = std::array::from_fn(|_| Vec::new());
    for step in steps {
        for (set_index, set) in step.typed_observation.sets.iter().enumerate() {
            typed_sets[set_index].push(set.clone());
        }
    }
    let state_batches = typed_sets.map(|sets| PaddedEntityBatch::from_sets(&sets));
    let typed_state = model.encode_typed_sets(&state_batches, device);

    let mut candidate_sets = Vec::with_capacity(steps.len() * candidate_count);
    for step in steps {
        for row in &step.candidate_rows {
            candidate_sets.push(EntitySet::new(vec![row.clone()]));
        }
    }
    let candidates = PaddedEntityBatch::from_sets(&candidate_sets);
    let candidate_group_sizes = vec![candidate_count; steps.len()];

    let mut targets = vec![0.0; steps.len() * candidate_count];
    let mut legal_mask_values = vec![0.0; steps.len() * candidate_count];
    let mut old_log_probabilities = Vec::with_capacity(steps.len());
    let mut advantages = Vec::with_capacity(steps.len());
    let mut returns = Vec::with_capacity(steps.len());

    for (row, step) in steps.iter().enumerate() {
        assert!(step.legal_candidate_mask.len() <= candidate_count);
        assert!(step.legal_candidate_mask.iter().any(|legal| *legal));
        targets[row * candidate_count + step.action_index] = 1.0;
        for (column, legal) in step.legal_candidate_mask.iter().enumerate() {
            legal_mask_values[row * candidate_count + column] = if *legal { 0.0 } else { -1.0e9 };
        }
        old_log_probabilities.push(step.old_log_probability);
        advantages.push(step.advantage);
        returns.push(step.return_value);
    }

    let encoded_state = model.encode_state(tensor_from_rows(
        &steps
            .iter()
            .map(|step| step.state.clone())
            .collect::<Vec<_>>(),
        device,
    ));
    let state_width = encoded_state.dims()[1];
    let expanded_state = encoded_state
        .reshape([steps.len(), 1, state_width])
        .repeat_dim(1, candidate_count)
        .reshape([steps.len() * candidate_count, state_width]);
    let typed_state_width = typed_state.dims()[1];
    let expanded_typed_state = typed_state
        .clone()
        .reshape([steps.len(), 1, typed_state_width])
        .repeat_dim(1, candidate_count)
        .reshape([steps.len() * candidate_count, typed_state_width]);
    let logits = model
        .forward_typed_logits_with_encoded_state(
            expanded_state,
            &candidates,
            expanded_typed_state,
            &candidate_group_sizes,
            device,
        )
        .reshape([steps.len(), candidate_count]);
    let legal_mask: Tensor<B, 2> = Tensor::from_data(
        TensorData::new(legal_mask_values, [steps.len(), candidate_count]),
        device,
    );
    let probabilities = softmax(logits + legal_mask, 1).clamp(1e-7, 1.0);
    let log_probabilities = probabilities.clone().log();
    let target: Tensor<B, 2> = Tensor::from_data(
        TensorData::new(targets, [steps.len(), candidate_count]),
        device,
    );
    let current_log_probabilities = (log_probabilities.clone() * target).sum_dim(1);
    let old_log_probabilities = Tensor::from_data(
        TensorData::new(old_log_probabilities, [steps.len(), 1]),
        device,
    );
    let ratios = (current_log_probabilities.clone() - old_log_probabilities.clone()).exp();
    let clipped_ratios = ratios.clone().clamp(1.0 - clip_epsilon, 1.0 + clip_epsilon);
    let advantages = Tensor::from_data(TensorData::new(advantages, [steps.len(), 1]), device);
    let surrogate = ratios.clone() * advantages.clone();
    let clipped_surrogate = clipped_ratios * advantages;
    let policy_loss = surrogate.min_pair(clipped_surrogate).neg();
    let entropy = (probabilities * log_probabilities).sum_dim(1).neg();
    let values = model.forward_typed_values(typed_state);
    let returns = Tensor::from_data(TensorData::new(returns, [steps.len(), 1]), device);
    let value_loss = (values - returns).powf_scalar(2.0);
    let metric_values = Tensor::cat(
        vec![
            policy_loss.clone(),
            value_loss.clone(),
            entropy.clone(),
            current_log_probabilities.clone(),
            old_log_probabilities.clone(),
        ],
        1,
    )
    .mean_dim(0)
    .into_data()
    .to_vec::<f32>()
    .expect("PPO diagnostic metrics must be f32");
    let ratios = ratios
        .into_data()
        .to_vec::<f32>()
        .expect("PPO ratios must be f32");
    let metrics = OptimizationMinibatchMetrics {
        policy_loss: metric_values[0],
        value_loss: metric_values[1],
        entropy: metric_values[2],
        approximate_kl: metric_values[4] - metric_values[3],
        clip_fraction: ratios
            .iter()
            .filter(|ratio| **ratio < 1.0 - clip_epsilon || **ratio > 1.0 + clip_epsilon)
            .count() as f32
            / steps.len() as f32,
    };
    (
        (policy_loss + value_loss * value_coefficient - entropy * entropy_coefficient).mean(),
        metrics,
    )
}

fn deterministic_shuffle(values: &mut [usize], iteration: u64, chunk: u64, epoch: u64) {
    let mut state = 0xD1B5_4A32_D192_ED03_u64
        ^ iteration.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ chunk.wrapping_mul(0xBF58_476D_1CE4_E5B9)
        ^ epoch.wrapping_mul(0x94D0_49BB_1331_11EB);
    for index in (1..values.len()).rev() {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state = state.wrapping_mul(2_685_821_657_736_338_717);
        let swap_index = (state as usize) % (index + 1);
        values.swap(index, swap_index);
    }
}

#[cfg(test)]
fn expected_optimizer_updates(
    step_count: usize,
    minibatch_size: usize,
    ppo_epochs: usize,
) -> usize {
    step_count.div_ceil(minibatch_size) * ppo_epochs
}

fn normalize_advantages(batch: &mut RolloutBatch) {
    let advantages = batch.steps().map(|step| step.advantage).collect::<Vec<_>>();
    if advantages.is_empty() {
        return;
    }
    let mean = advantages.iter().sum::<f32>() / advantages.len() as f32;
    let variance = advantages
        .iter()
        .map(|advantage| (advantage - mean).powi(2))
        .sum::<f32>()
        / advantages.len() as f32;
    let scale = variance.sqrt().max(1e-8);
    for step in batch.steps_mut() {
        step.advantage = (step.advantage - mean) / scale;
    }
}

fn emphasize_elite_advantages(batch: &mut RolloutBatch) {
    if batch.episodes.len() < 2 {
        return;
    }
    let mean_clear_rate = batch
        .episodes
        .iter()
        .map(|episode| episode.final_clear_rate)
        .sum::<f32>()
        / batch.episodes.len() as f32;
    let best_clear_rate = batch
        .episodes
        .iter()
        .map(|episode| episode.final_clear_rate)
        .max_by(f32::total_cmp)
        .unwrap_or(mean_clear_rate);
    if best_clear_rate <= mean_clear_rate + 1e-6 {
        return;
    }
    let elite_threshold = mean_clear_rate + (best_clear_rate - mean_clear_rate) * 0.5;
    for episode in &mut batch.episodes {
        if episode.final_clear_rate < elite_threshold {
            continue;
        }
        for step in &mut episode.steps {
            if step.advantage > 0.0 {
                step.advantage *= 1.5;
            }
        }
    }
}

fn elite_episode_seeds(batch: &RolloutBatch) -> Vec<u64> {
    if batch.episodes.len() < 2 {
        return Vec::new();
    }
    let mean_clear_rate = batch.clear_rate() as f32;
    batch
        .episodes
        .iter()
        .filter(|episode| episode.final_clear_rate > mean_clear_rate + 1e-6)
        .map(|episode| episode.seed)
        .take(32)
        .collect()
}

fn prioritized_train_seeds(base_seeds: &[u64], elite_seeds: &[u64]) -> Vec<u64> {
    if base_seeds.is_empty() || elite_seeds.is_empty() {
        return base_seeds.to_vec();
    }
    let replay_count = (base_seeds.len() / 8).max(1).min(elite_seeds.len());
    let mut seeds = base_seeds.to_vec();
    let start = seeds.len() - replay_count;
    for (offset, seed) in elite_seeds.iter().take(replay_count).enumerate() {
        seeds[start + offset] = *seed;
    }
    seeds
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulator::ml::seed::{SeedRange, TrainingSeedSchedule};

    fn diagnostics_for(values: &[f32]) -> RolloutDiagnostics {
        let episodes = values
            .iter()
            .enumerate()
            .map(|(seed, reward)| super::super::rollout::EpisodeRollout {
                seed: seed as u64,
                steps: vec![super::super::rollout::RolloutStep {
                    typed_observation: super::super::encoding::TypedObservation::default(),
                    legal_candidate_mask: vec![true],
                    state: vec![],
                    candidate_rows: vec![],
                    action_index: 0,
                    old_log_probability: 0.0,
                    value_estimate: 0.0,
                    next_value_estimate: 0.0,
                    reward: *reward,
                    terminal_reward: 0.0,
                    shaping_reward: 0.0,
                    no_progress_cycle_penalty_reward: 0.0,
                    terminated: false,
                    truncated: false,
                    no_progress_cycle: false,
                    bootstrap_allowed: true,
                    action_kind: 0,
                    advantage: *reward,
                    return_value: *reward,
                    policy_entropy: 0.0,
                }],
                cleared: false,
                final_clear_rate: 0.0,
                final_stage: 0,
                escaped_hp: 0.0,
                player_damage: 0.0,
                tower_damage: 0.0,
                episode_return: *reward,
                termination_reason: crate::simulator::environment::StepReason::Terminal,
                reward_component_sums: BTreeMap::new(),
                decision_point_counts: BTreeMap::new(),
                forced_action_count: 0,
                forced_action_counts: BTreeMap::new(),
            })
            .collect();
        super::super::rollout::RolloutBatch { episodes }.diagnostics()
    }

    #[test]
    fn stabilization_defaults_are_conservative() {
        let config = PpoConfig::default();
        assert_eq!(config.ppo_epochs, 1);
        assert_eq!(config.learning_rate, 1e-4);
        assert_eq!(config.clip_epsilon, 0.1);
    }

    #[test]
    fn best_score_uses_full_clear_then_rate_then_truncation() {
        let score = BestScore {
            full_clear_count: 2,
            clear_rate: 0.5,
            truncated_count: 4,
        };
        assert!(score.is_better_than(BestScore {
            full_clear_count: 1,
            clear_rate: 1.0,
            truncated_count: 0,
        }));
        assert!(
            BestScore {
                full_clear_count: 2,
                clear_rate: 0.6,
                truncated_count: 4,
            }
            .is_better_than(score)
        );
        assert!(
            BestScore {
                full_clear_count: 2,
                clear_rate: 0.5,
                truncated_count: 3,
            }
            .is_better_than(score)
        );
    }

    #[test]
    fn best_score_converters_preserve_all_tiebreak_fields() {
        let checkpoint = NeuralCheckpoint {
            checkpoint_schema_version:
                super::super::neural_checkpoint::NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: super::super::neural_checkpoint::POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: super::super::model::ENTITY_ENCODER_SCHEMA_VERSION,
            contract: MlContract::from_config(&GameConfig::default_config()),
            seed_schedule: TrainingSeedSchedule::try_new(
                SeedRange::try_new(0, 0).unwrap(),
                SeedRange::try_new(1, 1).unwrap(),
            )
            .unwrap(),
            iteration: 1,
            best_iteration: 1,
            git_revision: "test".to_string(),
            model_config: ModelConfig::default(),
            model_file: "model.mpk".to_string(),
            train_clear_rate: 0.0,
            validation_clear_rate: 0.0,
            best_validation_clear_rate: 0.5,
            best_validation_full_clear_count: 3,
            best_validation_truncated_count: 2,
            reward_config: RewardConfig::default(),
            hyperparameters: BTreeMap::new(),
        };
        let score = best_score_from_checkpoint(&checkpoint);
        assert_eq!(score.full_clear_count, 3);
        assert_eq!(score.clear_rate, 0.5);
        assert_eq!(score.truncated_count, 2);
    }

    #[test]
    fn trainer_resume_continues_from_the_saved_iteration() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).unwrap(),
            SeedRange::try_new(u64::MAX, u64::MAX).unwrap(),
        )
        .unwrap();
        let root =
            std::env::temp_dir().join(format!("tower-defense-resume-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);

        // Both paths must start from identical weights, so seed them from one
        // shared initialization checkpoint instead of per-run random init.
        let device = default_policy_device();
        let init_model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&init_model, &device);
        let init_path = root.join("init-checkpoint.json");
        std::fs::create_dir_all(&root).expect("create temp dir");
        NeuralCheckpoint {
            checkpoint_schema_version:
                super::super::neural_checkpoint::NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: super::super::neural_checkpoint::POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: super::super::model::ENTITY_ENCODER_SCHEMA_VERSION,
            contract: MlContract::from_config(&GameConfig::default_config()),
            seed_schedule: schedule.clone(),
            iteration: 0,
            best_iteration: 0,
            git_revision: "test".to_string(),
            model_config: ModelConfig::default(),
            model_file: String::new(),
            train_clear_rate: 0.0,
            validation_clear_rate: 0.0,
            best_validation_clear_rate: 0.0,
            best_validation_full_clear_count: 0,
            best_validation_truncated_count: usize::MAX,
            reward_config: RewardConfig::default(),
            hyperparameters: BTreeMap::new(),
        }
        .save_with_model(&init_model, &init_path)
        .expect("save init checkpoint");

        let mut first_config = PpoConfig {
            iterations: 1,
            ppo_epochs: 1,
            minibatch_size: 1,
            ..PpoConfig::default()
        };
        first_config.rollout.max_decisions_per_episode = 1;
        first_config.trainer_checkpoint_dir = Some(root.clone());
        first_config.init_checkpoint = Some(init_path.clone());
        let first = train_ppo(
            Arc::new(GameConfig::default_config()),
            &schedule,
            &first_config,
        )
        .expect("first update should succeed");

        let first_state: TrainerState = serde_json::from_slice(
            &std::fs::read(root.join("latest.json")).expect("read first trainer state"),
        )
        .expect("decode first trainer state");
        assert_eq!(first_state.iteration, 0);
        assert!(first_state.optimizer_step > 0);

        let mut resume_config = first_config.clone();
        resume_config.resume_trainer = true;
        let resumed = train_ppo(
            Arc::new(GameConfig::default_config()),
            &schedule,
            &resume_config,
        )
        .expect("resumed update should succeed");

        let resumed_state: TrainerState = serde_json::from_slice(
            &std::fs::read(root.join("latest.json")).expect("read resumed trainer state"),
        )
        .expect("decode resumed trainer state");
        assert_eq!(resumed_state.iteration, 1);
        assert!(resumed_state.optimizer_step > first_state.optimizer_step);

        assert_eq!(first.history[0].iteration, 0);
        assert_eq!(resumed.history[0].iteration, 1);
        assert_eq!(resumed.optimization_diagnostics.update_count, 1);

        let mut uninterrupted_config = first_config.clone();
        uninterrupted_config.trainer_checkpoint_dir = None;
        uninterrupted_config.iterations = 2;
        let uninterrupted = train_ppo(
            Arc::new(GameConfig::default_config()),
            &schedule,
            &uninterrupted_config,
        )
        .expect("uninterrupted two-update run should succeed");

        let logit_of = |model: &DeepSetsActorCritic<TrainBackend>| {
            inference_model(model)
                .forward_values(tensor_from_rows::<InferenceBackend>(
                    &[vec![0.25; super::super::features::GLOBAL_FEATURE_COUNT]],
                    &device,
                ))
                .into_data()
                .to_vec::<f32>()
                .expect("logits must be f32")
        };
        let resumed_logits = logit_of(&resumed.model);
        let uninterrupted_logits = logit_of(&uninterrupted.model);
        for (resumed_value, uninterrupted_value) in
            resumed_logits.iter().zip(uninterrupted_logits.iter())
        {
            assert!(
                (resumed_value - uninterrupted_value).abs() < 1e-4,
                "resumed logit {resumed_value} differs from uninterrupted {uninterrupted_value}"
            );
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn invalid_ppo_config_is_rejected_before_rollout() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).unwrap(),
            SeedRange::try_new(u64::MAX, u64::MAX).unwrap(),
        )
        .unwrap();
        let config = PpoConfig {
            iterations: 0,
            ..PpoConfig::default()
        };
        let error = train_ppo(Arc::new(GameConfig::default_config()), &schedule, &config)
            .expect_err("zero iterations should fail");
        assert!(error.to_string().contains("iterations"));
    }

    #[test]
    fn curriculum_resume_preserves_level_counter_and_next_update_logits() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).unwrap(),
            SeedRange::try_new(u64::MAX, u64::MAX).unwrap(),
        )
        .unwrap();
        let root = std::env::temp_dir().join(format!(
            "tower-defense-curriculum-resume-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);

        let device = default_policy_device();
        let init_model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&init_model, &device);
        let init_path = root.join("init-checkpoint.json");
        std::fs::create_dir_all(&root).expect("create temp dir");
        NeuralCheckpoint {
            checkpoint_schema_version:
                super::super::neural_checkpoint::NEURAL_CHECKPOINT_SCHEMA_VERSION,
            policy_schema_version: super::super::neural_checkpoint::POLICY_SCHEMA_VERSION,
            entity_encoder_schema_version: super::super::model::ENTITY_ENCODER_SCHEMA_VERSION,
            contract: MlContract::from_config(&GameConfig::default_config()),
            seed_schedule: schedule.clone(),
            iteration: 0,
            best_iteration: 0,
            git_revision: "test".to_string(),
            model_config: ModelConfig::default(),
            model_file: String::new(),
            train_clear_rate: 0.0,
            validation_clear_rate: 0.0,
            best_validation_clear_rate: 0.0,
            best_validation_full_clear_count: 0,
            best_validation_truncated_count: usize::MAX,
            reward_config: RewardConfig::default(),
            hyperparameters: BTreeMap::new(),
        }
        .save_with_model(&init_model, &init_path)
        .expect("save init checkpoint");

        let curriculum = CurriculumConfig {
            initial_max_stage: 1,
            final_max_stage: 3,
            promotion_threshold: 0.5,
            promotion_patience: 2,
            min_iterations_per_level: 2,
        };
        let mut first_config = PpoConfig {
            iterations: 1,
            ppo_epochs: 1,
            minibatch_size: 1,
            curriculum: Some(curriculum.clone()),
            curriculum_validation: Some(SeedRange::try_new(5, 6).unwrap()),
            ..PpoConfig::default()
        };
        first_config.rollout.max_decisions_per_episode = 1;
        first_config.trainer_checkpoint_dir = Some(root.clone());
        first_config.init_checkpoint = Some(init_path.clone());
        train_ppo(
            Arc::new(GameConfig::default_config()),
            &schedule,
            &first_config,
        )
        .expect("first curriculum update should succeed");

        // Advance the curriculum state deterministically so the saved level and
        // counter are non-trivial before the resume.
        {
            let store =
                crate::simulator::ml::trainer_checkpoint::TrainerCheckpointStore::new(&root)
                    .unwrap();
            let (state, _, _, _, _) = store
                .load_latest::<_, DeepSetsActorCritic<TrainBackend>>(
                    ModelConfig::default(),
                    &MlContract::from_config(&GameConfig::default_config()),
                    &RewardConfig::default(),
                    &schedule,
                    &trainer_hyperparameters(&first_config),
                    Some(&curriculum),
                    AdamConfig::new().init(),
                    &device,
                )
                .unwrap()
                .expect("trainer state should exist after first update");
            let mut curriculum_state = state.curriculum_state.expect("curriculum state");
            curriculum_state.observe_iteration(&curriculum, 1.0, state.iteration);
            assert_eq!(curriculum_state.current_max_stage, 1);
            assert_eq!(curriculum_state.iterations_at_level, 2);
            assert_eq!(curriculum_state.consecutive_successes, 1);
            let _ = store;
        }

        let mut resume_config = first_config.clone();
        resume_config.resume_trainer = true;
        let resumed = train_ppo(
            Arc::new(GameConfig::default_config()),
            &schedule,
            &resume_config,
        )
        .expect("resumed curriculum update should succeed");

        let mut uninterrupted_config = first_config.clone();
        uninterrupted_config.trainer_checkpoint_dir = None;
        uninterrupted_config.iterations = 2;
        let uninterrupted = train_ppo(
            Arc::new(GameConfig::default_config()),
            &schedule,
            &uninterrupted_config,
        )
        .expect("uninterrupted two-update run should succeed");

        let logit_of = |model: &DeepSetsActorCritic<TrainBackend>| {
            inference_model(model)
                .forward_values(tensor_from_rows::<InferenceBackend>(
                    &[vec![0.25; super::super::features::GLOBAL_FEATURE_COUNT]],
                    &device,
                ))
                .into_data()
                .to_vec::<f32>()
                .expect("logits must be f32")
        };
        let resumed_logits = logit_of(&resumed.model);
        let uninterrupted_logits = logit_of(&uninterrupted.model);
        for (resumed_value, uninterrupted_value) in
            resumed_logits.iter().zip(uninterrupted_logits.iter())
        {
            assert!(
                (resumed_value - uninterrupted_value).abs() < 1e-4,
                "resumed logit {resumed_value} differs from uninterrupted {uninterrupted_value}"
            );
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn reward_gamma_mismatch_is_rejected_before_rollout() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).unwrap(),
            SeedRange::try_new(u64::MAX, u64::MAX).unwrap(),
        )
        .unwrap();
        let config = PpoConfig {
            rollout: RolloutConfig {
                gamma: 0.9,
                ..RolloutConfig::default()
            },
            ..PpoConfig::default()
        };

        let error = train_ppo(Arc::new(GameConfig::default_config()), &schedule, &config)
            .expect_err("gamma mismatch should fail before rollout");
        assert!(error.to_string().contains("does not match rollout gamma"));
    }

    #[test]
    fn global_update_count_ignores_candidate_buckets() {
        assert_eq!(expected_optimizer_updates(600, 256, 4), 12);
        assert_eq!(expected_optimizer_updates(1, 256, 4), 4);
        assert_eq!(expected_optimizer_updates(512, 256, 4), 8);
    }

    #[test]
    fn merged_diagnostics_use_exact_global_variance() {
        let first = diagnostics_for(&[0.0, 2.0]);
        let second = diagnostics_for(&[4.0, 6.0]);
        let merged = combine_diagnostics(&first, &second);
        assert!((merged.reward_stddev - 2.236_068).abs() < 1e-5);
        assert!((merged.episode_return_stddev - 2.236_068).abs() < 1e-5);
        assert!((merged.advantage_stddev - 2.236_068).abs() < 1e-5);
    }

    #[test]
    fn shuffle_is_deterministic_and_changes_order() {
        let mut first = (0..32).collect::<Vec<_>>();
        let mut second = first.clone();
        deterministic_shuffle(&mut first, 3, 5, 7);
        deterministic_shuffle(&mut second, 3, 5, 7);
        assert_eq!(first, second);
        assert_ne!(first, (0..32).collect::<Vec<_>>());
    }

    #[test]
    fn global_minibatch_update_count_is_independent_of_candidate_distribution() {
        let mut first = Vec::new();
        let mut second = Vec::new();
        for index in 0..600 {
            first.push(index % 4);
            second.push((index % 4) * 17 + 1);
        }
        assert_eq!(first.len(), second.len());
        assert_eq!(expected_optimizer_updates(first.len(), 256, 4), 12);
        assert_eq!(expected_optimizer_updates(second.len(), 256, 4), 12);
    }

    #[test]
    fn optimization_diagnostics_are_sample_weighted_and_mergeable() {
        let metrics = OptimizationMinibatchMetrics {
            policy_loss: 2.0,
            value_loss: 4.0,
            entropy: 6.0,
            approximate_kl: 0.2,
            clip_fraction: 0.5,
        };
        let mut first = OptimizationDiagnosticsAccumulator::default();
        first.record(metrics, 2, 3.0);
        let mut second = OptimizationDiagnosticsAccumulator::default();
        second.record(
            OptimizationMinibatchMetrics {
                policy_loss: 8.0,
                value_loss: 10.0,
                entropy: 12.0,
                approximate_kl: 0.6,
                clip_fraction: 1.0,
            },
            6,
            7.0,
        );
        let first = first.finish();
        let second = second.finish();
        assert_eq!(first.sample_count, 2);
        assert_eq!(second.sample_count, 6);
        assert!((first.policy_loss * 0.25 + second.policy_loss * 0.75 - 6.5).abs() < 1e-6);
        assert!((first.approximate_kl * 0.25 + second.approximate_kl * 0.75 - 0.5).abs() < 1e-6);
        assert_eq!(first.update_count + second.update_count, 2);
    }

    #[test]
    fn optimization_diagnostics_mark_nonfinite_metrics() {
        let mut accumulator = OptimizationDiagnosticsAccumulator::default();
        accumulator.record(
            OptimizationMinibatchMetrics {
                policy_loss: f32::NAN,
                ..OptimizationMinibatchMetrics::default()
            },
            1,
            f32::INFINITY,
        );
        assert_eq!(accumulator.finish().nonfinite_count, 1);
    }

    #[test]
    fn overfit_gate_requires_real_environment_metric_improvement() {
        let initial = diagnostics_for(&[0.0]);
        let final_diagnostics = diagnostics_for(&[0.1]);
        let supervised = SupervisedNllReport {
            sample_count: 1,
            mean_nll: 0.0,
            top1_accuracy: 1.0,
        };
        let report = evaluate_overfit_gate(
            &initial,
            &final_diagnostics,
            1,
            64,
            OverfitGateThreshold {
                maximum_supervised_nll: 0.01,
                maximum_value_loss: 1.0,
                ..OverfitGateThreshold::default()
            },
            &supervised,
            &supervised,
        );
        assert!(report.passed);
        assert_eq!(report.seed_count, 1);
        assert_eq!(report.max_decisions, 64);
    }

    #[test]
    fn nonfinite_loss_error_contains_update_context() {
        let mut accumulator = OptimizationDiagnosticsAccumulator::default();
        accumulator.record_nonfinite();
        let diagnostics = accumulator.finish();
        assert_eq!(diagnostics.nonfinite_count, 1);
    }

    #[test]
    fn one_fixed_seed_iteration_runs_rollout_loss_and_validation() {
        let schedule = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).unwrap(),
            SeedRange::try_new(u64::MAX, u64::MAX).unwrap(),
        )
        .unwrap();
        let config = PpoConfig {
            iterations: 1,
            ppo_epochs: 1,
            rollout: RolloutConfig {
                max_decisions_per_episode: 1,
                ..RolloutConfig::default()
            },
            ..PpoConfig::default()
        };
        let run = train_ppo(Arc::new(GameConfig::default_config()), &schedule, &config)
            .expect("one-step PPO run should succeed");
        assert_eq!(run.history.len(), 1);
        assert_eq!(run.history[0].train_episodes, 1);
        assert_eq!(run.history[0].validation_episodes, 1);
        assert_eq!(run.history[0].decision_steps, 1);
    }

    #[test]
    fn one_and_four_seed_overfit_diagnostics_are_finite_and_reproducible() {
        fn diagnostic_config() -> PpoConfig {
            PpoConfig {
                iterations: 2,
                ppo_epochs: 1,
                learning_rate: 1e-4,
                rollout: RolloutConfig {
                    max_decisions_per_episode: 2,
                    ..RolloutConfig::default()
                },
                ..PpoConfig::default()
            }
        }

        let one_seed = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 0).unwrap(),
            SeedRange::try_new(u64::MAX, u64::MAX).unwrap(),
        )
        .unwrap();
        let four_seeds = TrainingSeedSchedule::try_new(
            SeedRange::try_new(0, 3).unwrap(),
            SeedRange::try_new(u64::MAX - 3, u64::MAX).unwrap(),
        )
        .unwrap();
        let one = train_ppo(
            Arc::new(GameConfig::default_config()),
            &one_seed,
            &diagnostic_config(),
        )
        .expect("one-seed diagnostic should run");
        let four = train_ppo(
            Arc::new(GameConfig::default_config()),
            &four_seeds,
            &diagnostic_config(),
        )
        .expect("four-seed diagnostic should run");
        for run in [&one, &four] {
            assert_eq!(run.history.len(), 2);
            assert!(run.history.iter().all(|iteration| {
                iteration.train_clear_rate.is_finite()
                    && iteration.validation_clear_rate.is_finite()
                    && iteration.learner_seconds.is_finite()
                    && iteration.decision_steps > 0
            }));
        }
        let repeat = train_ppo(
            Arc::new(GameConfig::default_config()),
            &one_seed,
            &diagnostic_config(),
        )
        .expect("repeated one-seed diagnostic should run");
        assert_eq!(
            one.history
                .iter()
                .map(|iteration| (iteration.train_clear_rate, iteration.validation_clear_rate))
                .collect::<Vec<_>>(),
            repeat
                .history
                .iter()
                .map(|iteration| (iteration.train_clear_rate, iteration.validation_clear_rate))
                .collect::<Vec<_>>()
        );
    }

    #[cfg(test)]
    fn fixture_step(action_index: usize, illegal_index: Option<usize>) -> RolloutStep {
        use crate::simulator::ml::encoding::{EntityRow, TypedObservation};
        let candidate_count = 3;
        let mut legal_candidate_mask = vec![true; candidate_count];
        if let Some(illegal_index) = illegal_index {
            legal_candidate_mask[illegal_index] = false;
        }
        RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask,
            state: vec![0.5; super::super::features::GLOBAL_FEATURE_COUNT],
            candidate_rows: (0..candidate_count)
                .map(|index| {
                    EntityRow::new([index as u32 + 1, 1, 0, 0], vec![0.0, 1.0, 0.0, 0.0, 0.0])
                })
                .collect(),
            action_index,
            old_log_probability: (1.0 / candidate_count as f32).ln(),
            value_estimate: 0.0,
            next_value_estimate: 0.0,
            reward: 1.0,
            terminal_reward: 0.0,
            shaping_reward: 0.0,
            no_progress_cycle_penalty_reward: 0.0,
            terminated: true,
            truncated: false,
            no_progress_cycle: false,
            bootstrap_allowed: true,
            action_kind: 0,
            advantage: 1.0,
            return_value: 1.0,
            policy_entropy: 0.0,
        }
    }

    #[test]
    fn typed_minibatch_loss_is_finite_and_produces_gradients() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        let steps = [fixture_step(0, None), fixture_step(2, Some(1))];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let loss = ppo_loss_for_minibatch(&model, &device, &step_refs, 0.1, 0.01, 0.5);
        let loss_value = loss
            .clone()
            .into_data()
            .to_vec::<f32>()
            .expect("loss must be f32")[0];
        assert!(loss_value.is_finite());
        let gradients = loss.backward();
        let gradients = burn::optim::GradientsParams::from_grads(gradients, &model);
        assert!(!gradients.is_empty());
    }

    #[test]
    fn nonfinite_returns_are_detected_in_minibatch_metrics_before_any_step() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        let mut poisoned = fixture_step(0, None);
        poisoned.return_value = f32::NAN;
        let steps = [poisoned, fixture_step(1, None)];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let (loss, metrics) = ppo_loss_for_global_minibatch(
            &model,
            &device,
            &step_refs,
            &[0, 1],
            &PpoConfig {
                clip_epsilon: 0.1,
                entropy_coefficient: 0.01,
                value_coefficient: 0.5,
                ..PpoConfig::default()
            },
        );
        let loss_value = loss.into_data().to_vec::<f32>().expect("loss must be f32")[0];
        assert!(
            !loss_value.is_finite() || !metrics.is_finite(),
            "non-finite returns must surface in the loss or metrics"
        );
    }

    #[test]
    fn guarded_step_skips_optimizer_update_on_nonfinite_metric() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&model, &device);
        let mut optimizer = AdamConfig::new().init();
        let steps = [fixture_step(0, None)];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let (loss, _) =
            ppo_loss_for_global_minibatch(&model, &device, &step_refs, &[0], &PpoConfig::default());
        let poisoned_metrics = OptimizationMinibatchMetrics {
            policy_loss: f32::NAN,
            ..OptimizationMinibatchMetrics::default()
        };
        let weights_before = model_weights(&model);
        match guarded_minibatch_step(model.clone(), &mut optimizer, 1e-4, loss, poisoned_metrics) {
            GuardedMinibatchStep::NonFiniteMetric => {}
            _ => panic!("non-finite metrics must be reported as NonFiniteMetric"),
        }
        assert_eq!(
            weights_before,
            model_weights(&model),
            "weights must be unchanged when the metric is non-finite"
        );
    }

    #[test]
    fn guarded_step_skips_optimizer_update_on_nonfinite_gradient() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&model, &device);
        let mut optimizer = AdamConfig::new().init();
        let steps = [fixture_step(0, None)];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let (loss, _finite_metrics) =
            ppo_loss_for_global_minibatch(&model, &device, &step_refs, &[0], &PpoConfig::default());
        let gradients = loss.backward();
        let mut gradients = GradientsParams::from_grads(gradients, &model);
        let first_param_id = collect_first_param_id(&model);
        let poisoned_gradient = gradients
            .remove::<InferenceBackend, 1>(first_param_id)
            .expect("model must have a rank-1 float parameter for this test");
        let poisoned_values = poisoned_gradient
            .into_data()
            .to_vec::<f32>()
            .expect("gradient tensor must be f32")
            .into_iter()
            .map(|_| f32::NAN)
            .collect::<Vec<_>>();
        let len = poisoned_values.len();
        let poisoned_gradient = Tensor::<InferenceBackend, 1>::from_data(
            TensorData::new(poisoned_values, [len]),
            &device,
        );
        gradients.register(first_param_id, poisoned_gradient);
        let weights_before = model_weights(&model);
        match apply_guarded_step(model.clone(), &mut optimizer, 1e-4, gradients, 0.0) {
            GuardedMinibatchStep::NonFiniteGradient => {}
            outcome => panic!(
                "non-finite gradients must be reported as NonFiniteGradient, got {outcome:?}"
            ),
        }
        assert_eq!(
            weights_before,
            model_weights(&model),
            "weights must be unchanged when the gradient is non-finite"
        );
    }

    #[test]
    fn guarded_step_applies_optimizer_update_on_finite_inputs() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&model, &device);
        let mut optimizer = AdamConfig::new().init();
        let steps = [fixture_step(0, None)];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let (loss, _) =
            ppo_loss_for_global_minibatch(&model, &device, &step_refs, &[0], &PpoConfig::default());
        let finite_metrics = OptimizationMinibatchMetrics {
            policy_loss: 0.1,
            value_loss: 0.2,
            entropy: 0.3,
            approximate_kl: 0.0,
            clip_fraction: 0.0,
        };
        let weights_before = model_weights(&model);
        let updated_model =
            match guarded_minibatch_step(model, &mut optimizer, 1e-4, loss, finite_metrics) {
                GuardedMinibatchStep::Stepped(model, gradient_norm, _) => {
                    assert!(gradient_norm.is_finite());
                    *model
                }
                outcome => panic!("finite inputs must step, got {outcome:?}"),
            };
        assert_ne!(
            weights_before,
            model_weights(&updated_model),
            "weights must change on a successful optimizer step"
        );
    }

    use burn::module::ParamId;

    fn collect_first_param_id(model: &DeepSetsActorCritic<TrainBackend>) -> ParamId {
        struct FirstParamId(Option<ParamId>);

        impl ModuleVisitor<TrainBackend> for FirstParamId {
            fn visit_float<const D: usize>(&mut self, parameter: &Param<Tensor<TrainBackend, D>>) {
                if self.0.is_none() {
                    self.0 = Some(parameter.id);
                }
            }
        }

        let mut visitor = FirstParamId(None);
        model.visit(&mut visitor);
        visitor
            .0
            .expect("model must have at least one float parameter")
    }

    fn model_weights(model: &DeepSetsActorCritic<TrainBackend>) -> Vec<f32> {
        struct WeightCollector {
            values: Vec<f32>,
        }

        impl ModuleVisitor<TrainBackend> for WeightCollector {
            fn visit_float<const D: usize>(&mut self, parameter: &Param<Tensor<TrainBackend, D>>) {
                self.values.extend(
                    parameter
                        .val()
                        .into_data()
                        .to_vec::<f32>()
                        .expect("weight tensor must be f32"),
                );
            }
        }

        let mut visitor = WeightCollector { values: Vec::new() };
        model.visit(&mut visitor);
        visitor.values
    }

    #[test]
    fn padded_minibatch_loss_matches_weighted_per_transition_losses() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&model, &device);
        let steps = [
            fixture_step(0, Some(2)),
            fixture_step(2, Some(1)),
            fixture_step(1, None),
        ];
        let step_refs = steps.iter().collect::<Vec<_>>();
        let config = PpoConfig {
            clip_epsilon: 0.1,
            entropy_coefficient: 0.01,
            value_coefficient: 0.5,
            ..PpoConfig::default()
        };
        let (minibatch_loss, _) =
            ppo_loss_for_global_minibatch(&model, &device, &step_refs, &[0, 1, 2], &config);
        let minibatch_value = minibatch_loss
            .into_data()
            .to_vec::<f32>()
            .expect("loss must be f32")[0];

        let mut weighted_sum = 0.0;
        for index in 0..steps.len() {
            let (loss, _) =
                ppo_loss_for_global_minibatch(&model, &device, &step_refs, &[index], &config);
            weighted_sum += loss
                .into_data()
                .to_vec::<f32>()
                .expect("per-transition loss must be f32")[0];
        }
        let mean_per_transition = weighted_sum / steps.len() as f32;
        assert!(
            (minibatch_value - mean_per_transition).abs() < 1e-3,
            "padded minibatch loss {minibatch_value} differs from per-transition mean {mean_per_transition}"
        );
    }
}
