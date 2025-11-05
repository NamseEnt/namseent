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

    /// 아이템이 현재 게임 상태에서 사용 가능한지 확인
    pub fn can_use(
        &self,
        game_state: &crate::game_state::GameState,
    ) -> Result<(), crate::game_state::effect::EffectExecutionError> {
        self.effect.can_execute(game_state)
    }

    /// 아이템을 사용할 수 없는 이유를 사용자에게 보여줄 메시지로 반환
    pub fn usage_error_message(
        &self,
        error: &crate::game_state::effect::EffectExecutionError,
        text_manager: &crate::l10n::TextManager,
    ) -> String {
        self.effect.execution_error_message(error, text_manager)
    }
}
