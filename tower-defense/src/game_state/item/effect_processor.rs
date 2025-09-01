use crate::{
    MapCoordF32,
    card::{Rank, Suit},
    game_state::{
        GameState,
        field_area_effect::{FieldAreaEffect, FieldAreaEffectKind},
        field_particle::{FieldParticleKind, emit_field_particle},
        item::{check_point_is_in_linear_area, linear_area_rect_points},
        schedule::CountBasedSchedule,
        upgrade::UpgradeState,
        user_status_effect::{UserStatusEffect, UserStatusEffectKind},
    },
};
use namui::*;

#[derive(Clone, Debug)]
pub enum ItemEffectKind {
    UserDamageReduction {
        multiply: f32,
        duration: Duration,
    },
    FieldArea {
        effect: FieldAreaEffectKind,
        schedule: CountBasedSchedule,
    },
    Direct {
        effect: DirectEffectKind,
    },
}

#[derive(Clone, Debug)]
pub enum DirectEffectKind {
    Heal {
        amount: f32,
    },
    Shield {
        amount: f32,
    },
    ExtraReroll,
    EarnGold {
        amount: usize,
    },
    RoundDamage {
        rank: crate::card::Rank,
        suit: crate::card::Suit,
        damage: f32,
        xy: MapCoordF32,
        radius: f32,
    },
    LinearDamage {
        rank: crate::card::Rank,
        suit: crate::card::Suit,
        damage: f32,
        center_xy: MapCoordF32,
        target_xy: MapCoordF32,
        thickness: f32,
    },
}

pub fn process_item_effect(game_state: &mut GameState, effect_kind: ItemEffectKind) {
    match effect_kind {
        ItemEffectKind::UserDamageReduction { multiply, duration } => {
            let status_effect = UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply: multiply,
                },
                end_at: game_state.now() + duration,
            };
            game_state.user_status_effects.push(status_effect);
        }
        ItemEffectKind::FieldArea { effect, schedule } => {
            create_field_area_effect(game_state, effect, schedule);
        }
        ItemEffectKind::Direct { effect } => {
            apply_direct_effect(game_state, effect);
        }
    }
}

fn create_field_area_effect(
    game_state: &mut GameState,
    effect: FieldAreaEffectKind,
    schedule: CountBasedSchedule,
) {
    let field_area_effect = FieldAreaEffect::new(effect.clone(), schedule);
    emit_field_particle(
        game_state,
        FieldParticleKind::FieldAreaEffect {
            field_area_effect: &field_area_effect,
        },
    );
    game_state.field_area_effects.push(field_area_effect);
}

fn apply_direct_effect(game_state: &mut GameState, effect: DirectEffectKind) {
    match effect {
        DirectEffectKind::Heal { amount } => {
            game_state.hp = (game_state.hp + amount).min(crate::game_state::MAX_HP);
        }
        DirectEffectKind::Shield { amount } => {
            game_state.shield += amount;
        }
        DirectEffectKind::ExtraReroll => {
            game_state.left_reroll_chance += 1;
        }
        DirectEffectKind::EarnGold { amount } => {
            game_state.earn_gold(amount);
        }
        DirectEffectKind::RoundDamage {
            rank,
            suit,
            damage,
            xy,
            radius,
        } => {
            apply_instant_round_damage(game_state, rank, suit, damage, xy, radius);
        }
        DirectEffectKind::LinearDamage {
            rank,
            suit,
            damage,
            center_xy,
            target_xy,
            thickness,
        } => {
            apply_instant_linear_damage(
                game_state, rank, suit, damage, center_xy, target_xy, thickness,
            );
        }
    }
}

pub fn apply_rank_and_suit_upgrades(
    upgrade_state: &UpgradeState,
    rank: Rank,
    suit: Suit,
    damage: &mut f32,
) {
    use crate::game_state::upgrade::TowerUpgradeTarget;

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

pub struct DamageApplicationResult {
    pub total_damage_dealt: f32,
    pub total_gold_earned: usize,
}

impl DamageApplicationResult {
    pub fn new() -> Self {
        Self {
            total_damage_dealt: 0.0,
            total_gold_earned: 0,
        }
    }

    pub fn add_damage(&mut self, damage: f32) {
        self.total_damage_dealt += damage;
    }

    pub fn add_gold(&mut self, gold: usize) {
        self.total_gold_earned += gold;
    }

    pub fn finalize(self, game_state: &mut GameState) {
        if self.total_gold_earned > 0 {
            game_state.earn_gold(self.total_gold_earned);
        }
    }
}

pub fn process_monster_damage<F>(
    game_state: &mut GameState,
    damage: f32,
    area_filter: F,
) -> DamageApplicationResult
where
    F: Fn(&crate::game_state::monster::Monster) -> bool,
{
    let mut result = DamageApplicationResult::new();

    game_state.monsters.retain_mut(|monster| {
        if !area_filter(monster) {
            return true;
        }

        monster.get_damage(damage);
        result.add_damage(damage);

        if monster.dead() {
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            result.add_gold(earn);
            return false;
        }

        true
    });

    result
}

fn apply_instant_round_damage(
    game_state: &mut GameState,
    rank: Rank,
    suit: Suit,
    damage: f32,
    xy: MapCoordF32,
    radius: f32,
) {
    let mut total_damage = damage;
    apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut total_damage);

    let result = process_monster_damage(game_state, total_damage, |monster| {
        monster.xy().distance(xy) <= radius
    });

    result.finalize(game_state);
}

fn apply_instant_linear_damage(
    game_state: &mut GameState,
    rank: Rank,
    suit: Suit,
    damage: f32,
    center_xy: MapCoordF32,
    target_xy: MapCoordF32,
    thickness: f32,
) {
    let mut total_damage = damage;
    apply_rank_and_suit_upgrades(&game_state.upgrade_state, rank, suit, &mut total_damage);

    let points = linear_area_rect_points(center_xy, target_xy, thickness);
    let result = process_monster_damage(game_state, total_damage, |monster| {
        check_point_is_in_linear_area(&points, monster.xy())
    });

    result.finalize(game_state);
}
