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

use crate::environment::{GameEnvironment, Observation};
use crate::joint_action::{MAP_POSITION_COUNT, footprint_cells, position_xy};

fn is_travel_point(x: usize, y: usize) -> bool {
    td_core::TRAVEL_POINTS.contains(&[x, y])
}

/// Whether the footprint at `(left, top)` is blocked by terrain/occupancy:
/// overlaps a fixed travel point or an existing tower's footprint. Does
/// not check path connectivity.
fn footprint_is_locally_blocked(
    tower_grid: &[Option<u64>],
    map_width: usize,
    left: usize,
    top: usize,
) -> bool {
    footprint_cells(left, top)
        .iter()
        .any(|&[x, y]| is_travel_point(x, y) || tower_grid[y * map_width + x].is_some())
}

/// Dense per-cell "is this cell on the current route" lookup, built once
/// per decision from `Observation::route_coords` instead of scanning the
/// route once per candidate footprint.
fn route_cell_grid(observation: &Observation) -> Vec<bool> {
    let mut grid = vec![false; observation.map_width * observation.map_height];
    for coord in &observation.route_coords {
        if coord.x < observation.map_width && coord.y < observation.map_height {
            grid[coord.y * observation.map_width + coord.x] = true;
        }
    }
    grid
}

fn footprint_overlaps_route(
    route_cell: &[bool],
    map_width: usize,
    left: usize,
    top: usize,
) -> bool {
    footprint_cells(left, top)
        .iter()
        .any(|&[x, y]| route_cell[y * map_width + x])
}

/// Breakdown of how a [`full_map_legality_mask`] call resolved each
/// position, for benchmarking.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct LegalityMaskStats {
    /// Blocked by a travel point or existing tower - resolved without any
    /// connectivity check.
    pub locally_blocked: usize,
    /// Legal because the footprint does not overlap the current route (see
    /// the fast-path argument on [`full_map_legality_mask`]) - resolved
    /// without any connectivity check.
    pub fast_path_legal: usize,
    /// Overlaps the current route, so the fast-path argument doesn't apply;
    /// resolved by falling back to `GameEnvironment::can_place_at`'s
    /// authoritative BFS-based connectivity check.
    pub connectivity_checked: usize,
}

impl LegalityMaskStats {
    pub fn total(&self) -> usize {
        self.locally_blocked + self.fast_path_legal + self.connectivity_checked
    }
}

/// Computes legality for every position in `0..joint_action::MAP_POSITION_COUNT`
/// for the current decision, matching `GameEnvironment::can_place_at`
/// exactly for every position, without calling it (and its BFS-based
/// connectivity check) for positions a cheaper argument already settles.
///
/// Fast-path correctness argument: `Observation::route_coords` is a path
/// through every consecutive `TRAVEL_POINTS` pair that avoids every
/// currently-occupied tower cell - it is the actual route the game is
/// using right now, computed the same way `can_place_at`'s connectivity
/// check is (same adjacency/diagonal-blocking rules). If a candidate
/// footprint shares no cell with that path, the same path still avoids
/// every blocker after adding the footprint as one, so it remains a
/// witness that connectivity holds; `can_place_at` would therefore also
/// return legal for that position. This only establishes sufficiency: a
/// footprint that *does* overlap the route may still be legal via a
/// different path the current route doesn't happen to take, so those
/// positions fall back to the authoritative check rather than being
/// assumed illegal.
pub fn full_map_legality_mask(
    environment: &GameEnvironment,
    observation: &Observation,
) -> (Vec<bool>, LegalityMaskStats) {
    #[cfg(feature = "diagnostics")]
    td_core::diagnostics::record(|counters| counters.full_map_legality_mask_scans += 1);
    let map_width = observation.map_width;
    let route_cell = route_cell_grid(observation);
    let mut stats = LegalityMaskStats::default();

    let mask = (0..MAP_POSITION_COUNT)
        .map(|index| {
            let (left, top) = position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
            if footprint_is_locally_blocked(&observation.tower_grid, map_width, left, top) {
                stats.locally_blocked += 1;
                return false;
            }
            if !footprint_overlaps_route(&route_cell, map_width, left, top) {
                stats.fast_path_legal += 1;
                return true;
            }
            stats.connectivity_checked += 1;
            environment.can_place_at(left, top)
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
                    let (mask, stats) = full_map_legality_mask(&environment, &observation);
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
        let observation = environment.snapshot();
        let route_cell = route_cell_grid(&observation);
        let (mask, stats) = full_map_legality_mask(&environment, &observation);

        assert_eq!(stats.total(), MAP_POSITION_COUNT);

        let mut locally_blocked = 0usize;
        let mut fast_path_legal = 0usize;
        let mut connectivity_checked = 0usize;
        for (index, &mask_legal) in mask.iter().enumerate() {
            let (left, top) = position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
            if footprint_is_locally_blocked(
                &observation.tower_grid,
                observation.map_width,
                left,
                top,
            ) {
                locally_blocked += 1;
                assert!(!mask_legal, "locally blocked position should be illegal");
            } else if !footprint_overlaps_route(&route_cell, observation.map_width, left, top) {
                fast_path_legal += 1;
                assert!(mask_legal, "fast-path position should be legal");
            } else {
                connectivity_checked += 1;
            }
        }
        assert_eq!(stats.locally_blocked, locally_blocked);
        assert_eq!(stats.fast_path_legal, fast_path_legal);
        assert_eq!(stats.connectivity_checked, connectivity_checked);
    }
}
