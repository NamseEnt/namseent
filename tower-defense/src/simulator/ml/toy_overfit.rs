//! Deterministic toy overfit checks proving the PPO objective can learn.
//!
//! These fixtures bypass [`crate::simulator::ml::ppo::train_ppo`] (which needs a
//! live [`GameEnvironment`](crate::config::GameConfig)) and instead drive the
//! shared loss/GAE primitives directly over synthetic [`RolloutStep`] data:
//!
//! 1. A 2-action contextual bandit where the correct action is signalled by the
//!    observation. Within a fixed update budget the policy must reach a target
//!    probability, cut its NLL by a fixed fraction, and fit the value head.
//! 2. A 2-step delayed-reward episode verifying GAE propagates credit to the
//!    first transition, that terminal steps never bootstrap, and that
//!    truncation bootstraps the next-value estimate.

use super::encoding::{EntityRow, EntitySet, PaddedEntityBatch, TypedObservation};
use super::features::GLOBAL_FEATURE_COUNT;
use super::model::{
    DeepSetsActorCritic, InferenceBackend, ModelConfig, PolicyDevice, TrainBackend,
    default_policy_device, inference_model, initialize_model, tensor_from_rows,
};
use super::rollout::{RolloutStep, compute_gae};
use burn::optim::grad_clipping::GradientClippingConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::tensor::activation::softmax;
use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, TensorData};
use std::sync::{Mutex, OnceLock};

/// Hard ceiling on optimizer updates for the bandit overfit gate.
pub const TOY_OVERFIT_UPDATE_BUDGET: usize = 400;

const BANDIT_CANDIDATE_COUNT: usize = 2;
const BANDIT_DATASET_SIZE: usize = 16;
const BANDIT_HIDDEN_SIZE: usize = 32;
const BANDIT_LEARNING_RATE: f64 = 2e-2;
const BANDIT_CLIP_EPSILON: f32 = 0.2;
const BANDIT_ENTROPY_COEFFICIENT: f32 = 0.0;
const BANDIT_VALUE_COEFFICIENT: f32 = 0.5;
const BANDIT_GAMMA: f32 = 0.99;
const BANDIT_GAE_LAMBDA: f32 = 0.95;
#[cfg(test)]
const BANDIT_TARGET_PROBABILITY: f32 = 0.95;
#[cfg(test)]
const BANDIT_NLL_REDUCTION_FRACTION: f32 = 0.8;
#[cfg(test)]
const BANDIT_VALUE_ERROR_TOLERANCE: f32 = 0.05;

/// Outcome of the contextual-bandit overfit run.
#[derive(Clone, Debug)]
pub struct ToyOverfitReport {
    pub updates: usize,
    pub initial_nll: f32,
    pub final_nll: f32,
    pub final_target_probability: f32,
    pub final_value_error: f32,
    pub losses: Vec<f32>,
}

struct BanditEvaluation {
    mean_target_probability: f32,
    mean_nll: f32,
    mean_value_error: f32,
    chosen_log_probabilities: Vec<f32>,
}

fn bandit_step(signal: f32, good_action: usize) -> RolloutStep {
    let mut state = vec![0.0; GLOBAL_FEATURE_COUNT];
    state[good_action] = 1.0;
    RolloutStep {
        typed_observation: TypedObservation::default(),
        legal_candidate_mask: vec![true; BANDIT_CANDIDATE_COUNT],
        state,
        candidate_rows: (0..BANDIT_CANDIDATE_COUNT)
            .map(|index| {
                EntityRow::new(
                    [index as u32 + 1, 1, 0, 0],
                    vec![signal, 1.0, 0.0, 0.0, 0.0],
                )
            })
            .collect(),
        action_index: good_action,
        old_log_probability: (1.0 / BANDIT_CANDIDATE_COUNT as f32).ln(),
        value_estimate: 0.0,
        next_value_estimate: 0.0,
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
        return_value: 1.0,
        policy_entropy: 0.0,
    }
}

/// Balanced dataset: even indices favour action 0 (signal +1), odd indices
/// favour action 1 (signal -1).
fn bandit_dataset() -> Vec<RolloutStep> {
    (0..BANDIT_DATASET_SIZE)
        .map(|index| {
            let good_action = index % BANDIT_CANDIDATE_COUNT;
            let signal = if good_action == 0 { 1.0 } else { -1.0 };
            bandit_step(signal, good_action)
        })
        .collect()
}

fn evaluate_bandit(
    model: &DeepSetsActorCritic<InferenceBackend>,
    device: &PolicyDevice,
    steps: &[RolloutStep],
) -> BanditEvaluation {
    let step_count = steps.len();
    let candidate_count = steps[0].legal_candidate_mask.len();
    let mut typed_sets = std::array::from_fn(|_| Vec::new());
    for step in steps {
        for (set_index, set) in step.typed_observation.sets.iter().enumerate() {
            typed_sets[set_index].push(set.clone());
        }
    }
    let state_batches = typed_sets.map(|sets| PaddedEntityBatch::from_sets(&sets));
    let typed_state = model.encode_typed_sets(&state_batches, device);
    let typed_width = typed_state.dims()[1];

    let mut state_rows = Vec::with_capacity(step_count * candidate_count);
    let mut candidate_sets = Vec::with_capacity(step_count * candidate_count);
    let mut legal_mask_values = Vec::with_capacity(step_count * candidate_count);
    for step in steps {
        for candidate_index in 0..candidate_count {
            state_rows.push(step.state.clone());
            legal_mask_values.push(if step.legal_candidate_mask[candidate_index] {
                0.0
            } else {
                -1.0e9
            });
        }
        for row in &step.candidate_rows {
            candidate_sets.push(EntitySet::new(vec![row.clone()]));
        }
    }
    let typed_expanded = Tensor::cat(
        (0..step_count)
            .flat_map(|index| std::iter::repeat_n(index, candidate_count))
            .map(|index| {
                typed_state
                    .clone()
                    .slice([index..index + 1, 0..typed_width])
            })
            .collect::<Vec<_>>(),
        0,
    );
    let candidates = PaddedEntityBatch::from_sets(&candidate_sets);
    let logits = model
        .forward_typed_logits_with_candidates(
            tensor_from_rows(&state_rows, device),
            &candidates,
            typed_expanded,
            device,
        )
        .reshape([step_count, candidate_count]);
    let legal_mask: Tensor<InferenceBackend, 2> = Tensor::from_data(
        TensorData::new(legal_mask_values, [step_count, candidate_count]),
        device,
    );
    let probabilities = softmax(logits + legal_mask, 1).clamp(1e-7, 1.0);
    let log_probabilities = probabilities.clone().log();
    let probability_values = probabilities
        .into_data()
        .to_vec::<f32>()
        .expect("bandit probabilities must be f32");
    let log_probability_values = log_probabilities
        .into_data()
        .to_vec::<f32>()
        .expect("bandit log probabilities must be f32");
    let value_values = model
        .forward_typed_values(typed_state)
        .into_data()
        .to_vec::<f32>()
        .expect("bandit values must be f32");

    let mut probability_sum = 0.0_f32;
    let mut nll_sum = 0.0_f32;
    for (row, step) in steps.iter().enumerate() {
        let chosen = step.action_index;
        let offset = row * candidate_count + chosen;
        probability_sum += probability_values[offset];
        nll_sum += -log_probability_values[offset];
    }
    let mut value_error_sum = 0.0_f32;
    for (row, step) in steps.iter().enumerate() {
        value_error_sum += (value_values[row] - step.return_value).abs();
    }

    BanditEvaluation {
        mean_target_probability: probability_sum / step_count as f32,
        mean_nll: nll_sum / step_count as f32,
        mean_value_error: value_error_sum / step_count as f32,
        chosen_log_probabilities: steps
            .iter()
            .enumerate()
            .map(|(row, step)| log_probability_values[row * candidate_count + step.action_index])
            .collect(),
    }
}

/// Trains the contextual bandit with the production PPO loss primitives and
/// reports learnability metrics. Every iteration refreshes behaviour
/// log-probabilities and value estimates from the current policy (mirroring a
/// fresh rollout) before taking exactly one optimizer update.
pub fn train_contextual_bandit(updates: usize) -> ToyOverfitReport {
    assert!(updates > 0, "bandit overfit requires at least one update");
    static TOY_RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _lock = TOY_RUN_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("toy overfit lock must not be poisoned");
    let device = default_policy_device();
    <InferenceBackend as Backend>::seed(&device, 0);
    <TrainBackend as Backend>::seed(&device, 0);
    let mut model = DeepSetsActorCritic::<TrainBackend>::new(
        ModelConfig {
            hidden_size: BANDIT_HIDDEN_SIZE,
        },
        &device,
    );
    initialize_model(&model, &device);
    let mut steps = bandit_dataset();
    let mut optimizer = AdamConfig::new()
        .with_grad_clipping(Some(GradientClippingConfig::Norm(0.5)))
        .init();

    let initial_evaluation = evaluate_bandit(&inference_model(&model), &device, &steps);
    let initial_nll = initial_evaluation.mean_nll;

    let mut losses = Vec::with_capacity(updates);
    for _ in 0..updates {
        let behaviour = evaluate_bandit(&inference_model(&model), &device, &steps);
        for (index, step) in steps.iter_mut().enumerate() {
            step.old_log_probability = behaviour.chosen_log_probabilities[index];
            step.value_estimate = 0.0;
            compute_gae(std::slice::from_mut(step), BANDIT_GAMMA, BANDIT_GAE_LAMBDA);
        }
        let loss = super::ppo::ppo_loss_for_minibatch(
            &model,
            &device,
            &steps.iter().collect::<Vec<_>>(),
            BANDIT_CLIP_EPSILON,
            BANDIT_ENTROPY_COEFFICIENT,
            BANDIT_VALUE_COEFFICIENT,
        );
        let loss_value = loss
            .clone()
            .into_data()
            .to_vec::<f32>()
            .expect("PPO loss must be f32")[0];
        losses.push(loss_value);
        let gradients = loss.backward();
        let gradients = GradientsParams::from_grads(gradients, &model);
        model = optimizer.step(BANDIT_LEARNING_RATE, model, gradients);
    }

    let final_evaluation = evaluate_bandit(&inference_model(&model), &device, &steps);
    ToyOverfitReport {
        updates,
        initial_nll,
        final_nll: final_evaluation.mean_nll,
        final_target_probability: final_evaluation.mean_target_probability,
        final_value_error: final_evaluation.mean_value_error,
        losses,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transition_step(
        reward: f32,
        value_estimate: f32,
        next_value_estimate: f32,
        terminated: bool,
    ) -> RolloutStep {
        RolloutStep {
            typed_observation: TypedObservation::default(),
            legal_candidate_mask: vec![true],
            state: vec![0.0; GLOBAL_FEATURE_COUNT],
            candidate_rows: vec![EntityRow::new([1, 0, 0, 0], vec![0.0])],
            action_index: 0,
            old_log_probability: 0.0,
            value_estimate,
            next_value_estimate,
            reward,
            terminal_reward: if terminated { reward } else { 0.0 },
            shaping_reward: 0.0,
            no_progress_cycle_penalty_reward: 0.0,
            terminated,
            truncated: !terminated,
            no_progress_cycle: false,
            bootstrap_allowed: !terminated,
            action_kind: 0,
            advantage: 0.0,
            return_value: 0.0,
            policy_entropy: 0.0,
        }
    }

    #[test]
    fn contextual_bandit_overfits_within_update_budget() {
        let report = train_contextual_bandit(TOY_OVERFIT_UPDATE_BUDGET);
        assert!(
            report.updates <= TOY_OVERFIT_UPDATE_BUDGET,
            "update budget exceeded: {}",
            report.updates
        );
        assert!(
            report.final_target_probability >= BANDIT_TARGET_PROBABILITY,
            "target probability {:.4} below required {:.4}",
            report.final_target_probability,
            BANDIT_TARGET_PROBABILITY
        );
        let nll_ceiling = report.initial_nll * (1.0 - BANDIT_NLL_REDUCTION_FRACTION);
        assert!(
            report.final_nll <= nll_ceiling,
            "final NLL {:.6} did not drop 80% below initial {:.6}",
            report.final_nll,
            report.initial_nll
        );
        assert!(
            report.final_value_error <= BANDIT_VALUE_ERROR_TOLERANCE,
            "final value error {:.6} above tolerance {:.6}",
            report.final_value_error,
            BANDIT_VALUE_ERROR_TOLERANCE
        );
    }

    #[test]
    fn contextual_bandit_training_is_reproducible() {
        let updates = 40;
        let first = train_contextual_bandit(updates);
        let second = train_contextual_bandit(updates);
        assert_eq!(first.losses.len(), second.losses.len());
        for (index, (left, right)) in first.losses.iter().zip(&second.losses).enumerate() {
            assert!(
                (left - right).abs() < 1e-6,
                "loss curve diverged at update {index}: {left} vs {right}"
            );
        }
        assert!(
            (first.final_target_probability - second.final_target_probability).abs() < 1e-6,
            "final target probability diverged"
        );
    }

    #[test]
    fn two_step_delayed_reward_gives_positive_first_advantage() {
        let mut steps = vec![
            transition_step(0.0, 0.0, 0.0, false),
            transition_step(1.0, 0.0, 0.0, true),
        ];
        compute_gae(&mut steps, BANDIT_GAMMA, BANDIT_GAE_LAMBDA);
        assert!(
            steps[0].advantage > 0.0,
            "delayed credit failed to reach first transition: {}",
            steps[0].advantage
        );
        let expected_first = BANDIT_GAMMA * BANDIT_GAE_LAMBDA;
        assert!((steps[0].advantage - expected_first).abs() < 1e-6);
        assert!((steps[1].advantage - 1.0).abs() < 1e-6);
        assert!((steps[1].return_value - 1.0).abs() < 1e-6);
    }

    #[test]
    fn terminal_steps_do_not_bootstrap_future_value() {
        let mut steps = vec![transition_step(1.0, 0.25, 123.0, true)];
        compute_gae(&mut steps, BANDIT_GAMMA, BANDIT_GAE_LAMBDA);
        assert!((steps[0].advantage - 0.75).abs() < 1e-6);
        assert!((steps[0].return_value - 1.0).abs() < 1e-6);
    }

    #[test]
    fn truncated_steps_bootstrap_next_value() {
        let mut steps = vec![transition_step(0.0, 0.25, 0.5, false)];
        compute_gae(&mut steps, BANDIT_GAMMA, BANDIT_GAE_LAMBDA);
        let expected_advantage = BANDIT_GAMMA * 0.5 - 0.25;
        assert!((steps[0].advantage - expected_advantage).abs() < 1e-6);
        assert!((steps[0].return_value - (expected_advantage + 0.25)).abs() < 1e-6);
    }
}
