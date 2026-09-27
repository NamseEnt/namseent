//! Shared raw-value <-> normalized-feature helpers for `ml::encoding`.
//!
//! Each constant here fixes the divisor for one numeric feature family
//! referenced by `docs/game-ai/03-observation-contract.md`'s normalization
//! contract (`TypedObservation` / `DenseBuildFeatureBundle` encoders only -
//! the legacy `ml::features` PPO/BC vector keeps its own independent
//! literals and is out of scope here). Changing a scale changes encoded
//! model input and must bump `FEATURE_SCHEMA_VERSION`.
//!
//! Every family has a `normalize_*`/`denormalize_*` pair so the raw ->
//! normalized -> raw round trip is checked by tests instead of only living
//! implicitly in encoder call sites.

pub const DAMAGE_RAW_SCALE: f32 = 10_000.0;
pub const RANGE_RAW_SCALE: f32 = 100_000.0;
pub const TICKS_SCALE: f32 = 600.0;
pub const POLISH_PCT_RAW_SCALE: f32 = 1_000.0;
pub const ROUTE_PROGRESS_RAW_SCALE: f32 = 1_000.0;
pub const ROUTE_INDEX_SCALE: f32 = 100.0;
pub const REROLLED_COUNT_SCALE: f32 = 20.0;
pub const UPGRADE_SCALAR_SCALE: f32 = 1_000.0;
pub const UPGRADE_RATIO_SCALE: f32 = 1_000_000.0;

/// `damage_raw` / `effective_damage_raw` / monster `damage_raw`.
pub fn normalize_damage_raw(raw: i64) -> f32 {
    raw as f32 / DAMAGE_RAW_SCALE
}

pub fn denormalize_damage_raw(normalized: f32) -> i64 {
    (normalized * DAMAGE_RAW_SCALE).round() as i64
}

/// Tower `range_raw`.
pub fn normalize_range_raw(raw: i64) -> f32 {
    raw as f32 / RANGE_RAW_SCALE
}

pub fn denormalize_range_raw(normalized: f32) -> i64 {
    (normalized * RANGE_RAW_SCALE).round() as i64
}

/// `cooldown_ticks` / `shoot_interval_ticks`.
pub fn normalize_ticks(ticks: u64) -> f32 {
    ticks as f32 / TICKS_SCALE
}

pub fn denormalize_ticks(normalized: f32) -> u64 {
    (normalized * TICKS_SCALE).round() as u64
}

/// Card `polish_pct_raw`.
pub fn normalize_polish_pct_raw(raw: i64) -> f32 {
    raw as f32 / POLISH_PCT_RAW_SCALE
}

pub fn denormalize_polish_pct_raw(normalized: f32) -> i64 {
    (normalized * POLISH_PCT_RAW_SCALE).round() as i64
}

/// Route/monster `route_progress_raw`.
pub fn normalize_route_progress_raw(raw: i64) -> f32 {
    raw as f32 / ROUTE_PROGRESS_RAW_SCALE
}

pub fn denormalize_route_progress_raw(normalized: f32) -> i64 {
    (normalized * ROUTE_PROGRESS_RAW_SCALE).round() as i64
}

/// Monster `route_index` (current route segment index).
pub fn normalize_route_index(index: usize) -> f32 {
    index as f32 / ROUTE_INDEX_SCALE
}

pub fn denormalize_route_index(normalized: f32) -> usize {
    (normalized * ROUTE_INDEX_SCALE).round() as usize
}

/// Tower/card `rerolled_count`.
pub fn normalize_rerolled_count(count: usize) -> f32 {
    count as f32 / REROLLED_COUNT_SCALE
}

pub fn denormalize_rerolled_count(normalized: f32) -> usize {
    (normalized * REROLLED_COUNT_SCALE).round() as usize
}

/// `OwnedUpgradeObservation::scalar_values`.
pub fn normalize_upgrade_scalar(value: usize) -> f32 {
    value as f32 / UPGRADE_SCALAR_SCALE
}

pub fn denormalize_upgrade_scalar(normalized: f32) -> usize {
    (normalized * UPGRADE_SCALAR_SCALE).round() as usize
}

/// `OwnedUpgradeObservation::ratio_values`.
pub fn normalize_upgrade_ratio(value: i64) -> f32 {
    value as f32 / UPGRADE_RATIO_SCALE
}

pub fn denormalize_upgrade_ratio(normalized: f32) -> i64 {
    (normalized * UPGRADE_RATIO_SCALE).round() as i64
}

/// `OwnedUpgradeObservation::bool_values`.
pub fn normalize_upgrade_bool(value: bool) -> f32 {
    value as u8 as f32
}

pub fn denormalize_upgrade_bool(normalized: f32) -> bool {
    normalized != 0.0
}

/// Position/index ratio against a variable extent (map width/height, route
/// coordinate list length): `value / extent.max(1)`. Not a fixed-scale
/// family, but the same round-trip contract applies for a fixed extent.
pub fn normalize_axis_ratio(value: usize, extent: usize) -> f32 {
    value as f32 / extent.max(1) as f32
}

pub fn denormalize_axis_ratio(normalized: f32, extent: usize) -> usize {
    (normalized * extent.max(1) as f32).round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_raw_round_trips() {
        for raw in [0_i64, 1, 100, 12_345, 100_000, 987_654] {
            let normalized = normalize_damage_raw(raw);
            assert_eq!(denormalize_damage_raw(normalized), raw);
        }
        assert_eq!(normalize_damage_raw(10_000), 1.0);
    }

    #[test]
    fn range_raw_round_trips() {
        for raw in [0_i64, 1, 3_000_000, 100_000, 12_345_678] {
            let normalized = normalize_range_raw(raw);
            assert_eq!(denormalize_range_raw(normalized), raw);
        }
        assert_eq!(normalize_range_raw(100_000), 1.0);
    }

    #[test]
    fn ticks_round_trip() {
        for ticks in [0_u64, 1, 30, 600, 12_345] {
            let normalized = normalize_ticks(ticks);
            assert_eq!(denormalize_ticks(normalized), ticks);
        }
        assert_eq!(normalize_ticks(600), 1.0);
    }

    #[test]
    fn polish_pct_raw_round_trips() {
        for raw in [0_i64, 250, 1_000, 5_000, 42_000] {
            let normalized = normalize_polish_pct_raw(raw);
            assert_eq!(denormalize_polish_pct_raw(normalized), raw);
        }
        assert_eq!(normalize_polish_pct_raw(1_000), 1.0);
    }

    #[test]
    fn route_progress_raw_round_trips() {
        for raw in [0_i64, 500, 1_000, 999_999] {
            let normalized = normalize_route_progress_raw(raw);
            assert_eq!(denormalize_route_progress_raw(normalized), raw);
        }
        assert_eq!(normalize_route_progress_raw(1_000), 1.0);
    }

    #[test]
    fn route_index_round_trips() {
        for index in [0_usize, 1, 50, 100, 1_234] {
            let normalized = normalize_route_index(index);
            assert_eq!(denormalize_route_index(normalized), index);
        }
        assert_eq!(normalize_route_index(100), 1.0);
    }

    #[test]
    fn rerolled_count_round_trips() {
        for count in [0_usize, 1, 5, 20, 123] {
            let normalized = normalize_rerolled_count(count);
            assert_eq!(denormalize_rerolled_count(normalized), count);
        }
        assert_eq!(normalize_rerolled_count(20), 1.0);
    }

    #[test]
    fn upgrade_scalar_round_trips() {
        for value in [0_usize, 1, 250, 1_000, 9_999] {
            let normalized = normalize_upgrade_scalar(value);
            assert_eq!(denormalize_upgrade_scalar(normalized), value);
        }
        assert_eq!(normalize_upgrade_scalar(1_000), 1.0);
    }

    #[test]
    fn upgrade_ratio_round_trips() {
        for value in [0_i64, 1, 250_000, 1_000_000, 5_000_000] {
            let normalized = normalize_upgrade_ratio(value);
            assert_eq!(denormalize_upgrade_ratio(normalized), value);
        }
        assert_eq!(normalize_upgrade_ratio(1_000_000), 1.0);
    }

    #[test]
    fn upgrade_bool_round_trips() {
        for value in [true, false] {
            assert_eq!(
                denormalize_upgrade_bool(normalize_upgrade_bool(value)),
                value
            );
        }
        assert_eq!(normalize_upgrade_bool(true), 1.0);
        assert_eq!(normalize_upgrade_bool(false), 0.0);
    }

    #[test]
    fn axis_ratio_round_trips() {
        for (value, extent) in [(0_usize, 32), (16, 32), (31, 32), (0, 0), (5, 1)] {
            let normalized = normalize_axis_ratio(value, extent);
            assert_eq!(denormalize_axis_ratio(normalized, extent), value);
        }
    }
}
