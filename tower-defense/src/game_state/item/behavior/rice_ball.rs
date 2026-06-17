use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

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
    fn key(&self) -> &'static str {
        "rice_ball"
    }

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
                    .l10n(Word::Health.name(), locale)
                    .with_health_value(format!(" +{:.0}", self.heal_amount));
            }
            crate::l10n::Language::English => {
                builder
                    .l10n(Word::Health.name(), locale)
                    .with_health_value(format!(" +{:.0}", self.heal_amount));
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
