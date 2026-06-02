use super::*;

use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct ShieldItem {
    pub shield_amount: f32,
}

impl ShieldItem {
    pub fn new(shield_amount: f32) -> Self {
        Self { shield_amount }
    }

    pub fn standard() -> Self {
        Self::new(25.0)
    }

    pub fn into_item(self) -> Item {
        Item::Shield(self)
    }
}

impl ItemBehavior for ShieldItem {
    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        game_state.action(crate::game_state::GameStateAction::GainShield(
            self.shield_amount,
        ));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "방어막",
            crate::l10n::Language::English => "Shield",
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
                    .with_icon_bold(IconKind::Shield, format!("+{:.0}", self.shield_amount))
                    .static_text(" 보호막");
            }
            crate::l10n::Language::English => {
                builder
                    .with_icon_bold(IconKind::Shield, format!("+{:.0}", self.shield_amount))
                    .static_text(" shield");
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
            crate::asset::image::thumbnail::SHIELD,
            width_height,
            stroke_px,
            shadow,
        )
    }
}
