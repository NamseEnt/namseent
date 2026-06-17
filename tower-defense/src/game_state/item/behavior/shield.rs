use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

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
    fn key(&self) -> &'static str {
        "shield"
    }

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
            crate::l10n::Language::Korean => "보호막",
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
                    .l10n(Word::Shield.name(), locale)
                    .with_shield_value(format!(" +{:.0}", self.shield_amount));
            }
            crate::l10n::Language::English => {
                builder
                    .l10n(Word::Shield.name(), locale)
                    .with_shield_value(format!(" +{:.0}", self.shield_amount));
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
