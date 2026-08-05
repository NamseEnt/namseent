use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct CandyItem {
    pub heal_amount: f32,
}

impl CandyItem {
    pub fn new(heal_amount: f32) -> Self {
        Self { heal_amount }
    }

    pub fn standard() -> Self {
        Self::new(3.0)
    }

    pub fn into_item(self) -> Item {
        Item::Candy(self)
    }
}

impl ItemBehavior for CandyItem {
    fn key(&self) -> &'static str {
        "candy"
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
            crate::l10n::Language::Korean => "사탕",
            crate::l10n::Language::English => "Candy",
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
                    .static_text(" 회복합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .static_text("Recover ")
                    .with_health_value(format!("{:.0}", self.heal_amount))
                    .static_text(" ")
                    .l10n(Word::Health.name(), locale)
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
            crate::asset::image::thumbnail::CANDY,
            width_height,
            stroke_px,
            shadow,
        )
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_candy_item, || {
        crate::Rarity::Common
    });

fn generate_candy_item(_rng: &mut dyn rand::RngCore) -> Item {
    CandyItem::standard().into_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generator_returns_standard_candy() {
        let mut rng = StdRng::seed_from_u64(1);

        assert_eq!(
            generate_candy_item(&mut rng),
            CandyItem::standard().into_item()
        );
    }

    #[test]
    fn candy_has_common_rarity() {
        assert_eq!(
            CandyItem::standard().into_item().discriminant(),
            ItemDiscriminants::Candy
        );
        assert_eq!(ItemDiscriminants::Candy.rarity(), crate::Rarity::Common);
    }

    #[test]
    fn using_candy_heals_three_health() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let max_hp = game_state.max_hp();
        game_state.hp = max_hp - 5.0;

        CandyItem::standard().use_item(&mut game_state);

        assert_eq!(game_state.hp, max_hp - 2.0);
    }
}
