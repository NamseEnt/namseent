use super::{Item, ItemKind};
use crate::{
    MapCoordF32,
    game_state::{
        GameState, MAX_HP, TRAVEL_POINTS,
        field_area_effect::{FieldAreaEffect, FieldAreaEffectKind},
        field_particle::{FieldParticleKind, emit_field_particle},
        monster::{MonsterStatusEffect, MonsterStatusEffectKind},
        quest::{QuestTriggerEvent, on_quest_trigger_event},
        schedule::CountBasedSchedule,
        tower::{TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind},
        user_status_effect::{UserStatusEffect, UserStatusEffectKind},
    },
};
use namui::*;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub enum ItemUsage {
    Instant,
    CircularArea { radius: f32 },
    LinearArea { thickness: f32 },
}

impl ItemKind {
    pub fn usage(&self) -> ItemUsage {
        match self {
            ItemKind::Heal { .. } => ItemUsage::Instant,
            ItemKind::AttackPowerPlusBuff { radius, .. }
            | ItemKind::AttackPowerMultiplyBuff { radius, .. }
            | ItemKind::AttackSpeedPlusBuff { radius, .. }
            | ItemKind::AttackSpeedMultiplyBuff { radius, .. }
            | ItemKind::AttackRangePlus { radius, .. }
            | ItemKind::MovementSpeedDebuff { radius, .. }
            | ItemKind::RoundDamage { radius, .. }
            | ItemKind::RoundDamageOverTime { radius, .. } => {
                ItemUsage::CircularArea { radius: *radius }
            }
            ItemKind::Lottery { .. } => ItemUsage::Instant,
            ItemKind::LinearDamage { thickness, .. }
            | ItemKind::LinearDamageOverTime { thickness, .. } => ItemUsage::LinearArea {
                thickness: *thickness,
            },
            ItemKind::ExtraReroll => ItemUsage::Instant,
            ItemKind::Shield { .. } => ItemUsage::Instant,
            ItemKind::DamageReduction { .. } => ItemUsage::Instant,
        }
    }
}

pub fn use_item(game_state: &mut GameState, item: &Item, xy: Option<MapCoordF32>) {
    game_state.item_used = true;
    match item.kind {
        ItemKind::Heal { amount } => game_state.hp = (game_state.hp + amount).min(MAX_HP),
        ItemKind::AttackPowerPlusBuff {
            amount,
            duration,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for AttackPowerPlusBuff item usage");
            add_tower_status_effect_in_round_area(
                game_state,
                xy,
                radius,
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageAdd { add: amount },
                    end_at: TowerStatusEffectEnd::Time {
                        end_at: game_state.now() + duration,
                    },
                },
            );
        }
        ItemKind::AttackPowerMultiplyBuff {
            amount,
            duration,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for AttackPowerMultiplyBuff item usage");
            add_tower_status_effect_in_round_area(
                game_state,
                xy,
                radius,
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul { mul: amount },
                    end_at: TowerStatusEffectEnd::Time {
                        end_at: game_state.now() + duration,
                    },
                },
            );
        }
        ItemKind::AttackSpeedPlusBuff {
            amount,
            duration,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for AttackSpeedPlusBuff item usage");
            add_tower_status_effect_in_round_area(
                game_state,
                xy,
                radius,
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::AttackSpeedAdd { add: amount },
                    end_at: TowerStatusEffectEnd::Time {
                        end_at: game_state.now() + duration,
                    },
                },
            );
        }
        ItemKind::AttackSpeedMultiplyBuff {
            amount,
            duration,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for AttackSpeedMultiplyBuff item usage");
            add_tower_status_effect_in_round_area(
                game_state,
                xy,
                radius,
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::AttackSpeedMul { mul: amount },
                    end_at: TowerStatusEffectEnd::Time {
                        end_at: game_state.now() + duration,
                    },
                },
            );
        }
        ItemKind::AttackRangePlus {
            amount,
            duration,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for AttackRangePlus item usage");
            add_tower_status_effect_in_round_area(
                game_state,
                xy,
                radius,
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::AttackRangeAdd { add: amount },
                    end_at: TowerStatusEffectEnd::Time {
                        end_at: game_state.now() + duration,
                    },
                },
            );
        }
        ItemKind::MovementSpeedDebuff {
            amount,
            duration,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for MovementSpeedDebuff item usage");
            add_monster_status_effect_in_round_area(
                game_state,
                xy,
                radius,
                MonsterStatusEffect {
                    kind: MonsterStatusEffectKind::SpeedMul { mul: amount },
                    end_at: game_state.now() + duration,
                },
            );
        }
        ItemKind::RoundDamage {
            rank,
            suit,
            damage,
            radius,
        } => {
            let xy = xy.expect("xy must be provided for RoundDamage item usage");
            let field_area_effect = FieldAreaEffect::new(
                FieldAreaEffectKind::RoundDamage {
                    rank,
                    suit,
                    damage,
                    xy,
                    radius,
                },
                CountBasedSchedule::new_once(game_state.now()),
            );
            emit_field_particle(
                game_state,
                FieldParticleKind::FieldAreaEffect {
                    field_area_effect: &field_area_effect,
                },
            );
            game_state.field_area_effects.push(field_area_effect);
        }
        ItemKind::RoundDamageOverTime {
            rank,
            suit,
            damage,
            radius,
            duration,
        } => {
            const TICK_INTERVAL: Duration = Duration::from_millis(500);
            let xy = xy.expect("xy must be provided for RoundDamageOverTime item usage");
            let damage_per_tick = damage / (duration / TICK_INTERVAL);
            let emit_count = (duration.as_millis() / TICK_INTERVAL.as_millis()) as usize;
            let field_area_effect = FieldAreaEffect::new(
                FieldAreaEffectKind::RoundDamageOverTime {
                    rank,
                    suit,
                    damage_per_tick,
                    xy,
                    radius,
                },
                CountBasedSchedule::new(TICK_INTERVAL, emit_count, game_state.now()),
            );
            emit_field_particle(
                game_state,
                FieldParticleKind::FieldAreaEffect {
                    field_area_effect: &field_area_effect,
                },
            );
            game_state.field_area_effects.push(field_area_effect);
        }
        ItemKind::Lottery {
            amount,
            probability,
        } => {
            let is_winner = thread_rng().gen_bool(probability as f64);
            if !is_winner {
                return;
            }
            game_state.earn_gold(amount as usize);
            // TODO: Show effect on win
        }
        ItemKind::LinearDamage {
            rank,
            suit,
            damage,
            thickness,
        } => {
            let xy = xy.expect("xy must be provided for LinearDamage item usage");
            let field_area_effect = FieldAreaEffect::new(
                FieldAreaEffectKind::LinearDamage {
                    rank,
                    suit,
                    damage,
                    center_xy: TRAVEL_POINTS.last().unwrap().map(|x| x as f32),
                    target_xy: xy,
                    thickness,
                },
                CountBasedSchedule::new_once(game_state.now()),
            );
            emit_field_particle(
                game_state,
                FieldParticleKind::FieldAreaEffect {
                    field_area_effect: &field_area_effect,
                },
            );
            game_state.field_area_effects.push(field_area_effect);
        }
        ItemKind::LinearDamageOverTime {
            rank,
            suit,
            damage,
            thickness,
            duration,
        } => {
            const TICK_INTERVAL: Duration = Duration::from_millis(500);
            let xy = xy.expect("xy must be provided for LinearDamageOverTime item usage");
            let damage_per_tick = damage / (duration / TICK_INTERVAL);
            let emit_count = (duration.as_millis() / TICK_INTERVAL.as_millis()) as usize;
            let field_area_effect = FieldAreaEffect::new(
                FieldAreaEffectKind::LinearDamageOverTime {
                    rank,
                    suit,
                    damage_per_tick,
                    center_xy: TRAVEL_POINTS.last().unwrap().map(|x| x as f32),
                    target_xy: xy,
                    thickness,
                },
                CountBasedSchedule::new(TICK_INTERVAL, emit_count, game_state.now()),
            );
            emit_field_particle(
                game_state,
                FieldParticleKind::FieldAreaEffect {
                    field_area_effect: &field_area_effect,
                },
            );
            game_state.field_area_effects.push(field_area_effect);
        }
        ItemKind::ExtraReroll => {
            game_state.left_reroll_chance += 1;
        }
        ItemKind::Shield { amount } => {
            game_state.shield += amount;
        }
        ItemKind::DamageReduction {
            damage_multiply,
            duration,
        } => {
            game_state.user_status_effects.push(UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction { damage_multiply },
                end_at: game_state.now() + duration,
            });
        }
    }

    on_quest_trigger_event(game_state, QuestTriggerEvent::UseItem);
}

fn add_tower_status_effect_in_round_area(
    game_state: &mut GameState,
    xy: MapCoordF32,
    radius: f32,
    status_effect: TowerStatusEffect,
) {
    for tower in game_state.towers.iter_mut() {
        if xy.distance(tower.center_xy_f32()) <= radius {
            tower.status_effects.push(status_effect.clone());
        }
    }
}

fn add_monster_status_effect_in_round_area(
    game_state: &mut GameState,
    xy: MapCoordF32,
    radius: f32,
    status_effect: MonsterStatusEffect,
) {
    for monster in game_state.monsters.iter_mut() {
        if xy.distance(monster.xy()) <= radius {
            monster.status_effects.push(status_effect.clone());
        }
    }
}

pub fn linear_area_rect_points(
    center: MapCoordF32,
    target: MapCoordF32,
    thickness: f32,
) -> [MapCoordF32; 4] {
    const LINEAR_AREA_LENGTH: f32 = 68.0;
    let half_thickness = thickness / 2.0;
    let dx = target.x - center.x;
    let dy = target.y - center.y;
    let distance = (dx * dx + dy * dy).sqrt();
    let direction_x = dx / distance;
    let direction_y = dy / distance;
    let end_x = center.x + direction_x * LINEAR_AREA_LENGTH;
    let end_y = center.y + direction_y * LINEAR_AREA_LENGTH;
    let perpendicular_x = -direction_y;
    let perpendicular_y = direction_x;
    let p0 = MapCoordF32::new(
        center.x + perpendicular_x * half_thickness,
        center.y + perpendicular_y * half_thickness,
    );
    let p1 = MapCoordF32::new(
        end_x + perpendicular_x * half_thickness,
        end_y + perpendicular_y * half_thickness,
    );
    let p2 = MapCoordF32::new(
        end_x - perpendicular_x * half_thickness,
        end_y - perpendicular_y * half_thickness,
    );
    let p3 = MapCoordF32::new(
        center.x - perpendicular_x * half_thickness,
        center.y - perpendicular_y * half_thickness,
    );
    [p0, p1, p2, p3]
}

pub fn check_point_is_in_linear_area(points: &[MapCoordF32; 4], point: MapCoordF32) -> bool {
    let mut count = 0;
    for i in 0..4 {
        let p1 = points[i];
        let p2 = points[(i + 1) % 4];
        if p1.y == p2.y {
            continue;
        }
        if point.y < p1.y.min(p2.y) || point.y >= p1.y.max(p2.y) {
            continue;
        }
        let x = (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
        if x > point.x {
            count += 1;
        }
    }
    count % 2 == 1
}
