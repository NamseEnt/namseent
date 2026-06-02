use super::*;

use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct LumpSugarItem {
    pub reroll_amount: usize,
}

impl LumpSugarItem {
    pub fn new(reroll_amount: usize) -> Self {
        Self { reroll_amount }
    }

    pub fn standard() -> Self {
        Self::new(1)
    }

    pub fn into_item(self) -> Item {
        Item::LumpSugar(self)
    }
}

impl ItemBehavior for LumpSugarItem {
    fn can_use(&self, game_state: &crate::game_state::GameState) -> Result<(), ItemUseError> {
        if game_state.stage_modifiers.is_item_use_disabled() {
            return Err(ItemUseError::ItemUseDisabled);
        }
        if !matches!(
            game_state.flow,
            crate::game_state::flow::GameFlow::SelectingTower(_)
        ) {
            return Err(ItemUseError::InvalidFlow {
                required: "SelectingTower".to_string(),
            });
        }
        Ok(())
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        game_state.action(crate::game_state::GameStateAction::GainRerolls(
            self.reroll_amount,
        ));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "각설탕",
            crate::l10n::Language::English => "Lump Sugar",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::Language::Korean => {
                builder
                    .with_icon_bold(IconKind::Refresh, format!("+{}", self.reroll_amount))
                    .static_text(" 리롤");
            }
            crate::l10n::Language::English => {
                builder
                    .with_icon_bold(IconKind::Refresh, format!("+{}", self.reroll_amount))
                    .static_text(" reroll");
            }
        }
    }

    fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree {
        render_sticker(
            crate::asset::image::thumbnail::LUMP_SUGAR,
            width_height,
            stroke_px,
            shadow,
        )
    }
}
