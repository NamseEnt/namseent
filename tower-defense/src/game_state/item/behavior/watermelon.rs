use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct WatermelonItem {
    pub max_hp_amount: f32,
    pub heal_amount: f32,
}

impl WatermelonItem {
    pub fn new(max_hp_amount: f32, heal_amount: f32) -> Self {
        Self {
            max_hp_amount,
            heal_amount,
        }
    }

    pub fn standard() -> Self {
        Self::new(8.0, 12.0)
    }

    pub fn into_item(self) -> Item {
        Item::Watermelon(self)
    }
}

impl ItemBehavior for WatermelonItem {
    fn key(&self) -> &'static str {
        "watermelon"
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        std::sync::Arc::make_mut(&mut game_state.config)
            .player
            .max_hp += self.max_hp_amount;
        game_state.action(crate::game_state::GameStateAction::Heal(self.heal_amount));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "수박",
            crate::l10n::Language::English => "Watermelon",
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
                    .static_text("을 ")
                    .with_health_value(format!("{:.0}", self.heal_amount))
                    .static_text(" 회복합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .with_health_value(format!("Increase max Health by {:.0}", self.max_hp_amount))
                    .static_text(" and recover ")
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
            crate::asset::image::thumbnail::WATERMELON,
            width_height,
            stroke_px,
            shadow,
        )
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_watermelon_item, || {
        crate::Rarity::Epic
    });

fn generate_watermelon_item(_rng: &mut dyn rand::RngCore) -> Item {
    WatermelonItem::standard().into_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generator_returns_standard_watermelon() {
        let mut rng = StdRng::seed_from_u64(1);

        assert_eq!(
            generate_watermelon_item(&mut rng),
            WatermelonItem::standard().into_item()
        );
    }

    #[test]
    fn watermelon_has_epic_rarity() {
        assert_eq!(
            WatermelonItem::standard().into_item().discriminant(),
            ItemDiscriminants::Watermelon
        );
        assert_eq!(ItemDiscriminants::Watermelon.rarity(), crate::Rarity::Epic);
    }

    #[test]
    fn using_watermelon_increases_max_health_and_heals_twelve_health() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let base_max_hp = game_state.max_hp();
        game_state.hp = base_max_hp - 16.0;

        WatermelonItem::standard().use_item(&mut game_state);

        assert_eq!(game_state.max_hp(), base_max_hp + 8.0);
        assert_eq!(game_state.hp, base_max_hp - 4.0);
    }
}
