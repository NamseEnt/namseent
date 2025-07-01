use super::effect_processor::{DirectEffectKind, ItemEffectKind};
use super::{Item, ItemKind};
use crate::game_state::item::effect_processor::process_item_effect;
use crate::{
    MapCoordF32,
    game_state::{
        GameState, TRAVEL_POINTS,
        field_area_effect::FieldAreaEffectKind,
        quest::{QuestTriggerEvent, on_quest_trigger_event},
        schedule::CountBasedSchedule,
    },
};
use namui::*;
use rand::{Rng, thread_rng};

const FIELD_TICK_INTERVAL: Duration = Duration::from_millis(500);
const LINEAR_AREA_LENGTH: f32 = 68.0;

#[derive(Debug, Clone)]
pub enum ItemUsage {
    Instant,
    CircularArea { radius: f32 },
    LinearArea { thickness: f32 },
}

impl ItemKind {
    pub fn usage(&self) -> ItemUsage {
        match self {
            ItemKind::Heal { .. }
            | ItemKind::Lottery { .. }
            | ItemKind::ExtraReroll
            | ItemKind::Shield { .. }
            | ItemKind::DamageReduction { .. } => ItemUsage::Instant,
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
            ItemKind::LinearDamage { thickness, .. }
            | ItemKind::LinearDamageOverTime { thickness, .. } => ItemUsage::LinearArea {
                thickness: *thickness,
            },
        }
    }

    pub fn effect_kind(&self, xy: Option<MapCoordF32>, now: Instant) -> ItemEffectKind {
        match self {
            ItemKind::Heal { amount } => ItemEffectKind::Direct {
                effect: DirectEffectKind::Heal { amount: *amount },
            },
            ItemKind::AttackPowerPlusBuff {
                amount,
                duration,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::TowerAttackPowerPlusBuffOverTime {
                        amount: *amount,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::AttackPowerMultiplyBuff {
                amount,
                duration,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::TowerAttackPowerMultiplyBuffOverTime {
                        amount: *amount,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::AttackSpeedPlusBuff {
                amount,
                duration,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::TowerAttackSpeedPlusBuffOverTime {
                        amount: *amount,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::AttackSpeedMultiplyBuff {
                amount,
                duration,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::TowerAttackSpeedMultiplyBuffOverTime {
                        amount: *amount,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::AttackRangePlus {
                amount,
                duration,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::TowerAttackRangePlusBuffOverTime {
                        amount: *amount,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::MovementSpeedDebuff {
                amount,
                duration,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::MovementSpeedDebuffOverTime {
                        speed_multiply: *amount,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::RoundDamage {
                rank,
                suit,
                damage,
                radius,
            } => {
                let xy = Self::expect_xy(xy);
                ItemEffectKind::Direct {
                    effect: DirectEffectKind::RoundDamage {
                        rank: *rank,
                        suit: *suit,
                        damage: *damage,
                        xy,
                        radius: *radius,
                    },
                }
            }
            ItemKind::RoundDamageOverTime {
                rank,
                suit,
                damage,
                radius,
                duration,
            } => {
                let xy = Self::expect_xy(xy);
                let damage_per_tick =
                    damage / (duration.as_secs_f32() / FIELD_TICK_INTERVAL.as_secs_f32());
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::RoundDamageOverTime {
                        rank: *rank,
                        suit: *suit,
                        damage_per_tick,
                        xy,
                        radius: *radius,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::Lottery {
                amount,
                probability,
            } => {
                let is_winner = thread_rng().gen_bool(*probability as f64);
                let gold = if is_winner { *amount as usize } else { 0 };
                ItemEffectKind::Direct {
                    effect: DirectEffectKind::EarnGold { amount: gold },
                }
            }
            ItemKind::LinearDamage {
                rank,
                suit,
                damage,
                thickness,
            } => {
                let xy = Self::expect_xy(xy);
                ItemEffectKind::Direct {
                    effect: DirectEffectKind::LinearDamage {
                        rank: *rank,
                        suit: *suit,
                        damage: *damage,
                        center_xy: TRAVEL_POINTS.last().unwrap().map(|x| x as f32),
                        target_xy: xy,
                        thickness: *thickness,
                    },
                }
            }
            ItemKind::LinearDamageOverTime {
                rank,
                suit,
                damage,
                thickness,
                duration,
            } => {
                let xy = Self::expect_xy(xy);
                let damage_per_tick =
                    damage / (duration.as_secs_f32() / FIELD_TICK_INTERVAL.as_secs_f32());
                let emit_count = (duration.as_millis() / FIELD_TICK_INTERVAL.as_millis()) as usize;
                ItemEffectKind::FieldArea {
                    effect: FieldAreaEffectKind::LinearDamageOverTime {
                        rank: *rank,
                        suit: *suit,
                        damage_per_tick,
                        center_xy: TRAVEL_POINTS.last().unwrap().map(|x| x as f32),
                        target_xy: xy,
                        thickness: *thickness,
                    },
                    schedule: CountBasedSchedule::new(FIELD_TICK_INTERVAL, emit_count, now),
                }
            }
            ItemKind::ExtraReroll => ItemEffectKind::Direct {
                effect: DirectEffectKind::ExtraReroll,
            },
            ItemKind::Shield { amount } => ItemEffectKind::Direct {
                effect: DirectEffectKind::Shield { amount: *amount },
            },
            ItemKind::DamageReduction {
                damage_multiply,
                duration,
            } => ItemEffectKind::UserDamageReduction {
                multiply: *damage_multiply,
                duration: *duration,
            },
        }
    }

    fn expect_xy(xy: Option<MapCoordF32>) -> MapCoordF32 {
        match xy {
            Some(val) => val,
            None => panic!("xy must be provided for this item usage"),
        }
    }
}

pub fn use_item(game_state: &mut GameState, item: &Item, xy: Option<MapCoordF32>) {
    game_state.item_used = true;
    let effect_kind = item.kind.effect_kind(xy, game_state.now());
    process_item_effect(game_state, effect_kind);
    on_quest_trigger_event(game_state, QuestTriggerEvent::UseItem);
}

pub fn linear_area_rect_points(
    center: MapCoordF32,
    target: MapCoordF32,
    thickness: f32,
) -> [MapCoordF32; 4] {
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
