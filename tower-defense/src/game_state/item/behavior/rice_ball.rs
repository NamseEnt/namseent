use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct RiceBallItem {
    pub heal_amount: f32,
    pub shield_amount: f32,
}

impl RiceBallItem {
    pub fn new(heal_amount: f32, shield_amount: f32) -> Self {
        Self {
            heal_amount,
            shield_amount,
        }
    }

    pub fn standard() -> Self {
        Self::new(3.0, 3.0)
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
                    .static_text("을 ")
                    .with_health_value(format!("{:.0}", self.heal_amount))
                    .static_text(" 회복하고, ")
                    .l10n(Word::Shield.name(), locale)
                    .static_text("을 ")
                    .with_shield_value(format!("{:.0}", self.shield_amount))
                    .static_text(" 획득합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .static_text("Recover ")
                    .with_health_value(format!("{:.0}", self.heal_amount))
                    .static_text(" ")
                    .l10n(Word::Health.name(), locale)
                    .static_text(" and gain ")
                    .with_shield_value(format!("{:.0}", self.shield_amount))
                    .static_text(" ")
                    .l10n(Word::Shield.name(), locale)
                    .static_text(".");
            }
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::RICE_BALL)
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            Word::Shield.tooltip_section(locale),
        ]
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_rice_ball_item, || {
        crate::Rarity::Common
    });

fn generate_rice_ball_item(_rng: &mut dyn rand::RngCore) -> Item {
    RiceBallItem::standard().into_item()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generator_returns_standard_rice_ball() {
        let mut rng = StdRng::seed_from_u64(1);

        assert_eq!(
            generate_rice_ball_item(&mut rng),
            RiceBallItem::standard().into_item()
        );
    }

    #[test]
    fn rice_ball_has_common_rarity() {
        assert_eq!(
            RiceBallItem::standard().into_item().discriminant(),
            ItemDiscriminants::RiceBall
        );
        assert_eq!(ItemDiscriminants::RiceBall.rarity(), crate::Rarity::Common);
    }

    #[test]
    fn using_rice_ball_heals_three_health_and_gains_three_shield() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let max_hp = game_state.max_hp();
        game_state.hp = max_hp - 10.0;

        RiceBallItem::standard().use_item(&mut game_state);

        assert_eq!(game_state.hp, max_hp - 7.0);
        assert_eq!(game_state.shield, 3.0);
    }
}
