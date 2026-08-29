use serde::{Deserialize, Serialize};

pub const CATEGORICAL_FIELDS: usize = 4;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRow {
    pub categorical: [u32; CATEGORICAL_FIELDS],
    pub numeric: Vec<f32>,
}

impl EntityRow {
    pub fn new(categorical: [u32; CATEGORICAL_FIELDS], numeric: Vec<f32>) -> Self {
        assert!(numeric.iter().all(|value| value.is_finite()));
        Self {
            categorical,
            numeric,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EntitySet {
    pub rows: Vec<EntityRow>,
}

impl EntitySet {
    pub fn new(rows: Vec<EntityRow>) -> Self {
        let numeric_width = rows.first().map_or(0, |row| row.numeric.len());
        assert!(rows.iter().all(|row| row.numeric.len() == numeric_width));
        Self { rows }
    }

    pub fn numeric_width(&self) -> usize {
        self.rows.first().map_or(0, |row| row.numeric.len())
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PooledSet {
    pub mean: Vec<f32>,
    pub max: Vec<f32>,
}

impl PooledSet {
    pub fn empty(width: usize) -> Self {
        Self {
            mean: vec![0.0; width],
            max: vec![0.0; width],
        }
    }
}

pub fn masked_mean_max(rows: &[Vec<f32>], mask: &[bool]) -> PooledSet {
    assert_eq!(rows.len(), mask.len());
    let width = rows.first().map_or(0, Vec::len);
    assert!(rows.iter().all(|row| row.len() == width));
    let mut mean = vec![0.0; width];
    let mut max = vec![f32::NEG_INFINITY; width];
    let mut count = 0.0;
    for (row, included) in rows.iter().zip(mask) {
        if *included {
            count += 1.0;
            for (index, value) in row.iter().enumerate() {
                mean[index] += value;
                max[index] = max[index].max(*value);
            }
        }
    }
    if count == 0.0 {
        return PooledSet::empty(width);
    }
    for value in &mut mean {
        *value /= count;
    }
    PooledSet { mean, max }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pooling_is_permutation_invariant() {
        let rows = vec![vec![1.0, 4.0], vec![3.0, 2.0], vec![5.0, 6.0]];
        let reordered = vec![rows[2].clone(), rows[0].clone(), rows[1].clone()];
        assert_eq!(
            masked_mean_max(&rows, &[true, true, true]),
            masked_mean_max(&reordered, &[true, true, true])
        );
    }

    #[test]
    fn padding_is_ignored_and_empty_is_finite_zero() {
        let pooled = masked_mean_max(&[vec![2.0, 5.0], vec![999.0, 999.0]], &[true, false]);
        assert_eq!(pooled.mean, vec![2.0, 5.0]);
        assert_eq!(pooled.max, vec![2.0, 5.0]);
        let empty = masked_mean_max(&[vec![999.0, 999.0]], &[false]);
        assert_eq!(empty.mean, vec![0.0, 0.0]);
        assert_eq!(empty.max, vec![0.0, 0.0]);
        assert!(
            empty
                .mean
                .iter()
                .chain(&empty.max)
                .all(|value| value.is_finite())
        );
    }
}
