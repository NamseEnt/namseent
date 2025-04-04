use super::{Item, ItemKind};
use crate::{
    MapCoordF32,
    game_state::{
        GameState, MAP_SIZE, MAX_HP, TRAVEL_POINTS,
        field_area_effect::{FieldAreaEffect, FieldAreaEffectEnd, FieldAreaEffectKind},
        monster::{MonsterStatusEffect, MonsterStatusEffectKind},
        quest::{QuestTriggerEvent, on_quest_trigger_event},
        tower::{TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind},
        user_status_effect::{UserStatusEffect, UserStatusEffectKind},
    },
};
use namui::{time::now, *};
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
            game_state.field_area_effects.push(FieldAreaEffect::new(
                FieldAreaEffectKind::RoundDamage {
                    rank,
                    suit,
                    damage,
                    xy,
                    radius,
                },
                FieldAreaEffectEnd::Once { fired: false },
            ));
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
            game_state.field_area_effects.push(FieldAreaEffect::new(
                FieldAreaEffectKind::RoundDamageOverTime {
                    rank,
                    suit,
                    damage_per_tick,
                    xy,
                    radius,
                    tick_interval: TICK_INTERVAL,
                    next_tick_at: game_state.now(),
                },
                FieldAreaEffectEnd::Once { fired: false },
            ));
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
            game_state.field_area_effects.push(FieldAreaEffect::new(
                FieldAreaEffectKind::LinearDamage {
                    rank,
                    suit,
                    damage,
                    center_xy: TRAVEL_POINTS.last().unwrap().map(|x| x as f32),
                    target_xy: xy,
                    thickness,
                },
                FieldAreaEffectEnd::Once { fired: false },
            ));
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
            game_state.field_area_effects.push(FieldAreaEffect::new(
                FieldAreaEffectKind::LinearDamageOverTime {
                    rank,
                    suit,
                    damage_per_tick,
                    center_xy: TRAVEL_POINTS.last().unwrap().map(|x| x as f32),
                    target_xy: xy,
                    thickness,
                    tick_interval: TICK_INTERVAL,
                    next_tick_at: game_state.now(),
                },
                FieldAreaEffectEnd::Once { fired: false },
            ));
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
    let half_thickness = thickness / 2.0;
    let long_value = MAP_SIZE.width as f32 * 2.0;
    let horizontal_rect = [
        MapCoordF32::new(-long_value, half_thickness),
        MapCoordF32::new(long_value, half_thickness),
        MapCoordF32::new(long_value, -half_thickness),
        MapCoordF32::new(-long_value, -half_thickness),
    ];

    let distance = center.distance(target);
    let dx = target.x - center.x;
    let dy = target.y - center.y;
    let cos = dx / distance;
    let sin = dy / distance;
    [
        rotate_around_origin(horizontal_rect[0], cos, sin) + center,
        rotate_around_origin(horizontal_rect[1], cos, sin) + center,
        rotate_around_origin(horizontal_rect[2], cos, sin) + center,
        rotate_around_origin(horizontal_rect[3], cos, sin) + center,
    ]
}
fn rotate_around_origin(point: MapCoordF32, cos: f32, sin: f32) -> MapCoordF32 {
    let x = point.x * cos - point.y * sin;
    let y = point.x * sin + point.y * cos;
    MapCoordF32::new(x, y)
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
