use super::encoding::observation::ENTITY_SET_COUNT;
use super::encoding::{
    PaddedEntityBatch,
    tensor::{EntityBatchTensors, to_tensors},
};
use super::features::{ACTION_FEATURE_COUNT, GLOBAL_FEATURE_COUNT};
use anyhow::Result;
use burn::module::{AutodiffModule, Module};
use burn::nn::{Embedding, EmbeddingConfig, Linear, LinearConfig, Relu};
use burn::record::{CompactRecorder, FullPrecisionSettings, NamedMpkBytesRecorder, Recorder};
use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, TensorData};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub type CpuInferenceBackend = burn_flex::Flex;
pub type CpuTrainBackend = burn_autodiff::Autodiff<CpuInferenceBackend>;
pub type CpuPolicyDevice = burn_flex::FlexDevice;

#[cfg(feature = "simulator-wgpu")]
pub type GpuInferenceBackend = burn_wgpu::Wgpu;
#[cfg(feature = "simulator-wgpu")]
pub type GpuTrainBackend = burn_autodiff::Autodiff<GpuInferenceBackend>;
#[cfg(feature = "simulator-wgpu")]
pub type GpuPolicyDevice = burn_wgpu::WgpuDevice;

pub type InferenceBackend = CpuInferenceBackend;
pub type TrainBackend = CpuTrainBackend;
pub type PolicyDevice = CpuPolicyDevice;

pub const ENTITY_ENCODER_SCHEMA_VERSION: u32 = 6;
pub const ENTITY_TYPE_COUNT: usize = ENTITY_SET_COUNT;
pub const ENTITY_PADDING_ID: u32 = 0;

pub fn default_policy_device() -> PolicyDevice {
    Default::default()
}

pub fn policy_backend_description() -> String {
    "burn-flex CPU".to_string()
}

#[cfg(feature = "simulator-wgpu")]
pub fn gpu_policy_backend_description() -> &'static str {
    "burn-wgpu GPU"
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelConfig {
    pub hidden_size: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self { hidden_size: 64 }
    }
}

#[derive(Module, Debug)]
pub struct DeepSetsActorCritic<B: Backend> {
    state_input: Linear<B>,
    state_hidden: Linear<B>,
    action_input: Linear<B>,
    action_hidden: Linear<B>,
    actor_hidden: Linear<B>,
    actor_output: Linear<B>,
    critic_hidden: Linear<B>,
    critic_output: Linear<B>,
    typed_actor_hidden: Linear<B>,
    typed_actor_output: Linear<B>,
    typed_critic_hidden: Linear<B>,
    typed_critic_output: Linear<B>,
    typed_fusion: Linear<B>,
    candidate_fusion: Linear<B>,
    entity_embedding: Embedding<B>,
    entity_input: Linear<B>,
    entity_hidden: Linear<B>,
    entity_output: Linear<B>,
    activation: Relu,
}

impl<B: Backend> DeepSetsActorCritic<B> {
    pub fn new(config: ModelConfig, device: &B::Device) -> Self {
        let hidden_size = config.hidden_size.max(8);
        Self {
            state_input: LinearConfig::new(GLOBAL_FEATURE_COUNT, hidden_size).init(device),
            state_hidden: LinearConfig::new(hidden_size, hidden_size).init(device),
            action_input: LinearConfig::new(ACTION_FEATURE_COUNT, hidden_size).init(device),
            action_hidden: LinearConfig::new(hidden_size, hidden_size).init(device),
            actor_hidden: LinearConfig::new(hidden_size * 2, hidden_size).init(device),
            actor_output: LinearConfig::new(hidden_size, 1).init(device),
            critic_hidden: LinearConfig::new(hidden_size, hidden_size).init(device),
            critic_output: LinearConfig::new(hidden_size, 1).init(device),
            typed_actor_hidden: LinearConfig::new(hidden_size * 4, hidden_size).init(device),
            typed_actor_output: LinearConfig::new(hidden_size, 1).init(device),
            typed_critic_hidden: LinearConfig::new(hidden_size, hidden_size).init(device),
            typed_critic_output: LinearConfig::new(hidden_size, 1).init(device),
            typed_fusion: LinearConfig::new((hidden_size * 2 + 1) * ENTITY_TYPE_COUNT, hidden_size)
                .init(device),
            candidate_fusion: LinearConfig::new(hidden_size * 2 + 1, hidden_size).init(device),
            entity_embedding: EmbeddingConfig::new(4096, 8).init(device),
            entity_input: LinearConfig::new(8 * 4 + 5, hidden_size).init(device),
            entity_hidden: LinearConfig::new(hidden_size, hidden_size).init(device),
            entity_output: LinearConfig::new(hidden_size, hidden_size).init(device),
            activation: Relu::new(),
        }
    }

    pub fn encode_entity_set(
        &self,
        categorical: Tensor<B, 2, burn::tensor::Int>,
        numeric: Tensor<B, 3>,
        mask: Tensor<B, 3>,
    ) -> Tensor<B, 2> {
        let [batch, entities, _] = numeric.dims();
        let entity_width = self.entity_output.weight.shape().dims::<2>()[1];
        let categorical =
            self.entity_embedding
                .forward(categorical)
                .reshape([batch, entities, 8 * 4]);
        let input =
            Tensor::cat(vec![categorical, numeric], 2).reshape([batch * entities, 8 * 4 + 5]);
        let encoded = self
            .entity_output
            .forward(
                self.activation.forward(
                    self.entity_hidden
                        .forward(self.activation.forward(self.entity_input.forward(input))),
                ),
            )
            .reshape([batch, entities, entity_width]);
        let mask = mask;
        let count = mask
            .clone()
            .sum_dim(1)
            .clamp_min(1.0)
            .reshape([batch, 1, 1]);
        let mean =
            ((encoded.clone() * mask.clone()).sum_dim(1) / count).reshape([batch, entity_width]);
        let max = encoded
            .mask_fill(mask.clone().equal_elem(0.0), -1.0e9)
            .max_dim(1)
            .reshape([batch, entity_width])
            .mask_fill(
                mask.clone().sum_dim(1).reshape([batch, 1]).equal_elem(0.0),
                0.0,
            );
        let count_feature = mask.sum_dim(1).reshape([batch, 1]).log1p().div_scalar(8.0);
        Tensor::cat(vec![mean, max, count_feature], 1)
    }

    pub fn forward_logits(&self, state: Tensor<B, 2>, action: Tensor<B, 2>) -> Tensor<B, 2> {
        let state = self.encode_state(state);
        let action = self.encode_action(action);
        let joined = Tensor::cat(vec![state, action], 1);
        self.actor_output
            .forward(self.activation.forward(self.actor_hidden.forward(joined)))
    }

    pub fn forward_values(&self, state: Tensor<B, 2>) -> Tensor<B, 2> {
        let state = self.encode_state(state);
        self.critic_output
            .forward(self.activation.forward(self.critic_hidden.forward(state)))
    }

    pub fn forward_typed_logits(
        &self,
        state: Tensor<B, 2>,
        action: Tensor<B, 2>,
        typed_state: Tensor<B, 2>,
    ) -> Tensor<B, 2> {
        let state = self.encode_state(state);
        let action = self.encode_action(action);
        let candidate_context = action
            .clone()
            .mean_dim(0)
            .reshape([1, action.dims()[1]])
            .repeat_dim(0, action.dims()[0]);
        let typed_state = typed_state.repeat_dim(0, action.dims()[0]);
        let joined = Tensor::cat(vec![state, action, typed_state, candidate_context], 1);
        self.typed_actor_output.forward(
            self.activation
                .forward(self.typed_actor_hidden.forward(joined)),
        )
    }

    pub fn forward_typed_logits_with_candidates(
        &self,
        state: Tensor<B, 2>,
        candidates: &PaddedEntityBatch,
        typed_state: Tensor<B, 2>,
        device: &B::Device,
    ) -> Tensor<B, 2> {
        let rows = candidates.categorical.len() / (candidates.max_entities * 4);
        self.forward_typed_logits_with_candidate_groups(
            state,
            candidates,
            typed_state,
            &[rows],
            device,
        )
    }

    pub fn forward_typed_logits_with_candidate_groups(
        &self,
        state: Tensor<B, 2>,
        candidates: &PaddedEntityBatch,
        typed_state: Tensor<B, 2>,
        group_sizes: &[usize],
        device: &B::Device,
    ) -> Tensor<B, 2> {
        self.forward_typed_logits_with_encoded_state(
            self.encode_state(state),
            candidates,
            typed_state,
            group_sizes,
            device,
        )
    }

    pub fn forward_typed_logits_with_encoded_state(
        &self,
        state: Tensor<B, 2>,
        candidates: &PaddedEntityBatch,
        typed_state: Tensor<B, 2>,
        group_sizes: &[usize],
        device: &B::Device,
    ) -> Tensor<B, 2> {
        let candidates = self.encode_candidates(candidates, device);
        self.forward_typed_logits_with_encoded_candidates(
            state,
            candidates,
            typed_state,
            group_sizes,
        )
    }

    pub fn forward_typed_logits_with_encoded_candidates(
        &self,
        state: Tensor<B, 2>,
        candidates: Tensor<B, 2>,
        typed_state: Tensor<B, 2>,
        group_sizes: &[usize],
    ) -> Tensor<B, 2> {
        let rows = candidates.dims()[0];
        assert_eq!(group_sizes.iter().sum::<usize>(), rows);
        assert_eq!(
            state.dims()[0],
            rows,
            "state batch must match candidate row count"
        );
        assert_eq!(
            typed_state.dims()[0],
            rows,
            "typed state batch must match candidate row count"
        );
        let candidate_width = candidates.dims()[1];
        let context = if group_sizes
            .first()
            .is_some_and(|&group_size| group_sizes.iter().all(|&size| size == group_size))
        {
            let group_size = group_sizes[0];
            candidates
                .clone()
                .reshape([group_sizes.len(), group_size, candidate_width])
                .mean_dim(1)
                .reshape([group_sizes.len(), 1, candidate_width])
                .repeat_dim(1, group_size)
                .reshape([rows, candidate_width])
        } else {
            let mut offset = 0;
            let contexts = group_sizes
                .iter()
                .map(|&group_size| {
                    let context = candidates
                        .clone()
                        .slice([offset..offset + group_size, 0..candidates.dims()[1]])
                        .mean_dim(0)
                        .reshape([1, candidates.dims()[1]])
                        .repeat_dim(0, group_size);
                    offset += group_size;
                    context
                })
                .collect::<Vec<_>>();
            Tensor::cat(contexts, 0)
        };
        let joined = Tensor::cat(vec![state, candidates, typed_state, context], 1);
        self.typed_actor_output.forward(
            self.activation
                .forward(self.typed_actor_hidden.forward(joined)),
        )
    }

    pub fn forward_typed_values(&self, typed_state: Tensor<B, 2>) -> Tensor<B, 2> {
        self.typed_critic_output.forward(
            self.activation
                .forward(self.typed_critic_hidden.forward(typed_state)),
        )
    }

    pub fn forward_typed_policy_and_value(
        &self,
        state: Tensor<B, 2>,
        candidates: &PaddedEntityBatch,
        typed_state: Tensor<B, 2>,
        device: &B::Device,
    ) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let value = self.forward_typed_values(typed_state.clone());
        let rows = candidates.categorical.len() / (candidates.max_entities * 4);
        let typed_state = typed_state.repeat_dim(0, rows);
        let logits =
            self.forward_typed_logits_with_candidates(state, candidates, typed_state, device);
        (logits, value)
    }

    pub fn encode_typed_sets(
        &self,
        sets: &[PaddedEntityBatch; ENTITY_SET_COUNT],
        device: &B::Device,
    ) -> Tensor<B, 2> {
        let pooled = sets
            .iter()
            .map(|set| self.encode_typed_batch(set, device))
            .collect::<Vec<_>>();
        self.activation
            .forward(self.typed_fusion.forward(Tensor::cat(pooled, 1)))
    }

    pub fn encode_typed_tensor_sets(
        &self,
        sets: [EntityBatchTensors<B>; ENTITY_SET_COUNT],
    ) -> Tensor<B, 2> {
        let pooled = sets
            .into_iter()
            .map(|set| self.encode_entity_set(set.categorical, set.numeric, set.mask))
            .collect::<Vec<_>>();
        self.activation
            .forward(self.typed_fusion.forward(Tensor::cat(pooled, 1)))
    }

    pub fn encode_typed_batch(
        &self,
        batch: &PaddedEntityBatch,
        device: &B::Device,
    ) -> Tensor<B, 2> {
        let tensors = to_tensors(batch, device);
        self.encode_entity_set(tensors.categorical, tensors.numeric, tensors.mask)
    }

    pub fn encode_candidates(
        &self,
        candidates: &PaddedEntityBatch,
        device: &B::Device,
    ) -> Tensor<B, 2> {
        self.activation.forward(
            self.candidate_fusion
                .forward(self.encode_typed_batch(candidates, device)),
        )
    }

    pub fn encode_candidate_tensors(&self, candidates: EntityBatchTensors<B>) -> Tensor<B, 2> {
        self.activation
            .forward(self.candidate_fusion.forward(self.encode_entity_set(
                candidates.categorical,
                candidates.numeric,
                candidates.mask,
            )))
    }

    pub fn encode_state(&self, state: Tensor<B, 2>) -> Tensor<B, 2> {
        let state = self.activation.forward(self.state_input.forward(state));
        self.activation.forward(self.state_hidden.forward(state))
    }

    fn encode_action(&self, action: Tensor<B, 2>) -> Tensor<B, 2> {
        let action = self.activation.forward(self.action_input.forward(action));
        self.activation.forward(self.action_hidden.forward(action))
    }
}

pub fn tensor_from_rows<B: Backend>(rows: &[Vec<f32>], device: &B::Device) -> Tensor<B, 2> {
    let columns = rows.first().map_or(0, Vec::len);
    assert!(columns > 0);
    assert!(rows.iter().all(|row| row.len() == columns));
    let values = rows.iter().flatten().copied().collect::<Vec<_>>();
    Tensor::from_data(TensorData::new(values, [rows.len(), columns]), device)
}

pub fn tensor_from_flat_rows<B: Backend>(
    values: Vec<f32>,
    rows: usize,
    columns: usize,
    device: &B::Device,
) -> Tensor<B, 2> {
    assert!(columns > 0);
    assert_eq!(values.len(), rows * columns);
    Tensor::from_data(TensorData::new(values, [rows, columns]), device)
}

pub fn tensor_from_repeated_row<B: Backend>(
    row: &[f32],
    count: usize,
    device: &B::Device,
) -> Tensor<B, 2> {
    assert!(!row.is_empty());
    let values = row
        .iter()
        .copied()
        .cycle()
        .take(row.len() * count)
        .collect::<Vec<_>>();
    Tensor::from_data(TensorData::new(values, [count, row.len()]), device)
}

#[cfg(test)]
mod tensor_layout_tests {
    use super::*;

    #[test]
    fn flat_and_repeated_rows_have_same_shape_and_values() {
        let device = default_policy_device();
        let row = vec![1.0, 2.0, 3.0];
        let repeated = tensor_from_repeated_row::<InferenceBackend>(&row, 4, &device)
            .into_data()
            .to_vec::<f32>()
            .expect("repeated tensor data should be f32");
        let flat =
            tensor_from_flat_rows::<InferenceBackend>(repeated.clone(), 4, row.len(), &device)
                .into_data()
                .to_vec::<f32>()
                .expect("flat tensor data should be f32");
        assert_eq!(flat, repeated);
    }
}

pub fn save_inference_model(
    model: DeepSetsActorCritic<InferenceBackend>,
    path: &Path,
) -> Result<()> {
    model.save_file(path, &CompactRecorder::new())?;
    Ok(())
}

pub fn model_to_full_precision_bytes<B: Backend>(model: DeepSetsActorCritic<B>) -> Result<Vec<u8>> {
    let recorder = NamedMpkBytesRecorder::<FullPrecisionSettings>::default();
    Ok(recorder.record(model.into_record(), ())?)
}

pub fn model_from_full_precision_bytes<B: Backend>(
    config: ModelConfig,
    bytes: Vec<u8>,
    device: &B::Device,
) -> Result<DeepSetsActorCritic<B>> {
    let recorder = NamedMpkBytesRecorder::<FullPrecisionSettings>::default();
    let record = recorder.load(bytes, device)?;
    Ok(DeepSetsActorCritic::new(config, device).load_record(record))
}

#[cfg(feature = "simulator-wgpu")]
pub fn cpu_model_to_gpu_train_model(
    model: DeepSetsActorCritic<CpuInferenceBackend>,
    config: ModelConfig,
    device: &GpuPolicyDevice,
) -> Result<DeepSetsActorCritic<GpuTrainBackend>> {
    let bytes = model_to_full_precision_bytes(model)?;
    model_from_full_precision_bytes::<GpuTrainBackend>(config, bytes, device)
}

#[cfg(feature = "simulator-wgpu")]
pub fn gpu_train_model_to_cpu_inference_model(
    model: &DeepSetsActorCritic<GpuTrainBackend>,
    config: ModelConfig,
    device: &CpuPolicyDevice,
) -> Result<DeepSetsActorCritic<CpuInferenceBackend>> {
    let bytes = model_to_full_precision_bytes(model.clone().valid())?;
    model_from_full_precision_bytes::<CpuInferenceBackend>(config, bytes, device)
}

#[cfg(feature = "simulator-wgpu")]
pub fn cpu_inference_model_to_cpu_train_model(
    model: DeepSetsActorCritic<CpuInferenceBackend>,
    config: ModelConfig,
    device: &CpuPolicyDevice,
) -> Result<DeepSetsActorCritic<CpuTrainBackend>> {
    let bytes = model_to_full_precision_bytes(model)?;
    model_from_full_precision_bytes::<CpuTrainBackend>(config, bytes, device)
}

pub fn inference_model(
    model: &DeepSetsActorCritic<TrainBackend>,
) -> DeepSetsActorCritic<InferenceBackend> {
    model.clone().valid()
}

pub fn initialize_model(model: &DeepSetsActorCritic<TrainBackend>, device: &PolicyDevice) {
    let state = vec![0.0; GLOBAL_FEATURE_COUNT];
    let action = vec![0.0; ACTION_FEATURE_COUNT];
    let _ = model.forward_logits(
        tensor_from_rows::<TrainBackend>(std::slice::from_ref(&state), device),
        tensor_from_rows::<TrainBackend>(std::slice::from_ref(&action), device),
    );
    let _ = model.forward_values(tensor_from_rows::<TrainBackend>(&[state], device));
}

pub fn save_train_model(model: &DeepSetsActorCritic<TrainBackend>, path: &Path) -> Result<()> {
    save_inference_model(inference_model(model), path)
}

pub fn load_train_model(
    config: ModelConfig,
    path: &Path,
    device: &PolicyDevice,
) -> Result<DeepSetsActorCritic<TrainBackend>> {
    let model = DeepSetsActorCritic::<TrainBackend>::new(config, device);
    let record = CompactRecorder::new().load(path.to_path_buf(), device)?;
    Ok(model.load_record(record))
}

pub fn load_inference_model(
    config: ModelConfig,
    path: &Path,
    device: &PolicyDevice,
) -> Result<DeepSetsActorCritic<InferenceBackend>> {
    let model = DeepSetsActorCritic::new(config, device);
    let record = CompactRecorder::new().load(path.to_path_buf(), device)?;
    Ok(model.load_record(record))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulator::environment::{AgentAction, GameEnvironment};
    use crate::simulator::ml::encoding::{EntityRow, EntitySet};
    use crate::simulator::ml::features::{candidate_features, observation_features};
    use std::sync::Arc;

    fn typed_batches_with_type(type_id: u32) -> [PaddedEntityBatch; ENTITY_TYPE_COUNT] {
        std::array::from_fn(|index| {
            let row = EntityRow::new(
                [
                    if index == 0 {
                        type_id
                    } else {
                        ENTITY_PADDING_ID
                    },
                    1,
                    0,
                    0,
                ],
                vec![1.0, 0.5, 0.25, 0.125, 1.0],
            );
            PaddedEntityBatch::from_sets(&[if index == 0 {
                EntitySet::new(vec![row])
            } else {
                EntitySet::default()
            }])
        })
    }

    #[cfg(feature = "simulator-wgpu")]
    #[test]
    #[ignore = "requires an AMD discrete GPU and a configured WGPU adapter"]
    fn wgpu_typed_forward_backward_smoke() {
        let device = GpuPolicyDevice::DiscreteGpu(0);
        let model = DeepSetsActorCritic::<GpuTrainBackend>::new(ModelConfig::default(), &device);
        let state = Tensor::zeros([1, GLOBAL_FEATURE_COUNT], &device);
        let action = Tensor::zeros([1, ACTION_FEATURE_COUNT], &device);
        let loss = model.forward_logits(state, action).sum();
        let gradients = loss.backward();
        let gradients = burn::optim::GradientsParams::from_grads(gradients, &model);
        assert!(!gradients.is_empty());
    }

    #[test]
    fn entity_encoder_contract_keeps_zero_as_padding_and_type_embedding_active() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let empty = typed_batches_with_type(ENTITY_PADDING_ID);
        let typed = typed_batches_with_type(1);
        let empty_state = model.encode_typed_sets(&empty, &device);
        let typed_state = model.encode_typed_sets(&typed, &device);
        let empty_values = model
            .forward_typed_values(empty_state)
            .into_data()
            .to_vec::<f32>()
            .expect("empty values");
        let typed_values = model
            .forward_typed_values(typed_state)
            .into_data()
            .to_vec::<f32>()
            .expect("typed values");
        assert!(empty_values.iter().all(|value| value.is_finite()));
        assert!(typed_values.iter().all(|value| value.is_finite()));
        assert_ne!(empty_values, typed_values);
    }

    #[test]
    fn each_entity_type_has_a_separate_typed_pooling_slot() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let base = typed_batches_with_type(1);
        let base_values = model
            .forward_typed_values(model.encode_typed_sets(&base, &device))
            .into_data()
            .to_vec::<f32>()
            .expect("base values");
        for index in 0..ENTITY_TYPE_COUNT {
            let mut changed = base.clone();
            changed[index] = PaddedEntityBatch::from_sets(&[EntitySet::new(vec![EntityRow::new(
                [1, 2, 0, 0],
                vec![2.0, 1.0, 0.5, 0.25, 1.0],
            )])]);
            let values = model
                .forward_typed_values(model.encode_typed_sets(&changed, &device))
                .into_data()
                .to_vec::<f32>()
                .expect("changed values");
            assert!(values.iter().all(|value| value.is_finite()));
            assert_ne!(
                values, base_values,
                "entity type slot {index} did not affect critic"
            );
        }
    }

    #[test]
    fn flex_forward_and_record_round_trip_work() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let config = Arc::new(crate::config::GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let state = observation_features(&environment.snapshot());
        let action = candidate_features(&environment.snapshot(), &AgentAction::StartSelectingTower);
        let logits = model.forward_logits(
            tensor_from_rows(&[state], &device),
            tensor_from_rows(&[action], &device),
        );
        assert_eq!(logits.dims(), [1, 1]);
    }

    #[test]
    fn full_precision_bytes_round_trip_preserves_logits() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let config = Arc::new(crate::config::GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let observation = environment.snapshot();
        let state = observation_features(&observation);
        let action = candidate_features(&observation, &AgentAction::StartSelectingTower);
        let before = model
            .forward_logits(
                tensor_from_rows(std::slice::from_ref(&state), &device),
                tensor_from_rows(std::slice::from_ref(&action), &device),
            )
            .into_data()
            .to_vec::<f32>()
            .expect("pre-bridge logits");
        let bytes = model_to_full_precision_bytes(model).expect("serialize model");
        let restored = model_from_full_precision_bytes::<InferenceBackend>(
            ModelConfig::default(),
            bytes,
            &device,
        )
        .expect("restore model");
        let after = restored
            .forward_logits(
                tensor_from_rows(std::slice::from_ref(&state), &device),
                tensor_from_rows(std::slice::from_ref(&action), &device),
            )
            .into_data()
            .to_vec::<f32>()
            .expect("post-bridge logits");
        assert_eq!(before, after);
    }

    #[cfg(feature = "simulator-wgpu")]
    #[test]
    #[ignore = "requires an AMD discrete GPU and a configured WGPU adapter"]
    fn wgpu_train_model_can_snapshot_back_to_cpu_inference() {
        let cpu_device = CpuPolicyDevice::default();
        let gpu_device = GpuPolicyDevice::DiscreteGpu(0);
        let cpu_model =
            DeepSetsActorCritic::<CpuInferenceBackend>::new(ModelConfig::default(), &cpu_device);
        let gpu_model =
            cpu_model_to_gpu_train_model(cpu_model, ModelConfig::default(), &gpu_device)
                .expect("CPU model should load into WGPU");
        let snapshot =
            gpu_train_model_to_cpu_inference_model(&gpu_model, ModelConfig::default(), &cpu_device)
                .expect("WGPU model should snapshot to CPU");
        let state = vec![0.0; GLOBAL_FEATURE_COUNT];
        let action = vec![0.0; ACTION_FEATURE_COUNT];
        let logits = snapshot.forward_logits(
            tensor_from_rows::<CpuInferenceBackend>(&[state], &cpu_device),
            tensor_from_rows::<CpuInferenceBackend>(&[action], &cpu_device),
        );
        assert!(logits.into_data().to_vec::<f32>().unwrap()[0].is_finite());
    }

    #[cfg(feature = "simulator-wgpu")]
    #[test]
    #[ignore = "requires an AMD discrete GPU and a configured WGPU adapter"]
    fn cpu_model_can_be_loaded_into_wgpu_train_model() {
        let cpu_device = default_policy_device();
        let cpu_model =
            DeepSetsActorCritic::<CpuInferenceBackend>::new(ModelConfig::default(), &cpu_device);
        let gpu_device = GpuPolicyDevice::DiscreteGpu(0);
        let gpu_model =
            cpu_model_to_gpu_train_model(cpu_model, ModelConfig::default(), &gpu_device)
                .expect("CPU model should load into WGPU train model");
        let state = Tensor::zeros([1, GLOBAL_FEATURE_COUNT], &gpu_device);
        let action = Tensor::zeros([1, ACTION_FEATURE_COUNT], &gpu_device);
        let loss = gpu_model.forward_logits(state, action).sum();
        let gradients = loss.backward();
        let gradients = burn::optim::GradientsParams::from_grads(gradients, &gpu_model);
        assert!(!gradients.is_empty());
    }

    #[cfg(feature = "simulator-wgpu")]
    #[test]
    #[ignore = "requires an AMD discrete GPU and a configured WGPU adapter"]
    fn wgpu_to_cpu_full_precision_bridge_preserves_logits() {
        let gpu_device = GpuPolicyDevice::DiscreteGpu(0);
        let gpu_model =
            DeepSetsActorCritic::<GpuInferenceBackend>::new(ModelConfig::default(), &gpu_device);
        let bytes = model_to_full_precision_bytes(gpu_model).expect("serialize GPU model");
        let cpu_device = default_policy_device();
        let cpu_model = model_from_full_precision_bytes::<InferenceBackend>(
            ModelConfig::default(),
            bytes,
            &cpu_device,
        )
        .expect("restore CPU model");
        let state = vec![0.0; GLOBAL_FEATURE_COUNT];
        let action = vec![0.0; ACTION_FEATURE_COUNT];
        let logits = cpu_model
            .forward_logits(
                tensor_from_rows(&[state], &cpu_device),
                tensor_from_rows(&[action], &cpu_device),
            )
            .into_data()
            .to_vec::<f32>()
            .expect("CPU logits");
        assert!(logits.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn typed_forward_has_expected_shapes_and_responds_to_entity_identity() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let config = Arc::new(crate::config::GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let observation = environment.snapshot();
        let typed =
            crate::simulator::ml::encoding::TypedObservation::from_observation(&observation);
        let batches = std::array::from_fn(|index| {
            crate::simulator::ml::encoding::PaddedEntityBatch::from_sets(&[
                typed.sets[index].clone()
            ])
        });
        let typed_state = model.encode_typed_sets(&batches, &device);
        let state = observation_features(&observation);
        let action = candidate_features(&observation, &AgentAction::StartSelectingTower);
        let logits = model.forward_typed_logits(
            tensor_from_rows(&[state], &device),
            tensor_from_rows(&[action], &device),
            typed_state.clone(),
        );
        let values = model.forward_typed_values(typed_state);
        assert_eq!(logits.dims(), [1, 1]);
        assert_eq!(values.dims(), [1, 1]);
        assert!(logits.into_data().to_vec::<f32>().expect("logits")[0].is_finite());
        assert!(values.into_data().to_vec::<f32>().expect("values")[0].is_finite());
    }

    #[test]
    fn autodiff_backward_and_record_round_trip_work() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        let state = tensor_from_rows::<TrainBackend>(&[vec![0.0; GLOBAL_FEATURE_COUNT]], &device);
        let action = tensor_from_rows::<TrainBackend>(&[vec![0.0; ACTION_FEATURE_COUNT]], &device);
        let loss = model.forward_logits(state, action).sum();
        let _gradients = loss.backward();
    }

    #[test]
    fn typed_actor_critic_backward_produces_gradients() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        let environment =
            GameEnvironment::new(Arc::new(crate::config::GameConfig::default_config()), 7);
        let observation = environment.snapshot();
        let typed =
            crate::simulator::ml::encoding::TypedObservation::from_observation(&observation);
        let batches = std::array::from_fn(|index| {
            crate::simulator::ml::encoding::PaddedEntityBatch::from_sets(&[
                typed.sets[index].clone()
            ])
        });
        let typed_state = model.encode_typed_sets(&batches, &device);
        let state = tensor_from_repeated_row::<TrainBackend>(
            &observation_features(&observation),
            2,
            &device,
        );
        let action = tensor_from_rows::<TrainBackend>(
            &[
                candidate_features(&observation, &AgentAction::StartSelectingTower),
                candidate_features(&observation, &AgentAction::Continue),
            ],
            &device,
        );
        let logits = model.forward_typed_logits(state, action, typed_state.clone());
        let values = model.forward_typed_values(typed_state);
        let loss = logits.sum() + values.sum();
        let gradients = loss.backward();
        let gradients = burn::optim::GradientsParams::from_grads(gradients, &model);
        assert!(!gradients.is_empty());
    }

    #[test]
    fn candidate_context_is_isolated_per_transition_group() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let state = tensor_from_repeated_row::<InferenceBackend>(
            &vec![0.0; GLOBAL_FEATURE_COUNT],
            4,
            &device,
        );
        let typed_state = tensor_from_repeated_row::<InferenceBackend>(&vec![0.0; 64], 4, &device);
        let first = crate::simulator::ml::encoding::EntityRow::new([1, 1, 0, 0], vec![1.0; 5]);
        let second = crate::simulator::ml::encoding::EntityRow::new([1, 2, 0, 0], vec![2.0; 5]);
        let candidates = PaddedEntityBatch::from_sets(&[
            crate::simulator::ml::encoding::EntitySet::new(vec![first.clone()]),
            crate::simulator::ml::encoding::EntitySet::new(vec![second.clone()]),
            crate::simulator::ml::encoding::EntitySet::new(vec![first.clone()]),
            crate::simulator::ml::encoding::EntitySet::new(vec![second.clone()]),
        ]);
        let isolated = model.forward_typed_logits_with_candidate_groups(
            state.clone(),
            &candidates,
            typed_state.clone(),
            &[2, 2],
            &device,
        );
        let changed_candidates = PaddedEntityBatch::from_sets(&[
            crate::simulator::ml::encoding::EntitySet::new(vec![first.clone()]),
            crate::simulator::ml::encoding::EntitySet::new(vec![second.clone()]),
            crate::simulator::ml::encoding::EntitySet::new(vec![first.clone()]),
            crate::simulator::ml::encoding::EntitySet::new(vec![first]),
        ]);
        let changed = model.forward_typed_logits_with_candidate_groups(
            state,
            &changed_candidates,
            typed_state,
            &[2, 2],
            &device,
        );
        let before = isolated
            .slice([0..2, 0..1])
            .into_data()
            .to_vec::<f32>()
            .expect("logits");
        let after = changed
            .slice([0..2, 0..1])
            .into_data()
            .to_vec::<f32>()
            .expect("logits");
        assert_eq!(before, after);
    }

    #[test]
    fn typed_candidate_forward_has_finite_logits() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let environment =
            GameEnvironment::new(Arc::new(crate::config::GameConfig::default_config()), 7);
        let observation = environment.snapshot();
        let typed =
            crate::simulator::ml::encoding::TypedObservation::from_observation(&observation);
        let state_sets = std::array::from_fn(|index| {
            crate::simulator::ml::encoding::PaddedEntityBatch::from_sets(&[
                typed.sets[index].clone()
            ])
        });
        let candidates = crate::simulator::ml::encoding::PolicyBatch::from_observation(
            &observation,
            &environment.legal_actions(),
        );
        let typed_state = model.encode_typed_sets(&state_sets, &device);
        let candidate_set = crate::simulator::ml::encoding::PaddedEntityBatch::from_sets(
            &candidates
                .candidate_rows
                .iter()
                .map(|rows| crate::simulator::ml::encoding::EntitySet::new(rows.clone()))
                .collect::<Vec<_>>(),
        );
        let logits = model.forward_typed_logits_with_candidates(
            tensor_from_repeated_row(
                &observation_features(&observation),
                candidate_set.batch_size,
                &device,
            ),
            &candidate_set,
            typed_state.repeat_dim(0, candidate_set.batch_size),
            &device,
        );
        assert!(
            logits
                .into_data()
                .to_vec::<f32>()
                .expect("logits")
                .iter()
                .all(|value| value.is_finite())
        );
    }

    #[test]
    fn model_record_save_and_load_work() {
        let device = default_policy_device();
        let model = DeepSetsActorCritic::<InferenceBackend>::new(ModelConfig::default(), &device);
        let path =
            std::env::temp_dir().join(format!("tower-defense-ml-model-{}.mpk", std::process::id()));
        save_inference_model(model, &path).expect("save model record");
        let loaded = load_inference_model(ModelConfig::default(), &path, &device)
            .expect("load model record");
        assert!(loaded.num_params() > 0);
        std::fs::remove_file(path).expect("remove model record");
    }

    #[test]
    fn inference_model_preserves_forward_logits() {
        let device = default_policy_device();
        let train_model = DeepSetsActorCritic::<TrainBackend>::new(ModelConfig::default(), &device);
        initialize_model(&train_model, &device);
        let inference_model = inference_model(&train_model);
        let state = vec![0.25; GLOBAL_FEATURE_COUNT];
        let action = vec![0.5; ACTION_FEATURE_COUNT];
        let train_logits = train_model
            .forward_logits(
                tensor_from_rows::<TrainBackend>(std::slice::from_ref(&state), &device),
                tensor_from_rows::<TrainBackend>(std::slice::from_ref(&action), &device),
            )
            .into_data()
            .to_vec::<f32>()
            .expect("train logits should be f32");
        let inference_logits = inference_model
            .forward_logits(
                tensor_from_rows::<InferenceBackend>(&[state], &device),
                tensor_from_rows::<InferenceBackend>(&[action], &device),
            )
            .into_data()
            .to_vec::<f32>()
            .expect("inference logits should be f32");

        assert_eq!(train_logits.len(), inference_logits.len());
        for (index, (train, inference)) in train_logits.iter().zip(inference_logits).enumerate() {
            assert!(
                (train - inference).abs() < 1e-4,
                "logit {index} differs: train={train}, inference={inference}, diff={}",
                (train - inference).abs()
            );
        }
    }
}
