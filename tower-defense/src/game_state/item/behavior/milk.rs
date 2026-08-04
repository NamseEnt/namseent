use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct MilkItem {
    pub shield_amount: f32,
}

impl MilkItem {
    pub fn new(shield_amount: f32) -> Self {
        Self { shield_amount }
    }

    pub fn standard() -> Self {
        Self::new(12.0)
    }

    pub fn into_item(self) -> Item {
        Item::Milk(self)
    }
}

impl ItemBehavior for MilkItem {
    fn key(&self) -> &'static str {
        "milk"
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
            crate::l10n::Language::Korean => "우유",
            crate::l10n::Language::English => "Milk",
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
                    .static_text("을 ")
                    .with_shield_value(format!("{:.0}", self.shield_amount))
                    .static_text(" 획득합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .static_text("Gain ")
                    .with_shield_value(format!("{:.0}", self.shield_amount))
                    .static_text(" ")
                    .l10n(Word::Shield.name(), locale)
                    .static_text(".");
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
            crate::asset::image::thumbnail::MILK,
            width_height,
            stroke_px,
            shadow,
        )
    }
}
