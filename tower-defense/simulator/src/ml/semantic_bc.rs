//! Behavior cloning of the Phase 4 semantic policy candidate set onto the
//! existing `DeepSetsActorCritic` actor, plus the search-free inference
//! policy used for terminal evaluation.

use super::bc::MaskedGroups;
use super::encoding::{PaddedEntityBatch, observation::ENTITY_SET_COUNT};
use super::feature_contract::InputContract;
use super::model::{
    DeepSetsActorCritic, InferenceBackend, ModelConfig, PolicyDevice, TrainBackend,
    TypedCandidateScorer, default_policy_device, initialize_model, model_from_full_precision_bytes,
    model_to_full_precision_bytes, tensor_from_rows,
};
use super::phase4_dataset::{DatasetProvenance, DecisionSample, EpisodeRecord};
use super::policy_v2::{
    KindMode, POLICY_REPRESENTATION_VERSION, PolicyNet, factorized_log_probs, module_to_bytes,
    policy_from_bytes,
};
use super::semantic_candidates::{
    CandidateMode, EncodedDecision, POLICY_CANDIDATE_SET_VERSION, PolicyCandidates,
    SEMANTIC_CANDIDATE_ENCODER_VERSION, candidate_batch, encode_decision, policy_candidates_with,
};
use super::spatial::CellSet;
use crate::environment::GameEnvironment;
use anyhow::{Context, Result, bail};
use burn::module::AutodiffModule;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::record::{BinFileRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub const SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION: u32 = 1;
const EVALUATION_BATCH_SIZE: usize = 128;

/// Which action of a recorded decision is the supervised target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelSource {
    /// The action the recorded trajectory actually took.
    Chosen,
    /// The canonical scripted action at that state.
    Canonical,
}

#[derive(Clone, Debug)]
pub struct BcSample {
    pub encoded: EncodedDecision,
    pub target: usize,
    pub canonical_index: usize,
    pub weight: f32,
    pub target_kind: String,
    pub candidate_kinds: Vec<String>,
    pub decision_point: String,
    pub teacher_override: Option<bool>,
    /// For a spatial target: the target option's cells and the target cell.
    pub target_cell: Option<(CellSet, usize)>,
}

pub fn prepare_samples(
    episodes: &[EpisodeRecord],
    label: LabelSource,
    override_weight: f32,
) -> Vec<BcSample> {
    let samples = episodes
        .iter()
        .flat_map(|episode| episode.samples.iter())
        .collect::<Vec<_>>();
    samples
        .par_iter()
        .map(|sample| prepare_sample(sample, label, override_weight))
        .collect()
}

pub fn prepare_sample(
    sample: &DecisionSample,
    label: LabelSource,
    override_weight: f32,
) -> BcSample {
    let target = match label {
        LabelSource::Chosen => sample.chosen_index,
        LabelSource::Canonical => sample.canonical_index,
    };
    let teacher_override = sample.teacher.as_ref().map(|label| label.teacher_override);
    BcSample {
        encoded: encode_decision(
            &sample.observation,
            &sample.policy_candidates(),
            sample.legal_mask.clone(),
        ),
        target,
        canonical_index: sample.canonical_index,
        weight: if teacher_override == Some(true) {
            override_weight
        } else {
            1.0
        },
        target_kind: sample.candidates[target].kind.clone(),
        candidate_kinds: sample
            .candidates
            .iter()
            .map(|candidate| candidate.kind.clone())
            .collect(),
        decision_point: format!("{:?}", sample.decision_point),
        teacher_override,
        target_cell: None,
    }
}

/// Samples of a spatial candidate mode, rebuilt by replaying each recorded
/// game from its seed with the recorded chosen actions (cell legality needs
/// the live environment). Every replayed state must match the recorded state
/// hash.
pub fn prepare_replayed_samples(
    config: std::sync::Arc<crate::config::GameConfig>,
    episodes: &[EpisodeRecord],
    mode: CandidateMode,
) -> Result<Vec<BcSample>> {
    let per_episode = episodes
        .par_iter()
        .map(|episode| replay_episode_samples(std::sync::Arc::clone(&config), episode, mode))
        .collect::<Result<Vec<_>>>()?;
    Ok(per_episode.into_iter().flatten().collect())
}

fn replay_episode_samples(
    config: std::sync::Arc<crate::config::GameConfig>,
    episode: &EpisodeRecord,
    mode: CandidateMode,
) -> Result<Vec<BcSample>> {
    let mut environment = GameEnvironment::new(config, episode.game_seed);
    let mut samples = Vec::with_capacity(episode.samples.len());
    for recorded in &episode.samples {
        if environment.state_hash() != recorded.state_hash {
            bail!(
                "seed {} decision {}: replay diverged from the recorded state",
                episode.game_seed,
                recorded.decision_index
            );
        }
        let decision = semantic_decision_with(&environment, mode)?;
        let action = recorded.candidates[recorded.chosen_index].action.clone();
        let (target, target_cell) = decision
            .candidates
            .locate(&environment, &action)
            .with_context(|| {
                format!(
                    "seed {} decision {}: chosen action {} is not representable",
                    episode.game_seed,
                    recorded.decision_index,
                    action.action_id()
                )
            })?;
        let canonical_index = decision
            .candidates
            .canonical_index()
            .context("canonical action missing from the spatial candidate set")?;
        let candidate_kinds = decision
            .candidates
            .candidates
            .iter()
            .map(|candidate| candidate.action.kind().wire_name().to_string())
            .collect::<Vec<_>>();
        samples.push(BcSample {
            target_kind: candidate_kinds[target].clone(),
            candidate_kinds,
            encoded: decision.encoded,
            target,
            canonical_index,
            weight: 1.0,
            decision_point: format!("{:?}", recorded.decision_point),
            teacher_override: None,
            target_cell,
        });
        super::phase4_dataset::step_to_next_decision(&mut environment, action)?;
    }
    Ok(samples)
}

/// `log P(target cell | target option)` of each spatial sample, `[n, 1]`,
/// with the positions of those samples in `samples`.
pub(crate) fn target_cell_log_probs<B: Backend>(
    model: &PolicyNet<B>,
    samples: &[&BcSample],
    device: &B::Device,
) -> Option<(Tensor<B, 2>, Vec<usize>, Tensor<B, 2>)> {
    let spatial = samples
        .iter()
        .enumerate()
        .filter(|(_, sample)| sample.target_cell.is_some())
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if spatial.is_empty() {
        return None;
    }
    let decisions = spatial
        .iter()
        .map(|index| &samples[*index].encoded)
        .collect::<Vec<_>>();
    let options = spatial
        .iter()
        .map(|index| samples[*index].target)
        .collect::<Vec<_>>();
    let cells = spatial
        .iter()
        .map(|index| &samples[*index].target_cell.as_ref().expect("spatial").0)
        .collect::<Vec<_>>();
    let (log_probs, _) =
        super::policy_v2::cell_log_probs(model, &decisions, &options, &cells, device);
    let target = Tensor::<B, 2, Int>::from_data(
        TensorData::new(
            spatial
                .iter()
                .map(|index| samples[*index].target_cell.as_ref().expect("spatial").1 as i64)
                .collect::<Vec<_>>(),
            [spatial.len(), 1],
        ),
        device,
    );
    Some((log_probs.clone().gather(1, target), spatial, log_probs))
}

/// Forward pass over a batch of decisions: `[groups, width]` masked
/// log-probabilities and the group layout.
pub(crate) fn batch_log_probs<B: Backend>(
    model: &PolicyNet<B>,
    decisions: &[&EncodedDecision],
    device: &B::Device,
) -> (Tensor<B, 2>, MaskedGroups) {
    let factorized = factorized_log_probs(model, decisions, device);
    (factorized.joint, factorized.groups)
}

/// Flat (v1) candidate softmax of a bare `DeepSetsActorCritic`.
pub(crate) fn batch_log_probs_flat<B: Backend>(
    model: &DeepSetsActorCritic<B>,
    decisions: &[&EncodedDecision],
    device: &B::Device,
) -> (Tensor<B, 2>, MaskedGroups) {
    let group_sizes = decisions
        .iter()
        .map(|decision| decision.candidates.len())
        .collect::<Vec<_>>();
    let masks = decisions
        .iter()
        .map(|decision| decision.legal_mask.as_slice())
        .collect::<Vec<_>>();
    let groups = MaskedGroups::new(&group_sizes, Some(&masks));
    let typed_batches: [PaddedEntityBatch; ENTITY_SET_COUNT] = std::array::from_fn(|index| {
        PaddedEntityBatch::from_sets(
            &decisions
                .iter()
                .map(|decision| decision.typed.sets[index].clone())
                .collect::<Vec<_>>(),
        )
    });
    let group_of_row = Tensor::<B, 1, Int>::from_data(
        TensorData::new(
            group_sizes
                .iter()
                .enumerate()
                .flat_map(|(group, size)| std::iter::repeat_n(group as i64, *size))
                .collect::<Vec<_>>(),
            [group_sizes.iter().sum::<usize>()],
        ),
        device,
    );
    let typed_state = model
        .encode_typed_sets(&typed_batches, device)
        .select(0, group_of_row.clone());
    let global = tensor_from_rows::<B>(
        &decisions
            .iter()
            .map(|decision| decision.global_features.clone())
            .collect::<Vec<_>>(),
        device,
    )
    .select(0, group_of_row);
    let logits = model.forward_typed_logits_with_candidate_groups(
        global,
        &candidate_batch(decisions),
        typed_state,
        &group_sizes,
        device,
    );
    (groups.log_probs(logits, device), groups)
}

pub(crate) fn batch_loss(
    model: &PolicyNet<TrainBackend>,
    samples: &[&BcSample],
    cell_label_smoothing: f32,
    device: &PolicyDevice,
) -> Tensor<TrainBackend, 1> {
    let decisions = samples
        .iter()
        .map(|sample| &sample.encoded)
        .collect::<Vec<_>>();
    let (log_probs, groups) = batch_log_probs(model, &decisions, device);
    let targets = groups.target_tensor::<TrainBackend>(
        &samples
            .iter()
            .map(|sample| sample.target)
            .collect::<Vec<_>>(),
        device,
    );
    let weights = samples
        .iter()
        .map(|sample| sample.weight)
        .collect::<Vec<_>>();
    let total_weight = weights.iter().sum::<f32>();
    let weights_tensor = Tensor::<TrainBackend, 2>::from_data(
        TensorData::new(weights.clone(), [samples.len(), 1]),
        device,
    );
    let mut log_likelihood = (log_probs.gather(1, targets) * weights_tensor).sum();
    if let Some((mut cell_log_probs, spatial, all_cells)) =
        target_cell_log_probs(model, samples, device)
    {
        if cell_label_smoothing > 0.0 {
            let width = all_cells.dims()[1];
            let lengths = spatial
                .iter()
                .map(|index| {
                    samples[*index]
                        .target_cell
                        .as_ref()
                        .expect("spatial")
                        .0
                        .len()
                })
                .collect::<Vec<_>>();
            let uniform = Tensor::<TrainBackend, 2>::from_data(
                TensorData::new(
                    lengths
                        .iter()
                        .flat_map(|length| {
                            (0..width).map(move |column| {
                                if column < *length {
                                    1.0 / *length as f32
                                } else {
                                    0.0
                                }
                            })
                        })
                        .collect::<Vec<_>>(),
                    [spatial.len(), width],
                ),
                device,
            );
            let mean_log_prob = (all_cells * uniform).sum_dim(1);
            cell_log_probs = cell_log_probs * (1.0 - cell_label_smoothing)
                + mean_log_prob * cell_label_smoothing;
        }
        let cell_weights = Tensor::<TrainBackend, 2>::from_data(
            TensorData::new(
                spatial
                    .iter()
                    .map(|index| weights[*index])
                    .collect::<Vec<_>>(),
                [spatial.len(), 1],
            ),
            device,
        );
        log_likelihood = log_likelihood + (cell_log_probs * cell_weights).sum();
    }
    -log_likelihood / total_weight
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AccuracyBucket {
    pub count: usize,
    pub top1_correct: usize,
    pub kind_correct: usize,
    pub top1_accuracy: f64,
    pub kind_accuracy: f64,
}

impl AccuracyBucket {
    fn add(&mut self, top1: bool, kind: bool) {
        self.count += 1;
        self.top1_correct += top1 as usize;
        self.kind_correct += kind as usize;
    }

    fn finish(&mut self) {
        self.top1_accuracy = self.top1_correct as f64 / self.count.max(1) as f64;
        self.kind_accuracy = self.kind_correct as f64 / self.count.max(1) as f64;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BcMetrics {
    pub samples: usize,
    pub nll: f64,
    pub weighted_nll: f64,
    pub top1_accuracy: f64,
    pub top3_accuracy: f64,
    pub action_kind_accuracy: f64,
    /// Argmax selections of a masked (illegal or padding) candidate.
    pub illegal_selections: usize,
    /// Candidates in the data that the legal mask excludes.
    pub masked_candidates: usize,
    /// Probability mass the masked softmax assigns to masked candidates.
    pub max_masked_probability: f64,
    pub by_decision_point: BTreeMap<String, AccuracyBucket>,
    pub by_action_kind: BTreeMap<String, AccuracyBucket>,
    /// `target kind -> predicted kind -> count`.
    pub kind_confusion: BTreeMap<String, BTreeMap<String, usize>>,
    /// Teacher-labelled samples only.
    pub override_samples: usize,
    pub override_top1_accuracy: Option<f64>,
    pub override_predicts_canonical: Option<f64>,
    pub agreement_samples: usize,
    pub agreement_top1_accuracy: Option<f64>,
    /// Samples whose target is a spatial option's cell.
    #[serde(default)]
    pub spatial_samples: usize,
    #[serde(default)]
    pub cell_nll: Option<f64>,
    #[serde(default)]
    pub cell_top1_accuracy: Option<f64>,
}

pub fn evaluate_samples<B: Backend>(
    model: &PolicyNet<B>,
    samples: &[BcSample],
    device: &B::Device,
) -> Result<BcMetrics> {
    let mut metrics = BcMetrics {
        samples: samples.len(),
        ..BcMetrics::default()
    };
    if samples.is_empty() {
        return Ok(metrics);
    }
    let mut nll_sum = 0.0f64;
    let mut weighted_sum = 0.0f64;
    let mut weight_total = 0.0f64;
    let mut top1 = 0usize;
    let mut top3 = 0usize;
    let mut kind_correct = 0usize;
    let mut override_correct = 0usize;
    let mut override_canonical = 0usize;
    let mut agreement_correct = 0usize;
    let mut cell_nll_sum = 0.0f64;
    let mut cell_correct = 0usize;
    for chunk in samples.chunks(EVALUATION_BATCH_SIZE) {
        let decisions = chunk
            .iter()
            .map(|sample| &sample.encoded)
            .collect::<Vec<_>>();
        let (log_probs, groups) = batch_log_probs(model, &decisions, device);
        let values = log_probs.into_data().to_vec::<f32>()?;
        let predicted = groups.argmax(&values);
        let width = groups.width();
        let chunk_refs = chunk.iter().collect::<Vec<_>>();
        let mut cell_outcome = vec![None; chunk.len()];
        if let Some((target_cell, spatial, cell_log_probs)) =
            target_cell_log_probs(model, &chunk_refs, device)
        {
            let target_cell = target_cell.into_data().to_vec::<f32>()?;
            let cell_width = cell_log_probs.dims()[1];
            let cell_values = cell_log_probs.into_data().to_vec::<f32>()?;
            for (offset, index) in spatial.iter().enumerate() {
                let row = &cell_values[offset * cell_width..(offset + 1) * cell_width];
                let best = row
                    .iter()
                    .enumerate()
                    .max_by(|left, right| {
                        left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0))
                    })
                    .map(|(cell, _)| cell)
                    .unwrap_or(0);
                let target = chunk[*index].target_cell.as_ref().expect("spatial").1;
                cell_outcome[*index] = Some((target_cell[offset] as f64, best == target));
            }
        }
        for (row, sample) in chunk.iter().enumerate() {
            let row_values = &values[row * width..(row + 1) * width];
            let cell_log_prob = cell_outcome[row].map_or(0.0, |(log_prob, _)| log_prob);
            if let Some((log_prob, correct)) = cell_outcome[row] {
                metrics.spatial_samples += 1;
                cell_nll_sum -= log_prob;
                cell_correct += correct as usize;
            }
            let target_log_prob = row_values[sample.target] as f64 + cell_log_prob;
            nll_sum -= target_log_prob;
            weighted_sum -= target_log_prob * sample.weight as f64;
            weight_total += sample.weight as f64;
            let prediction = predicted[row];
            if !groups.is_valid(row, prediction) {
                metrics.illegal_selections += 1;
            }
            for (column, legal) in sample.encoded.legal_mask.iter().enumerate() {
                if !legal {
                    metrics.masked_candidates += 1;
                    metrics.max_masked_probability = metrics
                        .max_masked_probability
                        .max((row_values[column] as f64).exp());
                }
            }
            let target_value = row_values[sample.target];
            let rank = row_values[..sample.encoded.candidates.len()]
                .iter()
                .enumerate()
                .filter(|(column, value)| {
                    groups.is_valid(row, *column)
                        && (**value > target_value
                            || (**value == target_value && *column < sample.target))
                })
                .count();
            let is_top1 =
                prediction == sample.target && cell_outcome[row].is_none_or(|(_, correct)| correct);
            let is_kind = sample.candidate_kinds[prediction] == sample.target_kind;
            top1 += is_top1 as usize;
            top3 += (rank < 3) as usize;
            kind_correct += is_kind as usize;
            metrics
                .by_decision_point
                .entry(sample.decision_point.clone())
                .or_default()
                .add(is_top1, is_kind);
            metrics
                .by_action_kind
                .entry(sample.target_kind.clone())
                .or_default()
                .add(is_top1, is_kind);
            *metrics
                .kind_confusion
                .entry(sample.target_kind.clone())
                .or_default()
                .entry(sample.candidate_kinds[prediction].clone())
                .or_insert(0) += 1;
            match sample.teacher_override {
                Some(true) => {
                    metrics.override_samples += 1;
                    override_correct += is_top1 as usize;
                    override_canonical += (prediction == sample.canonical_index) as usize;
                }
                Some(false) => {
                    metrics.agreement_samples += 1;
                    agreement_correct += is_top1 as usize;
                }
                None => {}
            }
        }
    }
    if metrics.spatial_samples > 0 {
        metrics.cell_nll = Some(cell_nll_sum / metrics.spatial_samples as f64);
        metrics.cell_top1_accuracy = Some(cell_correct as f64 / metrics.spatial_samples as f64);
    }
    let count = samples.len() as f64;
    metrics.nll = nll_sum / count;
    metrics.weighted_nll = weighted_sum / weight_total.max(f64::MIN_POSITIVE);
    metrics.top1_accuracy = top1 as f64 / count;
    metrics.top3_accuracy = top3 as f64 / count;
    metrics.action_kind_accuracy = kind_correct as f64 / count;
    for bucket in metrics
        .by_decision_point
        .values_mut()
        .chain(metrics.by_action_kind.values_mut())
    {
        bucket.finish();
    }
    if metrics.override_samples > 0 {
        let overrides = metrics.override_samples as f64;
        metrics.override_top1_accuracy = Some(override_correct as f64 / overrides);
        metrics.override_predicts_canonical = Some(override_canonical as f64 / overrides);
    }
    if metrics.agreement_samples > 0 {
        metrics.agreement_top1_accuracy =
            Some(agreement_correct as f64 / metrics.agreement_samples as f64);
    }
    Ok(metrics)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BcTrainConfig {
    pub hidden_size: usize,
    pub learning_rate: f64,
    pub batch_size: usize,
    pub epochs: usize,
    pub seed: u64,
    pub label: LabelSource,
    pub override_weight: f32,
    #[serde(default)]
    pub kind_mode: KindMode,
    #[serde(default)]
    pub input_contract: InputContract,
    #[serde(default)]
    pub candidate_mode: CandidateMode,
    /// Cell target `(1 - e) * one-hot + e * uniform over the option's legal
    /// cells`.
    #[serde(default)]
    pub cell_label_smoothing: f32,
}

impl Default for BcTrainConfig {
    fn default() -> Self {
        Self {
            hidden_size: 64,
            learning_rate: 1e-3,
            batch_size: 64,
            epochs: 20,
            seed: 0,
            label: LabelSource::Chosen,
            override_weight: 1.0,
            kind_mode: KindMode::LogSumExp,
            input_contract: InputContract::Raw,
            candidate_mode: CandidateMode::Top8,
            cell_label_smoothing: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochRecord {
    pub epoch: usize,
    pub mean_train_batch_loss: f64,
    pub validation: Option<BcMetrics>,
    pub seconds: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BcCheckpointMetadata {
    pub schema_version: u32,
    /// `policy_v2::POLICY_REPRESENTATION_VERSION`; absent (1, flat) in
    /// checkpoints written before policy v2.
    #[serde(default = "flat_policy_representation")]
    pub policy_representation_version: u32,
    pub policy_candidate_set_version: u32,
    pub candidate_encoder_version: u32,
    pub git_commit: String,
    pub model_config: ModelConfig,
    pub config: BcTrainConfig,
    pub train_datasets: Vec<String>,
    pub train_provenance: Vec<DatasetProvenance>,
    pub validation_dataset: Option<String>,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub init_checkpoint: Option<String>,
    pub completed_epochs: usize,
    pub best_epoch: Option<usize>,
    pub history: Vec<EpochRecord>,
}

pub struct BcTrainInput<'a> {
    pub train: &'a [BcSample],
    pub validation: &'a [BcSample],
    pub metadata: BcCheckpointMetadata,
    pub init_model: Option<PolicyNet<TrainBackend>>,
}

fn flat_policy_representation() -> u32 {
    1
}

/// Seed of the (unused in `LogSumExp` mode) family head attached when a flat
/// v1 checkpoint is wrapped into a `PolicyNet`.
const WRAPPED_FAMILY_HEAD_SEED: u64 = 0x0f1a_7000;

fn write_model_file<B: Backend>(model: PolicyNet<B>, path: &Path) -> Result<()> {
    let bytes = module_to_bytes(model)?;
    std::fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

/// Loads an actor checkpoint file: a flat v1 `DeepSetsActorCritic`
/// (`representation_version` 1, wrapped with `mode`) or a v2 `PolicyNet`.
pub fn load_policy_file<B: Backend>(
    representation_version: u32,
    config: ModelConfig,
    mode: KindMode,
    path: &Path,
    device: &B::Device,
) -> Result<PolicyNet<B>> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    match representation_version {
        1 => {
            let scorer = model_from_full_precision_bytes::<B>(config, bytes, device)?;
            let mut policy =
                seeded_policy_net::<B>(config, mode, WRAPPED_FAMILY_HEAD_SEED, device)?;
            policy.scorer = scorer;
            Ok(policy)
        }
        2 => {
            let family_only = super::policy_v2::family_only_from_bytes::<B>(config, bytes, device)?;
            Ok(
                seeded_policy_net::<B>(config, mode, WRAPPED_FAMILY_HEAD_SEED, device)?
                    .with_family_only(family_only),
            )
        }
        POLICY_REPRESENTATION_VERSION => policy_from_bytes::<B>(config, mode, bytes, device),
        other => bail!("{}: unknown policy representation {other}", path.display()),
    }
}

pub fn load_model_file<B: Backend>(
    config: ModelConfig,
    path: &Path,
    device: &B::Device,
) -> Result<DeepSetsActorCritic<B>> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    model_from_full_precision_bytes::<B>(config, bytes, device)
}

pub fn load_checkpoint_metadata(run_dir: &Path) -> Result<BcCheckpointMetadata> {
    let metadata: BcCheckpointMetadata = serde_json::from_slice(
        &std::fs::read(run_dir.join("bc.json"))
            .with_context(|| format!("read {}", run_dir.join("bc.json").display()))?,
    )?;
    if metadata.schema_version != SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION
        || metadata.policy_candidate_set_version != POLICY_CANDIDATE_SET_VERSION
        || metadata.candidate_encoder_version != SEMANTIC_CANDIDATE_ENCODER_VERSION
    {
        bail!("{}: incompatible BC checkpoint versions", run_dir.display());
    }
    Ok(metadata)
}

/// The selected (lowest validation NLL, else last) model of a run directory.
pub fn load_selected_model<B: Backend>(
    run_dir: &Path,
    device: &B::Device,
) -> Result<(BcCheckpointMetadata, PolicyNet<B>)> {
    let metadata = load_checkpoint_metadata(run_dir)?;
    let model = load_policy_file(
        metadata.policy_representation_version,
        metadata.model_config,
        metadata.config.kind_mode,
        &run_dir.join("selected-model.bin"),
        device,
    )?
    .with_inputs(metadata.config.input_contract);
    Ok((metadata, model))
}

/// A freshly initialized model whose every parameter is materialized. burn
/// initializes parameters lazily on first use, so without this a parameter
/// can be drawn at a different RNG position when it is first recorded than
/// when it is first used, which breaks save/resume reproducibility.
pub fn new_materialized_model(
    config: ModelConfig,
    device: &PolicyDevice,
) -> Result<DeepSetsActorCritic<TrainBackend>> {
    let model = DeepSetsActorCritic::<TrainBackend>::new(config, device);
    initialize_model(&model, device);
    let bytes = model_to_full_precision_bytes(model)?;
    model_from_full_precision_bytes::<TrainBackend>(config, bytes, device)
}

/// Overwrites every float parameter from a seeded ChaCha stream, in the
/// module's deterministic visit order, with burn's default distributions:
/// Kaiming-uniform `+-1/sqrt(fan_in)` for linear weights and biases, N(0, 1)
/// for the embedding table.
struct SeededInitializer {
    rng: rand_chacha::ChaCha8Rng,
    last_fan_in: usize,
}

impl<B: Backend> burn::module::ModuleMapper<B> for SeededInitializer {
    fn map_float<const D: usize>(
        &mut self,
        param: burn::module::Param<Tensor<B, D>>,
    ) -> burn::module::Param<Tensor<B, D>> {
        use rand::Rng;
        let (id, tensor, mapper) = param.consume();
        let shape = tensor.shape();
        let dims = shape.dims::<D>();
        let device = tensor.device();
        let count = shape.num_elements();
        let embedding = D == 2 && dims[0] == 4096;
        if D == 2 {
            self.last_fan_in = dims[0];
        }
        let bound = 1.0 / (self.last_fan_in.max(1) as f64).sqrt();
        let values = (0..count)
            .map(|_| {
                if embedding {
                    let u1: f64 = self.rng.r#gen::<f64>().max(f64::MIN_POSITIVE);
                    let u2: f64 = self.rng.r#gen();
                    ((-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()) as f32
                } else {
                    self.rng.gen_range(-bound..bound) as f32
                }
            })
            .collect::<Vec<_>>();
        let value = Tensor::<B, D>::from_data(TensorData::new(values, shape), &device);
        burn::module::Param::from_mapped_value(id, value, mapper)
    }
}

/// A fully materialized model whose parameters depend only on `seed`, not
/// on the process-global backend RNG (which concurrent code may advance).
pub fn seeded_materialized_model(
    config: ModelConfig,
    seed: u64,
    device: &PolicyDevice,
) -> Result<DeepSetsActorCritic<TrainBackend>> {
    use burn::module::Module;
    let model =
        DeepSetsActorCritic::<InferenceBackend>::new(config, device).map(&mut SeededInitializer {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            last_fan_in: 1,
        });
    let bytes = model_to_full_precision_bytes(model)?;
    model_from_full_precision_bytes::<TrainBackend>(config, bytes, device)
}

/// A `PolicyNet` whose parameters depend only on `seed`.
pub fn seeded_policy_net<B: Backend>(
    config: ModelConfig,
    mode: KindMode,
    seed: u64,
    device: &B::Device,
) -> Result<PolicyNet<B>> {
    use burn::module::Module;
    let policy = PolicyNet::<B>::new(config, mode, device).map(&mut SeededInitializer {
        rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
        last_fan_in: 1,
    });
    policy_from_bytes::<B>(config, mode, module_to_bytes(policy)?, device)
}

fn epoch_dir(run_dir: &Path, epoch: usize) -> PathBuf {
    run_dir.join(format!("epoch-{epoch:03}"))
}

/// Trains (or resumes) a BC run in `run_dir`. Every completed epoch saves the
/// model, the Adam state and the metadata; a rerun continues after the last
/// completed epoch with the same deterministic data order.
pub fn train_bc_run(run_dir: &Path, input: BcTrainInput<'_>) -> Result<BcCheckpointMetadata> {
    let config = input.metadata.config.clone();
    if config.epochs == 0
        || config.batch_size == 0
        || config.learning_rate.is_nan()
        || config.learning_rate <= 0.0
    {
        bail!("invalid BC training config");
    }
    if input.train.is_empty() {
        bail!("BC training set is empty");
    }
    if input.metadata.policy_representation_version != POLICY_REPRESENTATION_VERSION {
        bail!("BC training writes policy representation {POLICY_REPRESENTATION_VERSION} only");
    }
    std::fs::create_dir_all(run_dir)?;
    let device = default_policy_device();
    let model_config = input.metadata.model_config;
    let kind_mode = config.kind_mode;
    let mut optimizer = AdamConfig::new().init::<TrainBackend, PolicyNet<TrainBackend>>();
    let (mut metadata, mut model) = if run_dir.join("bc.json").exists() {
        let metadata = load_checkpoint_metadata(run_dir)?;
        if metadata.config != config
            || metadata.train_datasets != input.metadata.train_datasets
            || metadata.validation_dataset != input.metadata.validation_dataset
            || metadata.train_samples != input.metadata.train_samples
            || metadata.init_checkpoint != input.metadata.init_checkpoint
        {
            bail!(
                "{}: existing run has a different configuration",
                run_dir.display()
            );
        }
        let directory = epoch_dir(run_dir, metadata.completed_epochs);
        let model = load_policy_file::<TrainBackend>(
            POLICY_REPRESENTATION_VERSION,
            model_config,
            kind_mode,
            &directory.join("model.bin"),
            &device,
        )?
        .with_inputs(config.input_contract);
        let record = BinFileRecorder::<FullPrecisionSettings>::default()
            .load(directory.join("optimizer.bin"), &device)
            .context("load optimizer state")?;
        optimizer = optimizer.load_record(record);
        eprintln!(
            "resuming {} after epoch {}",
            run_dir.display(),
            metadata.completed_epochs
        );
        (metadata, model)
    } else {
        let model = match input.init_model {
            Some(model) => model.with_mode(kind_mode),
            None => seeded_policy_net::<TrainBackend>(
                model_config,
                kind_mode,
                input.metadata.config.seed,
                &device,
            )?,
        }
        .with_inputs(config.input_contract);
        let metadata = input.metadata;
        let directory = epoch_dir(run_dir, 0);
        std::fs::create_dir_all(&directory)?;
        write_model_file(model.clone().valid(), &directory.join("model.bin"))?;
        BinFileRecorder::<FullPrecisionSettings>::default()
            .record(optimizer.to_record(), directory.join("optimizer.bin"))?;
        (metadata, model)
    };
    let train = input.train.iter().collect::<Vec<_>>();
    for epoch in metadata.completed_epochs + 1..=config.epochs {
        let started = Instant::now();
        let mut order = (0..train.len()).collect::<Vec<_>>();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(
            config.seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ epoch as u64,
        );
        order.shuffle(&mut rng);
        let mut loss_sum = 0.0f64;
        let mut batches = 0usize;
        for chunk in order.chunks(config.batch_size) {
            let batch = chunk.iter().map(|index| train[*index]).collect::<Vec<_>>();
            let loss = batch_loss(&model, &batch, config.cell_label_smoothing, &device);
            loss_sum += loss.clone().into_data().to_vec::<f32>()?[0] as f64;
            batches += 1;
            let gradients = GradientsParams::from_grads(loss.backward(), &model);
            model = optimizer.step(config.learning_rate, model, gradients);
        }
        let inference = model.clone().valid();
        let validation = if input.validation.is_empty() {
            None
        } else {
            Some(evaluate_samples(&inference, input.validation, &device)?)
        };
        let record = EpochRecord {
            epoch,
            mean_train_batch_loss: loss_sum / batches.max(1) as f64,
            validation,
            seconds: started.elapsed().as_secs_f64(),
        };
        eprintln!(
            "epoch {epoch}: train loss {:.4}{} ({:.1}s)",
            record.mean_train_batch_loss,
            record
                .validation
                .as_ref()
                .map_or(String::new(), |metrics| format!(
                    ", validation nll {:.4} top1 {:.4} kind {:.4}",
                    metrics.nll, metrics.top1_accuracy, metrics.action_kind_accuracy
                )),
            record.seconds
        );
        let directory = epoch_dir(run_dir, epoch);
        std::fs::create_dir_all(&directory)?;
        write_model_file(inference, &directory.join("model.bin"))?;
        BinFileRecorder::<FullPrecisionSettings>::default()
            .record(optimizer.to_record(), directory.join("optimizer.bin"))?;
        metadata.history.push(record);
        metadata.completed_epochs = epoch;
        metadata.best_epoch = select_epoch(&metadata.history);
        let temporary = run_dir.join("bc.json.tmp");
        std::fs::write(&temporary, serde_json::to_vec_pretty(&metadata)?)?;
        std::fs::rename(&temporary, run_dir.join("bc.json"))?;
    }
    let selected = metadata.best_epoch.unwrap_or(metadata.completed_epochs);
    std::fs::copy(
        epoch_dir(run_dir, selected).join("model.bin"),
        run_dir.join("selected-model.bin"),
    )?;
    Ok(metadata)
}

/// Lowest validation NLL; without validation, the last epoch.
fn select_epoch(history: &[EpochRecord]) -> Option<usize> {
    let with_validation = history
        .iter()
        .filter_map(|record| {
            record
                .validation
                .as_ref()
                .map(|metrics| (record.epoch, metrics.nll))
        })
        .min_by(|left, right| {
            left.1
                .total_cmp(&right.1)
                .then_with(|| left.0.cmp(&right.0))
        });
    with_validation
        .map(|(epoch, _)| epoch)
        .or_else(|| history.last().map(|record| record.epoch))
}

/// Search-free learned policy: candidate generation, encoding and one
/// network forward per decision.
#[derive(Clone)]
pub struct SemanticPolicy {
    model: PolicyNet<InferenceBackend>,
    device: PolicyDevice,
    candidate_mode: CandidateMode,
    build_option_head: Option<TypedCandidateScorer<InferenceBackend>>,
    /// Checkpoint zero uses A1' argmax for the BuildTower option while its
    /// sampled distribution explores the dense option set. After the first
    /// PPO update, greedy selection follows the learned option head.
    build_option_greedy_from_a1: bool,
    greedy_build_option_decisions: Arc<AtomicUsize>,
    greedy_build_option_outside: Arc<AtomicUsize>,
    greedy_build_option_choices: Arc<Mutex<Vec<super::semantic_candidates::BuildOptionInfo>>>,
}

#[derive(Clone, Debug)]
pub struct PolicyChoice {
    pub candidates: PolicyCandidates,
    pub legal_mask: Vec<bool>,
    pub index: usize,
    pub log_probs: Vec<f32>,
    /// For a spatial option: its cells and the chosen cell.
    pub cell: Option<(CellSet, usize)>,
    /// The environment action (the candidate completed with its cell).
    pub action: crate::environment::AgentAction,
    pub forward_seconds: f64,
}

impl SemanticPolicy {
    pub fn new(model: PolicyNet<InferenceBackend>) -> Self {
        Self {
            model,
            device: default_policy_device(),
            candidate_mode: CandidateMode::Top8,
            build_option_head: None,
            build_option_greedy_from_a1: false,
            greedy_build_option_decisions: Arc::new(AtomicUsize::new(0)),
            greedy_build_option_outside: Arc::new(AtomicUsize::new(0)),
            greedy_build_option_choices: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_candidate_mode(mut self, candidate_mode: CandidateMode) -> Self {
        self.candidate_mode = candidate_mode;
        self
    }

    pub fn with_build_option_head(
        mut self,
        build_option_head: TypedCandidateScorer<InferenceBackend>,
    ) -> Self {
        self.build_option_head = Some(build_option_head);
        self
    }

    pub fn with_build_option_greedy_from_a1(mut self, enabled: bool) -> Self {
        self.build_option_greedy_from_a1 = enabled;
        self
    }

    pub fn candidate_mode(&self) -> CandidateMode {
        self.candidate_mode
    }

    pub fn greedy_build_option_usage(&self) -> (usize, usize) {
        (
            self.greedy_build_option_decisions.load(Ordering::Relaxed),
            self.greedy_build_option_outside.load(Ordering::Relaxed),
        )
    }

    pub fn greedy_build_option_choices(&self) -> Vec<super::semantic_candidates::BuildOptionInfo> {
        self.greedy_build_option_choices
            .lock()
            .map(|choices| choices.clone())
            .unwrap_or_default()
    }

    pub fn reset_greedy_build_option_usage(&self) {
        self.greedy_build_option_decisions
            .store(0, Ordering::Relaxed);
        self.greedy_build_option_outside.store(0, Ordering::Relaxed);
        if let Ok(mut choices) = self.greedy_build_option_choices.lock() {
            choices.clear();
        }
    }

    /// A BC run directory (`bc.json`) or a PPO iteration directory
    /// (`ppo-actor.json`).
    pub fn from_path(path: &Path) -> Result<Self> {
        if path.join("ppo-actor.json").exists() {
            let device = default_policy_device();
            let (model, candidate_mode) = super::semantic_ppo::load_ppo_policy(path, &device)?;
            let mut policy = Self {
                model,
                device,
                candidate_mode,
                build_option_head: None,
                build_option_greedy_from_a1: path
                    .file_name()
                    .is_some_and(|name| name == "iter-0000"),
                greedy_build_option_decisions: Arc::new(AtomicUsize::new(0)),
                greedy_build_option_outside: Arc::new(AtomicUsize::new(0)),
                greedy_build_option_choices: Arc::new(Mutex::new(Vec::new())),
            };
            if candidate_mode == CandidateMode::BuildOptionA1Marginal {
                policy.build_option_head = Some(super::semantic_ppo::load_build_option_head(
                    path,
                    &policy.device,
                )?);
            }
            return Ok(policy);
        }
        Self::from_run_dir(path)
    }

    pub fn from_run_dir(run_dir: &Path) -> Result<Self> {
        let device = default_policy_device();
        let (metadata, model) = load_selected_model::<InferenceBackend>(run_dir, &device)?;
        Ok(Self {
            model,
            device,
            candidate_mode: metadata.config.candidate_mode,
            build_option_head: None,
            build_option_greedy_from_a1: false,
            greedy_build_option_decisions: Arc::new(AtomicUsize::new(0)),
            greedy_build_option_outside: Arc::new(AtomicUsize::new(0)),
            greedy_build_option_choices: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn model(&self) -> &PolicyNet<InferenceBackend> {
        &self.model
    }

    pub fn log_probs(&self, decision: &EncodedDecision) -> Result<Vec<f32>> {
        if self.candidate_mode == CandidateMode::BuildOptionA1Marginal
            && decision.projection.is_some()
        {
            let head = self
                .build_option_head
                .as_ref()
                .context("option-only policy is missing its BuildTower option head")?;
            return super::policy_v2::build_option_log_probs(
                &self.model,
                head,
                decision,
                &self.device,
            );
        }
        let (log_probs, _) = batch_log_probs(&self.model, &[decision], &self.device);
        Ok(log_probs.into_data().to_vec::<f32>()?[..decision.candidates.len()].to_vec())
    }

    /// Greedy choice among legal candidates; a masked candidate is never
    /// returned.
    pub fn choose_encoded(&self, decision: &EncodedDecision) -> Result<(usize, Vec<f32>)> {
        let log_probs = self.log_probs(decision)?;
        let index = log_probs
            .iter()
            .enumerate()
            .filter(|(index, _)| decision.legal_mask[*index])
            .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
            .map(|(index, _)| index)
            .context("no legal candidate")?;
        Ok((index, log_probs))
    }

    /// Greedy cell of spatial option `index`.
    pub fn choose_cell(
        &self,
        encoded: &EncodedDecision,
        index: usize,
        cells: &CellSet,
    ) -> Result<(usize, Vec<f32>)> {
        let (log_probs, _) = super::policy_v2::cell_log_probs(
            &self.model,
            &[encoded],
            &[index],
            &[cells],
            &self.device,
        );
        let log_probs = log_probs.into_data().to_vec::<f32>()?[..cells.len()].to_vec();
        let cell = log_probs
            .iter()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
            .map(|(cell, _)| cell)
            .context("option without cells")?;
        Ok((cell, log_probs))
    }

    pub fn choose(&self, environment: &GameEnvironment) -> Result<PolicyChoice> {
        let SemanticDecision {
            candidates,
            legal_mask,
            encoded,
        } = semantic_decision_with(environment, self.candidate_mode)?;
        let started = Instant::now();
        let (index, log_probs) = if self.candidate_mode == CandidateMode::BuildOptionA1Marginal
            && let Some(projection) = &encoded.projection
        {
            let mut source = encoded.clone();
            source.candidates = projection.source_candidates.clone();
            source.legal_mask = projection.source_legal_mask.clone();
            source.families = projection.source_families.clone();
            source.projection = None;
            let (source_index, _) = self.choose_encoded(&source)?;
            let source_kind = projection.source_families[source_index] as usize;
            let build_kind = crate::environment::ActionKind::BuildTower.index();
            let log_probs = self.log_probs(&encoded)?;
            let index = if source_kind == build_kind && self.build_option_greedy_from_a1 {
                projection
                    .output_members
                    .iter()
                    .position(|members| members.contains(&source_index))
                    .context("A1' greedy BuildTower option is missing from dense options")?
            } else if source_kind == build_kind {
                log_probs
                    .iter()
                    .enumerate()
                    .filter(|(candidate, _)| {
                        legal_mask[*candidate]
                            && candidates.candidates[*candidate].action.kind()
                                == crate::environment::ActionKind::BuildTower
                    })
                    .max_by(|left, right| {
                        left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0))
                    })
                    .map(|(candidate, _)| candidate)
                    .context("BuildTower source choice has no dense legal options")?
            } else {
                projection
                    .output_members
                    .iter()
                    .position(|members| members.contains(&source_index))
                    .context("A1' greedy non-Build action is missing from option projection")?
            };
            (index, log_probs)
        } else if self.candidate_mode == CandidateMode::FullPositionA1Marginal
            && let Some(projection) = &encoded.projection
        {
            // Preserve A1's deterministic non-position choice. The PPO
            // distribution is the exact marginal of A1's Top8 probabilities
            // over each option, but argmax(sum(option mass)) need not equal
            // the projection of argmax(single Top8 action). Greedy evaluation
            // freezes that latter choice and delegates only the cell to the
            // full-position conditional head.
            let mut source = encoded.clone();
            source.candidates = projection.source_candidates.clone();
            source.legal_mask = projection.source_legal_mask.clone();
            source.families = projection.source_families.clone();
            source.projection = None;
            let (source_index, _) = self.choose_encoded(&source)?;
            let index = projection
                .output_members
                .iter()
                .position(|members| members.contains(&source_index))
                .context("A1' greedy candidate is missing from its option projection")?;
            (index, self.log_probs(&encoded)?)
        } else {
            self.choose_encoded(&encoded)?
        };
        if self.candidate_mode == CandidateMode::BuildOptionA1Marginal
            && let Some(Some(option)) = candidates.build_option_info.get(index)
        {
            self.greedy_build_option_decisions
                .fetch_add(1, Ordering::Relaxed);
            if option.outside_v1_top8 {
                self.greedy_build_option_outside
                    .fetch_add(1, Ordering::Relaxed);
            }
            if let Ok(mut choices) = self.greedy_build_option_choices.lock() {
                choices.push(option.clone());
            }
        }
        let cell = match candidates.cells(environment, index) {
            Some(cells) => {
                let (cell, _) = self.choose_cell(&encoded, index, &cells)?;
                Some((cells, cell))
            }
            None => None,
        };
        let forward_seconds = started.elapsed().as_secs_f64();
        let action = candidates
            .action(index, cell.as_ref())
            .context("chosen candidate has no action")?;
        Ok(PolicyChoice {
            candidates,
            legal_mask,
            index,
            log_probs,
            cell,
            action,
            forward_seconds,
        })
    }
}

/// The policy input of one decision state: candidate set, legal mask and
/// encoding. The BC evaluator and the PPO actor both build their decisions
/// with this function, so they always see identical candidate rows, mask and
/// action-index semantics.
#[derive(Clone, Debug)]
pub struct SemanticDecision {
    pub candidates: PolicyCandidates,
    pub legal_mask: Vec<bool>,
    pub encoded: EncodedDecision,
}

pub fn semantic_decision(environment: &GameEnvironment) -> Result<SemanticDecision> {
    semantic_decision_with(environment, CandidateMode::Top8)
}

pub fn semantic_decision_with(
    environment: &GameEnvironment,
    mode: CandidateMode,
) -> Result<SemanticDecision> {
    let candidates = policy_candidates_with(environment, mode)?;
    let legal_mask = environment.semantic_actions_are_legal(
        &candidates
            .candidates
            .iter()
            .map(|candidate| &candidate.action)
            .collect::<Vec<_>>(),
    );
    if !legal_mask.iter().any(|legal| *legal) {
        bail!(
            "no legal policy candidate at state {}",
            environment.state_hash()
        );
    }
    let encoded = match &candidates.a1_marginal {
        Some(projection) => {
            let source_legal_mask = environment.semantic_actions_are_legal(
                &projection
                    .source_candidates
                    .iter()
                    .map(|candidate| &candidate.action)
                    .collect::<Vec<_>>(),
            );
            super::semantic_candidates::encode_projected_decision(
                &candidates.observation,
                &candidates.candidates,
                legal_mask.clone(),
                projection,
                source_legal_mask,
            )
        }
        None => encode_decision(
            &candidates.observation,
            &candidates.candidates,
            legal_mask.clone(),
        ),
    };
    Ok(SemanticDecision {
        candidates,
        legal_mask,
        encoded,
    })
}

/// Inverse-CDF sample of a masked categorical distribution given its
/// log-probabilities and a uniform `u` in `[0, 1)`. Masked candidates have
/// probability zero and are never returned.
pub fn sample_masked(log_probs: &[f32], legal_mask: &[bool], u: f64) -> usize {
    let probabilities = log_probs
        .iter()
        .zip(legal_mask)
        .map(|(log_prob, legal)| {
            if *legal {
                (*log_prob as f64).exp()
            } else {
                0.0
            }
        })
        .collect::<Vec<_>>();
    let total = probabilities.iter().sum::<f64>();
    let target = u * total;
    let mut cumulative = 0.0;
    let mut last_legal = None;
    for (index, probability) in probabilities.iter().enumerate() {
        if !legal_mask[index] {
            continue;
        }
        last_legal = Some(index);
        cumulative += probability;
        if target < cumulative {
            return index;
        }
    }
    last_legal.expect("a masked distribution has at least one legal candidate")
}

/// Mean seconds per BC optimizer step (forward, backward, Adam) and per
/// inference forward on `decisions`, for a device comparison.
pub fn benchmark_bc_backend<B: burn::tensor::backend::AutodiffBackend>(
    model: DeepSetsActorCritic<B>,
    samples: &[BcSample],
    batch_size: usize,
    repeats: usize,
    device: &B::Device,
) -> Result<(f64, f64)> {
    let batch = samples.iter().take(batch_size).collect::<Vec<_>>();
    let decisions = batch
        .iter()
        .map(|sample| &sample.encoded)
        .collect::<Vec<_>>();
    let targets = batch.iter().map(|sample| sample.target).collect::<Vec<_>>();
    let mut optimizer = AdamConfig::new().init::<B, DeepSetsActorCritic<B>>();
    let mut model = model;
    let mut train_seconds = 0.0;
    for repeat in 0..repeats + 1 {
        let started = Instant::now();
        let (log_probs, groups) = batch_log_probs_flat(&model, &decisions, device);
        let target = groups.target_tensor::<B>(&targets, device);
        let loss = -log_probs.gather(1, target).mean();
        let _ = loss.clone().into_data().to_vec::<f32>()?;
        let gradients = GradientsParams::from_grads(loss.backward(), &model);
        model = optimizer.step(1e-4, model, gradients);
        let (check, _) = batch_log_probs_flat(&model.clone().valid(), &decisions[..1], device);
        let _ = check.into_data().to_vec::<f32>()?;
        if repeat > 0 {
            train_seconds += started.elapsed().as_secs_f64();
        }
    }
    let inference = model.valid();
    let mut inference_seconds = 0.0;
    for repeat in 0..repeats + 1 {
        let started = Instant::now();
        let (log_probs, _) = batch_log_probs_flat(&inference, &decisions, device);
        let _ = log_probs.into_data().to_vec::<f32>()?;
        if repeat > 0 {
            inference_seconds += started.elapsed().as_secs_f64();
        }
    }
    Ok((
        train_seconds / repeats as f64,
        inference_seconds / repeats as f64,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::ml::phase4_dataset::{
        DatasetProvenance, Phase4Split, SourcePolicy, collect_canonical_episode,
    };
    use burn::module::Module;
    use std::sync::Arc;

    fn canonical_episodes(seeds: &[u64]) -> Vec<EpisodeRecord> {
        let config = Arc::new(GameConfig::default_config());
        let provenance = DatasetProvenance::new(
            &config,
            SourcePolicy::Canonical,
            Phase4Split::CanonicalTrain,
            None,
        )
        .expect("provenance");
        seeds
            .iter()
            .map(|seed| {
                collect_canonical_episode(Arc::clone(&config), provenance.clone(), *seed)
                    .expect("episode")
            })
            .collect()
    }

    fn metadata(config: BcTrainConfig, samples: usize) -> BcCheckpointMetadata {
        BcCheckpointMetadata {
            schema_version: SEMANTIC_BC_CHECKPOINT_SCHEMA_VERSION,
            policy_representation_version: crate::ml::policy_v2::POLICY_REPRESENTATION_VERSION,
            policy_candidate_set_version: POLICY_CANDIDATE_SET_VERSION,
            candidate_encoder_version: SEMANTIC_CANDIDATE_ENCODER_VERSION,
            git_commit: "test".to_string(),
            model_config: ModelConfig {
                hidden_size: config.hidden_size,
            },
            config,
            train_datasets: vec!["tiny".to_string()],
            train_provenance: Vec::new(),
            validation_dataset: None,
            train_samples: samples,
            validation_samples: 0,
            init_checkpoint: None,
            completed_epochs: 0,
            best_epoch: None,
            history: Vec::new(),
        }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        path
    }

    fn tiny_samples() -> Vec<BcSample> {
        let episodes = canonical_episodes(&[0]);
        let mut samples = prepare_samples(&episodes, LabelSource::Chosen, 1.0);
        samples.retain(|sample| sample.encoded.candidates.len() > 1);
        samples.truncate(48);
        samples
    }

    /// Phase 4A BC tiny-overfit gate (distinct from the PPO `toy_overfit`
    /// bandit): a small deterministic canonical dataset must be memorized.
    #[test]
    fn bc_tiny_overfit_memorizes_canonical_decisions() {
        let samples = tiny_samples();
        assert!(samples.len() >= 32);
        let config = BcTrainConfig {
            epochs: 60,
            batch_size: 16,
            learning_rate: 3e-3,
            ..BcTrainConfig::default()
        };
        let run_dir = temp_dir("phase4-bc-tiny");
        let result = train_bc_run(
            &run_dir,
            BcTrainInput {
                train: &samples,
                validation: &[],
                metadata: metadata(config, samples.len()),
                init_model: None,
            },
        )
        .expect("train");
        let device = default_policy_device();
        let (_, model) = load_selected_model::<InferenceBackend>(&run_dir, &device).expect("load");
        let metrics = evaluate_samples(&model, &samples, &device).expect("evaluate");
        let first_loss = result.history.first().unwrap().mean_train_batch_loss;
        let last_loss = result.history.last().unwrap().mean_train_batch_loss;
        eprintln!(
            "tiny overfit: loss {first_loss:.4} -> {last_loss:.4}, top1 {:.4}, nll {:.4}",
            metrics.top1_accuracy, metrics.nll
        );
        assert!(
            metrics.top1_accuracy >= 0.99,
            "top1 {}",
            metrics.top1_accuracy
        );
        assert!(last_loss < first_loss * 0.1);
        assert_eq!(metrics.illegal_selections, 0);
        std::fs::remove_dir_all(run_dir).expect("cleanup");
    }

    #[test]
    fn masked_candidate_is_never_selected_and_has_zero_probability() {
        let samples = tiny_samples();
        let device = default_policy_device();
        let model = PolicyNet::<InferenceBackend>::new(
            ModelConfig::default(),
            KindMode::LogSumExp,
            &device,
        );
        let policy = SemanticPolicy::new(model);
        let mut checked = 0;
        for sample in samples.iter().take(16) {
            let mut decision = sample.encoded.clone();
            let (best, log_probs) = policy.choose_encoded(&decision).expect("choose");
            assert!((log_probs.iter().map(|value| value.exp()).sum::<f32>() - 1.0).abs() < 1e-4);
            decision.legal_mask[best] = false;
            if !decision.legal_mask.iter().any(|legal| *legal) {
                continue;
            }
            let (next, masked_log_probs) = policy.choose_encoded(&decision).expect("choose");
            assert_ne!(next, best);
            assert_eq!(masked_log_probs[best].exp(), 0.0);
            let total = masked_log_probs
                .iter()
                .map(|value| value.exp() as f64)
                .sum::<f64>();
            assert!((total - 1.0).abs() < 1e-4);
            checked += 1;
        }
        assert!(checked > 8);
    }

    #[test]
    fn checkpoint_roundtrip_and_resume_reproduce_outputs() {
        let samples = tiny_samples();
        let config = BcTrainConfig {
            epochs: 2,
            batch_size: 16,
            ..BcTrainConfig::default()
        };
        let full_dir = temp_dir("phase4-bc-full");
        train_bc_run(
            &full_dir,
            BcTrainInput {
                train: &samples,
                validation: &samples[..8],
                metadata: metadata(config.clone(), samples.len()),
                init_model: None,
            },
        )
        .expect("full run");
        let resumed_dir = temp_dir("phase4-bc-resumed");
        train_bc_run(
            &resumed_dir,
            BcTrainInput {
                train: &samples,
                validation: &samples[..8],
                metadata: metadata(
                    BcTrainConfig {
                        epochs: 1,
                        ..config.clone()
                    },
                    samples.len(),
                ),
                init_model: None,
            },
        )
        .expect("first epoch");
        let mut partial = load_checkpoint_metadata(&resumed_dir).expect("metadata");
        partial.config.epochs = 2;
        std::fs::write(
            resumed_dir.join("bc.json"),
            serde_json::to_vec_pretty(&partial).unwrap(),
        )
        .unwrap();
        train_bc_run(
            &resumed_dir,
            BcTrainInput {
                train: &samples,
                validation: &samples[..8],
                metadata: metadata(config, samples.len()),
                init_model: None,
            },
        )
        .expect("resume");
        let device = default_policy_device();
        let full = load_policy_file::<InferenceBackend>(
            POLICY_REPRESENTATION_VERSION,
            ModelConfig::default(),
            KindMode::LogSumExp,
            &epoch_dir(&full_dir, 2).join("model.bin"),
            &device,
        )
        .unwrap();
        let resumed = load_policy_file::<InferenceBackend>(
            POLICY_REPRESENTATION_VERSION,
            ModelConfig::default(),
            KindMode::LogSumExp,
            &epoch_dir(&resumed_dir, 2).join("model.bin"),
            &device,
        )
        .unwrap();
        for sample in samples.iter().take(8) {
            let left = SemanticPolicy::new(full.clone())
                .log_probs(&sample.encoded)
                .unwrap();
            let right = SemanticPolicy::new(resumed.clone())
                .log_probs(&sample.encoded)
                .unwrap();
            // Bit-identical when run alone; under parallel test load the
            // CPU backend's parallel reductions may reorder float sums.
            let max_diff = left
                .iter()
                .zip(&right)
                .map(|(left, right)| (left - right).abs())
                .fold(0.0f32, f32::max);
            assert!(
                max_diff < 1e-4,
                "resumed run diverged from the uninterrupted run by {max_diff}"
            );
        }
        let path = full_dir.join("roundtrip.bin");
        write_model_file(full.clone(), &path).unwrap();
        let reloaded = load_policy_file::<InferenceBackend>(
            POLICY_REPRESENTATION_VERSION,
            ModelConfig::default(),
            KindMode::LogSumExp,
            &path,
            &device,
        )
        .unwrap();
        for sample in samples.iter().take(8) {
            assert_eq!(
                SemanticPolicy::new(full.clone())
                    .log_probs(&sample.encoded)
                    .unwrap(),
                SemanticPolicy::new(reloaded.clone())
                    .log_probs(&sample.encoded)
                    .unwrap()
            );
        }
        std::fs::remove_dir_all(full_dir).unwrap();
        std::fs::remove_dir_all(resumed_dir).unwrap();
    }

    #[test]
    fn inference_is_deterministic_on_a_fixed_state() {
        let config = Arc::new(GameConfig::default_config());
        let environment = GameEnvironment::new(config, 11);
        let device = default_policy_device();
        let model = PolicyNet::<InferenceBackend>::new(
            ModelConfig::default(),
            KindMode::LogSumExp,
            &device,
        );
        let policy = SemanticPolicy::new(model);
        let first = policy.choose(&environment).expect("choose");
        for _ in 0..3 {
            let again = policy.choose(&environment).expect("choose");
            assert_eq!(again.index, first.index);
            assert_eq!(again.log_probs, first.log_probs);
        }
    }

    #[test]
    fn bc_actor_weights_load_into_ppo_model_with_separate_value_head() {
        let device = default_policy_device();
        let trained = new_materialized_model(ModelConfig::default(), &device).unwrap();
        let bytes = model_to_full_precision_bytes(trained.clone().valid()).unwrap();
        let ppo =
            model_from_full_precision_bytes::<TrainBackend>(ModelConfig::default(), bytes, &device)
                .unwrap();
        let samples = tiny_samples();
        let decisions = samples
            .iter()
            .take(4)
            .map(|s| &s.encoded)
            .collect::<Vec<_>>();
        let (left, _) = batch_log_probs_flat(&trained.clone().valid(), &decisions, &device);
        let (right, _) = batch_log_probs_flat(&ppo.clone().valid(), &decisions, &device);
        assert_eq!(
            left.into_data().to_vec::<f32>().unwrap(),
            right.into_data().to_vec::<f32>().unwrap()
        );
        assert_eq!(trained.num_params(), ppo.num_params());
    }

    #[cfg(feature = "simulator-cuda")]
    fn compare_bc_step_with_device<G: burn::tensor::backend::AutodiffBackend>(
        label: &str,
        device: G::Device,
    ) {
        let episodes = canonical_episodes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let samples = prepare_samples(&episodes, LabelSource::Chosen, 1.0);
        let cpu_device = default_policy_device();
        let cpu_model = new_materialized_model(ModelConfig::default(), &cpu_device).unwrap();
        let bytes = model_to_full_precision_bytes(cpu_model.clone().valid()).unwrap();
        let gpu_model =
            model_from_full_precision_bytes::<G>(ModelConfig::default(), bytes, &device).unwrap();
        eprintln!("samples {}", samples.len());
        for batch_size in [1usize, 64, 256, 1024] {
            let batch_size = batch_size.min(samples.len());
            let repeats = if batch_size >= 256 { 5 } else { 20 };
            let (cpu_train, cpu_infer) = benchmark_bc_backend(
                cpu_model.clone(),
                &samples,
                batch_size,
                repeats,
                &cpu_device,
            )
            .unwrap();
            let (gpu_train, gpu_infer) =
                benchmark_bc_backend(gpu_model.clone(), &samples, batch_size, repeats, &device)
                    .unwrap();
            eprintln!(
                "batch {batch_size}: train step cpu {:.2} ms {label} {:.2} ms ({:.2}x) | inference cpu {:.2} ms {label} {:.2} ms ({:.2}x)",
                cpu_train * 1e3,
                gpu_train * 1e3,
                cpu_train / gpu_train,
                cpu_infer * 1e3,
                gpu_infer * 1e3,
                cpu_infer / gpu_infer,
            );
        }
    }

    #[cfg(feature = "simulator-cuda")]
    #[test]
    #[ignore = "device benchmark; run with --release --features simulator-cuda -- --ignored --nocapture"]
    fn cpu_vs_cuda_bc_step_benchmark() {
        compare_bc_step_with_device::<crate::ml::model::CudaTrainBackend>(
            "cuda",
            crate::ml::model::CudaPolicyDevice::new(0),
        );
    }
}
