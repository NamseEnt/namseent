//! Decomposed, vectorizable full-map tower-placement legality.
//!
//! `td_core`'s `TowerPlacementContext::can_place_at` is the authoritative
//! legality check and is not changed by this module. `GameEnvironment`
//! callers that need legality for every map position in one decision
//! (e.g. `joint_action::DenseBuildTowerScoreTable`) previously called
//! `can_place_at` once per position, each call re-running a fresh
//! BFS-based connectivity check. This module computes an equivalent
//! result while only running that BFS-based check for the small subset of
//! positions where a cheaper argument doesn't already settle it.
//!
//! `can_place_at` decomposes into independent conditions:
//! - terrain / bounds: whether the tower's 2x2 footprint stays inside the
//!   map. Guaranteed by construction here - every `(left, top)` this
//!   module is given comes from `joint_action::position_xy`, whose domain
//!   already excludes any footprint that would leave the map.
//! - occupancy: whether the footprint overlaps an existing tower's
//!   footprint (`Observation::tower_grid`).
//! - the fixed `TRAVEL_POINTS` cells, which can never host a tower.
//! - path connectivity: whether adding the footprint as a blocker still
//!   leaves every consecutive `TRAVEL_POINTS` pair connected. This is the
//!   only condition that needs a graph search, and the only one this
//!   module tries to avoid recomputing per position.
//!
//! None of these conditions depend on which card subset produced the
//! tower: `TowerPlacementContext::can_place_at` and
//! `can_place_tower` take no card/subset/template information at all, and
//! a tower's footprint size does not vary with its kind. Legality is
//! therefore computed once per decision and shared across every subset in
//! `joint_action::DenseBuildTowerScoreTable`, exactly as the map-level
//! coverage/route-distance grids already are.

use crate::environment::GameEnvironment;
use crate::joint_action::{MAP_POSITION_COUNT, position_index, position_xy};
use td_core::PlacementCheck;

/// Breakdown of how a [`full_map_legality_mask`] call resolved each
/// position, for benchmarking.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct LegalityMaskStats {
    /// Blocked by a travel point or existing tower - resolved without any
    /// connectivity check.
    pub locally_blocked: usize,
    /// Legal because the footprint avoids every cell the current witness
    /// route depends on - resolved without any connectivity check.
    pub fast_path_legal: usize,
    /// Resolved by the authoritative BFS-based connectivity check.
    pub connectivity_checked: usize,
}

impl LegalityMaskStats {
    pub fn total(&self) -> usize {
        self.locally_blocked + self.fast_path_legal + self.connectivity_checked
    }
}

/// Placement legality of every position in `0..joint_action::MAP_POSITION_COUNT`
/// for one set of occupied cells. Legality depends on nothing else, so one
/// prepared grid serves every consumer while the occupied cells are
/// unchanged.
pub struct PreparedPlacementLegality {
    occupied: Vec<[usize; 2]>,
    checks: Vec<PlacementCheck>,
}

impl PreparedPlacementLegality {
    pub(crate) fn compute(context: &td_core::TowerPlacementContext) -> Self {
        #[cfg(feature = "diagnostics")]
        td_core::diagnostics::record(|counters| counters.full_map_legality_mask_scans += 1);
        td_core::diag_scope!(PlacementScan);
        let checks = (0..MAP_POSITION_COUNT)
            .map(|index| {
                let (left, top) = position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
                context.check_placement(left, top)
            })
            .collect();
        Self {
            occupied: context.occupied().to_vec(),
            checks,
        }
    }

    pub(crate) fn matches(&self, context: &td_core::TowerPlacementContext) -> bool {
        self.occupied == context.occupied()
    }

    pub fn check(&self, left: usize, top: usize) -> PlacementCheck {
        position_index(left, top)
            .map(|index| self.checks[index])
            .unwrap_or(PlacementCheck::Invalid)
    }

    pub fn is_legal(&self, left: usize, top: usize) -> bool {
        self.check(left, top).is_legal()
    }

    pub fn checks(&self) -> &[PlacementCheck] {
        &self.checks
    }
}

/// Legality for every position in `0..joint_action::MAP_POSITION_COUNT`,
/// taken from the environment's prepared placement legality.
pub fn full_map_legality_mask(environment: &GameEnvironment) -> (Vec<bool>, LegalityMaskStats) {
    let prepared = environment.prepared_placement_legality();
    let mut stats = LegalityMaskStats::default();
    let mask = prepared
        .checks()
        .iter()
        .map(|&check| {
            match check {
                PlacementCheck::Invalid => stats.locally_blocked += 1,
                PlacementCheck::RouteCertified => stats.fast_path_legal += 1,
                PlacementCheck::Connected | PlacementCheck::Disconnected => {
                    stats.connectivity_checked += 1
                }
            }
            check.is_legal()
        })
        .collect();
    (mask, stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::AgentAction;
    use crate::joint_action::position_xy;
    use std::sync::Arc;

    fn environment(seed: u64) -> GameEnvironment {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
    }

    /// Exhaustively compares the fast mask against `can_place_at` for every
    /// map position (not a sample) across multiple seeds/decision states,
    /// including states with existing towers placed (so the occupancy and
    /// route-overlap fast path both get exercised, not just the initial
    /// empty-map state).
    #[test]
    #[ignore = "exhaustive oracle comparison; slow under debug (~3 min) because the oracle side \
        calls can_place_at's full BFS-based connectivity check for every one of 1225 positions \
        per state - run with cargo test --release -- --ignored \
        full_map_legality_mask_matches_can_place_at_for_every_position"]
    fn full_map_legality_mask_matches_can_place_at_for_every_position() {
        use crate::policy_runner::scripted_expert_action;

        const SEEDS: std::ops::Range<u64> = 0..6;
        const SAMPLES_PER_SEED: usize = 4;
        const MAX_DECISIONS_PER_SEED: usize = 30;

        let mut checked_states = 0usize;
        for seed in SEEDS {
            let mut environment = environment(seed);
            let mut samples_collected = 0usize;
            let mut decision_index = 0usize;
            while samples_collected < SAMPLES_PER_SEED && decision_index < MAX_DECISIONS_PER_SEED {
                decision_index += 1;
                let observation = environment.snapshot();
                let has_build_candidate = environment
                    .semantic_legal_actions_with_position_limit(Some(1))
                    .iter()
                    .any(|legal| matches!(legal.action, AgentAction::BuildTower { .. }));

                if has_build_candidate {
                    let (mask, stats) = full_map_legality_mask(&environment);
                    assert_eq!(mask.len(), MAP_POSITION_COUNT);
                    assert_eq!(stats.total(), MAP_POSITION_COUNT);
                    for (index, &mask_legal) in mask.iter().enumerate() {
                        let (left, top) =
                            position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
                        let oracle = environment.can_place_at(left, top);
                        assert_eq!(
                            mask_legal, oracle,
                            "seed {seed} decision {decision_index} position ({left},{top}): \
                            fast mask disagreed with can_place_at"
                        );
                    }
                    samples_collected += 1;
                    checked_states += 1;
                }

                let legal_actions =
                    environment.semantic_legal_actions_with_position_limit(Some(64));
                let action = scripted_expert_action(&observation, &legal_actions)
                    .expect("scripted expert should find an action");
                let outcome = environment
                    .semantic_step(action)
                    .expect("scripted step should be accepted");
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }

        let seed_count = (SEEDS.end - SEEDS.start) as usize;
        assert!(
            checked_states >= seed_count * 2,
            "expected to check a meaningful number of decision states, got {checked_states}"
        );
    }

    #[test]
    fn full_map_legality_mask_stats_are_internally_consistent() {
        let environment = environment(0);
        let (mask, stats) = full_map_legality_mask(&environment);

        assert_eq!(stats.total(), MAP_POSITION_COUNT);
        assert!(stats.fast_path_legal > 0);
        let legal_count = mask.iter().filter(|&&legal| legal).count();
        assert!(legal_count >= stats.fast_path_legal);
        assert!(legal_count <= stats.fast_path_legal + stats.connectivity_checked);
    }

    #[test]
    fn prepared_placement_legality_matches_fresh_computation_along_trajectories() {
        use crate::policy_runner::canonical_scripted_semantic_action;

        let mut checked = 0usize;
        for seed in 0..6u64 {
            let mut environment =
                GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
            while !matches!(
                environment.decision_point(),
                crate::environment::DecisionPoint::Terminal
            ) {
                let prepared = environment.prepared_placement_legality();
                let fresh =
                    PreparedPlacementLegality::compute(&environment.tower_placement_context());
                assert_eq!(prepared.checks(), fresh.checks(), "seed {seed}");
                checked += 1;
                let action = canonical_scripted_semantic_action(&environment).unwrap();
                let outcome = environment.semantic_step(action).unwrap();
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }
        assert!(checked > 300, "checked {checked}");
    }
}
