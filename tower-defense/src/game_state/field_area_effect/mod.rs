use super::{
    GameState,
    item::{check_point_is_in_linear_area, linear_area_rect_points},
    quest::{QuestTriggerEvent, on_quest_trigger_event},
    upgrade::{TowerUpgradeTarget, UpgradeState},
};
use crate::{
    MapCoordF32,
    card::{Rank, Suit},
};
use namui::*;

pub struct FieldAreaEffect {
    kind: FieldAreaEffectKind,
    end_at: FieldAreaEffectEnd,
}
impl FieldAreaEffect {
    pub fn new(kind: FieldAreaEffectKind, end_at: FieldAreaEffectEnd) -> Self {
        Self { kind, end_at }
    }
}

pub enum FieldAreaEffectKind {
    RoundDamage {
        rank: Rank,
        suit: Suit,
        damage: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    RoundDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage_per_tick: f32,
        xy: MapCoordF32,
        radius: f32,
        tick_interval: Duration,
        next_tick_at: Instant,
    },
    LinearDamage {
        rank: Rank,
        suit: Suit,
        damage: f32,
        center_xy: MapCoordF32,
        target_xy: MapCoordF32,
        thickness: f32,
    },
    LinearDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage_per_tick: f32,
        center_xy: MapCoordF32,
        target_xy: MapCoordF32,
        thickness: f32,
        tick_interval: Duration,
        next_tick_at: Instant,
    },
}

pub enum FieldAreaEffectEnd {
    AtTime { end_at: Instant },
    Once { fired: bool },
}

pub fn field_area_effect_tick(game_state: &mut GameState, now: Instant) {
    let mut monster_dealt_damage = 0.0;
    let mut total_earn_gold = 0;
    for effect in game_state.field_area_effects.iter_mut() {
        match effect.kind {
            FieldAreaEffectKind::RoundDamage {
                rank,
                suit,
                damage,
                xy,
                radius,
            } => {
                let mut damage = damage;
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
            FieldAreaEffectKind::RoundDamageOverTime {
                rank,
                suit,
                damage_per_tick,
                xy,
                radius,
                tick_interval,
                mut next_tick_at,
            } => {
                if now < next_tick_at {
                    continue;
                }
                next_tick_at += tick_interval;

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
            FieldAreaEffectKind::LinearDamage {
                rank,
                suit,
                damage,
                center_xy,
                target_xy,
                thickness,
            } => {
                let mut damage = damage;
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
            FieldAreaEffectKind::LinearDamageOverTime {
                rank,
                suit,
                damage_per_tick,
                center_xy,
                target_xy,
                thickness,
                tick_interval,
                mut next_tick_at,
            } => {
                if now < next_tick_at {
                    continue;
                }
                next_tick_at += tick_interval;

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
        }

        if let FieldAreaEffectEnd::Once { fired } = effect.end_at {
            if !fired {
                effect.end_at = FieldAreaEffectEnd::Once { fired: true };
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
    game_state.field_area_effects.retain(|e| match e.end_at {
        FieldAreaEffectEnd::AtTime { end_at } => now < end_at,
        FieldAreaEffectEnd::Once { fired } => !fired,
    });
}
