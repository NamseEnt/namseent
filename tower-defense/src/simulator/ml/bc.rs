use super::dataset::{ExpertDataset, read_jsonl, validate_dataset};
use super::encoding::{
    EntitySet, PaddedEntityBatch, TypedObservation, candidate_rows_for_legal_actions,
};
use super::features::observation_features;
use super::model::{
    DeepSetsActorCritic, PolicyDevice, TrainBackend, default_policy_device, initialize_model,
    tensor_from_rows,
};
use crate::simulator::environment::ActionKind;
use anyhow::{Context, Result, bail};
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::tensor::Tensor;
use burn::tensor::TensorData;
use burn::tensor::activation::softmax;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

const BC_EVALUATION_BATCH_SIZE: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct BcConfig {
    pub epochs: usize,
    pub learning_rate: f64,
    pub batch_size: usize,
    pub priority_action_kinds: bool,
}

impl Default for BcConfig {
    fn default() -> Self {
        Self {
            epochs: 10,
            learning_rate: 1e-3,
            batch_size: 64,
            priority_action_kinds: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BcReport {
    pub epochs: usize,
    pub sample_count: usize,
    pub updates: usize,
    pub initial_nll: f32,
    pub final_nll: f32,
    pub top1_accuracy: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BcEvaluationReport {
    pub sample_count: usize,
    pub nll: f32,
    pub top1_accuracy: f32,
    pub action_kind_buckets: BTreeMap<String, BcActionKindBucket>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BcActionKindBucket {
    pub sample_count: usize,
    pub correct_count: usize,
    pub accuracy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BcSuccessThreshold {
    pub nll_reduction_fraction: f32,
    pub minimum_top1_accuracy: f32,
}

impl Default for BcSuccessThreshold {
    fn default() -> Self {
        Self {
            nll_reduction_fraction: 0.90,
            minimum_top1_accuracy: 0.99,
        }
    }
}

pub fn passes_bc_success_gate(
    initial_nll: f32,
    final_report: &BcEvaluationReport,
    threshold: BcSuccessThreshold,
) -> bool {
    initial_nll.is_finite()
        && initial_nll > 0.0
        && final_report.nll.is_finite()
        && final_report.top1_accuracy.is_finite()
        && final_report.nll <= initial_nll * (1.0 - threshold.nll_reduction_fraction)
        && final_report.top1_accuracy >= threshold.minimum_top1_accuracy
}

pub fn train_bc_from_jsonl(
    path: &Path,
    config: &crate::config::GameConfig,
    model_config: super::model::ModelConfig,
    bc_config: &BcConfig,
) -> Result<(DeepSetsActorCritic<TrainBackend>, BcReport)> {
    let dataset = read_jsonl(path)
        .with_context(|| format!("failed to read expert dataset {}", path.display()))?;
    train_bc(&dataset, config, model_config, bc_config)
}

pub fn train_bc(
    dataset: &ExpertDataset,
    config: &crate::config::GameConfig,
    model_config: super::model::ModelConfig,
    bc_config: &BcConfig,
) -> Result<(DeepSetsActorCritic<TrainBackend>, BcReport)> {
    let device = default_policy_device();
    let model = DeepSetsActorCritic::<TrainBackend>::new(model_config, &device);
    initialize_model(&model, &device);
    train_bc_from_model(model, dataset, config, bc_config)
}

pub fn train_bc_from_model(
    mut model: DeepSetsActorCritic<TrainBackend>,
    dataset: &ExpertDataset,
    config: &crate::config::GameConfig,
    bc_config: &BcConfig,
) -> Result<(DeepSetsActorCritic<TrainBackend>, BcReport)> {
    validate_dataset(dataset, config)?;
    if bc_config.epochs == 0 {
        bail!("BC epochs must be positive");
    }
    if bc_config.learning_rate <= 0.0 || !bc_config.learning_rate.is_finite() {
        bail!("BC learning rate must be positive and finite");
    }
    if bc_config.batch_size == 0 {
        bail!("BC batch size must be positive");
    }
    let samples = dataset
        .episodes
        .iter()
        .flat_map(|episode| episode.steps.iter())
        .collect::<Vec<_>>();
    if samples.is_empty() {
        bail!("expert dataset contains no training steps");
    }
    let device = default_policy_device();
    let mut optimizer = AdamConfig::new().init();
    let initial_nll = mean_nll(&model, &device, &samples)?;
    let mut updates = 0;
    for _ in 0..bc_config.epochs {
        for batch in samples.chunks(bc_config.batch_size) {
            let (loss, _, _) =
                bc_loss_for_batch(&model, &device, batch, bc_config.priority_action_kinds)?;
            let gradients = GradientsParams::from_grads(loss.backward(), &model);
            model = optimizer.step(bc_config.learning_rate, model, gradients);
            updates += 1;
        }
    }
    let final_nll = mean_nll(&model, &device, &samples)?;
    let top1_accuracy = mean_accuracy(&model, &device, &samples)?;
    Ok((
        model,
        BcReport {
            epochs: bc_config.epochs,
            sample_count: samples.len(),
            updates,
            initial_nll,
            final_nll,
            top1_accuracy,
        },
    ))
}

pub fn evaluate_bc(
    model: &DeepSetsActorCritic<TrainBackend>,
    dataset: &ExpertDataset,
    config: &crate::config::GameConfig,
) -> Result<BcEvaluationReport> {
    validate_dataset(dataset, config)?;
    let samples = dataset
        .episodes
        .iter()
        .flat_map(|episode| episode.steps.iter())
        .collect::<Vec<_>>();
    if samples.is_empty() {
        bail!("expert evaluation dataset contains no training steps");
    }
    let device = default_policy_device();
    let mut buckets = BTreeMap::new();
    for kind in [
        ActionKind::PurchaseShopItem,
        ActionKind::StartSelectingTower,
        ActionKind::BeginRerollSelection,
        ActionKind::BeginTowerSelection,
        ActionKind::SelectHandCard,
        ActionKind::DeselectHandCard,
        ActionKind::ConfirmCardSelection,
        ActionKind::CancelCardSelection,
        ActionKind::Reroll,
        ActionKind::SelectTower,
        ActionKind::PlaceTower,
        ActionKind::RemoveTower,
        ActionKind::StartDefense,
        ActionKind::SelectTreasure,
        ActionKind::SelectCardServiceCard,
        ActionKind::ConfirmCardServiceSelection,
        ActionKind::UseInventoryItem,
        ActionKind::Continue,
    ] {
        let kind_samples = samples
            .iter()
            .filter(|step| step.action.kind() == kind)
            .copied()
            .collect::<Vec<_>>();
        let correct_count = kind_samples
            .chunks(BC_EVALUATION_BATCH_SIZE)
            .map(|batch| {
                bc_loss_for_batch(model, &device, batch, false).map(|(_, _, correct)| correct)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum();
        let sample_count = kind_samples.len();
        buckets.insert(
            kind.wire_name().to_string(),
            BcActionKindBucket {
                sample_count,
                correct_count,
                accuracy: correct_count as f32 / sample_count.max(1) as f32,
            },
        );
    }
    Ok(BcEvaluationReport {
        sample_count: samples.len(),
        nll: mean_nll(model, &device, &samples)?,
        top1_accuracy: mean_accuracy(model, &device, &samples)?,
        action_kind_buckets: buckets,
    })
}

fn mean_nll(
    model: &DeepSetsActorCritic<TrainBackend>,
    device: &PolicyDevice,
    samples: &[&crate::simulator::trajectory::TrajectoryStep],
) -> Result<f32> {
    let total = samples
        .chunks(BC_EVALUATION_BATCH_SIZE)
        .map(|batch| bc_loss_for_batch(model, device, batch, false).map(|(_, nll, _)| nll))
        .collect::<Result<Vec<_>>>()?;
    Ok(total.iter().sum::<f32>() / samples.len() as f32)
}

fn mean_accuracy(
    model: &DeepSetsActorCritic<TrainBackend>,
    device: &PolicyDevice,
    samples: &[&crate::simulator::trajectory::TrajectoryStep],
) -> Result<f32> {
    let correct = samples
        .chunks(BC_EVALUATION_BATCH_SIZE)
        .map(|batch| bc_loss_for_batch(model, device, batch, false).map(|(_, _, correct)| correct))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum::<usize>();
    Ok(correct as f32 / samples.len() as f32)
}

fn bc_loss_for_batch(
    model: &DeepSetsActorCritic<TrainBackend>,
    device: &PolicyDevice,
    samples: &[&crate::simulator::trajectory::TrajectoryStep],
    priority_action_kinds: bool,
) -> Result<(Tensor<TrainBackend, 1>, f32, usize)> {
    let mut candidate_sets = Vec::new();
    let mut group_sizes = Vec::with_capacity(samples.len());
    let mut state_rows = Vec::new();
    let mut typed_observations = Vec::with_capacity(samples.len());
    let mut target_indices = Vec::with_capacity(samples.len());
    let mut sample_weights = Vec::with_capacity(samples.len());
    let mut candidate_offset = 0;

    for step in samples {
        let action_index = step
            .legal_actions
            .iter()
            .position(|legal| legal.action == step.action)
            .context("expert action is not present in legal actions")?;
        let candidate_rows =
            candidate_rows_for_legal_actions(&step.pre_observation, &step.legal_actions);
        let group_size = candidate_rows.len();
        group_sizes.push(group_size);
        candidate_sets.extend(
            candidate_rows
                .iter()
                .map(|row| EntitySet::new(vec![row.clone()])),
        );
        state_rows.extend(
            std::iter::repeat_with(|| observation_features(&step.pre_observation)).take(group_size),
        );
        typed_observations.push(TypedObservation::from_observation(&step.pre_observation));
        target_indices.push(action_index);
        sample_weights.push(if priority_action_kinds {
            match step.action.kind() {
                ActionKind::Continue => 0.25,
                ActionKind::SelectHandCard
                | ActionKind::DeselectHandCard
                | ActionKind::ConfirmCardSelection
                | ActionKind::CancelCardSelection => 3.0,
                ActionKind::SelectTreasure
                | ActionKind::SelectCardServiceCard
                | ActionKind::ConfirmCardServiceSelection
                | ActionKind::UseInventoryItem => 5.0,
                _ => 1.0,
            }
        } else {
            1.0
        });
        candidate_offset += group_size;
    }

    let candidates = PaddedEntityBatch::from_sets(&candidate_sets);
    let typed_batches = std::array::from_fn(|index| {
        PaddedEntityBatch::from_sets(
            &typed_observations
                .iter()
                .map(|observation| observation.sets[index].clone())
                .collect::<Vec<_>>(),
        )
    });
    let typed_state = model.encode_typed_sets(&typed_batches, device);
    let typed_state = Tensor::cat(
        group_sizes
            .iter()
            .enumerate()
            .map(|(index, group_size)| {
                typed_state
                    .clone()
                    .slice([index..index + 1, 0..typed_state.dims()[1]])
                    .repeat_dim(0, *group_size)
            })
            .collect::<Vec<_>>(),
        0,
    );
    let logits = model
        .forward_typed_logits_with_candidate_groups(
            tensor_from_rows::<TrainBackend>(&state_rows, device),
            &candidates,
            typed_state,
            &group_sizes,
            device,
        )
        .reshape([candidate_offset, 1]);
    let mut selected_terms = Vec::with_capacity(group_sizes.len());
    let mut group_probabilities = Vec::with_capacity(group_sizes.len());
    let mut offset = 0;
    for ((group_size, target_index), sample_weight) in
        group_sizes.iter().zip(&target_indices).zip(&sample_weights)
    {
        let group_logits = logits
            .clone()
            .slice([offset..offset + group_size, 0..1])
            .reshape([1, *group_size]);
        let probabilities = softmax(group_logits, 1).clamp(1e-7, 1.0);
        let target = Tensor::from_data(
            TensorData::new(
                (0..*group_size)
                    .map(|index| if index == *target_index { 1.0 } else { 0.0 })
                    .collect::<Vec<_>>(),
                [1, *group_size],
            ),
            device,
        );
        selected_terms.push((probabilities.clone().log() * target).sum() * *sample_weight);
        group_probabilities.push(probabilities.into_data().to_vec::<f32>()?);
        offset += group_size;
    }
    let total_weight = sample_weights.iter().sum::<f32>();
    let selected = Tensor::cat(selected_terms, 0).sum() / total_weight;
    let nll = -selected.clone().into_data().to_vec::<f32>()?[0];
    let correct = group_sizes
        .iter()
        .zip(&target_indices)
        .zip(&group_probabilities)
        .filter(|((group_size, target_index), probabilities)| {
            let predicted = (0..**group_size)
                .max_by(|left, right| probabilities[*left].total_cmp(&probabilities[*right]))
                .expect("batch candidate group should not be empty");
            predicted == **target_index
        })
        .count();
    Ok((selected.neg(), nll, correct))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::simulator::ml::contract::{DATASET_SCHEMA_VERSION, MlContract};
    use crate::simulator::ml::dataset::{ExpertDataset, ExpertDatasetMetadata};
    use crate::simulator::ml::seed::SeedRange;

    #[test]
    fn bc_rejects_empty_dataset_before_training() {
        let config = GameConfig::default_config();
        let range = SeedRange::try_new(0, 0).expect("range");
        let dataset = ExpertDataset {
            metadata: ExpertDatasetMetadata {
                dataset_schema_version: DATASET_SCHEMA_VERSION,
                contract: MlContract::from_config(&config),
                expert_policy_id: "test-policy".to_string(),
                expert_policy_version: 1,
                dataset_role: crate::simulator::ml::dataset::DatasetRole::BehaviorSmoke,
                seed_start: range.start,
                seed_end: range.end_inclusive,
                seed_digest: range.digest(),
                max_decisions_per_episode: 1,
                episode_count: 0,
                step_count: 0,
                action_kind_counts: BTreeMap::new(),
                includes_truncated: false,
                git_revision: "test-revision".to_string(),
            },
            episodes: Vec::new(),
        };
        let result = train_bc(&dataset, &config, Default::default(), &BcConfig::default());
        assert!(result.is_err());
    }

    #[test]
    fn bc_success_gate_requires_nll_reduction_and_accuracy() {
        let report = BcEvaluationReport {
            sample_count: 100,
            nll: 0.09,
            top1_accuracy: 0.99,
            action_kind_buckets: BTreeMap::new(),
        };
        assert!(passes_bc_success_gate(
            1.0,
            &report,
            BcSuccessThreshold::default()
        ));
        assert!(!passes_bc_success_gate(
            1.0,
            &BcEvaluationReport {
                nll: 0.2,
                ..report.clone()
            },
            BcSuccessThreshold::default(),
        ));
        assert!(!passes_bc_success_gate(
            1.0,
            &BcEvaluationReport {
                top1_accuracy: 0.98,
                ..report
            },
            BcSuccessThreshold::default(),
        ));
    }

    #[test]
    fn action_kind_buckets_have_stable_zero_entries() {
        let report = BcEvaluationReport {
            sample_count: 2,
            nll: 0.1,
            top1_accuracy: 0.5,
            action_kind_buckets: BTreeMap::from([
                (
                    "continue".to_string(),
                    BcActionKindBucket {
                        sample_count: 2,
                        correct_count: 1,
                        accuracy: 0.5,
                    },
                ),
                (
                    "purchase_shop_item".to_string(),
                    BcActionKindBucket {
                        sample_count: 0,
                        correct_count: 0,
                        accuracy: 0.0,
                    },
                ),
            ]),
        };
        assert_eq!(
            report
                .action_kind_buckets
                .values()
                .map(|bucket| bucket.sample_count)
                .sum::<usize>(),
            report.sample_count
        );
        assert_eq!(
            report
                .action_kind_buckets
                .values()
                .map(|bucket| bucket.correct_count)
                .sum::<usize>(),
            1
        );
        assert_eq!(
            report.action_kind_buckets["purchase_shop_item"].accuracy,
            0.0
        );
    }
}
