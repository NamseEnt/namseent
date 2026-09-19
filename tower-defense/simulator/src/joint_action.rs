//! Dense `(card_subset, position)` joint index space for `BuildTower`.
//!
//! Foundation for replacing per-`AgentAction` candidate enumeration and
//! scoring with vectorized, map-level computation (see
//! `docs/game-ai/11-candidate-architecture-review.md`, Option B). This module
//! only builds the indexing and dense scoring primitives; it does not change
//! `AgentAction`, the PPO/BC model, or candidate proposal/limit behavior.

use crate::environment::{
    AgentAction, BuildTowerCandidateObservation, GameEnvironment, HandItemObservation, Observation,
    RouteCoordObservation,
};
use crate::policy_runner::tower_range_raw;
use std::collections::HashMap;

/// Valid top-left corners for a tower's 2x2 footprint: `left, top` each range
/// over `0..MAP_SIZE[axis] - 1` (the footprint's bottom-right corner must
/// still be in bounds). This never depends on which cards produced the
/// tower - the footprint size is fixed.
pub const MAP_POSITION_WIDTH: usize = td_core::MAP_SIZE[0] - 1;
pub const MAP_POSITION_HEIGHT: usize = td_core::MAP_SIZE[1] - 1;
pub const MAP_POSITION_COUNT: usize = MAP_POSITION_WIDTH * MAP_POSITION_HEIGHT;

/// Maps a top-left footprint corner to a dense, row-major position index in
/// `0..MAP_POSITION_COUNT`. Returns `None` for any `(left, top)` whose
/// footprint would leave the map - out-of-map positions are never
/// representable as a `position_index`.
pub fn position_index(left: usize, top: usize) -> Option<usize> {
    if left < MAP_POSITION_WIDTH && top < MAP_POSITION_HEIGHT {
        Some(top * MAP_POSITION_WIDTH + left)
    } else {
        None
    }
}

/// Inverse of [`position_index`]. Every value in `0..MAP_POSITION_COUNT`
/// round-trips to a valid, in-map footprint corner.
pub fn position_xy(position_index: usize) -> Option<(usize, usize)> {
    if position_index < MAP_POSITION_COUNT {
        Some((
            position_index % MAP_POSITION_WIDTH,
            position_index / MAP_POSITION_WIDTH,
        ))
    } else {
        None
    }
}

/// The four map cells a tower's 2x2 footprint occupies when its top-left
/// corner is at `(left, top)`. Matches
/// `td_core::TowerPlacementContext`'s internal `placement_coords` cell
/// order and values exactly. Callers are expected to only pass `left, top`
/// from a valid `position_index` (see [`position_xy`]), which keeps every
/// returned cell within `MAP_SIZE` without an explicit bounds check here.
pub fn footprint_cells(left: usize, top: usize) -> [[usize; 2]; 4] {
    [
        [left, top],
        [left + 1, top],
        [left, top + 1],
        [left + 1, top + 1],
    ]
}

/// A stable, hand-slot-independent enumeration of a hand's non-empty card
/// subsets.
///
/// Bit `i` of a subset's 1-based mask refers to the `i`-th smallest
/// currently-held stable card id (`card_ids_sorted[i]`), never a hand slot
/// index. Rerolling or reordering the hand changes which cards exist, but
/// never changes how a given *set* of held card ids maps to a subset_index,
/// unlike the previous `subset_mask` used in
/// `environment::semantic_card_actions`, which was keyed by hand slot
/// position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardSubsetTable {
    card_ids_sorted: Vec<usize>,
}

impl CardSubsetTable {
    pub fn from_hand_card_ids(card_ids: impl IntoIterator<Item = usize>) -> Self {
        let mut card_ids_sorted: Vec<usize> = card_ids.into_iter().collect();
        card_ids_sorted.sort_unstable();
        card_ids_sorted.dedup();
        Self { card_ids_sorted }
    }

    pub fn from_observation(observation: &Observation) -> Self {
        Self::from_hand_card_ids(observation.hand.iter().filter_map(|slot| match &slot.item {
            HandItemObservation::Card(card) => Some(card.id),
            HandItemObservation::Tower(_) => None,
        }))
    }

    pub fn card_count(&self) -> usize {
        self.card_ids_sorted.len()
    }

    /// Number of non-empty subsets (`2^card_count - 1`), addressed as a
    /// dense, zero-based `subset_index` in `0..subset_count()`.
    pub fn subset_count(&self) -> usize {
        (1usize << self.card_ids_sorted.len()).saturating_sub(1)
    }

    /// The stable card ids selected by `subset_index` (`0..subset_count()`),
    /// in ascending card-id order. The last `subset_index` (all cards
    /// selected) returns an empty `Vec`, matching the existing "use the
    /// full hand" convention `environment::semantic_card_actions` and
    /// `AgentAction::BuildTower`'s `to_player_command` resolution already
    /// rely on for that case.
    pub fn card_ids_for_subset(&self, subset_index: usize) -> Option<Vec<usize>> {
        let subset_count = self.subset_count();
        if subset_index >= subset_count {
            return None;
        }
        let mask = subset_index + 1;
        if mask == subset_count {
            return Some(Vec::new());
        }
        Some(
            self.card_ids_sorted
                .iter()
                .enumerate()
                .filter_map(|(bit, &id)| (mask & (1usize << bit) != 0).then_some(id))
                .collect(),
        )
    }

    /// Inverse of [`Self::card_ids_for_subset`]: the `subset_index` for a
    /// given, order-independent set of card ids. An empty slice is treated
    /// as the canonical "full hand" sentinel, matching
    /// [`Self::card_ids_for_subset`]'s last-subset convention. Returns
    /// `None` if any id isn't currently held.
    pub fn subset_index_for_card_ids(&self, card_ids: &[usize]) -> Option<usize> {
        let subset_count = self.subset_count();
        if subset_count == 0 {
            return None;
        }
        if card_ids.is_empty() {
            return Some(subset_count - 1);
        }
        let mut mask = 0usize;
        for &id in card_ids {
            let bit = self
                .card_ids_sorted
                .iter()
                .position(|&candidate| candidate == id)?;
            mask |= 1usize << bit;
        }
        if mask == 0 || mask > subset_count {
            return None;
        }
        Some(mask - 1)
    }
}

/// Resolves a `(subset_index, hand_slot_index, position_index)` joint index
/// to the semantic `BuildTower` action it stands for. `hand_slot_index`
/// indexes the tower array `SelectTower` produces
/// (`tower_selection::start_placing_tower_from_template`): `0` is always
/// the selected card subset's own template; `1..build_slot_count` is
/// `stage_modifiers.extra_tower_cards`, one fixed (subset-independent)
/// template per entry, in that `Vec`'s order.
pub fn build_tower_action(
    subsets: &CardSubsetTable,
    subset_index: usize,
    hand_slot_index: usize,
    position_index: usize,
) -> Option<AgentAction> {
    let card_ids = subsets.card_ids_for_subset(subset_index)?;
    let (left, top) = position_xy(position_index)?;
    Some(AgentAction::BuildTower {
        card_ids,
        hand_slot_index,
        left,
        top,
    })
}

/// Inverse of [`build_tower_action`]. Returns `None` for any other action
/// shape, or if the action's card ids aren't a subset this table can
/// represent.
pub fn joint_index_for_action(
    subsets: &CardSubsetTable,
    action: &AgentAction,
) -> Option<(usize, usize, usize)> {
    let AgentAction::BuildTower {
        card_ids,
        hand_slot_index,
        left,
        top,
    } = action
    else {
        return None;
    };
    let subset_index = subsets.subset_index_for_card_ids(card_ids)?;
    let position_index = position_index(*left, *top)?;
    Some((subset_index, *hand_slot_index, position_index))
}

/// The resulting tower's scoring-relevant properties for one card subset,
/// looked up once per subset rather than recomputed per position.
struct SubsetTemplate {
    range_raw: i64,
    damage_raw: i64,
}

fn subset_templates(
    subsets: &CardSubsetTable,
    observation: &Observation,
) -> Vec<Option<SubsetTemplate>> {
    let mut by_card_ids: HashMap<Vec<usize>, &BuildTowerCandidateObservation> = HashMap::new();
    for candidate in &observation.build_tower_candidates {
        let mut key = candidate.card_ids.clone();
        key.sort_unstable();
        by_card_ids.insert(key, candidate);
    }
    (0..subsets.subset_count())
        .map(|subset_index| {
            let card_ids = subsets.card_ids_for_subset(subset_index)?;
            by_card_ids.get(&card_ids).map(|candidate| SubsetTemplate {
                range_raw: tower_range_raw(&candidate.template.kind),
                damage_raw: candidate.template.damage_raw,
            })
        })
        .collect()
}

/// The resulting tower's scoring-relevant properties for one
/// `stage_modifiers.extra_tower_cards` entry (`hand_slot_index >= 1`).
/// Unlike [`SubsetTemplate`], this is fixed regardless of which card
/// subset is selected - `start_placing_tower_from_template` builds these
/// towers with no `used_cards` - so index `i` (0-based) always means
/// `hand_slot_index == i + 1` for every `subset_index`.
struct ExtraSlotTemplate {
    range_raw: i64,
    damage_raw: i64,
}

fn extra_slot_templates(observation: &Observation) -> Vec<ExtraSlotTemplate> {
    observation
        .extra_tower_card_templates
        .iter()
        .map(|template| ExtraSlotTemplate {
            range_raw: tower_range_raw(&template.kind),
            damage_raw: template.damage_raw,
        })
        .collect()
}

/// Distance (in map cells, Manhattan) from every position to the nearest
/// route cell, computed once per decision instead of once per candidate.
/// Matches the distance metric `policy_runner::rank_build_tower_actions_by_heuristic`
/// uses for `nearest_route`.
pub fn nearest_route_grid(route_coords: &[RouteCoordObservation]) -> Vec<usize> {
    (0..MAP_POSITION_COUNT)
        .map(|index| {
            let (left, top) = position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
            route_coords
                .iter()
                .map(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
                .min()
                .unwrap_or(usize::MAX)
        })
        .collect()
}

/// Count of route cells within `range_raw` (world-unit radius, matching
/// `policy_runner::tower_range_raw`'s scale) of every position, for one
/// specific range value. There are at most 9 distinct range values (one per
/// poker-hand tower kind), so calling this once per distinct range and
/// reusing the result across every subset sharing that range is far cheaper
/// than the previous per-candidate route scan. Matches
/// `rank_build_tower_actions_by_heuristic`'s `covered_route` formula exactly.
pub fn coverage_grid(route_coords: &[RouteCoordObservation], range_raw: i64) -> Vec<usize> {
    (0..MAP_POSITION_COUNT)
        .map(|index| {
            let (left, top) = position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
            route_coords
                .iter()
                .filter(|coord| {
                    let dx = (coord.x as i64 - left as i64)
                        .saturating_mul(1_000_000)
                        .saturating_sub(500_000);
                    let dy = (coord.y as i64 - top as i64)
                        .saturating_mul(1_000_000)
                        .saturating_sub(500_000);
                    dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
                        <= range_raw.saturating_mul(range_raw)
                })
                .count()
        })
        .collect()
}

/// One `(subset_index, position_index)` joint candidate's score, matching
/// `policy_runner::BuildTowerHeuristicScore`'s fields without eagerly
/// materializing an `AgentAction`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JointBuildTowerScore {
    pub covered_route: usize,
    pub nearest_route: usize,
    pub damage_raw: i64,
}

impl JointBuildTowerScore {
    /// Same primary/secondary/tertiary ordering as
    /// `BuildTowerHeuristicScore::sort_key`, minus the `action_id` string
    /// tie-break (no `AgentAction` exists yet at this point). Ties are
    /// broken by `(subset_index, position_index)` ascending instead - see
    /// [`DenseBuildTowerScoreTable::best`].
    fn ordering_key(&self) -> (usize, std::cmp::Reverse<usize>, i64) {
        (
            self.covered_route,
            std::cmp::Reverse(self.nearest_route),
            self.damage_raw,
        )
    }
}

/// Dense `[subset_index][hand_slot_index][position_index]` score table for
/// `BuildTower`, stored as one flat, contiguous `Vec` (`(subset_index *
/// build_slot_count + hand_slot_index) * position_count + position_index`).
/// `hand_slot_index` ranges over every tower hand slot a `BuildTower`
/// selection can resolve to (see `GameEnvironment::build_tower_slot_count`):
/// `0` is the selected subset's own template; `1..build_slot_count` is
/// `stage_modifiers.extra_tower_cards`, fixed regardless of subset. `None`
/// entries are a position that fails `GameEnvironment::can_place_at` - the
/// same legality authority `environment::semantic_legal_positions` already
/// uses, just evaluated over every position instead of a proposal-limited
/// subset, and only once (subset- and slot-independent) rather than once
/// per subset.
pub struct DenseBuildTowerScoreTable {
    pub subsets: CardSubsetTable,
    pub position_count: usize,
    pub build_slot_count: usize,
    pub legality_stats: crate::legality::LegalityMaskStats,
    scores: Vec<Option<JointBuildTowerScore>>,
}

impl DenseBuildTowerScoreTable {
    pub fn compute(environment: &GameEnvironment, observation: &Observation) -> Self {
        let subsets = CardSubsetTable::from_observation(observation);
        let subset_count = subsets.subset_count();
        let templates = subset_templates(&subsets, observation);
        let extra_templates = extra_slot_templates(observation);
        let build_slot_count = extra_templates.len() + 1;

        let (legal, legality_stats) =
            crate::legality::full_map_legality_mask(environment, observation);

        let mut coverage_by_range: HashMap<i64, Vec<usize>> = HashMap::new();
        let nearest_route = nearest_route_grid(&observation.route_coords);
        for template in templates.iter().flatten() {
            coverage_by_range
                .entry(template.range_raw)
                .or_insert_with(|| coverage_grid(&observation.route_coords, template.range_raw));
        }
        for extra in &extra_templates {
            coverage_by_range
                .entry(extra.range_raw)
                .or_insert_with(|| coverage_grid(&observation.route_coords, extra.range_raw));
        }

        let row_stride = build_slot_count * MAP_POSITION_COUNT;
        let mut scores = vec![None; subset_count * row_stride];
        for (subset_index, template) in templates.iter().enumerate() {
            let subset_base = subset_index * row_stride;
            if let Some(template) = template {
                let coverage = &coverage_by_range[&template.range_raw];
                for position_index in 0..MAP_POSITION_COUNT {
                    if !legal[position_index] {
                        continue;
                    }
                    scores[subset_base + position_index] = Some(JointBuildTowerScore {
                        covered_route: coverage[position_index],
                        nearest_route: nearest_route[position_index],
                        damage_raw: template.damage_raw,
                    });
                }
            }
            // Every non-empty card subset is a legal `SelectTower` choice
            // (a poker-hand pattern always resolves, worst case to a
            // high-card template - see `tower_selection::
            // select_tower_build_template`'s final fallback), so unlike
            // slot 0 above, extra slots are never gated on this subset
            // having its own scored template.
            for (extra_offset, extra) in extra_templates.iter().enumerate() {
                let hand_slot_index = extra_offset + 1;
                let coverage = &coverage_by_range[&extra.range_raw];
                let slot_base = subset_base + hand_slot_index * MAP_POSITION_COUNT;
                for position_index in 0..MAP_POSITION_COUNT {
                    if !legal[position_index] {
                        continue;
                    }
                    scores[slot_base + position_index] = Some(JointBuildTowerScore {
                        covered_route: coverage[position_index],
                        nearest_route: nearest_route[position_index],
                        damage_raw: extra.damage_raw,
                    });
                }
            }
        }

        Self {
            subsets,
            position_count: MAP_POSITION_COUNT,
            build_slot_count,
            legality_stats,
            scores,
        }
    }

    fn flat_index(&self, subset_index: usize, hand_slot_index: usize, position_index: usize) -> usize {
        (subset_index * self.build_slot_count + hand_slot_index) * self.position_count
            + position_index
    }

    /// Inverse of [`Self::flat_index`].
    fn joint_index(&self, flat_index: usize) -> (usize, usize, usize) {
        let per_subset = self.build_slot_count * self.position_count;
        let subset_index = flat_index / per_subset;
        let remainder = flat_index % per_subset;
        (
            subset_index,
            remainder / self.position_count,
            remainder % self.position_count,
        )
    }

    pub fn score(
        &self,
        subset_index: usize,
        hand_slot_index: usize,
        position_index: usize,
    ) -> Option<JointBuildTowerScore> {
        self.scores
            .get(self.flat_index(subset_index, hand_slot_index, position_index))
            .copied()
            .flatten()
    }

    /// The best legal `(subset_index, hand_slot_index, position_index)` by
    /// [`JointBuildTowerScore::ordering_key`], ties broken by the smallest
    /// `(subset_index, hand_slot_index, position_index)`.
    pub fn best_index(&self) -> Option<(usize, usize, usize)> {
        self.scores
            .iter()
            .enumerate()
            .filter_map(|(flat_index, score)| score.map(|score| (flat_index, score)))
            .max_by_key(|(flat_index, score)| {
                let (subset_index, hand_slot_index, position_index) = self.joint_index(*flat_index);
                (
                    score.ordering_key(),
                    std::cmp::Reverse(subset_index),
                    std::cmp::Reverse(hand_slot_index),
                    std::cmp::Reverse(position_index),
                )
            })
            .map(|(flat_index, _)| self.joint_index(flat_index))
    }

    /// The best legal `BuildTower` action by [`Self::best_index`], or `None`
    /// if no legal candidate exists.
    pub fn best_action(&self) -> Option<AgentAction> {
        let (subset_index, hand_slot_index, position_index) = self.best_index()?;
        build_tower_action(&self.subsets, subset_index, hand_slot_index, position_index)
    }

    /// The `k` best legal `(subset_index, hand_slot_index, position_index)`
    /// triples, sorted descending by [`JointBuildTowerScore::ordering_key`]
    /// and, on ties, ascending by `(subset_index, hand_slot_index,
    /// position_index)` - the same tie-break [`Self::best_index`] uses, so
    /// `top_k_indices(1).first()` always equals `best_index()`.
    /// Deterministic regardless of hand slot order or any
    /// `AgentAction::action_id()` string: this never materializes an action
    /// to rank candidates.
    pub fn top_k_indices(&self, k: usize) -> Vec<(usize, usize, usize)> {
        if k == 0 {
            return Vec::new();
        }
        let mut scored: Vec<(usize, usize, usize, JointBuildTowerScore)> = self
            .scores
            .iter()
            .enumerate()
            .filter_map(|(flat_index, score)| {
                score.map(|score| {
                    let (subset_index, hand_slot_index, position_index) =
                        self.joint_index(flat_index);
                    (subset_index, hand_slot_index, position_index, score)
                })
            })
            .collect();
        scored.sort_unstable_by(|left, right| {
            right
                .3
                .ordering_key()
                .cmp(&left.3.ordering_key())
                .then_with(|| left.0.cmp(&right.0))
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| left.2.cmp(&right.2))
        });
        scored.truncate(k);
        scored
            .into_iter()
            .map(|(subset_index, hand_slot_index, position_index, _)| {
                (subset_index, hand_slot_index, position_index)
            })
            .collect()
    }

    /// [`Self::top_k_indices`], materialized to `AgentAction::BuildTower`.
    pub fn top_k_actions(&self, k: usize) -> Vec<AgentAction> {
        self.top_k_indices(k)
            .into_iter()
            .filter_map(|(subset_index, hand_slot_index, position_index)| {
                build_tower_action(&self.subsets, subset_index, hand_slot_index, position_index)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::{AgentAction, GameEnvironment};
    use crate::policy_runner::{rank_build_tower_actions_by_heuristic, scripted_expert_action};
    use std::sync::Arc;

    fn environment(seed: u64) -> GameEnvironment {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
    }

    #[test]
    fn position_index_round_trips_every_in_map_cell() {
        for index in 0..MAP_POSITION_COUNT {
            let (left, top) = position_xy(index).expect("index should be in range");
            assert_eq!(position_index(left, top), Some(index));
        }
        assert_eq!(position_xy(MAP_POSITION_COUNT), None);
        assert_eq!(position_index(MAP_POSITION_WIDTH, 0), None);
        assert_eq!(position_index(0, MAP_POSITION_HEIGHT), None);
    }

    #[test]
    fn card_subset_table_round_trips_every_subset() {
        let subsets = CardSubsetTable::from_hand_card_ids([11, 3, 42, 7, 1]);
        assert_eq!(subsets.subset_count(), 31);
        for subset_index in 0..subsets.subset_count() {
            let card_ids = subsets
                .card_ids_for_subset(subset_index)
                .expect("subset should exist");
            assert_eq!(
                subsets.subset_index_for_card_ids(&card_ids),
                Some(subset_index),
                "subset {subset_index} (card_ids {card_ids:?}) should round-trip"
            );
        }
        // Explicit full-hand id list resolves to the same canonical subset
        // as the empty-Vec sentinel.
        assert_eq!(
            subsets.subset_index_for_card_ids(&[1, 3, 7, 11, 42]),
            subsets.subset_index_for_card_ids(&[]),
        );
    }

    #[test]
    fn card_subset_table_rejects_unheld_card_ids() {
        let subsets = CardSubsetTable::from_hand_card_ids([1, 2, 3]);
        assert_eq!(subsets.subset_index_for_card_ids(&[999]), None);
        assert_eq!(subsets.subset_index_for_card_ids(&[1, 999]), None);
    }

    #[test]
    fn joint_index_round_trips_through_build_tower_action() {
        let environment = environment(0);
        let observation = environment.snapshot();
        let subsets = CardSubsetTable::from_observation(&observation);
        assert!(subsets.subset_count() > 0);
        let sample_positions = [0, 1, MAP_POSITION_COUNT / 2, MAP_POSITION_COUNT - 1];
        let sample_hand_slots = [0, 1, 3];
        for subset_index in 0..subsets.subset_count() {
            for &hand_slot_index in &sample_hand_slots {
                for &position_index in &sample_positions {
                    let action =
                        build_tower_action(&subsets, subset_index, hand_slot_index, position_index)
                            .expect("action should be materializable for any in-range index pair");
                    assert_eq!(
                        joint_index_for_action(&subsets, &action),
                        Some((subset_index, hand_slot_index, position_index)),
                        "round trip failed for subset {subset_index} hand_slot {hand_slot_index} \
                        position {position_index}"
                    );
                }
            }
        }
    }

    #[test]
    fn joint_index_for_action_rejects_non_build_tower_and_out_of_hand_actions() {
        let subsets = CardSubsetTable::from_hand_card_ids([1, 2, 3]);
        assert_eq!(
            joint_index_for_action(&subsets, &AgentAction::StartSelectingTower),
            None
        );
        assert_eq!(
            joint_index_for_action(
                &subsets,
                &AgentAction::BuildTower {
                    card_ids: vec![999],
                    hand_slot_index: 0,
                    left: 0,
                    top: 0,
                },
            ),
            None,
            "card ids not held by this hand are never representable"
        );
        assert_eq!(
            joint_index_for_action(
                &subsets,
                &AgentAction::BuildTower {
                    card_ids: vec![1],
                    hand_slot_index: 1,
                    left: 0,
                    top: 0,
                },
            ),
            Some((subsets.subset_index_for_card_ids(&[1]).unwrap(), 1, 0)),
            "hand_slot_index >= 1 (extra_tower_cards) is representable - the actual legal range is \
            bounded by DenseBuildTowerScoreTable::build_slot_count / GameEnvironment::build_tower_slot_count, \
            not by this index/action mapping"
        );
    }

    /// Compares the dense scorer against the existing exhaustive heuristic
    /// oracle (`rank_build_tower_actions_by_heuristic` over the full,
    /// unpruned legal candidate set) across many seeds and decision points.
    ///
    /// Tie-break policy: `rank_build_tower_actions_by_heuristic` breaks ties
    /// by `AgentAction::action_id()`, a string keyed on the action's
    /// `card_ids` in the order `environment::semantic_card_actions`
    /// generated them (hand-slot order) - exactly the hand-slot dependency
    /// this module is built to avoid, so its exact tie-break is not
    /// something the dense scorer (subset_index/position_index order,
    /// independent of hand slot) can or should reproduce byte-for-byte.
    /// Instead this test asserts the *scores* match exactly, which is the
    /// property that actually matters (both pick an equally-good candidate);
    /// it additionally asserts the dense scorer's pick is one of the
    /// exhaustive oracle's legal candidates with that same top score, so an
    /// on-tie disagreement can only be between two candidates the oracle
    /// itself considers equally best, never a worse one.
    #[test]
    #[ignore = "exhaustive oracle comparison; slow under debug (~15 min) because it \
        deliberately calls the full O(map) legality scan this module's callers will \
        eventually replace - run with cargo test --release -- --ignored \
        dense_scorer_matches_exhaustive_heuristic_oracle"]
    fn dense_scorer_matches_exhaustive_heuristic_oracle() {
        const SEEDS: std::ops::Range<u64> = 0..12;
        const SAMPLES_PER_SEED: usize = 6;
        const MAX_DECISIONS_PER_SEED: usize = 40;

        let mut checked_decisions = 0usize;
        for seed in SEEDS {
            let mut environment = environment(seed);
            let mut samples_collected = 0usize;
            let mut decision_index = 0usize;
            while samples_collected < SAMPLES_PER_SEED && decision_index < MAX_DECISIONS_PER_SEED {
                decision_index += 1;
                let observation = environment.snapshot();
                let oracle_build_actions = environment
                    .semantic_legal_actions()
                    .into_iter()
                    .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                    .collect::<Vec<_>>();

                if !oracle_build_actions.is_empty() {
                    let oracle_ranking =
                        rank_build_tower_actions_by_heuristic(&observation, &oracle_build_actions);
                    let oracle_best = oracle_ranking
                        .first()
                        .expect("non-empty oracle candidates should rank at least one");
                    let oracle_best_score = (
                        oracle_best.covered_route,
                        oracle_best.nearest_route,
                        oracle_best.damage_raw,
                    );

                    let table = DenseBuildTowerScoreTable::compute(&environment, &observation);
                    let (dense_subset_index, dense_hand_slot_index, dense_position_index) = table
                        .best_index()
                        .expect("dense table should also find a legal candidate");
                    let dense_score = table
                        .score(dense_subset_index, dense_hand_slot_index, dense_position_index)
                        .expect("best_index should always point at a scored entry");
                    let dense_best_score = (
                        dense_score.covered_route,
                        dense_score.nearest_route,
                        dense_score.damage_raw,
                    );

                    assert_eq!(
                        oracle_best_score, dense_best_score,
                        "seed {seed} decision {decision_index}: dense scorer's best score should equal the exhaustive oracle's"
                    );

                    let dense_best_action = build_tower_action(
                        &table.subsets,
                        dense_subset_index,
                        dense_hand_slot_index,
                        dense_position_index,
                    )
                    .expect("dense best index should materialize an action");
                    let AgentAction::BuildTower {
                        hand_slot_index: dense_hand_slot,
                        left: dense_left,
                        top: dense_top,
                        ..
                    } = dense_best_action
                    else {
                        unreachable!("build_tower_action always returns AgentAction::BuildTower");
                    };
                    let matches_an_oracle_tie = oracle_ranking.iter().any(|score| {
                        (score.covered_route, score.nearest_route, score.damage_raw)
                            == dense_best_score
                            && matches!(
                                &score.action,
                                AgentAction::BuildTower { hand_slot_index, left, top, .. }
                                    if *hand_slot_index == dense_hand_slot
                                        && *left == dense_left
                                        && *top == dense_top
                            )
                    });
                    assert!(
                        matches_an_oracle_tie,
                        "seed {seed} decision {decision_index}: dense scorer's pick \
                        (hand_slot_index={dense_hand_slot}, left={dense_left}, top={dense_top}) \
                        should be one of the oracle's top-scoring candidates"
                    );

                    // The dense scorer must never select an action the
                    // authoritative legality check rejects.
                    let mut probe = environment
                        .fork_for_rollout_seed(0)
                        .expect("fork should succeed");
                    probe
                        .semantic_step(dense_best_action)
                        .expect("dense scorer's best action should be accepted by the environment");

                    checked_decisions += 1;
                    samples_collected += 1;
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
            checked_decisions >= seed_count * 2,
            "expected to check a meaningful number of build decisions, got {checked_decisions}"
        );
    }

    #[derive(serde::Serialize)]
    struct ScoringBenchmarkSamplePoint {
        seed: u64,
        decision_index: usize,
        oracle_candidate_count: usize,
        old_legality_scan_seconds: f64,
        new_legality_mask_seconds: f64,
        legality_stats: crate::legality::LegalityMaskStats,
        exhaustive_heuristic_scoring_seconds: f64,
        dense_table_total_seconds: f64,
    }

    #[derive(serde::Serialize)]
    struct ScoringBenchmarkReport {
        methodology: String,
        sample_points: Vec<ScoringBenchmarkSamplePoint>,
        mean_old_legality_scan_seconds: f64,
        mean_new_legality_mask_seconds: f64,
        legality_speedup: f64,
        mean_exhaustive_heuristic_scoring_seconds: f64,
        mean_dense_table_total_seconds: f64,
        mean_old_total_pipeline_seconds: f64,
        mean_new_total_pipeline_seconds: f64,
        pipeline_speedup: f64,
    }

    /// Compares wall-clock cost, on the same decision states, between the
    /// pre-existing per-position/per-candidate approach and the new
    /// dense/vectorized one:
    /// - `old_legality_scan_seconds`: `GameEnvironment::can_place_at` called
    ///   once per map position (what `environment::semantic_legal_positions`
    ///   and `environment::tower_placement_actions` still do).
    /// - `new_legality_mask_seconds`: `legality::full_map_legality_mask`,
    ///   the fast-path-first replacement.
    /// - `exhaustive_heuristic_scoring_seconds`: the existing per-candidate
    ///   heuristic scoring (`rank_build_tower_actions_by_heuristic`, given
    ///   an already-generated oracle candidate list) - the per-candidate
    ///   route scan `joint_action`'s dense grids replace.
    /// - `dense_table_total_seconds`: `DenseBuildTowerScoreTable::compute`'s
    ///   total cost end to end (now using the new legality mask internally,
    ///   plus map-level grids and fill) - the new pipeline's full cost for
    ///   the same decision.
    ///
    /// `old_total_pipeline_seconds = old_legality_scan_seconds +
    /// exhaustive_heuristic_scoring_seconds` is the fair "old approach,
    /// same shape as the new one" comparison point for
    /// `dense_table_total_seconds` (`new_total_pipeline_seconds`).
    ///
    /// Run with: cargo test --release -- --ignored dense_scorer_vs_exhaustive_heuristic_benchmark --nocapture
    #[test]
    #[ignore = "manual release benchmark; writes artifacts/benchmarks/dense-scorer-throughput.json"]
    fn dense_scorer_vs_exhaustive_heuristic_benchmark() {
        use std::time::Instant;

        const SEEDS: std::ops::Range<u64> = 0..12;
        const SAMPLES_PER_SEED: usize = 6;
        const MAX_DECISIONS_PER_SEED: usize = 40;

        let mut sample_points = Vec::new();

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
                    let oracle_build_actions = environment
                        .semantic_legal_actions()
                        .into_iter()
                        .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                        .collect::<Vec<_>>();

                    let old_legality_start = Instant::now();
                    let _legal = (0..MAP_POSITION_COUNT)
                        .map(|index| {
                            let (left, top) =
                                position_xy(index).expect("index is in 0..MAP_POSITION_COUNT");
                            environment.can_place_at(left, top)
                        })
                        .collect::<Vec<_>>();
                    let old_legality_scan_seconds = old_legality_start.elapsed().as_secs_f64();

                    let new_legality_start = Instant::now();
                    let (_new_mask, legality_stats) =
                        crate::legality::full_map_legality_mask(&environment, &observation);
                    let new_legality_mask_seconds = new_legality_start.elapsed().as_secs_f64();

                    let heuristic_start = Instant::now();
                    let _ranking =
                        rank_build_tower_actions_by_heuristic(&observation, &oracle_build_actions);
                    let exhaustive_heuristic_scoring_seconds =
                        heuristic_start.elapsed().as_secs_f64();

                    let dense_start = Instant::now();
                    let _table = DenseBuildTowerScoreTable::compute(&environment, &observation);
                    let dense_table_total_seconds = dense_start.elapsed().as_secs_f64();

                    sample_points.push(ScoringBenchmarkSamplePoint {
                        seed,
                        decision_index,
                        oracle_candidate_count: oracle_build_actions.len(),
                        old_legality_scan_seconds,
                        new_legality_mask_seconds,
                        legality_stats,
                        exhaustive_heuristic_scoring_seconds,
                        dense_table_total_seconds,
                    });
                    samples_collected += 1;
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

        let count = sample_points.len().max(1) as f64;
        let mean_old_legality_scan_seconds = sample_points
            .iter()
            .map(|s| s.old_legality_scan_seconds)
            .sum::<f64>()
            / count;
        let mean_new_legality_mask_seconds = sample_points
            .iter()
            .map(|s| s.new_legality_mask_seconds)
            .sum::<f64>()
            / count;
        let mean_exhaustive_heuristic_scoring_seconds = sample_points
            .iter()
            .map(|s| s.exhaustive_heuristic_scoring_seconds)
            .sum::<f64>()
            / count;
        let mean_dense_table_total_seconds = sample_points
            .iter()
            .map(|s| s.dense_table_total_seconds)
            .sum::<f64>()
            / count;
        let mean_old_total_pipeline_seconds =
            mean_old_legality_scan_seconds + mean_exhaustive_heuristic_scoring_seconds;
        let mean_new_total_pipeline_seconds = mean_dense_table_total_seconds;
        let legality_speedup =
            mean_old_legality_scan_seconds / mean_new_legality_mask_seconds.max(f64::EPSILON);
        let pipeline_speedup =
            mean_old_total_pipeline_seconds / mean_new_total_pipeline_seconds.max(f64::EPSILON);

        println!(
            "sample_count={} mean_old_legality_scan_seconds={:.6} mean_new_legality_mask_seconds={:.6} \
            legality_speedup={:.2}x mean_exhaustive_heuristic_scoring_seconds={:.6} \
            mean_dense_table_total_seconds={:.6} mean_old_total_pipeline_seconds={:.6} \
            mean_new_total_pipeline_seconds={:.6} pipeline_speedup={:.2}x",
            sample_points.len(),
            mean_old_legality_scan_seconds,
            mean_new_legality_mask_seconds,
            legality_speedup,
            mean_exhaustive_heuristic_scoring_seconds,
            mean_dense_table_total_seconds,
            mean_old_total_pipeline_seconds,
            mean_new_total_pipeline_seconds,
            pipeline_speedup
        );

        let report = ScoringBenchmarkReport {
            methodology: "old_legality_scan_seconds times can_place_at called once per map \
                position (what semantic_legal_positions/tower_placement_actions still do). \
                new_legality_mask_seconds times legality::full_map_legality_mask, the fast-path- \
                first replacement wired into DenseBuildTowerScoreTable::compute. \
                exhaustive_heuristic_scoring_seconds times rank_build_tower_actions_by_heuristic \
                given an already-generated oracle candidate list - the per-candidate route scan \
                joint_action's dense grids replace. dense_table_total_seconds times \
                DenseBuildTowerScoreTable::compute end to end (now using the new legality mask \
                internally, plus map-level grids and fill). old/new_total_pipeline_seconds are \
                the fair same-shape comparison points."
                .to_string(),
            sample_points,
            mean_old_legality_scan_seconds,
            mean_new_legality_mask_seconds,
            legality_speedup,
            mean_exhaustive_heuristic_scoring_seconds,
            mean_dense_table_total_seconds,
            mean_old_total_pipeline_seconds,
            mean_new_total_pipeline_seconds,
            pipeline_speedup,
        };

        let json = serde_json::to_string_pretty(&report).expect("report should serialize");
        std::fs::create_dir_all("../artifacts/benchmarks")
            .expect("benchmark artifact directory should be creatable");
        std::fs::write(
            "../artifacts/benchmarks/dense-scorer-throughput.json",
            &json,
        )
        .expect("benchmark report should be written");
    }
}
