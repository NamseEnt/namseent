//! Policy v2 stage B: full placement space.
//!
//! A spatial family (PlaceTower, and optionally BuildTower) is chosen in two
//! steps below the family head: an *option* (a tower hand slot, or a
//! `(card subset, hand slot)` build pair), scored like any other candidate,
//! then a *cell* among every legal footprint position of that option. The
//! heuristic placement order enters only as cell features (percentile rank,
//! top-1, top-8), never as a filter, so the policy can pick any legal cell.

use super::features::{nearby_tower_occupancy, placement_coverage};
use crate::environment::{AgentAction, GameEnvironment, LegalAction, Observation};
use crate::joint_action::{DenseBuildTowerScoreTable, position_xy};
use crate::policy_runner::rank_place_tower_actions;
use serde::{Deserialize, Serialize};

pub const CELL_FEATURE_COUNT: usize = 8;
/// Cells whose heuristic rank is below this count as "heuristic top-k"
/// (the v1 candidate limit).
pub const HEURISTIC_TOP_K: usize = 8;

/// What a spatial option becomes once a cell is chosen.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpatialAction {
    Place {
        hand_slot_index: usize,
    },
    Build {
        card_ids: Vec<usize>,
        hand_slot_index: usize,
    },
}

impl SpatialAction {
    pub fn at(&self, left: usize, top: usize) -> AgentAction {
        match self {
            Self::Place { hand_slot_index } => AgentAction::PlaceTower {
                hand_slot_index: *hand_slot_index,
                left,
                top,
            },
            Self::Build {
                card_ids,
                hand_slot_index,
            } => AgentAction::BuildTower {
                card_ids: card_ids.clone(),
                hand_slot_index: *hand_slot_index,
                left,
                top,
            },
        }
    }

    /// The option an action belongs to, and its cell.
    pub fn of(action: &AgentAction) -> Option<(Self, usize, usize)> {
        match action {
            AgentAction::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => Some((
                Self::Place {
                    hand_slot_index: *hand_slot_index,
                },
                *left,
                *top,
            )),
            AgentAction::BuildTower {
                card_ids,
                hand_slot_index,
                left,
                top,
            } => {
                let mut card_ids = card_ids.clone();
                card_ids.sort_unstable();
                Some((
                    Self::Build {
                        card_ids,
                        hand_slot_index: *hand_slot_index,
                    },
                    *left,
                    *top,
                ))
            }
            _ => None,
        }
    }
}

/// Every legal cell of one option with its features, ordered by the
/// heuristic (best first).
#[derive(Clone, Debug, PartialEq)]
pub struct CellSet {
    pub positions: Vec<(u16, u16)>,
    /// `positions.len() x CELL_FEATURE_COUNT`, row-major.
    pub features: Vec<f32>,
}

impl CellSet {
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    pub fn index_of(&self, left: usize, top: usize) -> Option<usize> {
        self.positions
            .iter()
            .position(|(x, y)| *x as usize == left && *y as usize == top)
    }

    pub fn outside_heuristic_top_k(&self, index: usize) -> bool {
        self.features[index * CELL_FEATURE_COUNT + 2] == 0.0
    }

    /// Heuristic rank percentile of cell `index` (0 = best cell).
    pub fn rank_percentile(&self, index: usize) -> f32 {
        1.0 - self.features[index * CELL_FEATURE_COUNT]
    }

    /// Manhattan distance from the heuristic-best cell.
    pub fn distance_from_best(&self, index: usize) -> usize {
        let (best_x, best_y) = self.positions[0];
        let (x, y) = self.positions[index];
        (best_x.abs_diff(x) + best_y.abs_diff(y)) as usize
    }

    /// `ranked` = `(left, top, coverage, nearest_route_norm)` best first.
    fn from_ranked(observation: &Observation, ranked: &[(usize, usize, f32, f32)]) -> Self {
        let count = ranked.len();
        let mut positions = Vec::with_capacity(count);
        let mut features = Vec::with_capacity(count * CELL_FEATURE_COUNT);
        for (rank, (left, top, coverage, nearest_route)) in ranked.iter().enumerate() {
            positions.push((*left as u16, *top as u16));
            features.extend([
                1.0 - rank as f32 / (count.max(2) - 1) as f32,
                (rank == 0) as u8 as f32,
                (rank < HEURISTIC_TOP_K) as u8 as f32,
                *coverage,
                *nearest_route,
                nearby_tower_occupancy(observation, *left, *top),
                *left as f32 / observation.map_width.max(1) as f32,
                *top as f32 / observation.map_height.max(1) as f32,
            ]);
        }
        Self {
            positions,
            features,
        }
    }
}

fn nearest_route_norm(observation: &Observation, left: usize, top: usize) -> f32 {
    observation
        .route_coords
        .iter()
        .map(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
        .min()
        .map_or(1.0, |distance| {
            distance as f32 / (observation.map_width + observation.map_height).max(1) as f32
        })
}

/// Legal cells of placing hand slot `hand_slot_index`, in the canonical
/// placement order.
pub fn place_cells(
    observation: &Observation,
    legal: &[LegalAction],
    hand_slot_index: usize,
    range_raw: i64,
) -> CellSet {
    let slot_actions = legal
        .iter()
        .filter(|legal| {
            matches!(legal.action, AgentAction::PlaceTower { hand_slot_index: slot, .. } if slot == hand_slot_index)
        })
        .cloned()
        .collect::<Vec<_>>();
    let ranked = rank_place_tower_actions(observation, &slot_actions)
        .into_iter()
        .filter_map(|legal| match legal.action {
            AgentAction::PlaceTower { left, top, .. } => Some((
                left,
                top,
                placement_coverage(observation, left, top, range_raw),
                nearest_route_norm(observation, left, top),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    CellSet::from_ranked(observation, &ranked)
}

/// Legal cells of one build pair, in the dense build table's order.
pub fn build_cells(
    observation: &Observation,
    table: &DenseBuildTowerScoreTable,
    subset_index: usize,
    hand_slot_index: usize,
) -> CellSet {
    let mut scored = table.position_scores(subset_index, hand_slot_index);
    scored.sort_by(|(left_position, left), (right_position, right)| {
        right
            .ordering_key()
            .cmp(&left.ordering_key())
            .then_with(|| left_position.cmp(right_position))
    });
    let route_length = observation.route_coords.len().max(1) as f32;
    let span = (observation.map_width + observation.map_height).max(1) as f32;
    let ranked = scored
        .into_iter()
        .filter_map(|(position_index, score)| {
            let (left, top) = position_xy(position_index)?;
            Some((
                left,
                top,
                score.covered_route as f32 / route_length,
                score.nearest_route as f32 / span,
            ))
        })
        .collect::<Vec<_>>();
    CellSet::from_ranked(observation, &ranked)
}

/// The cells of a spatial option in the current state.
pub fn option_cells(
    environment: &GameEnvironment,
    observation: &Observation,
    table: Option<&DenseBuildTowerScoreTable>,
    option: &SpatialAction,
) -> Option<CellSet> {
    match option {
        SpatialAction::Place { hand_slot_index } => {
            let range_raw = observation.hand.iter().find_map(|item| {
                (item.index == *hand_slot_index)
                    .then_some(match &item.item {
                        crate::environment::HandItemObservation::Tower(tower) => {
                            Some(tower.range_raw)
                        }
                        crate::environment::HandItemObservation::Card(_) => None,
                    })
                    .flatten()
            })?;
            Some(place_cells(
                observation,
                &environment.semantic_non_build_actions(),
                *hand_slot_index,
                range_raw,
            ))
        }
        SpatialAction::Build {
            card_ids,
            hand_slot_index,
        } => {
            let table = table?;
            let subset_index = table.subsets.subset_index_for_card_ids(card_ids)?;
            Some(build_cells(
                observation,
                table,
                subset_index,
                *hand_slot_index,
            ))
        }
    }
}
