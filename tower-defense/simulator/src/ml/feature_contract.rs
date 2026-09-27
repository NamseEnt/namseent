//! Policy v2 input normalization contract.
//!
//! Every numeric input slot the actor and critic read - the global feature
//! vector, each typed entity set's numeric columns and the candidate rows'
//! numeric columns - is either kept as is or passed through the signed
//! `ln(1 + |x|)` squash. The table is derived once from the Phase 4B
//! canonical dataset (`feature_scale_survey`) and frozen here:
//!
//! - binary slots ({0, 1}) and already normalized slots (max |x| <= 2 on the
//!   canonical data: ratios, coordinates, bounded counts, log-scaled values)
//!   are kept;
//! - unbounded slots (max |x| > 2, e.g. card polish as `polish_pct_raw /
//!   1000`, tower damage as `damage_raw / 10000`) are squashed.

use super::encoding::observation::ENTITY_SET_COUNT;
use super::encoding::{ENTITY_NUMERIC_WIDTH, EntitySet};
use super::features::GLOBAL_FEATURE_COUNT;
use super::semantic_candidates::EncodedDecision;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transform {
    Keep,
    SignedLog1p,
}

pub fn signed_log1p(value: f32) -> f32 {
    value.signum() * value.abs().ln_1p()
}

impl Transform {
    pub fn apply(self, value: f32) -> f32 {
        match self {
            Self::Keep => value,
            Self::SignedLog1p => signed_log1p(value),
        }
    }
}

/// Which inputs a policy or critic reads.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    clap::ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum InputContract {
    /// Encoded features as produced (v1).
    #[default]
    Raw,
    /// Encoded features passed through the frozen contract table.
    Normalized,
}

/// Global feature slots squashed by the contract.
pub const SQUASHED_GLOBAL_SLOTS: [usize; 12] = [21, 29, 30, 56, 59, 60, 77, 78, 79, 80, 81, 82];
/// `(typed entity set, numeric column)` slots squashed by the contract.
pub const SQUASHED_ENTITY_SLOTS: [(usize, usize); 12] = [
    (0, 0),
    (1, 0),
    (2, 1),
    (3, 0),
    (4, 0),
    (5, 2),
    (5, 4),
    (5, 5),
    (6, 1),
    (6, 3),
    (9, 3),
    (9, 4),
];
/// Candidate numeric columns squashed by the contract.
pub const SQUASHED_CANDIDATE_COLUMNS: [usize; 2] = [0, 2];

fn squash_set(set: &mut EntitySet, columns: impl Fn(usize) -> bool) {
    for row in &mut set.rows {
        for (column, value) in row.numeric.iter_mut().enumerate() {
            if columns(column) {
                *value = signed_log1p(*value);
            }
        }
    }
}

/// `decision` with the contract applied (`Raw` returns it unchanged).
pub fn apply_contract(decision: &EncodedDecision, contract: InputContract) -> EncodedDecision {
    let mut decision = decision.clone();
    if contract == InputContract::Raw {
        return decision;
    }
    for slot in SQUASHED_GLOBAL_SLOTS {
        decision.global_features[slot] = signed_log1p(decision.global_features[slot]);
    }
    for (index, set) in decision.typed.sets.iter_mut().enumerate() {
        squash_set(set, |column| {
            SQUASHED_ENTITY_SLOTS.contains(&(index, column))
        });
    }
    for set in &mut decision.candidates {
        squash_set(set, |column| SQUASHED_CANDIDATE_COLUMNS.contains(&column));
    }
    decision
}

/// Slot statistics of one numeric input position.
#[derive(Clone, Copy, Debug, Default)]
pub struct SlotStats {
    pub count: usize,
    pub max_abs: f32,
    pub binary: bool,
}

impl SlotStats {
    fn add(&mut self, value: f32) {
        if self.count == 0 {
            self.binary = true;
        }
        self.count += 1;
        self.max_abs = self.max_abs.max(value.abs());
        self.binary &= value == 0.0 || value == 1.0;
    }

    pub fn transform(&self) -> Transform {
        if self.binary || self.max_abs <= 2.0 {
            Transform::Keep
        } else {
            Transform::SignedLog1p
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScaleSurvey {
    pub global: Vec<SlotStats>,
    pub entity: Vec<[SlotStats; ENTITY_NUMERIC_WIDTH]>,
    pub candidate: [SlotStats; ENTITY_NUMERIC_WIDTH],
}

fn add_set(stats: &mut [SlotStats; ENTITY_NUMERIC_WIDTH], set: &EntitySet) {
    for row in &set.rows {
        for (column, value) in row.numeric.iter().enumerate().take(ENTITY_NUMERIC_WIDTH) {
            stats[column].add(*value);
        }
    }
}

pub fn feature_scale_survey<'a>(
    decisions: impl Iterator<Item = &'a EncodedDecision>,
) -> ScaleSurvey {
    let mut survey = ScaleSurvey {
        global: vec![SlotStats::default(); GLOBAL_FEATURE_COUNT],
        entity: vec![[SlotStats::default(); ENTITY_NUMERIC_WIDTH]; ENTITY_SET_COUNT],
        candidate: [SlotStats::default(); ENTITY_NUMERIC_WIDTH],
    };
    for decision in decisions {
        for (index, value) in decision.global_features.iter().enumerate() {
            survey.global[index].add(*value);
        }
        for (index, set) in decision.typed.sets.iter().enumerate() {
            add_set(&mut survey.entity[index], set);
        }
        for set in &decision.candidates {
            add_set(&mut survey.candidate, set);
        }
    }
    survey
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::ml::phase4_dataset::load_episodes;
    use crate::ml::semantic_bc::{LabelSource, prepare_samples};
    use std::path::Path;

    #[test]
    fn contract_squashes_only_the_frozen_slots() {
        let config = std::sync::Arc::new(GameConfig::default_config());
        let environment = crate::environment::GameEnvironment::new(config, 4_000_001);
        let decision = crate::ml::semantic_bc::semantic_decision(&environment)
            .unwrap()
            .encoded;
        let mut probe = decision.clone();
        for value in &mut probe.global_features {
            *value = 1000.0;
        }
        let squashed = apply_contract(&probe, InputContract::Normalized);
        for (slot, value) in squashed.global_features.iter().enumerate() {
            let expected = if SQUASHED_GLOBAL_SLOTS.contains(&slot) {
                signed_log1p(1000.0)
            } else {
                1000.0
            };
            assert_eq!(*value, expected, "global slot {slot}");
        }
        assert_eq!(apply_contract(&decision, InputContract::Raw), decision);
        assert!(signed_log1p(-3.0) < 0.0 && signed_log1p(0.0) == 0.0);
    }

    #[test]
    #[ignore = "survey of the Phase 4B canonical dataset; run with -- --ignored --nocapture"]
    fn canonical_feature_scale_survey() {
        let directory =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/phase4b/canonical-train-2048");
        let mut episodes = load_episodes(&directory, &GameConfig::default_config()).unwrap();
        episodes.truncate(256);
        let samples = prepare_samples(&episodes, LabelSource::Canonical, 1.0);
        let survey = feature_scale_survey(samples.iter().map(|sample| &sample.encoded));
        eprintln!("decisions {}", samples.len());
        let describe = |stats: &SlotStats| {
            format!(
                "{:?} (max |x| {:.3}{})",
                stats.transform(),
                stats.max_abs,
                if stats.binary { ", binary" } else { "" }
            )
        };
        for (index, stats) in survey.global.iter().enumerate() {
            eprintln!("global {index:2}: {}", describe(stats));
        }
        for (set, columns) in survey.entity.iter().enumerate() {
            for (column, stats) in columns.iter().enumerate() {
                if stats.count > 0 {
                    eprintln!("entity set {set} column {column}: {}", describe(stats));
                }
            }
        }
        for (column, stats) in survey.candidate.iter().enumerate() {
            eprintln!("candidate column {column}: {}", describe(stats));
        }
    }
}
