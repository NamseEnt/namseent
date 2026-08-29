use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

pub const SEED_SCHEDULE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SeedRange {
    pub start: u64,
    pub end_inclusive: u64,
}

impl SeedRange {
    pub fn try_new(start: u64, end_inclusive: u64) -> Result<Self, SeedRangeError> {
        if start > end_inclusive {
            return Err(SeedRangeError::InvalidBounds {
                start,
                end_inclusive,
            });
        }
        Ok(Self {
            start,
            end_inclusive,
        })
    }

    fn len(self) -> u128 {
        u128::from(self.end_inclusive) - u128::from(self.start) + 1
    }

    pub fn contains(self, seed: u64) -> bool {
        self.start <= seed && seed <= self.end_inclusive
    }

    pub fn iter(self) -> RangeInclusive<u64> {
        self.start..=self.end_inclusive
    }

    pub fn seeds(self) -> Vec<u64> {
        self.iter().collect()
    }

    pub fn digest(self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"tower-defense-ml-seed-list");
        hasher.update(SEED_SCHEDULE_SCHEMA_VERSION.to_be_bytes());
        hasher.update(self.len().to_be_bytes());
        for seed in self.iter() {
            hasher.update(seed.to_be_bytes());
        }
        hex_digest(hasher.finalize())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeedRangeError {
    InvalidBounds {
        start: u64,
        end_inclusive: u64,
    },
    OverlappingRanges {
        train: SeedRange,
        validation: SeedRange,
    },
}

impl Display for SeedRangeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBounds {
                start,
                end_inclusive,
            } => write!(
                formatter,
                "seed range start {start} is after inclusive end {end_inclusive}"
            ),
            Self::OverlappingRanges { train, validation } => write!(
                formatter,
                "training seed range {}..={} overlaps validation range {}..={}",
                train.start, train.end_inclusive, validation.start, validation.end_inclusive
            ),
        }
    }
}

impl std::error::Error for SeedRangeError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingSeedSchedule {
    pub schema_version: u32,
    pub train: SeedRange,
    pub validation: SeedRange,
    pub train_digest: String,
    pub validation_digest: String,
}

impl TrainingSeedSchedule {
    pub fn try_new(train: SeedRange, validation: SeedRange) -> Result<Self, SeedRangeError> {
        if train.start <= validation.end_inclusive && validation.start <= train.end_inclusive {
            return Err(SeedRangeError::OverlappingRanges { train, validation });
        }
        Ok(Self {
            schema_version: SEED_SCHEDULE_SCHEMA_VERSION,
            train,
            validation,
            train_digest: train.digest(),
            validation_digest: validation.digest(),
        })
    }

    pub fn train_seeds(&self) -> Vec<u64> {
        self.train.seeds()
    }

    pub fn validation_seeds(&self) -> Vec<u64> {
        self.validation.seeds()
    }

    pub fn digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"tower-defense-ml-training-seed-schedule");
        hasher.update(self.schema_version.to_be_bytes());
        hasher.update(self.train_digest.as_bytes());
        hasher.update(self.validation_digest.as_bytes());
        hex_digest(hasher.finalize())
    }
}

fn hex_digest(digest: impl AsRef<[u8]>) -> String {
    digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inclusive_seed_ranges_materialize_in_order() {
        let range = SeedRange::try_new(0, 3).expect("valid range");

        assert_eq!(range.len(), 4);
        assert_eq!(range.seeds(), vec![0, 1, 2, 3]);
        assert!(range.contains(0));
        assert!(range.contains(3));
        assert!(!range.contains(4));
    }

    #[test]
    fn max_seed_range_does_not_overflow() {
        let range = SeedRange::try_new(u64::MAX - 2, u64::MAX).expect("valid range");

        assert_eq!(range.len(), 3);
        assert_eq!(range.seeds(), vec![u64::MAX - 2, u64::MAX - 1, u64::MAX]);
    }

    #[test]
    fn reversed_ranges_are_rejected() {
        assert_eq!(
            SeedRange::try_new(4, 3),
            Err(SeedRangeError::InvalidBounds {
                start: 4,
                end_inclusive: 3,
            })
        );
    }

    #[test]
    fn training_schedule_requires_disjoint_ranges_and_keeps_digests() {
        let train = SeedRange::try_new(0, 3).expect("valid train range");
        let validation = SeedRange::try_new(u64::MAX - 3, u64::MAX).expect("valid validation");
        let schedule = TrainingSeedSchedule::try_new(train, validation).expect("valid schedule");

        assert_eq!(schedule.train_seeds(), vec![0, 1, 2, 3]);
        assert_eq!(
            schedule.validation_seeds(),
            vec![u64::MAX - 3, u64::MAX - 2, u64::MAX - 1, u64::MAX]
        );
        assert_eq!(schedule.train_digest, train.digest());
        assert_eq!(schedule.validation_digest, validation.digest());
        assert_eq!(schedule.digest(), schedule.digest());
    }

    #[test]
    fn overlapping_training_and_validation_ranges_are_rejected() {
        let train = SeedRange::try_new(0, 3).expect("valid train range");
        let validation = SeedRange::try_new(3, 4).expect("valid validation range");

        assert!(matches!(
            TrainingSeedSchedule::try_new(train, validation),
            Err(SeedRangeError::OverlappingRanges { .. })
        ));
    }
}
