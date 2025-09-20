use crate::game_state::{
    GameState,
    user_status_effect::{UserStatusEffect, UserStatusEffectKind},
};
use namui::*;

#[derive(Clone, Debug)]
pub enum ItemEffectKind {
    UserDamageReduction { multiply: f32, duration: Duration },
    Direct { effect: DirectEffectKind },
}

#[derive(Clone, Debug)]
pub enum DirectEffectKind {
    Heal { amount: f32 },
    Shield { amount: f32 },
    ExtraReroll,
    EarnGold { amount: usize },
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
        ItemEffectKind::Direct { effect } => {
            apply_direct_effect(game_state, effect);
        }
    }
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
    }
}
