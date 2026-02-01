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
    pub fn name_text(&self) -> crate::l10n::effect::EffectText {
        self.effect.name_text()
    }

    pub fn description_text(&self) -> crate::l10n::effect::EffectText {
        self.effect.description_text()
    }

    /// 아이템이 현재 게임 상태에서 사용 가능한지 확인
    pub fn can_use(
        &self,
        game_state: &crate::game_state::GameState,
    ) -> Result<(), crate::game_state::effect::EffectExecutionError> {
        self.effect.can_execute(game_state)
    }

    /// 아이템을 사용할 수 없는 이유를 사용자에게 보여줄 메시지로 반환
    pub fn usage_error_message<'a>(
        &self,
        error: &crate::game_state::effect::EffectExecutionError,
        text_manager: &crate::l10n::TextManager,
        builder: crate::theme::typography::TypographyBuilder<'a>,
    ) -> crate::theme::typography::TypographyBuilder<'a> {
        self.effect
            .execution_error_message(error, text_manager, builder)
    }
}
