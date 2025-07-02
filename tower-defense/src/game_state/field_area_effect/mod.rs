use super::{
    GameState,
    item::{
        check_point_is_in_linear_area,
        effect_processor::{apply_rank_and_suit_upgrades, process_monster_damage},
        linear_area_rect_points,
    },
    monster::{MonsterStatusEffect, MonsterStatusEffectKind},
    schedule::CountBasedSchedule,
    tower::{TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind},
};
use crate::{
    MapCoordF32,
    card::{Rank, Suit},
    game_state::tower::TowerKind,
};
use namui::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct FieldAreaEffect {
    pub kind: FieldAreaEffectKind,
    pub schedule: CountBasedSchedule,
}
impl FieldAreaEffect {
    pub fn new(kind: FieldAreaEffectKind, schedule: CountBasedSchedule) -> Self {
        Self { kind, schedule }
    }
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum FieldAreaEffectKind {
    RoundDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage_per_tick: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    LinearDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage_per_tick: f32,
        center_xy: MapCoordF32,
        target_xy: MapCoordF32,
        thickness: f32,
    },
    MovementSpeedDebuffOverTime {
        speed_multiply: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    TowerAttackPowerPlusBuffOverTime {
        amount: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    TowerAttackPowerMultiplyBuffOverTime {
        amount: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    TowerAttackSpeedPlusBuffOverTime {
        amount: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    TowerAttackSpeedMultiplyBuffOverTime {
        amount: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    TowerAttackRangePlusBuffOverTime {
        amount: f32,
        xy: MapCoordF32,
        radius: f32,
    },
}

pub fn field_area_effect_tick(game_state: &mut GameState, now: Instant) {
    let current_time = now;
    let mut effects_to_process = Vec::new();

    // Collect effects that need processing
    for (index, effect) in game_state.field_area_effects.iter_mut().enumerate() {
        if effect.schedule.try_emit(now) {
            effects_to_process.push((index, effect.kind.clone(), effect.schedule.interval));
        }
    }

    // Process each effect
    for (_, effect_kind, effect_interval) in effects_to_process {
        match effect_kind {
            FieldAreaEffectKind::RoundDamageOverTime {
                rank,
                suit,
                damage_per_tick,
                xy,
                radius,
            } => {
                process_round_damage_over_time(game_state, rank, suit, damage_per_tick, xy, radius);
            }
            FieldAreaEffectKind::LinearDamageOverTime {
                rank,
                suit,
                damage_per_tick,
                center_xy,
                target_xy,
                thickness,
            } => {
                process_linear_damage_over_time(
                    game_state,
                    rank,
                    suit,
                    damage_per_tick,
                    center_xy,
                    target_xy,
                    thickness,
                );
            }
            FieldAreaEffectKind::MovementSpeedDebuffOverTime {
                speed_multiply,
                xy,
                radius,
            } => {
                for monster in game_state.monsters.iter_mut() {
                    if monster.xy().distance(xy) <= radius {
                        let status_effect = MonsterStatusEffect {
                            kind: MonsterStatusEffectKind::SpeedMul {
                                mul: speed_multiply,
                            },
                            end_at: current_time + effect_interval,
                        };
                        monster.status_effects.push(status_effect);
                    }
                }
            }
            FieldAreaEffectKind::TowerAttackPowerPlusBuffOverTime { amount, xy, radius } => {
                apply_tower_status_effect_in_circular_area(
                    game_state,
                    TowerStatusEffectKind::DamageAdd { add: amount },
                    xy,
                    radius,
                    current_time,
                    effect_interval,
                );
            }
            FieldAreaEffectKind::TowerAttackPowerMultiplyBuffOverTime { amount, xy, radius } => {
                apply_tower_status_effect_in_circular_area(
                    game_state,
                    TowerStatusEffectKind::DamageMul { mul: amount },
                    xy,
                    radius,
                    current_time,
                    effect_interval,
                );
            }
            FieldAreaEffectKind::TowerAttackSpeedPlusBuffOverTime { amount, xy, radius } => {
                apply_tower_status_effect_in_circular_area(
                    game_state,
                    TowerStatusEffectKind::AttackSpeedAdd { add: amount },
                    xy,
                    radius,
                    current_time,
                    effect_interval,
                );
            }
            FieldAreaEffectKind::TowerAttackSpeedMultiplyBuffOverTime { amount, xy, radius } => {
                apply_tower_status_effect_in_circular_area(
                    game_state,
                    TowerStatusEffectKind::AttackSpeedMul { mul: amount },
                    xy,
                    radius,
                    current_time,
                    effect_interval,
                );
            }
            FieldAreaEffectKind::TowerAttackRangePlusBuffOverTime { amount, xy, radius } => {
                apply_tower_status_effect_in_circular_area(
                    game_state,
                    TowerStatusEffectKind::AttackRangeAdd { add: amount },
                    xy,
                    radius,
                    current_time,
                    effect_interval,
                );
            }
        }
    }
}

pub fn remove_finished_field_area_effects(game_state: &mut GameState, now: Instant) {
    game_state
        .field_area_effects
        .retain(|e| !e.schedule.is_done(now));
}

fn process_round_damage_over_time(
    game_state: &mut GameState,
    rank: Rank,
    suit: Suit,
    damage_per_tick: f32,
    xy: MapCoordF32,
    radius: f32,
) {
    let mut damage = damage_per_tick;
    apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut damage);

    let result = process_monster_damage(game_state, damage, |monster| {
        monster.xy().distance(xy) <= radius
    });

    result.finalize(game_state);
}

fn process_linear_damage_over_time(
    game_state: &mut GameState,
    rank: Rank,
    suit: Suit,
    damage_per_tick: f32,
    center_xy: MapCoordF32,
    target_xy: MapCoordF32,
    thickness: f32,
) {
    let mut damage = damage_per_tick;
    apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut damage);

    let points = linear_area_rect_points(center_xy, target_xy, thickness);
    let result = process_monster_damage(game_state, damage, |monster| {
        check_point_is_in_linear_area(&points, monster.xy())
    });

    result.finalize(game_state);
}

fn apply_tower_status_effect_in_circular_area(
    game_state: &mut GameState,
    effect_kind: TowerStatusEffectKind,
    center_position: MapCoordF32,
    radius: f32,
    current_time: Instant,
    duration: Duration,
) {
    for tower in game_state.towers.iter_mut() {
        if matches!(tower.kind, TowerKind::Barricade) {
            continue;
        }
        if tower.center_xy_f32().distance(center_position) <= radius {
            let status_effect = TowerStatusEffect {
                kind: effect_kind,
                end_at: TowerStatusEffectEnd::Time {
                    end_at: current_time + duration,
                },
            };
            tower.status_effects.push(status_effect);
        }
    }
}
