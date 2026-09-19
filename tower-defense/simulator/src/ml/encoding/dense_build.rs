//! Dense, model-facing `BuildTower`/`PlaceTower` feature bundle.
//!
//! This is a new feature representation for a future dense policy head. It
//! is not wired into the existing PPO/BC model, dataset, or trajectory path:
//! it only turns an [`Observation`] into structured, per-`(subset,
//! hand_slot, position)` features, reusing the same map-level primitives
//! (`joint_action::nearest_route_grid`, `joint_action::coverage_grid`) the
//! dense `BuildTower` scorer already uses instead of any per-candidate route
//! scan. Legality is computed and owned elsewhere (`PolicyActionSpace`,
//! `legality::full_map_legality_mask`); this module never filters or masks
//! anything itself.

use super::entity::{CATEGORICAL_FIELDS, EntityRow};
use crate::environment::{
    HandItemObservation, Observation, RouteCoordObservation, TowerTemplateObservation,
};
use crate::joint_action::{
    CardSubsetTable, MAP_POSITION_COUNT, MAP_POSITION_HEIGHT, MAP_POSITION_WIDTH, coverage_grid,
    nearest_route_grid, position_xy,
};
use crate::ml::features::nearby_tower_occupancy;
use crate::ml::vocabulary::{rank_id, suit_id};
use std::collections::HashMap;

/// `[x_norm, y_norm, nearest_route_norm, occupancy_ratio]`.
pub const POSITION_FEATURE_WIDTH: usize = 4;

/// Per-position spatial features, computed once over the whole map instead
/// of once per `(subset, position)` candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct PositionFeatureTable {
    position_count: usize,
    nearest_route: Vec<usize>,
    rows: Vec<[f32; POSITION_FEATURE_WIDTH]>,
}

impl PositionFeatureTable {
    pub fn compute(observation: &Observation) -> Self {
        let nearest_route = nearest_route_grid(&observation.route_coords);
        let route_distance_norm = (MAP_POSITION_WIDTH + MAP_POSITION_HEIGHT).max(1) as f32;
        let rows = (0..MAP_POSITION_COUNT)
            .map(|position_index| {
                let (left, top) = position_xy(position_index)
                    .expect("position_index is in 0..MAP_POSITION_COUNT");
                let x_norm = left as f32 / MAP_POSITION_WIDTH.max(1) as f32;
                let y_norm = top as f32 / MAP_POSITION_HEIGHT.max(1) as f32;
                let nearest_route_norm =
                    (nearest_route[position_index] as f32 / route_distance_norm).min(1.0);
                let occupancy_ratio = nearby_tower_occupancy(observation, left, top);
                [x_norm, y_norm, nearest_route_norm, occupancy_ratio]
            })
            .collect();
        Self {
            position_count: MAP_POSITION_COUNT,
            nearest_route,
            rows,
        }
    }

    pub fn position_count(&self) -> usize {
        self.position_count
    }

    pub fn row(&self, position_index: usize) -> Option<&[f32; POSITION_FEATURE_WIDTH]> {
        self.rows.get(position_index)
    }

    pub fn nearest_route_raw(&self, position_index: usize) -> Option<usize> {
        self.nearest_route.get(position_index).copied()
    }
}

/// Shared full-map coverage grid per distinct tower range, computed once per
/// distinct `range_raw` value rather than once per `(subset, position)`
/// candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct RangeCoverageTable {
    distinct_ranges: Vec<i64>,
    grids: Vec<Vec<usize>>,
}

impl RangeCoverageTable {
    pub fn compute(
        route_coords: &[RouteCoordObservation],
        ranges: impl IntoIterator<Item = i64>,
    ) -> Self {
        let mut distinct_ranges: Vec<i64> = ranges.into_iter().collect();
        distinct_ranges.sort_unstable();
        distinct_ranges.dedup();
        let grids = distinct_ranges
            .iter()
            .map(|&range_raw| coverage_grid(route_coords, range_raw))
            .collect();
        Self {
            distinct_ranges,
            grids,
        }
    }

    pub fn distinct_range_count(&self) -> usize {
        self.distinct_ranges.len()
    }

    pub fn distinct_ranges(&self) -> &[i64] {
        &self.distinct_ranges
    }

    pub fn range_row_index(&self, range_raw: i64) -> Option<usize> {
        self.distinct_ranges.binary_search(&range_raw).ok()
    }

    pub fn coverage(&self, range_row: usize, position_index: usize) -> Option<usize> {
        self.grids.get(range_row)?.get(position_index).copied()
    }
}

/// Categorical field count of a dense template row: `[kind_id, suit_id,
/// rank_id, used_cards.len()]`, same layout as
/// `ml::encoding::observation::tower_row`.
pub const BUILD_TEMPLATE_CATEGORICAL_FIELDS: usize = CATEGORICAL_FIELDS;
/// Numeric field count of a dense template row: `[rerolled_count, damage_raw,
/// range_raw, shoot_interval_ticks]`.
pub const BUILD_TEMPLATE_NUMERIC_WIDTH: usize = 4;

fn template_feature_row(template: &TowerTemplateObservation) -> EntityRow {
    EntityRow::new(
        [
            template.kind_id as u32,
            template
                .suit
                .as_deref()
                .map_or(0, |value| suit_id(value) as u32),
            template
                .rank
                .as_deref()
                .map_or(0, |value| rank_id(value) as u32),
            template.used_cards.len() as u32,
        ],
        vec![
            template.rerolled_count as f32 / 20.0,
            template.damage_raw as f32 / 10_000.0,
            template.range_raw as f32 / 100_000.0,
            template.shoot_interval_ticks as f32 / 600.0,
        ],
    )
}

/// Shape/size summary of a [`DenseBuildFeatureBundle`], for building model
/// input tensors without recomputing the bundle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenseBuildFeatureShape {
    pub subset_count: usize,
    pub build_slot_count: usize,
    pub position_count: usize,
    pub template_categorical_fields: usize,
    pub template_numeric_width: usize,
    pub position_feature_width: usize,
    pub distinct_range_count: usize,
}

/// Dense `BuildTower` feature bundle: every `(subset_index,
/// hand_slot_index)` template row plus the shared position/coverage tables
/// they're scored against. `&Observation` is the only input - no
/// `GameEnvironment`, private simulator state, RNG, or search cache.
#[derive(Clone, Debug, PartialEq)]
pub struct DenseBuildFeatureBundle {
    pub subsets: CardSubsetTable,
    pub build_slot_count: usize,
    pub position: PositionFeatureTable,
    pub coverage: RangeCoverageTable,
    template_rows: Vec<EntityRow>,
    template_range_row: Vec<Option<usize>>,
}

impl DenseBuildFeatureBundle {
    pub fn compute(observation: &Observation) -> Self {
        let subsets = CardSubsetTable::from_observation(observation);
        let subset_count = subsets.subset_count();
        let extra_templates = &observation.extra_tower_card_templates;
        let build_slot_count = extra_templates.len() + 1;

        let mut by_card_ids: HashMap<Vec<usize>, &TowerTemplateObservation> = HashMap::new();
        for candidate in &observation.build_tower_candidates {
            let mut key = candidate.card_ids.clone();
            key.sort_unstable();
            by_card_ids.insert(key, &candidate.template);
        }

        let mut template_rows = Vec::with_capacity(subset_count * build_slot_count);
        let mut all_ranges = Vec::with_capacity(subset_count * build_slot_count);
        let mut per_template_range = Vec::with_capacity(subset_count * build_slot_count);

        for subset_index in 0..subset_count {
            let card_ids = subsets
                .card_ids_for_subset(subset_index)
                .expect("subset_index is in 0..subset_count");
            let template = *by_card_ids
                .get(&card_ids)
                .expect("every non-empty card subset should have a build candidate");
            template_rows.push(template_feature_row(template));
            per_template_range.push(template.range_raw);
            all_ranges.push(template.range_raw);

            for extra in extra_templates {
                template_rows.push(template_feature_row(extra));
                per_template_range.push(extra.range_raw);
                all_ranges.push(extra.range_raw);
            }
        }

        let coverage = RangeCoverageTable::compute(&observation.route_coords, all_ranges);
        let template_range_row = per_template_range
            .into_iter()
            .map(|range_raw| coverage.range_row_index(range_raw))
            .collect();

        Self {
            subsets,
            build_slot_count,
            position: PositionFeatureTable::compute(observation),
            coverage,
            template_rows,
            template_range_row,
        }
    }

    pub fn shape(&self) -> DenseBuildFeatureShape {
        DenseBuildFeatureShape {
            subset_count: self.subsets.subset_count(),
            build_slot_count: self.build_slot_count,
            position_count: self.position.position_count(),
            template_categorical_fields: BUILD_TEMPLATE_CATEGORICAL_FIELDS,
            template_numeric_width: BUILD_TEMPLATE_NUMERIC_WIDTH,
            position_feature_width: POSITION_FEATURE_WIDTH,
            distinct_range_count: self.coverage.distinct_range_count(),
        }
    }

    fn flat_index(&self, subset_index: usize, hand_slot_index: usize) -> Option<usize> {
        if hand_slot_index >= self.build_slot_count {
            return None;
        }
        subset_index
            .checked_mul(self.build_slot_count)?
            .checked_add(hand_slot_index)
    }

    pub fn template_row(&self, subset_index: usize, hand_slot_index: usize) -> Option<&EntityRow> {
        self.template_rows
            .get(self.flat_index(subset_index, hand_slot_index)?)
    }

    pub fn coverage_at(
        &self,
        subset_index: usize,
        hand_slot_index: usize,
        position_index: usize,
    ) -> Option<usize> {
        let flat_index = self.flat_index(subset_index, hand_slot_index)?;
        let range_row = (*self.template_range_row.get(flat_index)?)?;
        self.coverage.coverage(range_row, position_index)
    }
}

/// Dense `PlaceTower` feature bundle for a hand slot already holding a
/// resulting tower (`stage_modifiers.extra_tower_cards` overflow). Reuses
/// the exact same [`PositionFeatureTable`]/[`RangeCoverageTable`]/template
/// row semantics as [`DenseBuildFeatureBundle`] - `PlaceTower` is not a
/// separate spatial formula.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceTowerFeatureBundle {
    pub position: PositionFeatureTable,
    pub coverage: RangeCoverageTable,
    template_row: EntityRow,
}

impl PlaceTowerFeatureBundle {
    pub fn compute(observation: &Observation, hand_slot_index: usize) -> Option<Self> {
        let item = observation.hand.get(hand_slot_index)?;
        let HandItemObservation::Tower(template) = &item.item else {
            return None;
        };
        let position = PositionFeatureTable::compute(observation);
        let coverage = RangeCoverageTable::compute(&observation.route_coords, [template.range_raw]);
        let template_row = template_feature_row(template);
        Some(Self {
            position,
            coverage,
            template_row,
        })
    }

    pub fn template_row(&self) -> &EntityRow {
        &self.template_row
    }

    pub fn coverage_at(&self, position_index: usize) -> Option<usize> {
        self.coverage.coverage(0, position_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::{AgentAction, GameEnvironment};
    use std::sync::Arc;

    fn environment(seed: u64) -> GameEnvironment {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
    }

    #[test]
    fn compute_is_deterministic() {
        let environment = environment(0);
        let observation = environment.snapshot();
        let first = DenseBuildFeatureBundle::compute(&observation);
        let second = DenseBuildFeatureBundle::compute(&observation);
        assert_eq!(first, second);
    }

    /// Same card ID set, different physical hand slot order, must resolve
    /// to the same subset identity with the same template row/range mapping
    /// and coverage.
    #[test]
    fn subset_identity_is_stable_across_hand_reorder() {
        let original = environment(0);
        let reordered = environment(0);
        // Reverse the card hand slots in-place by swapping the underlying
        // slot order via repeated select/deselect is not available at this
        // layer, so instead assert identity using the observation's own
        // hand as-is plus a synthetic reversed-order observation.
        let observation = original.snapshot();
        let mut reordered_observation = reordered.snapshot();
        let mut card_slots: Vec<_> = reordered_observation
            .hand
            .iter()
            .cloned()
            .filter(|item| matches!(item.item, HandItemObservation::Card(_)))
            .collect();
        card_slots.reverse();
        let mut reversed_hand = reordered_observation.hand.clone();
        let mut card_iter = card_slots.into_iter();
        for slot in &mut reversed_hand {
            if matches!(slot.item, HandItemObservation::Card(_))
                && let Some(replacement) = card_iter.next()
            {
                slot.item = replacement.item;
            }
        }
        reordered_observation.hand = reversed_hand;

        let original_bundle = DenseBuildFeatureBundle::compute(&observation);
        let reordered_bundle = DenseBuildFeatureBundle::compute(&reordered_observation);

        let subsets = CardSubsetTable::from_observation(&observation);
        for subset_index in 0..subsets.subset_count() {
            let card_ids = subsets.card_ids_for_subset(subset_index).unwrap();
            let reordered_subset_index = CardSubsetTable::from_observation(&reordered_observation)
                .subset_index_for_card_ids(&card_ids)
                .expect("same card id set should resolve to a subset in the reordered hand");
            assert_eq!(
                original_bundle.template_row(subset_index, 0),
                reordered_bundle.template_row(reordered_subset_index, 0),
                "template row should be identical for the same card id subset"
            );
            assert_eq!(
                original_bundle.coverage_at(subset_index, 0, 0),
                reordered_bundle.coverage_at(reordered_subset_index, 0, 0),
                "coverage should be identical for the same card id subset"
            );
        }
    }

    #[test]
    fn distinct_ranges_are_ascending_deduped_and_stable() {
        let environment = environment(0);
        let observation = environment.snapshot();
        let bundle = DenseBuildFeatureBundle::compute(&observation);
        let ranges = bundle.coverage.distinct_ranges();
        let mut sorted = ranges.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(ranges, sorted.as_slice());
    }

    #[test]
    fn nearest_route_matches_grid_for_every_position() {
        let environment = environment(0);
        let observation = environment.snapshot();
        let position = PositionFeatureTable::compute(&observation);
        let grid = nearest_route_grid(&observation.route_coords);
        for position_index in 0..MAP_POSITION_COUNT {
            assert_eq!(
                position.nearest_route_raw(position_index),
                Some(grid[position_index])
            );
        }
    }

    #[test]
    fn coverage_matches_grid_for_every_range_and_position() {
        let environment = environment(0);
        let observation = environment.snapshot();
        let bundle = DenseBuildFeatureBundle::compute(&observation);
        for (range_row, &range_raw) in bundle.coverage.distinct_ranges().iter().enumerate() {
            let grid = coverage_grid(&observation.route_coords, range_raw);
            for position_index in 0..MAP_POSITION_COUNT {
                assert_eq!(
                    bundle.coverage.coverage(range_row, position_index),
                    Some(grid[position_index])
                );
            }
        }
    }

    #[test]
    fn template_row_matches_observation_for_every_subset_and_slot() {
        let environment = environment(0);
        let observation = environment.snapshot();
        let bundle = DenseBuildFeatureBundle::compute(&observation);
        let subsets = CardSubsetTable::from_observation(&observation);

        let mut by_card_ids: HashMap<Vec<usize>, &TowerTemplateObservation> = HashMap::new();
        for candidate in &observation.build_tower_candidates {
            let mut key = candidate.card_ids.clone();
            key.sort_unstable();
            by_card_ids.insert(key, &candidate.template);
        }

        for subset_index in 0..subsets.subset_count() {
            let card_ids = subsets.card_ids_for_subset(subset_index).unwrap();
            let expected = by_card_ids.get(&card_ids).expect("candidate should exist");
            let row = bundle
                .template_row(subset_index, 0)
                .expect("slot 0 row should exist");
            assert_eq!(row.categorical[0], expected.kind_id as u32);
            assert_eq!(row.numeric[0], expected.rerolled_count as f32 / 20.0);
            assert_eq!(row.numeric[1], expected.damage_raw as f32 / 10_000.0);
            assert_eq!(row.numeric[2], expected.range_raw as f32 / 100_000.0);
            assert_eq!(row.numeric[3], expected.shoot_interval_ticks as f32 / 600.0);

            for (extra_offset, extra) in observation.extra_tower_card_templates.iter().enumerate() {
                let hand_slot_index = extra_offset + 1;
                let extra_row = bundle
                    .template_row(subset_index, hand_slot_index)
                    .expect("extra slot row should exist");
                assert_eq!(extra_row.categorical[0], extra.kind_id as u32);
                assert_eq!(extra_row.numeric[2], extra.range_raw as f32 / 100_000.0);
            }
        }
    }

    #[test]
    fn extra_tower_card_fixture_one_extra_slot() {
        let mut environment = environment(0);
        environment
            .test_only_seed_extra_tower_cards(1)
            .expect("seeding extra tower cards should succeed");
        let observation = environment.snapshot();
        let bundle = DenseBuildFeatureBundle::compute(&observation);
        assert_eq!(bundle.build_slot_count, 2);
        let subsets = CardSubsetTable::from_observation(&observation);
        for subset_index in 0..subsets.subset_count() {
            assert!(bundle.template_row(subset_index, 0).is_some());
            assert!(bundle.template_row(subset_index, 1).is_some());
            assert!(bundle.template_row(subset_index, 2).is_none());
        }
    }

    #[test]
    fn extra_tower_card_fixture_two_extra_slots() {
        let mut environment = environment(0);
        environment
            .test_only_seed_extra_tower_cards(2)
            .expect("seeding extra tower cards should succeed");
        let observation = environment.snapshot();
        let bundle = DenseBuildFeatureBundle::compute(&observation);
        assert_eq!(bundle.build_slot_count, 3);
        let subsets = CardSubsetTable::from_observation(&observation);
        for subset_index in 0..subsets.subset_count() {
            assert!(bundle.template_row(subset_index, 0).is_some());
            assert!(bundle.template_row(subset_index, 1).is_some());
            assert!(bundle.template_row(subset_index, 2).is_some());
            assert!(bundle.template_row(subset_index, 3).is_none());
        }
    }

    #[test]
    fn place_tower_bundle_matches_dense_build_semantics() {
        let mut environment = environment(0);
        environment
            .test_only_seed_extra_tower_cards(1)
            .expect("seeding extra tower cards should succeed");
        let build_action = environment
            .semantic_legal_actions()
            .into_iter()
            .map(|legal| legal.action)
            .find(|action| matches!(action, AgentAction::BuildTower { hand_slot_index, .. } if *hand_slot_index == 0))
            .expect("a legal BuildTower action for hand_slot_index 0 should exist");
        let outcome = environment
            .semantic_step(build_action)
            .expect("build tower should be legal");
        assert!(!outcome.terminated);
        let placement_observation = environment.snapshot();
        assert_eq!(
            placement_observation.decision_point,
            crate::environment::DecisionPoint::TowerPlacement
        );
        let remaining_tower_slot = placement_observation
            .hand
            .iter()
            .find(|item| matches!(item.item, HandItemObservation::Tower(_)))
            .expect("a tower hand slot should remain for PlaceTower")
            .index;

        let place_bundle =
            PlaceTowerFeatureBundle::compute(&placement_observation, remaining_tower_slot)
                .expect("place tower bundle should compute for a tower hand slot");
        let HandItemObservation::Tower(hand_template) =
            &placement_observation.hand[remaining_tower_slot].item
        else {
            panic!("expected tower hand item");
        };
        assert_eq!(
            place_bundle.template_row().numeric[2],
            hand_template.range_raw as f32 / 100_000.0
        );
        assert_eq!(
            place_bundle.template_row().numeric[3],
            hand_template.shoot_interval_ticks as f32 / 600.0
        );

        let dense_position = PositionFeatureTable::compute(&placement_observation);
        for position_index in [0usize, MAP_POSITION_COUNT / 2, MAP_POSITION_COUNT - 1] {
            assert_eq!(
                place_bundle.position.row(position_index),
                dense_position.row(position_index)
            );
            let expected_coverage =
                coverage_grid(&placement_observation.route_coords, hand_template.range_raw)
                    [position_index];
            assert_eq!(
                place_bundle.coverage_at(position_index),
                Some(expected_coverage)
            );
        }
    }

    #[derive(serde::Serialize)]
    struct DenseBuildBenchmarkSamplePoint {
        seed: u64,
        decision_index: usize,
        legacy_build_tower_candidate_count: usize,
        position_feature_row_count: usize,
        distinct_coverage_range_count: usize,
        dense_compute_seconds: f64,
        legacy_seconds: f64,
    }

    #[derive(serde::Serialize)]
    struct DenseBuildBenchmarkReport {
        methodology: String,
        sample_points: Vec<DenseBuildBenchmarkSamplePoint>,
        mean_dense_compute_seconds: f64,
        mean_legacy_seconds: f64,
        mean_position_feature_row_count: f64,
        mean_distinct_coverage_range_count: f64,
        mean_legacy_build_tower_candidate_count: f64,
    }

    /// Compares `DenseBuildFeatureBundle::compute` (one call per decision,
    /// covering every `(subset, hand_slot, position)` triple) against the
    /// legacy path of materializing every legal `BuildTower` `AgentAction`
    /// (`semantic_legal_actions()`) and calling `ml::features::
    /// candidate_features` once per candidate (each of which does its own
    /// per-candidate route scan via `placement_coverage`/`route_distance`).
    /// This isn't chasing an absolute speed target - it exists to catch a
    /// regression back to per-candidate `AgentAction` materialization plus
    /// per-action route scans, the exact structure `joint_action`'s dense
    /// scorer and this bundle both replace.
    ///
    /// Run with: cargo test --release -- --ignored dense_build_features_benchmark --nocapture
    #[test]
    #[ignore = "manual release benchmark; writes artifacts/benchmarks/dense-build-features-benchmark.json"]
    fn dense_build_features_benchmark() {
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
                let build_tower_actions = environment
                    .semantic_legal_actions()
                    .into_iter()
                    .filter(|legal| matches!(legal.action, AgentAction::BuildTower { .. }))
                    .map(|legal| legal.action)
                    .collect::<Vec<_>>();

                if !build_tower_actions.is_empty() {
                    let dense_start = Instant::now();
                    let bundle = DenseBuildFeatureBundle::compute(&observation);
                    let dense_compute_seconds = dense_start.elapsed().as_secs_f64();
                    let shape = bundle.shape();

                    let legacy_start = Instant::now();
                    for action in &build_tower_actions {
                        let _features =
                            crate::ml::features::candidate_features(&observation, action);
                    }
                    let legacy_seconds = legacy_start.elapsed().as_secs_f64();

                    sample_points.push(DenseBuildBenchmarkSamplePoint {
                        seed,
                        decision_index,
                        legacy_build_tower_candidate_count: build_tower_actions.len(),
                        position_feature_row_count: shape.position_count,
                        distinct_coverage_range_count: shape.distinct_range_count,
                        dense_compute_seconds,
                        legacy_seconds,
                    });
                    samples_collected += 1;
                }

                let legal_actions =
                    environment.semantic_legal_actions_with_position_limit(Some(64));
                let action =
                    crate::policy_runner::scripted_expert_action(&observation, &legal_actions)
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
        let mean_dense_compute_seconds = sample_points
            .iter()
            .map(|s| s.dense_compute_seconds)
            .sum::<f64>()
            / count;
        let mean_legacy_seconds =
            sample_points.iter().map(|s| s.legacy_seconds).sum::<f64>() / count;
        let mean_legacy_build_tower_candidate_count = sample_points
            .iter()
            .map(|s| s.legacy_build_tower_candidate_count as f64)
            .sum::<f64>()
            / count;
        let mean_position_feature_row_count = sample_points
            .iter()
            .map(|s| s.position_feature_row_count as f64)
            .sum::<f64>()
            / count;
        let mean_distinct_coverage_range_count = sample_points
            .iter()
            .map(|s| s.distinct_coverage_range_count as f64)
            .sum::<f64>()
            / count;

        println!(
            "sample_count={} mean_dense_compute_seconds={:.6} mean_legacy_seconds={:.6} \
            mean_legacy_build_tower_candidate_count={:.1}",
            sample_points.len(),
            mean_dense_compute_seconds,
            mean_legacy_seconds,
            mean_legacy_build_tower_candidate_count,
        );

        let report = DenseBuildBenchmarkReport {
            methodology: "dense_compute_seconds times DenseBuildFeatureBundle::compute (one call \
                per decision, covering every (subset, hand_slot, position) triple via shared \
                PositionFeatureTable/RangeCoverageTable). legacy_seconds times \
                ml::features::candidate_features called once per legal BuildTower AgentAction \
                from semantic_legal_actions() (each doing its own per-candidate route scan via \
                placement_coverage/route_distance). Same sampled decision states for both \
                (SEEDS x SAMPLES_PER_SEED, scripted-expert trajectory)."
                .to_string(),
            sample_points,
            mean_dense_compute_seconds,
            mean_legacy_seconds,
            mean_position_feature_row_count,
            mean_distinct_coverage_range_count,
            mean_legacy_build_tower_candidate_count,
        };

        let json = serde_json::to_string_pretty(&report).expect("report should serialize");
        std::fs::create_dir_all("../artifacts/benchmarks")
            .expect("benchmark artifact directory should be creatable");
        std::fs::write(
            "../artifacts/benchmarks/dense-build-features-benchmark.json",
            &json,
        )
        .expect("benchmark report should be written");
    }
}
