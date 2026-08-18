use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct DonutItem {
    pub heal_amount: f32,
}

impl DonutItem {
    pub fn new(heal_amount: f32) -> Self {
        Self { heal_amount }
    }

    pub fn standard() -> Self {
        Self::new(7.0)
    }

    pub fn into_item(self) -> Item {
        Item::Donut(self)
    }
}

impl ItemBehavior for DonutItem {
    fn key(&self) -> &'static str {
        "donut"
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
            crate::l10n::Language::Korean => "도넛",
            crate::l10n::Language::English => "Donut",
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
                    .with_bold(format!("{:.0}", self.heal_amount))
                    .static_text(" 회복합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .static_text("Recover ")
                    .with_bold(format!("{:.0}", self.heal_amount))
                    .static_text(" ")
                    .l10n(Word::Health.name(), locale)
                    .static_text(".");
            }
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::DONUT)
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_donut_item, || {
        crate::Rarity::Rare
    });

fn generate_donut_item(_rng: &mut dyn rand::RngCore) -> Item {
    DonutItem::standard().into_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generator_returns_standard_donut() {
        let mut rng = StdRng::seed_from_u64(1);

        assert_eq!(
            generate_donut_item(&mut rng),
            DonutItem::standard().into_item()
        );
    }

    #[test]
    fn donut_has_rare_rarity() {
        assert_eq!(
            DonutItem::standard().into_item().discriminant(),
            ItemDiscriminants::Donut
        );
        assert_eq!(ItemDiscriminants::Donut.rarity(), crate::Rarity::Rare);
    }

    #[test]
    fn using_donut_heals_seven_health() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let max_hp = game_state.max_hp();
        game_state.hp = max_hp - 10.0;

        DonutItem::standard().use_item(&mut game_state);

        assert_eq!(game_state.hp, max_hp - 3.0);
    }
}
