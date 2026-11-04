use burn::tensor::Tensor;
use burn::tensor::TensorData;
use burn::tensor::activation::log_softmax;
use burn::tensor::backend::Backend;
use burn::tensor::{Bool, Int};

/// Candidate groups of variable size laid out as one flat logit column,
/// padded to `[groups, max_group]` for a vectorized per-group softmax.
/// Padding and masked-out (illegal) candidates get probability exactly zero.
pub(crate) struct MaskedGroups {
    groups: usize,
    width: usize,
    gather_index: Vec<i64>,
    valid: Vec<bool>,
}

impl MaskedGroups {
    pub(crate) fn new(group_sizes: &[usize], legal_masks: Option<&[&[bool]]>) -> Self {
        let width = group_sizes.iter().copied().max().unwrap_or(0).max(1);
        let mut gather_index = Vec::with_capacity(group_sizes.len() * width);
        let mut valid = Vec::with_capacity(group_sizes.len() * width);
        let mut offset = 0usize;
        for (group, &size) in group_sizes.iter().enumerate() {
            for column in 0..width {
                if column < size {
                    gather_index.push((offset + column) as i64);
                    valid.push(legal_masks.is_none_or(|masks| masks[group][column]));
                } else {
                    gather_index.push(0);
                    valid.push(false);
                }
            }
            assert!(
                valid[group * width..(group + 1) * width].iter().any(|v| *v),
                "candidate group {group} has no legal candidate"
            );
            offset += size;
        }
        Self {
            groups: group_sizes.len(),
            width,
            gather_index,
            valid,
        }
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    /// Per-candidate rows `[rows, features]` -> `[groups, width, features]`.
    /// Padding slots repeat row 0; callers mask them.
    pub(crate) fn padded_rows<B: Backend>(
        &self,
        rows: Tensor<B, 2>,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        let features = rows.dims()[1];
        let index = Tensor::<B, 1, Int>::from_data(
            TensorData::new(self.gather_index.clone(), [self.gather_index.len()]),
            device,
        );
        rows.select(0, index)
            .reshape([self.groups, self.width, features])
    }

    /// Flat candidate column `[rows, 1]` -> `[groups, width]`, with padding
    /// and masked candidates set to -1e9, plus the invalid mask.
    pub(crate) fn padded_logits<B: Backend>(
        &self,
        logits: Tensor<B, 2>,
        device: &B::Device,
    ) -> (Tensor<B, 2>, Tensor<B, 2, Bool>) {
        let rows = logits.dims()[0];
        let index = Tensor::<B, 1, Int>::from_data(
            TensorData::new(self.gather_index.clone(), [self.gather_index.len()]),
            device,
        );
        let invalid = Tensor::<B, 2, Bool>::from_data(
            TensorData::new(
                self.valid.iter().map(|valid| !valid).collect::<Vec<_>>(),
                [self.groups, self.width],
            ),
            device,
        );
        let padded = logits
            .reshape([rows])
            .select(0, index)
            .reshape([self.groups, self.width])
            .mask_fill(invalid.clone(), -1.0e9);
        (padded, invalid)
    }

    pub(crate) fn is_valid(&self, group: usize, column: usize) -> bool {
        self.valid[group * self.width + column]
    }

    /// `logits` is the flat `[rows, 1]` candidate column; returns
    /// `[groups, width]` log-probabilities.
    pub(crate) fn log_probs<B: Backend>(
        &self,
        logits: Tensor<B, 2>,
        device: &B::Device,
    ) -> Tensor<B, 2> {
        let rows = logits.dims()[0];
        let index = Tensor::<B, 1, Int>::from_data(
            TensorData::new(self.gather_index.clone(), [self.gather_index.len()]),
            device,
        );
        let invalid = Tensor::<B, 2, Bool>::from_data(
            TensorData::new(
                self.valid.iter().map(|valid| !valid).collect::<Vec<_>>(),
                [self.groups, self.width],
            ),
            device,
        );
        let padded = logits
            .reshape([rows])
            .select(0, index)
            .reshape([self.groups, self.width])
            .mask_fill(invalid.clone(), -1.0e9);
        log_softmax(padded, 1).mask_fill(invalid, -1.0e9)
    }

    pub(crate) fn target_tensor<B: Backend>(
        &self,
        targets: &[usize],
        device: &B::Device,
    ) -> Tensor<B, 2, Int> {
        assert_eq!(targets.len(), self.groups);
        Tensor::<B, 2, Int>::from_data(
            TensorData::new(
                targets
                    .iter()
                    .map(|target| *target as i64)
                    .collect::<Vec<_>>(),
                [self.groups, 1],
            ),
            device,
        )
    }

    /// Highest-probability valid column of each group; never a masked one.
    pub(crate) fn argmax(&self, log_probs: &[f32]) -> Vec<usize> {
        (0..self.groups)
            .map(|group| {
                (0..self.width)
                    .filter(|column| self.is_valid(group, *column))
                    .max_by(|left, right| {
                        log_probs[group * self.width + left]
                            .total_cmp(&log_probs[group * self.width + right])
                            .then_with(|| right.cmp(left))
                    })
                    .expect("every group has a valid column")
            })
            .collect()
    }
}
