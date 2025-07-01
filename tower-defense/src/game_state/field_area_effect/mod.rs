use super::{
    GameState,
    item::{check_point_is_in_linear_area, linear_area_rect_points},
    monster::{MonsterStatusEffect, MonsterStatusEffectKind},
    quest::{QuestTriggerEvent, on_quest_trigger_event},
    schedule::CountBasedSchedule,
    tower::{TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind},
    upgrade::{TowerUpgradeTarget, UpgradeState},
};
use crate::{
    MapCoordF32,
    card::{Rank, Suit},
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
    let mut monster_dealt_damage = 0.0;
    let mut total_earn_gold = 0;
    let current_time = now;

    for effect in game_state.field_area_effects.iter_mut() {
        if !effect.schedule.try_emit(now) {
            continue;
        }

        match effect.kind {
            FieldAreaEffectKind::RoundDamageOverTime {
                rank,
                suit,
                damage_per_tick,
                xy,
                radius,
            } => {
                let mut damage = damage_per_tick;
                apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut damage);

                game_state.monsters.retain_mut(|monster| {
                    if monster.xy().distance(xy) > radius {
                        return true;
                    }

                    monster.get_damage(damage);
                    monster_dealt_damage += damage;

                    if monster.dead() {
                        let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
                        total_earn_gold += earn;
                        return false;
                    }

                    true
                });
            }
            FieldAreaEffectKind::LinearDamageOverTime {
                rank,
                suit,
                damage_per_tick,
                center_xy,
                target_xy,
                thickness,
            } => {
                let mut damage = damage_per_tick;
                apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut damage);

                let points = linear_area_rect_points(center_xy, target_xy, thickness);
                game_state.monsters.retain_mut(|monster| {
                    if !check_point_is_in_linear_area(&points, monster.xy()) {
                        return true;
                    }

                    monster.get_damage(damage);
                    monster_dealt_damage += damage;

                    if monster.dead() {
                        let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
                        total_earn_gold += earn;
                        return false;
                    }

                    true
                });
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
                            end_at: current_time + Duration::from_millis(500),
                        };
                        monster.status_effects.push(status_effect);
                    }
                }
            }
            FieldAreaEffectKind::TowerAttackPowerPlusBuffOverTime { amount, xy, radius } => {
                let mut affected_tower_positions = Vec::new();
                for tower in game_state.towers.iter_mut() {
                    if tower.center_xy_f32().distance(xy) <= radius {
                        let status_effect = TowerStatusEffect {
                            kind: TowerStatusEffectKind::DamageAdd { add: amount },
                            end_at: TowerStatusEffectEnd::Time {
                                end_at: current_time + Duration::from_millis(500),
                            },
                        };
                        tower.status_effects.push(status_effect);
                        affected_tower_positions.push(tower.center_xy_f32());
                    }
                }

                for tower_position in affected_tower_positions {
                    let emitter = crate::game_state::field_particle::FieldParticleEmitter::TowerStatusEffect {
                        emitter: crate::game_state::field_particle::emitter::TowerStatusEffectEmitter::new_with_default_duration(
                            current_time,
                            tower_position,
                            FieldAreaEffectKind::TowerAttackPowerPlusBuffOverTime { amount, xy, radius },
                        ),
                    };
                    let system =
                        crate::game_state::field_particle::FieldParticleSystem::new(vec![emitter]);
                    game_state.field_particle_system_manager.add_system(system);
                }
            }
            FieldAreaEffectKind::TowerAttackPowerMultiplyBuffOverTime { amount, xy, radius } => {
                let mut affected_tower_positions = Vec::new();
                for tower in game_state.towers.iter_mut() {
                    if tower.center_xy_f32().distance(xy) <= radius {
                        let status_effect = TowerStatusEffect {
                            kind: TowerStatusEffectKind::DamageMul { mul: amount },
                            end_at: TowerStatusEffectEnd::Time {
                                end_at: current_time + Duration::from_millis(500),
                            },
                        };
                        tower.status_effects.push(status_effect);
                        affected_tower_positions.push(tower.center_xy_f32());
                    }
                }

                for tower_position in affected_tower_positions {
                    let emitter = crate::game_state::field_particle::FieldParticleEmitter::TowerStatusEffect {
                        emitter: crate::game_state::field_particle::emitter::TowerStatusEffectEmitter::new_with_default_duration(
                            current_time,
                            tower_position,
                            FieldAreaEffectKind::TowerAttackPowerMultiplyBuffOverTime { amount, xy, radius },
                        ),
                    };
                    let system =
                        crate::game_state::field_particle::FieldParticleSystem::new(vec![emitter]);
                    game_state.field_particle_system_manager.add_system(system);
                }
            }
            FieldAreaEffectKind::TowerAttackSpeedPlusBuffOverTime { amount, xy, radius } => {
                let mut affected_tower_positions = Vec::new();
                for tower in game_state.towers.iter_mut() {
                    if tower.center_xy_f32().distance(xy) <= radius {
                        let status_effect = TowerStatusEffect {
                            kind: TowerStatusEffectKind::AttackSpeedAdd { add: amount },
                            end_at: TowerStatusEffectEnd::Time {
                                end_at: current_time + Duration::from_millis(500),
                            },
                        };
                        tower.status_effects.push(status_effect);
                        affected_tower_positions.push(tower.center_xy_f32());
                    }
                }

                for tower_position in affected_tower_positions {
                    let emitter = crate::game_state::field_particle::FieldParticleEmitter::TowerStatusEffect {
                        emitter: crate::game_state::field_particle::emitter::TowerStatusEffectEmitter::new_with_default_duration(
                            current_time,
                            tower_position,
                            FieldAreaEffectKind::TowerAttackSpeedPlusBuffOverTime { amount, xy, radius },
                        ),
                    };
                    let system =
                        crate::game_state::field_particle::FieldParticleSystem::new(vec![emitter]);
                    game_state.field_particle_system_manager.add_system(system);
                }
            }
            FieldAreaEffectKind::TowerAttackSpeedMultiplyBuffOverTime { amount, xy, radius } => {
                let mut affected_tower_positions = Vec::new();
                for tower in game_state.towers.iter_mut() {
                    if tower.center_xy_f32().distance(xy) <= radius {
                        let status_effect = TowerStatusEffect {
                            kind: TowerStatusEffectKind::AttackSpeedMul { mul: amount },
                            end_at: TowerStatusEffectEnd::Time {
                                end_at: current_time + Duration::from_millis(500),
                            },
                        };
                        tower.status_effects.push(status_effect);
                        affected_tower_positions.push(tower.center_xy_f32());
                    }
                }

                for tower_position in affected_tower_positions {
                    let emitter = crate::game_state::field_particle::FieldParticleEmitter::TowerStatusEffect {
                        emitter: crate::game_state::field_particle::emitter::TowerStatusEffectEmitter::new_with_default_duration(
                            current_time,
                            tower_position,
                            FieldAreaEffectKind::TowerAttackSpeedMultiplyBuffOverTime { amount, xy, radius },
                        ),
                    };
                    let system =
                        crate::game_state::field_particle::FieldParticleSystem::new(vec![emitter]);
                    game_state.field_particle_system_manager.add_system(system);
                }
            }
            FieldAreaEffectKind::TowerAttackRangePlusBuffOverTime { amount, xy, radius } => {
                let mut affected_tower_positions = Vec::new();
                for tower in game_state.towers.iter_mut() {
                    if tower.center_xy_f32().distance(xy) <= radius {
                        let status_effect = TowerStatusEffect {
                            kind: TowerStatusEffectKind::AttackRangeAdd { add: amount },
                            end_at: TowerStatusEffectEnd::Time {
                                end_at: current_time + Duration::from_millis(500),
                            },
                        };
                        tower.status_effects.push(status_effect);
                        affected_tower_positions.push(tower.center_xy_f32());
                    }
                }

                for tower_position in affected_tower_positions {
                    let emitter = crate::game_state::field_particle::FieldParticleEmitter::TowerStatusEffect {
                        emitter: crate::game_state::field_particle::emitter::TowerStatusEffectEmitter::new_with_default_duration(
                            current_time,
                            tower_position,
                            FieldAreaEffectKind::TowerAttackRangePlusBuffOverTime { amount, xy, radius },
                        ),
                    };
                    let system =
                        crate::game_state::field_particle::FieldParticleSystem::new(vec![emitter]);
                    game_state.field_particle_system_manager.add_system(system);
                }
            }
        }
    }

    if total_earn_gold > 0 {
        game_state.earn_gold(total_earn_gold);
    }

    if monster_dealt_damage > 0.0 {
        on_quest_trigger_event(
            game_state,
            QuestTriggerEvent::DealDamageWithItem {
                damage: monster_dealt_damage,
            },
        );
    }
}

fn apply_rank_and_suit_upgrades(
    upgrade_state: &UpgradeState,
    rank: Rank,
    suit: Suit,
    damage: &mut f32,
) {
    let rank_upgrades = upgrade_state
        .tower_upgrade_states
        .get(&TowerUpgradeTarget::Rank { rank });
    let suit_upgrades = upgrade_state
        .tower_upgrade_states
        .get(&TowerUpgradeTarget::Suit { suit });

    for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
        *damage += upgrade.damage_plus;
    }
    for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
        *damage *= upgrade.damage_multiplier;
    }
}

pub fn remove_finished_field_area_effects(game_state: &mut GameState, now: Instant) {
    game_state
        .field_area_effects
        .retain(|e| !e.schedule.is_done(now));
}
