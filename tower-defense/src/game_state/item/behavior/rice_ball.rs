use super::*;

use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct RiceBallItem {
    pub heal_amount: f32,
}

impl RiceBallItem {
    pub fn new(heal_amount: f32) -> Self {
        Self { heal_amount }
    }

    pub fn standard() -> Self {
        Self::new(14.0)
    }

    pub fn into_item(self) -> Item {
        Item::RiceBall(self)
    }
}

impl ItemBehavior for RiceBallItem {
    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        game_state.action(crate::game_state::GameStateAction::Heal(self.heal_amount));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "주먹밥",
            crate::l10n::Language::English => "Rice Ball",
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
                    .with_icon_bold(IconKind::Health, format!("+{:.0}", self.heal_amount))
                    .static_text(" 체력 회복");
            }
            crate::l10n::Language::English => {
                builder
                    .with_icon_bold(IconKind::Health, format!("+{:.0}", self.heal_amount))
                    .static_text(" HP");
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
            crate::asset::image::thumbnail::RICE_BALL,
            width_height,
            stroke_px,
            shadow,
        )
    }
}
