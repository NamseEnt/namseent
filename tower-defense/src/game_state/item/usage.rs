use super::effect_processor::{DirectEffectKind, ItemEffectKind};
use super::{Item, ItemKind};
use crate::game_state::GameState;
use namui::*;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub enum ItemUsage {
    Instant,
}

impl ItemKind {
    pub fn usage(&self) -> ItemUsage {
        match self {
            ItemKind::Heal { .. }
            | ItemKind::Lottery { .. }
            | ItemKind::ExtraReroll
            | ItemKind::Shield { .. }
            | ItemKind::DamageReduction { .. } => ItemUsage::Instant,
        }
    }

    pub fn effect_kind(&self) -> ItemEffectKind {
        match self {
            ItemKind::Heal { amount } => ItemEffectKind::Direct {
                effect: DirectEffectKind::Heal { amount: *amount },
            },
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
}

pub fn use_item(game_state: &mut GameState, item: &Item) {
    game_state.use_item(item);
}
