use super::{
    GameState,
    item::{check_point_is_in_linear_area, linear_area_rect_points},
};
use crate::{
    MapCoordF32,
    card::{Rank, Suit},
    upgrade::{TowerUpgradeTarget, UpgradeState},
};
use namui::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FieldAreaEffect {
    id: usize,
    kind: FieldAreaEffectKind,
    end_at: FieldAreaEffectEnd,
}
impl FieldAreaEffect {
    pub fn new(kind: FieldAreaEffectKind, end_at: FieldAreaEffectEnd) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            kind,
            end_at,
        }
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

                for monster in game_state.monsters.iter_mut() {
                    if monster.xy().distance(xy) > radius {
                        return;
                    }

                    monster.get_damage(damage);
                }
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
                    return;
                }
                next_tick_at += tick_interval;

                let mut damage = damage_per_tick;
                apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut damage);

                for monster in game_state.monsters.iter_mut() {
                    if monster.xy().distance(xy) > radius {
                        return;
                    }

                    monster.get_damage(damage);
                }
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
                for monster in game_state.monsters.iter_mut() {
                    if !check_point_is_in_linear_area(&points, monster.xy()) {
                        return;
                    }
                    monster.get_damage(damage);
                }
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
                    return;
                }
                next_tick_at += tick_interval;

                let mut damage = damage_per_tick;
                apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut damage);

                let points = linear_area_rect_points(center_xy, target_xy, thickness);
                for monster in game_state.monsters.iter_mut() {
                    if !check_point_is_in_linear_area(&points, monster.xy()) {
                        return;
                    }
                    monster.get_damage(damage);
                }
            }
        }

        if let FieldAreaEffectEnd::Once { fired } = effect.end_at {
            if !fired {
                effect.end_at = FieldAreaEffectEnd::Once { fired: true };
            }
        }
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
