use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct CarrotItem {
    pub max_hp_amount: f32,
}

impl CarrotItem {
    pub fn new(max_hp_amount: f32) -> Self {
        Self { max_hp_amount }
    }

    pub fn standard() -> Self {
        Self::new(6.0)
    }

    pub fn into_item(self) -> Item {
        Item::Carrot(self)
    }
}

impl ItemBehavior for CarrotItem {
    fn key(&self) -> &'static str {
        "carrot"
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        std::sync::Arc::make_mut(&mut game_state.config)
            .player
            .max_hp += self.max_hp_amount;
        game_state.action(crate::game_state::GameStateAction::Heal(
            game_state.max_hp() - game_state.hp,
        ));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "당근",
            crate::l10n::Language::English => "Carrot",
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
                    .static_text("최대 체력을 ")
                    .with_health_value(format!("{:.0}", self.max_hp_amount))
                    .static_text(" 늘리고, ")
                    .l10n(Word::Health.name(), locale)
                    .static_text("을 모두 회복합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .with_health_value(format!("Increase max Health by {:.0}", self.max_hp_amount))
                    .static_text(" and fully recover ")
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
            crate::asset::image::thumbnail::CARROT,
            width_height,
            stroke_px,
            shadow,
        )
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_carrot_item, || {
        crate::Rarity::Legendary
    });

fn generate_carrot_item(_rng: &mut dyn rand::RngCore) -> Item {
    CarrotItem::standard().into_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generator_returns_standard_carrot() {
        let mut rng = StdRng::seed_from_u64(1);

        assert_eq!(
            generate_carrot_item(&mut rng),
            CarrotItem::standard().into_item()
        );
    }

    #[test]
    fn carrot_has_legendary_rarity() {
        assert_eq!(
            CarrotItem::standard().into_item().discriminant(),
            ItemDiscriminants::Carrot
        );
        assert_eq!(ItemDiscriminants::Carrot.rarity(), crate::Rarity::Legendary);
    }

    #[test]
    fn using_carrot_increases_max_health_and_fully_heals() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let base_max_hp = game_state.max_hp();
        game_state.hp = base_max_hp - 10.0;

        CarrotItem::standard().use_item(&mut game_state);

        assert_eq!(game_state.max_hp(), base_max_hp + 6.0);
        assert_eq!(game_state.hp, game_state.max_hp());
    }
}
