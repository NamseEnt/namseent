//! Model-facing structured wave feature bundle.
//!
//! `Observation.stage_wave`/`queued_wave` are variable-cardinality,
//! order-sensitive group lists and must not be collapsed into a fixed-width
//! row or a per-kind aggregate (see `docs/game-ai/03-observation-contract.md`,
//! section "웨이브와 장기 상태"). This module gives a future structured policy head
//! a way to consume that full set without collapsing it: one row per stage
//! wave group and one row per queued wave group, plus current spawn timing.
//!
//! This is not wired into the existing PPO/BC model, dataset, or trajectory
//! path, and does not change `TypedObservation::ENTITY_SET_COUNT`. It takes
//! only `&Observation` - no `GameEnvironment` or private simulator state.

use crate::environment::{Observation, QueuedMonsterGroupObservation, WaveGroupObservation};

/// One row per wave group (either a `stage_wave` or `queued_wave` entry),
/// preserving that group's `order_index` exactly as `Observation` reports it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveFeatureRow {
    pub order_index: usize,
    pub kind_id: u16,
    pub count: usize,
    pub max_hp_raw: i64,
    pub velocity_raw: i64,
    pub damage_raw: i64,
    pub reward: usize,
}

/// Structured, per-group wave/spawn-timing feature bundle. `&Observation` is
/// the only input. `stage_groups` is the current stage's full configured
/// composition (available in every decision point); `queued_groups` is the
/// actual remaining runtime spawn queue (empty outside Defense or once
/// exhausted) - the two must not be merged, see
/// `docs/game-ai/03-observation-contract.md`.
#[derive(Clone, Debug, PartialEq)]
pub struct WaveFeatureBundle {
    pub stage_groups: Vec<WaveFeatureRow>,
    pub queued_groups: Vec<WaveFeatureRow>,
    pub spawn_interval_ticks: u64,
    pub next_spawn_in_ticks: Option<u64>,
}

impl WaveFeatureBundle {
    pub fn compute(observation: &Observation) -> Self {
        Self {
            stage_groups: observation
                .stage_wave
                .iter()
                .map(stage_wave_feature_row)
                .collect(),
            queued_groups: observation
                .queued_wave
                .iter()
                .map(queued_wave_feature_row)
                .collect(),
            spawn_interval_ticks: observation.spawn_interval_ticks,
            next_spawn_in_ticks: observation.next_spawn_in_ticks,
        }
    }
}

fn stage_wave_feature_row(group: &WaveGroupObservation) -> WaveFeatureRow {
    WaveFeatureRow {
        order_index: group.order_index,
        kind_id: group.kind_id,
        count: group.count,
        max_hp_raw: group.max_hp_raw,
        velocity_raw: group.velocity_raw,
        damage_raw: group.damage_raw,
        reward: group.reward,
    }
}

fn queued_wave_feature_row(group: &QueuedMonsterGroupObservation) -> WaveFeatureRow {
    WaveFeatureRow {
        order_index: group.order_index,
        kind_id: group.kind_id,
        count: group.count,
        max_hp_raw: group.max_hp_raw,
        velocity_raw: group.velocity_raw,
        damage_raw: group.damage_raw,
        reward: group.reward,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::GameEnvironment;
    use std::sync::Arc;

    fn observation_after_reset(seed: u64) -> Observation {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        environment.reset(seed, Arc::new(GameConfig::default_config()))
    }

    #[test]
    fn stage_groups_mirror_observation_order_and_stats() {
        let observation = observation_after_reset(0);
        let bundle = WaveFeatureBundle::compute(&observation);
        assert_eq!(bundle.stage_groups.len(), observation.stage_wave.len());
        for (row, group) in bundle
            .stage_groups
            .iter()
            .zip(observation.stage_wave.iter())
        {
            assert_eq!(row.order_index, group.order_index);
            assert_eq!(row.kind_id, group.kind_id);
            assert_eq!(row.count, group.count);
            assert_eq!(row.max_hp_raw, group.max_hp_raw);
            assert_eq!(row.velocity_raw, group.velocity_raw);
            assert_eq!(row.damage_raw, group.damage_raw);
            assert_eq!(row.reward, group.reward);
        }
    }

    #[test]
    fn compute_is_deterministic_for_same_observation() {
        let observation = observation_after_reset(1);
        let first = WaveFeatureBundle::compute(&observation);
        let second = WaveFeatureBundle::compute(&observation);
        assert_eq!(first, second);
    }

    #[test]
    fn spawn_timing_is_carried_through_unchanged() {
        let observation = observation_after_reset(2);
        let bundle = WaveFeatureBundle::compute(&observation);
        assert_eq!(
            bundle.spawn_interval_ticks,
            observation.spawn_interval_ticks
        );
        assert_eq!(bundle.next_spawn_in_ticks, observation.next_spawn_in_ticks);
    }
}
