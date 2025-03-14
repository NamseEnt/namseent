use super::GameState;
use crate::{
    MapCoordF32,
    card::{Rank, Suit},
    upgrade::TowerUpgradeTarget,
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

                let rank_upgrades = game_state
                    .upgrade_state
                    .tower_upgrade_states
                    .get(&TowerUpgradeTarget::Rank { rank });
                let suit_upgrades = game_state
                    .upgrade_state
                    .tower_upgrade_states
                    .get(&TowerUpgradeTarget::Suit { suit });

                for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
                    damage += upgrade.damage_plus;
                }
                for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
                    damage *= upgrade.damage_multiplier;
                }

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

                let rank_upgrades = game_state
                    .upgrade_state
                    .tower_upgrade_states
                    .get(&TowerUpgradeTarget::Rank { rank });
                let suit_upgrades = game_state
                    .upgrade_state
                    .tower_upgrade_states
                    .get(&TowerUpgradeTarget::Suit { suit });

                for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
                    damage += upgrade.damage_plus;
                }
                for upgrade in rank_upgrades.iter().chain(suit_upgrades.iter()) {
                    damage *= upgrade.damage_multiplier;
                }

                for monster in game_state.monsters.iter_mut() {
                    if monster.xy().distance(xy) > radius {
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

pub fn remove_finished_field_area_effects(game_state: &mut GameState, now: Instant) {
    game_state.field_area_effects.retain(|e| match e.end_at {
        FieldAreaEffectEnd::AtTime { end_at } => now < end_at,
        FieldAreaEffectEnd::Once { fired } => !fired,
    });
}
