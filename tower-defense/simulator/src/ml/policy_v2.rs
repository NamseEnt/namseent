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
use super::model::{DeepSetsActorCritic, ModelConfig, tensor_from_rows};
use super::semantic_candidates::{EncodedDecision, candidate_batch};
use anyhow::Result;
use burn::module::Module;
use burn::nn::{Linear, LinearConfig, Relu};
use burn::record::{FullPrecisionSettings, NamedMpkBytesRecorder, Recorder};
use burn::tensor::activation::log_softmax;
use burn::tensor::backend::Backend;
use burn::tensor::{Bool, Int, Tensor, TensorData};
use serde::{Deserialize, Serialize};

/// 1: flat softmax over candidates (`DeepSetsActorCritic` checkpoint).
/// 2: family-factorized `PolicyNet` checkpoint.
pub const POLICY_REPRESENTATION_VERSION: u32 = 2;
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
    activation: Relu,
    #[module(skip)]
    pub mode: KindMode,
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
            kind_hidden: LinearConfig::new(hidden_size * 2, hidden_size).init(device),
            kind_output: LinearConfig::new(hidden_size, FAMILY_COUNT).init(device),
            activation: Relu::new(),
            mode,
        }
    }

    pub fn with_mode(mut self, mode: KindMode) -> Self {
        self.mode = mode;
        self
    }
}

pub fn module_to_bytes<B: Backend, M: Module<B>>(module: M) -> Result<Vec<u8>> {
    let recorder = NamedMpkBytesRecorder::<FullPrecisionSettings>::default();
    Ok(recorder.record(module.into_record(), ())?)
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
}

pub(crate) fn factorized_log_probs<B: Backend>(
    model: &PolicyNet<B>,
    decisions: &[&EncodedDecision],
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
    let logits = model.scorer.forward_typed_logits_with_candidate_groups(
        global.clone().select(0, group_of_row.clone()),
        &candidate_batch(decisions),
        typed_state.clone().select(0, group_of_row),
        &group_sizes,
        device,
    );
    let (padded, invalid) = groups.padded_logits(logits, device);

    let mut family_index = vec![0i64; group_count * width];
    let mut one_hot = vec![0.0f32; group_count * width * FAMILY_COUNT];
    let mut present = vec![false; group_count * FAMILY_COUNT];
    for (group, decision) in decisions.iter().enumerate() {
        for (column, family) in decision.families.iter().enumerate() {
            if !groups.is_valid(group, column) {
                continue;
            }
            let family = *family as usize;
            family_index[group * width + column] = family as i64;
            one_hot[(group * width + column) * FAMILY_COUNT + family] = 1.0;
            present[group * FAMILY_COUNT + family] = true;
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
    let family_sum = (shifted.unsqueeze_dim::<3>(2) * one_hot)
        .sum_dim(1)
        .reshape([group_count, FAMILY_COUNT]);
    let family_lse = family_sum.clamp_min(1.0e-30).log() + max;
    let conditional = (padded - family_lse.clone().gather(1, family_index.clone()))
        .mask_fill(invalid.clone(), -1.0e9);
    let kind_logits = match model.mode {
        KindMode::LogSumExp => family_lse,
        KindMode::Learned => {
            let state = Tensor::cat(vec![model.scorer.encode_state(global), typed_state], 1);
            model
                .kind_output
                .forward(model.activation.forward(model.kind_hidden.forward(state)))
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
    }
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
}
