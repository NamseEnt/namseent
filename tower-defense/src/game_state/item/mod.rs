pub mod generation;
mod thumbnail;
mod usage;

pub use crate::game_state::effect::Effect;
use crate::rarity::Rarity;
use namui::*;
pub use usage::*;

#[derive(Debug, Clone, PartialEq, State)]
pub struct Item {
    pub effect: Effect,
    pub rarity: Rarity,
    pub value: OneZero,
}

impl Item {
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        self.effect.name(text_manager)
    }

    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        self.effect.description(text_manager)
    }

    pub fn effect_kind(&self) -> &Effect {
        &self.effect
    }
}
