use super::{CATEGORICAL_FIELDS, PaddedEntityBatch};
use burn::tensor::Tensor;
use burn::tensor::backend::Backend;

#[derive(Clone)]
pub struct EntityBatchTensors<B: Backend> {
    pub categorical: Tensor<B, 2, burn::tensor::Int>,
    pub numeric: Tensor<B, 3>,
    pub mask: Tensor<B, 3>,
    pub max_entities: usize,
}

impl<B: Backend> EntityBatchTensors<B> {
    pub fn select(self, indices: Tensor<B, 1, burn::tensor::Int>) -> Self {
        let entity_indices = indices
            .clone()
            .reshape([indices.dims()[0], 1])
            .repeat_dim(1, self.max_entities)
            .reshape([indices.dims()[0] * self.max_entities]);
        Self {
            categorical: self.categorical.select(0, entity_indices),
            numeric: self.numeric.select(0, indices.clone()),
            mask: self.mask.select(0, indices),
            max_entities: self.max_entities,
        }
    }
}

pub fn to_tensors<B: Backend>(
    batch: &PaddedEntityBatch,
    device: &B::Device,
) -> EntityBatchTensors<B> {
    assert!(!batch.is_empty());
    EntityBatchTensors {
        categorical: Tensor::<B, 2, burn::tensor::Int>::from_data(
            burn::tensor::TensorData::new(
                batch
                    .categorical
                    .iter()
                    .map(|value| *value as i32)
                    .collect::<Vec<_>>(),
                [batch.batch_size * batch.max_entities, CATEGORICAL_FIELDS],
            ),
            device,
        ),
        numeric: Tensor::from_data(
            burn::tensor::TensorData::new(
                batch.numeric.clone(),
                [batch.batch_size, batch.max_entities, batch.numeric_width],
            ),
            device,
        ),
        mask: Tensor::from_data(
            burn::tensor::TensorData::new(
                batch.mask.clone(),
                [batch.batch_size, batch.max_entities, 1],
            ),
            device,
        ),
        max_entities: batch.max_entities,
    }
}
