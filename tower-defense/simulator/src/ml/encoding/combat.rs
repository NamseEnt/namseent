//! Model-facing structured tower combat/status feature bundle.
//!
//! `TowerObservation.status_effects` is variable-cardinality and must not be
//! truncated or aggregated into a fixed-width row (see
//! `docs/game-ai/03-observation-contract.md`). This module gives a future
//! structured policy head a way to consume that full set without collapsing
//! it: one row per tower, one row per status effect (linked back to its
//! owning tower by a stable local index), and one row per splash effect.
//!
//! This is not wired into the existing PPO/BC model, dataset, or trajectory
//! path. It takes only `&Observation` - no `GameEnvironment` or private
//! simulator state.

use super::normalize::normalize_axis_ratio;
use crate::environment::{DamageSplashObservation, Observation, TowerObservation};
use crate::environment::{TowerStatusEffectObservation, TowerStatusEffectObservationKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TowerStatusFeatureKind {
    DamageMul,
    DamageAdd,
}

/// One row per active status effect on a tower, linked back to its owning
/// tower by `tower_index` (a stable local index into
/// `TowerCombatFeatureBundle::towers`, not the runtime tower ID). Different
/// effects on the same tower always get different rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TowerStatusFeatureRow {
    pub tower_index: usize,
    pub kind: TowerStatusFeatureKind,
    pub value_raw: i64,
    pub remaining_ticks: Option<u64>,
    pub never_end: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplashTriggerFeatureKind {
    OnHit,
    OnAttack,
}

/// One row per actual runtime splash effect a placed tower currently
/// carries, linked back to its owning tower by `tower_index`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TowerSplashFeatureRow {
    pub tower_index: usize,
    pub trigger_kind: SplashTriggerFeatureKind,
    pub radius_raw: i64,
    pub damage_pct_raw: i64,
}

/// One row per placed tower, in a deterministic order (ascending tower ID)
/// independent of `Observation.towers`' runtime `Vec` order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TowerCombatFeatureRow {
    pub tower_index: usize,
    pub tower_id: u64,
    pub left_norm: f32,
    pub top_norm: f32,
    pub kind_id: u16,
    pub attack_damage_raw: i64,
    pub range_raw: i64,
    pub cooldown_ticks: u64,
}

/// Structured, per-tower/per-status/per-splash combat feature bundle.
/// `&Observation` is the only input.
#[derive(Clone, Debug, PartialEq)]
pub struct TowerCombatFeatureBundle {
    pub towers: Vec<TowerCombatFeatureRow>,
    pub status_effects: Vec<TowerStatusFeatureRow>,
    pub splashes: Vec<TowerSplashFeatureRow>,
}

impl TowerCombatFeatureBundle {
    pub fn compute(observation: &Observation) -> Self {
        let mut ordered_towers: Vec<&TowerObservation> = observation.towers.iter().collect();
        ordered_towers.sort_by_key(|tower| tower.id);

        let mut towers = Vec::with_capacity(ordered_towers.len());
        let mut status_effects = Vec::new();
        let mut splashes = Vec::new();

        for (tower_index, tower) in ordered_towers.into_iter().enumerate() {
            towers.push(TowerCombatFeatureRow {
                tower_index,
                tower_id: tower.id,
                left_norm: normalize_axis_ratio(tower.left, observation.map_width),
                top_norm: normalize_axis_ratio(tower.top, observation.map_height),
                kind_id: tower.template.kind_id,
                attack_damage_raw: tower.attack_damage_raw,
                range_raw: tower.range_raw,
                cooldown_ticks: tower.cooldown_ticks,
            });

            for status in &tower.status_effects {
                status_effects.push(status_feature_row(tower_index, status));
            }

            for splash in &tower.on_hit_splashes {
                splashes.push(splash_feature_row(
                    tower_index,
                    SplashTriggerFeatureKind::OnHit,
                    splash,
                ));
            }
            for splash in &tower.on_attack_splashes {
                splashes.push(splash_feature_row(
                    tower_index,
                    SplashTriggerFeatureKind::OnAttack,
                    splash,
                ));
            }
        }

        Self {
            towers,
            status_effects,
            splashes,
        }
    }
}

fn status_feature_row(
    tower_index: usize,
    status: &TowerStatusEffectObservation,
) -> TowerStatusFeatureRow {
    let (kind, value_raw) = match status.kind {
        TowerStatusEffectObservationKind::DamageMul { mul_raw } => {
            (TowerStatusFeatureKind::DamageMul, mul_raw)
        }
        TowerStatusEffectObservationKind::DamageAdd { add_raw } => {
            (TowerStatusFeatureKind::DamageAdd, add_raw)
        }
    };
    TowerStatusFeatureRow {
        tower_index,
        kind,
        value_raw,
        remaining_ticks: status.remaining_ticks,
        never_end: status.remaining_ticks.is_none(),
    }
}

fn splash_feature_row(
    tower_index: usize,
    trigger_kind: SplashTriggerFeatureKind,
    splash: &DamageSplashObservation,
) -> TowerSplashFeatureRow {
    TowerSplashFeatureRow {
        tower_index,
        trigger_kind,
        radius_raw: splash.radius_raw,
        damage_pct_raw: splash.damage_pct_raw,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::GameEnvironment;
    use std::sync::Arc;

    /// Drives `environment` with the scripted expert policy (same approach
    /// as `dense_build`'s benchmark test) until at least `count` towers are
    /// placed, rather than hand-assembling `BuildTower`/`PlaceTower`
    /// sequences that depend on flow details (extra tower card slots,
    /// shop/selecting-tower transitions) this module doesn't need to know
    /// about.
    fn environment_with_placed_towers(seed: u64, count: usize) -> GameEnvironment {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        for _ in 0..200 {
            if environment.snapshot().towers.len() >= count {
                break;
            }
            let observation = environment.snapshot();
            let legal_actions = environment.semantic_legal_actions_with_position_limit(Some(64));
            let action = crate::policy_runner::scripted_expert_action(&observation, &legal_actions)
                .expect("scripted expert should find an action");
            let Ok(outcome) = environment.semantic_step(action) else {
                break;
            };
            if outcome.terminated || outcome.truncated {
                break;
            }
        }
        assert!(
            environment.snapshot().towers.len() >= count,
            "expected at least {count} towers to be placed"
        );
        environment
    }

    #[test]
    fn tower_ordering_is_deterministic_regardless_of_observation_vec_order() {
        let environment = environment_with_placed_towers(0, 2);
        let observation = environment.snapshot();
        let bundle = TowerCombatFeatureBundle::compute(&observation);

        let mut reversed_observation = observation.clone();
        reversed_observation.towers.reverse();
        let reversed_bundle = TowerCombatFeatureBundle::compute(&reversed_observation);

        assert_eq!(bundle.towers, reversed_bundle.towers);
        let ids: Vec<u64> = bundle.towers.iter().map(|row| row.tower_id).collect();
        let mut sorted_ids = ids.clone();
        sorted_ids.sort_unstable();
        assert_eq!(ids, sorted_ids);
    }

    #[test]
    fn status_rows_map_to_correct_owning_tower() {
        let environment = environment_with_placed_towers(1, 2);
        let mut observation = environment.snapshot();
        observation.towers[0].status_effects = vec![TowerStatusEffectObservation {
            kind: TowerStatusEffectObservationKind::DamageAdd { add_raw: 777 },
            remaining_ticks: Some(30),
        }];
        observation.towers[1].status_effects = vec![TowerStatusEffectObservation {
            kind: TowerStatusEffectObservationKind::DamageMul { mul_raw: 1_500_000 },
            remaining_ticks: None,
        }];

        let tower_zero_id = observation.towers[0].id;
        let tower_one_id = observation.towers[1].id;
        let bundle = TowerCombatFeatureBundle::compute(&observation);

        let expected_index_zero = bundle
            .towers
            .iter()
            .find(|row| row.tower_id == tower_zero_id)
            .unwrap()
            .tower_index;
        let expected_index_one = bundle
            .towers
            .iter()
            .find(|row| row.tower_id == tower_one_id)
            .unwrap()
            .tower_index;

        assert_eq!(bundle.status_effects.len(), 2);
        assert!(bundle.status_effects.contains(&TowerStatusFeatureRow {
            tower_index: expected_index_zero,
            kind: TowerStatusFeatureKind::DamageAdd,
            value_raw: 777,
            remaining_ticks: Some(30),
            never_end: false,
        }));
        assert!(bundle.status_effects.contains(&TowerStatusFeatureRow {
            tower_index: expected_index_one,
            kind: TowerStatusFeatureKind::DamageMul,
            value_raw: 1_500_000,
            remaining_ticks: None,
            never_end: true,
        }));
    }

    #[test]
    fn compute_is_deterministic_for_same_observation() {
        let environment = environment_with_placed_towers(2, 2);
        let observation = environment.snapshot();
        let first = TowerCombatFeatureBundle::compute(&observation);
        let second = TowerCombatFeatureBundle::compute(&observation);
        assert_eq!(first, second);
    }
}
