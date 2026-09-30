//! Policy v2: family-factorized actor.
//!
//! `P(action) = P(family | state) * P(candidate | family, state)`, where the
//! family is the candidate's `ActionKind`. The candidate scorer is the
//! unchanged `DeepSetsActorCritic` actor path; the conditional distribution
//! is a softmax over the legal candidates of the chosen family.
//!
//! [`KindMode::LogSumExp`] sets each family logit to the log-sum-exp of its
//! candidate logits. Then `P(f) * P(i | f) = softmax(z)_i` exactly, so a v1
//! (flat softmax) checkpoint wrapped in this mode is the same policy.
//! [`KindMode::Learned`] gets the family logits from a separate head over the
//! state, so a family's probability no longer grows with its candidate count.

use super::bc::MaskedGroups;
use super::encoding::{PaddedEntityBatch, observation::ENTITY_SET_COUNT};
use super::feature_contract::{InputContract, apply_contract};
use super::model::{DeepSetsActorCritic, ModelConfig, TypedCandidateScorer, tensor_from_rows};
use super::semantic_candidates::{EncodedDecision, candidate_batch};
use super::spatial::{CELL_FEATURE_COUNT, CellSet};
use anyhow::{Context, Result};
use burn::module::Module;
use burn::nn::{Linear, LinearConfig, Relu};
use burn::record::{FullPrecisionSettings, NamedMpkBytesRecorder, Recorder};
use burn::tensor::activation::log_softmax;
use burn::tensor::backend::Backend;
use burn::tensor::{Bool, Int, Tensor, TensorData};
use serde::{Deserialize, Serialize};

/// 1: flat softmax over candidates (`DeepSetsActorCritic` checkpoint).
/// 2: family-factorized `PolicyNet` without a cell head (stage A).
/// 3: family-factorized `PolicyNet` with the spatial cell head (stage B).
pub const POLICY_REPRESENTATION_VERSION: u32 = 3;
pub const FAMILY_COUNT: usize = crate::environment::ActionKind::COUNT;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum KindMode {
    /// Family logit = log-sum-exp of its candidate logits (flat-equivalent).
    #[default]
    LogSumExp,
    /// Family logits from a separate state head.
    Learned,
}

#[derive(Module, Debug)]
pub struct PolicyNet<B: Backend> {
    pub scorer: DeepSetsActorCritic<B>,
    kind_hidden: Linear<B>,
    kind_output: Linear<B>,
    cell_input: Linear<B>,
    cell_hidden: Linear<B>,
    cell_output: Linear<B>,
    activation: Relu,
    #[module(skip)]
    pub mode: KindMode,
    #[module(skip)]
    pub inputs: InputContract,
}

/// The only actor parameters trained by the position-only PPO ablation.
#[derive(Module, Debug)]
pub struct SpatialCellHead<B: Backend> {
    pub cell_input: Linear<B>,
    pub cell_hidden: Linear<B>,
    pub cell_output: Linear<B>,
}

/// Stage-A (`representation_version` 2) layout, kept for loading.
#[derive(Module, Debug)]
pub struct FamilyOnlyPolicyNet<B: Backend> {
    pub scorer: DeepSetsActorCritic<B>,
    kind_hidden: Linear<B>,
    kind_output: Linear<B>,
    activation: Relu,
}

impl<B: Backend> FamilyOnlyPolicyNet<B> {
    pub fn new(config: ModelConfig, device: &B::Device) -> Self {
        let hidden_size = config.hidden_size.max(8);
        Self {
            scorer: DeepSetsActorCritic::new(config, device),
            kind_hidden: LinearConfig::new(hidden_size * 3 + 1 + FAMILY_COUNT, hidden_size)
                .init(device),
            kind_output: LinearConfig::new(hidden_size, 1).init(device),
            activation: Relu::new(),
        }
    }
}

impl<B: Backend> PolicyNet<B> {
    pub fn new(config: ModelConfig, mode: KindMode, device: &B::Device) -> Self {
        Self::from_scorer(
            DeepSetsActorCritic::new(config, device),
            config,
            mode,
            device,
        )
    }

    pub fn from_scorer(
        scorer: DeepSetsActorCritic<B>,
        config: ModelConfig,
        mode: KindMode,
        device: &B::Device,
    ) -> Self {
        let hidden_size = config.hidden_size.max(8);
        Self {
            scorer,
            kind_hidden: LinearConfig::new(hidden_size * 3 + 1 + FAMILY_COUNT, hidden_size)
                .init(device),
            kind_output: LinearConfig::new(hidden_size, 1).init(device),
            cell_input: LinearConfig::new(hidden_size * 3 + CELL_FEATURE_COUNT, hidden_size)
                .init(device),
            cell_hidden: LinearConfig::new(hidden_size, hidden_size).init(device),
            cell_output: LinearConfig::new(hidden_size, 1).init(device),
            activation: Relu::new(),
            mode,
            inputs: InputContract::Raw,
        }
    }

    /// This net with the scorer and family head of a stage-A checkpoint.
    pub fn with_family_only(mut self, family_only: FamilyOnlyPolicyNet<B>) -> Self {
        self.scorer = family_only.scorer;
        self.kind_hidden = family_only.kind_hidden;
        self.kind_output = family_only.kind_output;
        self
    }

    pub fn with_mode(mut self, mode: KindMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_inputs(mut self, inputs: InputContract) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn spatial_cell_head(&self) -> SpatialCellHead<B> {
        SpatialCellHead {
            cell_input: self.cell_input.clone(),
            cell_hidden: self.cell_hidden.clone(),
            cell_output: self.cell_output.clone(),
        }
    }

    pub fn with_spatial_cell_head(mut self, head: SpatialCellHead<B>) -> Self {
        self.cell_input = head.cell_input;
        self.cell_hidden = head.cell_hidden;
        self.cell_output = head.cell_output;
        self
    }
}

pub fn module_to_bytes<B: Backend, M: Module<B>>(module: M) -> Result<Vec<u8>> {
    let recorder = NamedMpkBytesRecorder::<FullPrecisionSettings>::default();
    Ok(recorder.record(module.into_record(), ())?)
}

pub fn family_only_from_bytes<B: Backend>(
    config: ModelConfig,
    bytes: Vec<u8>,
    device: &B::Device,
) -> Result<FamilyOnlyPolicyNet<B>> {
    let recorder = NamedMpkBytesRecorder::<FullPrecisionSettings>::default();
    let record = recorder.load(bytes, device)?;
    Ok(FamilyOnlyPolicyNet::new(config, device).load_record(record))
}

pub fn policy_from_bytes<B: Backend>(
    config: ModelConfig,
    mode: KindMode,
    bytes: Vec<u8>,
    device: &B::Device,
) -> Result<PolicyNet<B>> {
    let recorder = NamedMpkBytesRecorder::<FullPrecisionSettings>::default();
    let record = recorder.load(bytes, device)?;
    Ok(PolicyNet::new(config, mode, device).load_record(record))
}

/// Batched factorized log-probabilities. Padding and masked candidates are
/// -1e9 in `joint`/`conditional`; families without a legal candidate are
/// -1e9 in `kind`.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct FactorizedLogProbs<B: Backend> {
    /// `[groups, width]` `log P(candidate)` = kind + conditional.
    pub joint: Tensor<B, 2>,
    /// `[groups, FAMILY_COUNT]` `log P(family)`.
    pub kind: Tensor<B, 2>,
    /// `[groups, width]` `log P(candidate | its family)`.
    pub conditional: Tensor<B, 2>,
    pub groups: MaskedGroups,
    /// `[groups, FAMILY_COUNT]` whether the family has a legal candidate.
    pub present: Vec<bool>,
    /// `[groups, width, FAMILY_COUNT]` legal-candidate family membership.
    pub membership: Tensor<B, 3>,
    /// `[groups, FAMILY_COUNT]` legal candidates per family.
    pub family_counts: Vec<usize>,
}

pub(crate) fn factorized_log_probs<B: Backend>(
    model: &PolicyNet<B>,
    decisions: &[&EncodedDecision],
    device: &B::Device,
) -> FactorizedLogProbs<B> {
    if decisions
        .iter()
        .any(|decision| decision.projection.is_some())
    {
        let mut sources = Vec::with_capacity(decisions.len());
        let mut output_members = Vec::with_capacity(decisions.len());
        for decision in decisions {
            let mut source = (*decision).clone();
            match source.projection.take() {
                Some(projection) => {
                    source.candidates = projection.source_candidates;
                    source.legal_mask = projection.source_legal_mask;
                    source.families = projection.source_families;
                    output_members.push(projection.output_members);
                }
                None => {
                    output_members.push(
                        (0..source.candidates.len())
                            .map(|index| vec![index])
                            .collect(),
                    );
                }
            }
            sources.push(source);
        }
        let source_refs = sources.iter().collect::<Vec<_>>();
        let source = factorized_log_probs(model, &source_refs, device);
        return project_factorized_log_probs(source, decisions, &output_members, device);
    }
    let contracted;
    let decisions = if model.inputs == InputContract::Raw {
        decisions.to_vec()
    } else {
        contracted = decisions
            .iter()
            .map(|decision| apply_contract(decision, model.inputs))
            .collect::<Vec<_>>();
        contracted.iter().collect::<Vec<_>>()
    };
    let decisions = decisions.as_slice();
    let group_sizes = decisions
        .iter()
        .map(|decision| decision.candidates.len())
        .collect::<Vec<_>>();
    let masks = decisions
        .iter()
        .map(|decision| decision.legal_mask.as_slice())
        .collect::<Vec<_>>();
    let groups = MaskedGroups::new(&group_sizes, Some(&masks));
    let group_count = decisions.len();
    let width = groups.width();
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
    let typed_state = model.scorer.encode_typed_sets(&typed_batches, device);
    let global = tensor_from_rows::<B>(
        &decisions
            .iter()
            .map(|decision| decision.global_features.clone())
            .collect::<Vec<_>>(),
        device,
    );
    let encoded_candidates = model
        .scorer
        .encode_candidates(&candidate_batch(decisions), device);
    let logits = model.scorer.forward_typed_logits_with_encoded_candidates(
        model
            .scorer
            .encode_state(global.clone().select(0, group_of_row.clone())),
        encoded_candidates.clone(),
        typed_state.clone().select(0, group_of_row),
        &group_sizes,
    );
    let (padded, invalid) = groups.padded_logits(logits, device);

    let mut family_index = vec![0i64; group_count * width];
    let mut one_hot = vec![0.0f32; group_count * width * FAMILY_COUNT];
    let mut present = vec![false; group_count * FAMILY_COUNT];
    let mut family_counts = vec![0usize; group_count * FAMILY_COUNT];
    for (group, decision) in decisions.iter().enumerate() {
        for (column, family) in decision.families.iter().enumerate() {
            if !groups.is_valid(group, column) {
                continue;
            }
            let family = *family as usize;
            family_index[group * width + column] = family as i64;
            one_hot[(group * width + column) * FAMILY_COUNT + family] = 1.0;
            present[group * FAMILY_COUNT + family] = true;
            family_counts[group * FAMILY_COUNT + family] += 1;
        }
    }
    let family_index =
        Tensor::<B, 2, Int>::from_data(TensorData::new(family_index, [group_count, width]), device);
    let one_hot = Tensor::<B, 3>::from_data(
        TensorData::new(one_hot, [group_count, width, FAMILY_COUNT]),
        device,
    );
    let absent = Tensor::<B, 2, Bool>::from_data(
        TensorData::new(
            present.iter().map(|present| !present).collect::<Vec<_>>(),
            [group_count, FAMILY_COUNT],
        ),
        device,
    );

    let max = padded.clone().max_dim(1);
    let shifted = (padded.clone() - max.clone()).exp();
    let family_sum = (shifted.unsqueeze_dim::<3>(2) * one_hot.clone())
        .sum_dim(1)
        .reshape([group_count, FAMILY_COUNT]);
    let family_lse = family_sum.clamp_min(1.0e-30).log() + max;
    let conditional = (padded - family_lse.clone().gather(1, family_index.clone()))
        .mask_fill(invalid.clone(), -1.0e9);
    let kind_logits = match model.mode {
        KindMode::LogSumExp => family_lse,
        KindMode::Learned => {
            // Each family is scored from the state and a summary of its own
            // candidates (mean embedding, log count, family identity), with
            // one MLP shared across families.
            let hidden = encoded_candidates.dims()[1];
            let candidates = groups.padded_rows(encoded_candidates, device);
            let membership = one_hot.clone().swap_dims(1, 2);
            let counts = membership.clone().sum_dim(2);
            let mean = membership.matmul(candidates) / counts.clone().clamp_min(1.0);
            let state = Tensor::cat(vec![model.scorer.encode_state(global), typed_state], 1)
                .unsqueeze_dim::<3>(1)
                .repeat_dim(1, FAMILY_COUNT);
            let identity = Tensor::<B, 2>::eye(FAMILY_COUNT, device)
                .unsqueeze_dim::<3>(0)
                .repeat_dim(0, group_count);
            let input = Tensor::cat(
                vec![state, mean, counts.log1p().div_scalar(8.0), identity],
                2,
            )
            .reshape([group_count * FAMILY_COUNT, hidden * 3 + 1 + FAMILY_COUNT]);
            model
                .kind_output
                .forward(model.activation.forward(model.kind_hidden.forward(input)))
                .reshape([group_count, FAMILY_COUNT])
        }
    };
    let kind =
        log_softmax(kind_logits.mask_fill(absent.clone(), -1.0e9), 1).mask_fill(absent, -1.0e9);
    let joint =
        (kind.clone().gather(1, family_index) + conditional.clone()).mask_fill(invalid, -1.0e9);
    FactorizedLogProbs {
        joint,
        kind,
        conditional,
        groups,
        present,
        membership: one_hot,
        family_counts,
    }
}

/// Inference probabilities for the isolated BuildTower option head. The
/// frozen A1' policy supplies the family probability and every non-Build
/// action probability; only the conditional distribution over dense Build
/// options comes from `option_head`.
pub(crate) fn build_option_log_probs<B: Backend>(
    frozen: &PolicyNet<B>,
    option_head: &TypedCandidateScorer<B>,
    decision: &EncodedDecision,
    device: &B::Device,
) -> Result<Vec<f32>> {
    let projection = decision
        .projection
        .as_ref()
        .context("BuildTower option decision is missing its A1' projection")?;
    let mut source = decision.clone();
    source.candidates = projection.source_candidates.clone();
    source.legal_mask = projection.source_legal_mask.clone();
    source.families = projection.source_families.clone();
    source.projection = None;
    let source_factorized = factorized_log_probs(frozen, &[&source], device);
    let source_joint = source_factorized.joint.into_data().to_vec::<f32>()?;
    let kind_log_probs = source_factorized.kind.into_data().to_vec::<f32>()?;

    let build_kind = crate::environment::ActionKind::BuildTower.index();
    let option_indices = decision
        .families
        .iter()
        .enumerate()
        .filter_map(|(index, family)| (*family as usize == build_kind).then_some(index))
        .collect::<Vec<_>>();
    if option_indices.is_empty() {
        let mut result = vec![-1.0e9; decision.candidates.len()];
        for (index, members) in projection.output_members.iter().enumerate() {
            if decision.legal_mask[index] {
                if let Some(source_index) = members.first() {
                    result[index] = source_joint[*source_index];
                }
            }
        }
        return Ok(result);
    }

    let option_log_probs =
        build_option_conditional_log_probs(frozen, option_head, decision, &option_indices, device)?
            .into_data()
            .to_vec::<f32>()?;
    let mut result = vec![-1.0e9; decision.candidates.len()];
    let build_family_log_prob = kind_log_probs[build_kind];
    for (index, members) in projection.output_members.iter().enumerate() {
        if !decision.legal_mask[index] || decision.families[index] as usize == build_kind {
            continue;
        }
        if let Some(source_index) = members.first() {
            result[index] = source_joint[*source_index];
        }
    }
    for (rank, index) in option_indices.into_iter().enumerate() {
        result[index] = build_family_log_prob + option_log_probs[rank];
    }
    Ok(result)
}

/// Differentiable conditional distribution over this decision's dense
/// BuildTower options. All candidate and state encoders belong to `frozen`;
/// only `option_head` receives optimizer updates.
pub(crate) fn build_option_conditional_log_probs<B: Backend>(
    frozen: &PolicyNet<B>,
    option_head: &TypedCandidateScorer<B>,
    decision: &EncodedDecision,
    option_indices: &[usize],
    device: &B::Device,
) -> Result<Tensor<B, 2>> {
    let projection = decision
        .projection
        .as_ref()
        .context("BuildTower option decision is missing its A1' projection")?;
    let mut source = decision.clone();
    source.candidates = projection.source_candidates.clone();
    source.legal_mask = projection.source_legal_mask.clone();
    source.families = projection.source_families.clone();
    source.projection = None;
    let contracted;
    let decision = if frozen.inputs == super::feature_contract::InputContract::Raw {
        decision
    } else {
        contracted = super::feature_contract::apply_contract(decision, frozen.inputs);
        &contracted
    };
    let source_contracted;
    let source = if frozen.inputs == super::feature_contract::InputContract::Raw {
        &source
    } else {
        source_contracted = super::feature_contract::apply_contract(&source, frozen.inputs);
        &source_contracted
    };
    let global = tensor_from_rows::<B>(&[decision.global_features.clone()], device);
    let typed_batches: [PaddedEntityBatch; ENTITY_SET_COUNT] = std::array::from_fn(|set| {
        PaddedEntityBatch::from_sets(&[decision.typed.sets[set].clone()])
    });
    let typed_state = frozen.scorer.encode_typed_sets(&typed_batches, device);
    let state = frozen.scorer.encode_state(global);
    let source_embedded = frozen
        .scorer
        .encode_candidates(&candidate_batch(&[source]), device);
    let source_width = source_embedded.dims()[1];
    let context = source_embedded
        .mean_dim(0)
        .reshape([1, source_width])
        .repeat_dim(0, option_indices.len());
    let options = option_indices
        .iter()
        .map(|index| decision.candidates[*index].clone())
        .collect::<Vec<_>>();
    let option_batch = PaddedEntityBatch::from_sets(&options);
    let option_embeddings = frozen.scorer.encode_candidates(&option_batch, device);
    let joined = Tensor::cat(
        vec![
            state.repeat_dim(0, option_indices.len()),
            option_embeddings,
            typed_state.repeat_dim(0, option_indices.len()),
            context,
        ],
        1,
    );
    Ok(log_softmax(
        option_head
            .forward(joined)
            .reshape([1, option_indices.len()]),
        1,
    ))
}

fn project_factorized_log_probs<B: Backend>(
    source: FactorizedLogProbs<B>,
    decisions: &[&EncodedDecision],
    output_members: &[Vec<Vec<usize>>],
    device: &B::Device,
) -> FactorizedLogProbs<B> {
    let group_sizes = decisions
        .iter()
        .map(|decision| decision.candidates.len())
        .collect::<Vec<_>>();
    let masks = decisions
        .iter()
        .map(|decision| decision.legal_mask.as_slice())
        .collect::<Vec<_>>();
    let groups = MaskedGroups::new(&group_sizes, Some(&masks));
    let group_count = decisions.len();
    let width = groups.width();
    let source_width = source.groups.width();
    let mut projection = vec![0.0f32; group_count * width * source_width];
    let mut family_index = vec![0i64; group_count * width];
    let mut one_hot = vec![0.0f32; group_count * width * FAMILY_COUNT];
    let mut family_counts = vec![0usize; group_count * FAMILY_COUNT];
    let mut present = vec![false; group_count * FAMILY_COUNT];
    let mut invalid_values = Vec::with_capacity(group_count * width);
    for group in 0..group_count {
        for column in 0..width {
            invalid_values.push((!groups.is_valid(group, column)) as u8);
        }
    }
    for (group, decision) in decisions.iter().enumerate() {
        for (column, family) in decision.families.iter().enumerate() {
            if !groups.is_valid(group, column) {
                continue;
            }
            let family = *family as usize;
            family_index[group * width + column] = family as i64;
            one_hot[(group * width + column) * FAMILY_COUNT + family] = 1.0;
            present[group * FAMILY_COUNT + family] = true;
            family_counts[group * FAMILY_COUNT + family] += 1;
            for source_index in &output_members[group][column] {
                assert!(*source_index < source_width);
                projection[(group * width + column) * source_width + source_index] = 1.0;
            }
        }
    }
    let projection = Tensor::<B, 3>::from_data(
        TensorData::new(projection, [group_count, width, source_width]),
        device,
    );
    let invalid = Tensor::<B, 2, Bool>::from_data(
        TensorData::new(invalid_values, [group_count, width]),
        device,
    );
    let joint = (projection * source.joint.exp().unsqueeze_dim::<3>(1))
        .sum_dim(2)
        .reshape([group_count, width])
        .clamp_min(1.0e-30)
        .log()
        .mask_fill(invalid.clone(), -1.0e9);
    let family_index =
        Tensor::<B, 2, Int>::from_data(TensorData::new(family_index, [group_count, width]), device);
    let one_hot = Tensor::<B, 3>::from_data(
        TensorData::new(one_hot, [group_count, width, FAMILY_COUNT]),
        device,
    );
    let conditional =
        (joint.clone() - source.kind.clone().gather(1, family_index)).mask_fill(invalid, -1.0e9);
    FactorizedLogProbs {
        joint,
        kind: source.kind,
        conditional,
        groups,
        present,
        membership: one_hot,
        family_counts,
    }
}

/// `log P(cell | option)` over the legal cells of the chosen option of each
/// decision: `[decisions, max cells]`, padding -1e9, plus the cell counts.
/// `options[i]` is the candidate index of decision `i`'s spatial option and
/// `cells[i]` that option's cell set.
pub(crate) fn cell_log_probs<B: Backend>(
    model: &PolicyNet<B>,
    decisions: &[&EncodedDecision],
    options: &[usize],
    cells: &[&CellSet],
    device: &B::Device,
) -> (Tensor<B, 2>, Vec<usize>) {
    assert_eq!(decisions.len(), options.len());
    assert_eq!(decisions.len(), cells.len());
    let context = cell_contexts(model, decisions, options, device);
    cell_log_probs_from_context(model, &context, cells, device)
}

/// Frozen encoder output used by the spatial conditional head. Position-only
/// PPO caches this on the inference backend once per rollout batch so the
/// shared scorer is neither differentiated nor rerun in every PPO epoch.
pub(crate) fn cell_contexts<B: Backend>(
    model: &PolicyNet<B>,
    decisions: &[&EncodedDecision],
    options: &[usize],
    device: &B::Device,
) -> Tensor<B, 2> {
    assert_eq!(decisions.len(), options.len());
    let contracted = decisions
        .iter()
        .map(|decision| apply_contract(decision, model.inputs))
        .collect::<Vec<_>>();
    let typed_batches: [PaddedEntityBatch; ENTITY_SET_COUNT] = std::array::from_fn(|index| {
        PaddedEntityBatch::from_sets(
            &contracted
                .iter()
                .map(|decision| decision.typed.sets[index].clone())
                .collect::<Vec<_>>(),
        )
    });
    let typed_state = model.scorer.encode_typed_sets(&typed_batches, device);
    let global = tensor_from_rows::<B>(
        &contracted
            .iter()
            .map(|decision| decision.global_features.clone())
            .collect::<Vec<_>>(),
        device,
    );
    let option_sets = contracted
        .iter()
        .zip(options)
        .map(|(decision, option)| decision.candidates[*option].clone())
        .collect::<Vec<_>>();
    let option_embedding = model
        .scorer
        .encode_candidates(&PaddedEntityBatch::from_sets(&option_sets), device);
    let context = Tensor::cat(
        vec![
            model.scorer.encode_state(global),
            typed_state,
            option_embedding,
        ],
        1,
    );
    context
}

/// Conditional cell distribution using a previously computed frozen context.
pub(crate) fn cell_log_probs_from_context<B: Backend>(
    model: &PolicyNet<B>,
    context: &Tensor<B, 2>,
    cells: &[&CellSet],
    device: &B::Device,
) -> (Tensor<B, 2>, Vec<usize>) {
    let count = cells.len();
    let context_width = context.dims()[1];
    let lengths = cells.iter().map(|cells| cells.len()).collect::<Vec<_>>();
    let width = lengths.iter().copied().max().unwrap_or(1).max(1);
    let mut features = vec![0.0f32; count * width * CELL_FEATURE_COUNT];
    let mut invalid = vec![true; count * width];
    for (row, cells) in cells.iter().enumerate() {
        let start = row * width * CELL_FEATURE_COUNT;
        features[start..start + cells.features.len()].copy_from_slice(&cells.features);
        for column in 0..cells.len() {
            invalid[row * width + column] = false;
        }
    }
    let features = Tensor::<B, 3>::from_data(
        TensorData::new(features, [count, width, CELL_FEATURE_COUNT]),
        device,
    );
    let invalid = Tensor::<B, 2, Bool>::from_data(TensorData::new(invalid, [count, width]), device);
    let input = Tensor::cat(
        vec![
            context.clone().unsqueeze_dim::<3>(1).repeat_dim(1, width),
            features,
        ],
        2,
    )
    .reshape([count * width, context_width + CELL_FEATURE_COUNT]);
    let hidden = model.activation.forward(model.cell_input.forward(input));
    let hidden = model.activation.forward(model.cell_hidden.forward(hidden));
    let logits = model
        .cell_output
        .forward(hidden)
        .reshape([count, width])
        .mask_fill(invalid.clone(), -1.0e9);
    (log_softmax(logits, 1).mask_fill(invalid, -1.0e9), lengths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::GameEnvironment;
    use crate::ml::model::{InferenceBackend, default_policy_device};
    use crate::ml::semantic_bc::{
        batch_log_probs_flat, seeded_materialized_model, semantic_decision,
    };
    use crate::teacher::settle_forced_actions;
    use burn::module::AutodiffModule;
    use std::sync::Arc;

    fn decisions(count: usize) -> Vec<EncodedDecision> {
        let config = Arc::new(GameConfig::default_config());
        let mut result = Vec::new();
        for seed in [0u64, 1, 2] {
            let mut environment = GameEnvironment::new(Arc::clone(&config), seed);
            while result.len() < count * (seed as usize + 1)
                && !matches!(
                    environment.decision_point(),
                    crate::environment::DecisionPoint::Terminal
                )
            {
                let decision = semantic_decision(&environment).unwrap();
                let action = decision.candidates.canonical_action.clone();
                result.push(decision.encoded);
                let mut outcome = environment.semantic_step(action).unwrap();
                settle_forced_actions(&mut environment, &mut outcome).unwrap();
                if outcome.terminated {
                    break;
                }
            }
        }
        result
    }

    #[test]
    fn logsumexp_factorization_equals_the_flat_softmax() {
        let device = default_policy_device();
        let scorer = seeded_materialized_model(ModelConfig::default(), 3, &device)
            .unwrap()
            .valid();
        let flat = scorer.clone();
        let policy = PolicyNet::<InferenceBackend>::from_scorer(
            scorer,
            ModelConfig::default(),
            KindMode::LogSumExp,
            &device,
        );
        let decisions = decisions(40);
        let refs = decisions.iter().collect::<Vec<_>>();
        let factorized = factorized_log_probs(&policy, &refs, &device);
        let (flat_log_probs, _) = batch_log_probs_flat(&flat, &refs, &device);
        let width = factorized.groups.width();
        let joint = factorized.joint.into_data().to_vec::<f32>().unwrap();
        let kind = factorized.kind.into_data().to_vec::<f32>().unwrap();
        let conditional = factorized.conditional.into_data().to_vec::<f32>().unwrap();
        let flat_values = flat_log_probs.into_data().to_vec::<f32>().unwrap();
        let mut compared = 0;
        let mut multi_family = 0;
        for (group, decision) in decisions.iter().enumerate() {
            let families = decision
                .families
                .iter()
                .collect::<std::collections::BTreeSet<_>>();
            multi_family += (families.len() > 1) as usize;
            let mut kind_mass = 0.0f64;
            for family in 0..FAMILY_COUNT {
                let value = kind[group * FAMILY_COUNT + family];
                if factorized.present[group * FAMILY_COUNT + family] {
                    kind_mass += (value as f64).exp();
                } else {
                    assert!(value < -1.0e8);
                }
            }
            assert!(
                (kind_mass - 1.0).abs() < 1e-4,
                "family distribution mass {kind_mass}"
            );
            for column in 0..decision.candidates.len() {
                let index = group * width + column;
                let family = decision.families[column] as usize;
                let product = kind[group * FAMILY_COUNT + family] + conditional[index];
                assert!((product - joint[index]).abs() < 1e-5);
                assert!(
                    (joint[index] - flat_values[index]).abs() < 1e-4,
                    "group {group} column {column}: factorized {} vs flat {}",
                    joint[index],
                    flat_values[index]
                );
                compared += 1;
            }
        }
        assert!(compared > 200);
        assert!(
            multi_family > 10,
            "fixture needs decisions with several families"
        );
    }

    #[test]
    fn learned_family_head_gives_a_normalized_joint_distribution() {
        let device = default_policy_device();
        let policy =
            PolicyNet::<InferenceBackend>::new(ModelConfig::default(), KindMode::Learned, &device);
        let decisions = decisions(20);
        let refs = decisions.iter().collect::<Vec<_>>();
        let factorized = factorized_log_probs(&policy, &refs, &device);
        let width = factorized.groups.width();
        let joint = factorized.joint.into_data().to_vec::<f32>().unwrap();
        for (group, decision) in decisions.iter().enumerate() {
            let mass = (0..decision.candidates.len())
                .map(|column| (joint[group * width + column] as f64).exp())
                .sum::<f64>();
            assert!((mass - 1.0).abs() < 1e-4, "group {group} mass {mass}");
            for column in decision.candidates.len()..width {
                assert!(joint[group * width + column] < -1.0e8);
            }
        }
    }

    #[test]
    fn build_option_distribution_preserves_a1_family_and_non_build_mass() {
        let device = default_policy_device();
        let scorer = seeded_materialized_model(ModelConfig::default(), 19, &device)
            .unwrap()
            .valid();
        let policy = PolicyNet::<InferenceBackend>::from_scorer(
            scorer,
            ModelConfig::default(),
            KindMode::Learned,
            &device,
        );
        let head = policy.scorer.typed_candidate_scorer();
        let config = std::sync::Arc::new(GameConfig::default_config());
        let mut environment = GameEnvironment::new(config, 7);
        let decision = loop {
            let result = super::super::semantic_bc::semantic_decision_with(
                &environment,
                super::super::semantic_candidates::CandidateMode::BuildOptionA1Marginal,
            )
            .unwrap();
            if result.encoded.projection.is_some() {
                break result.encoded;
            }
            let action = result.candidates.canonical_action;
            let mut outcome = environment.semantic_step(action).unwrap();
            crate::teacher::settle_forced_actions(&mut environment, &mut outcome).unwrap();
        };
        let option_log_probs = build_option_log_probs(&policy, &head, &decision, &device).unwrap();
        let mass = option_log_probs
            .iter()
            .map(|value| value.exp())
            .sum::<f32>();
        assert!((mass - 1.0).abs() < 1e-4, "joint mass {mass}");
        let source = decision.projection.as_ref().unwrap();
        let mut source_decision = decision.clone();
        source_decision.candidates = source.source_candidates.clone();
        source_decision.legal_mask = source.source_legal_mask.clone();
        source_decision.families = source.source_families.clone();
        source_decision.projection = None;
        let source_probs = factorized_log_probs(&policy, &[&source_decision], &device);
        let source_joint = source_probs.joint.into_data().to_vec::<f32>().unwrap();
        for (index, members) in source.output_members.iter().enumerate() {
            if decision.families[index] as usize
                == crate::environment::ActionKind::BuildTower.index()
                || !decision.legal_mask[index]
            {
                continue;
            }
            assert!((option_log_probs[index] - source_joint[members[0]]).abs() < 1e-5);
        }
        let build_kind = crate::environment::ActionKind::BuildTower.index();
        let build_mass = decision
            .families
            .iter()
            .enumerate()
            .filter(|(index, family)| {
                decision.legal_mask[*index] && **family as usize == build_kind
            })
            .map(|(index, _)| option_log_probs[index].exp())
            .sum::<f32>();
        let source_kind = source_probs.kind.into_data().to_vec::<f32>().unwrap()[build_kind].exp();
        assert!((build_mass - source_kind).abs() < 1e-5);
    }

    #[test]
    fn cached_spatial_context_preserves_distribution_and_decisions() {
        let device = default_policy_device();
        let policy =
            PolicyNet::<InferenceBackend>::new(ModelConfig::default(), KindMode::Learned, &device);
        let decision = decisions(1).remove(0);
        let option = decision
            .families
            .iter()
            .position(|family| {
                *family == crate::environment::ActionKind::BuildTower as u8
                    || *family == crate::environment::ActionKind::PlaceTower as u8
            })
            .expect("fixture needs a spatial action");
        let cells = CellSet {
            positions: (0..17).map(|index| (index as u16, 0)).collect(),
            features: (0..17 * CELL_FEATURE_COUNT)
                .map(|index| (index % 13) as f32 / 13.0)
                .collect(),
        };
        let direct = cell_log_probs(&policy, &[&decision], &[option], &[&cells], &device);
        let context = cell_contexts(&policy, &[&decision], &[option], &device);
        let cached = cell_log_probs_from_context(&policy, &context, &[&cells], &device);
        assert_eq!(direct.1, cached.1);
        let direct = direct.0.into_data().to_vec::<f32>().unwrap();
        let cached = cached.0.into_data().to_vec::<f32>().unwrap();
        for (left, right) in direct.iter().zip(&cached) {
            assert!((left - right).abs() < 1.0e-6, "{left} != {right}");
        }

        let probabilities = direct.iter().map(|value| value.exp()).collect::<Vec<_>>();
        let cached_probabilities = cached.iter().map(|value| value.exp()).collect::<Vec<_>>();
        let entropy = probabilities
            .iter()
            .zip(&direct)
            .map(|(probability, log_probability)| -probability * log_probability)
            .sum::<f32>();
        let cached_entropy = cached_probabilities
            .iter()
            .zip(&cached)
            .map(|(probability, log_probability)| -probability * log_probability)
            .sum::<f32>();
        assert!((entropy - cached_entropy).abs() < 1.0e-6);

        let greedy = |log_probs: &[f32]| {
            log_probs
                .iter()
                .enumerate()
                .max_by(|left, right| left.1.total_cmp(right.1))
                .unwrap()
                .0
        };
        let sample = |log_probs: &[f32], uniform: f32| {
            let mut cumulative = 0.0;
            log_probs
                .iter()
                .enumerate()
                .find_map(|(index, log_probability)| {
                    cumulative += log_probability.exp();
                    (uniform < cumulative).then_some(index)
                })
                .unwrap_or(log_probs.len() - 1)
        };
        assert_eq!(greedy(&direct), greedy(&cached));
        for uniform in [0.0, 0.123_456, 0.5, 0.999_999] {
            assert_eq!(sample(&direct, uniform), sample(&cached, uniform));
        }
    }

    #[test]
    fn projected_policy_probabilities_equal_the_source_mass_sum() {
        let device = default_policy_device();
        let model =
            PolicyNet::<InferenceBackend>::new(ModelConfig::default(), KindMode::Learned, &device);
        let original = decisions(1).remove(0);
        let mut source = original.clone();
        source.candidates.push(original.candidates[0].clone());
        source.legal_mask.push(original.legal_mask[0]);
        source.families.push(original.families[0]);
        let mut projected = original.clone();
        projected.projection = Some(
            super::super::semantic_candidates::EncodedCandidateProjection {
                source_candidates: source.candidates.clone(),
                source_legal_mask: source.legal_mask.clone(),
                source_families: source.families.clone(),
                output_members: (0..original.candidates.len())
                    .map(|index| {
                        if index == 0 {
                            vec![0, original.candidates.len()]
                        } else {
                            vec![index]
                        }
                    })
                    .collect(),
            },
        );
        let source_probs = factorized_log_probs(&model, &[&source], &device);
        let projected_probs = factorized_log_probs(&model, &[&projected], &device);
        let source_values = source_probs.joint.into_data().to_vec::<f32>().unwrap();
        let projected_values = projected_probs.joint.into_data().to_vec::<f32>().unwrap();
        for (output, members) in projected
            .projection
            .as_ref()
            .unwrap()
            .output_members
            .iter()
            .enumerate()
        {
            let expected = members
                .iter()
                .map(|source| source_values[*source].exp())
                .sum::<f32>();
            assert!(
                (projected_values[output].exp() - expected).abs() < 1e-5,
                "output mass {} vs summed source mass {expected}",
                projected_values[output].exp()
            );
        }
    }
}
