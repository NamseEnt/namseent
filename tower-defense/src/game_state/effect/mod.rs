use crate::game_state::{
    GameState,
    user_status_effect::{UserStatusEffect, UserStatusEffectKind},
};
use namui::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
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
    Lottery {
        amount: f32,
        probability: f32,
    },
    DamageReduction {
        damage_multiply: f32,
        duration: Duration,
    },
    UserDamageReduction {
        multiply: f32,
        duration: Duration,
    },
}

pub fn run_effect(game_state: &mut GameState, effect: &Effect) {
    match effect {
        Effect::Heal { amount } => {
            game_state.hp = (game_state.hp + amount).min(crate::game_state::MAX_HP);
        }
        Effect::Shield { amount } => {
            game_state.shield += amount;
        }
        Effect::ExtraReroll => {
            game_state.left_reroll_chance += 1;
        }
        Effect::EarnGold { amount } => {
            game_state.earn_gold(*amount);
        }
        Effect::Lottery {
            amount,
            probability,
        } => {
            use rand::{Rng, thread_rng};
            let is_winner = thread_rng().gen_bool(*probability as f64);
            let gold = if is_winner { *amount as usize } else { 0 };
            game_state.earn_gold(gold);
        }
        Effect::DamageReduction {
            damage_multiply,
            duration,
        } => {
            let status_effect = UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply: *damage_multiply,
                },
                end_at: game_state.now() + *duration,
            };
            game_state.user_status_effects.push(status_effect);
        }
        Effect::UserDamageReduction { multiply, duration } => {
            let status_effect = UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply: *multiply,
                },
                end_at: game_state.now() + *duration,
            };
            game_state.user_status_effects.push(status_effect);
        }
    }
}

impl Effect {
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.effect_name(self)
    }

    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.effect_description(self)
    }
}
