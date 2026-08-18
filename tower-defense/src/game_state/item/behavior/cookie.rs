use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct CookieItem {
    pub heal_amount: f32,
}

impl CookieItem {
    pub fn new(heal_amount: f32) -> Self {
        Self { heal_amount }
    }

    pub fn standard() -> Self {
        Self::new(5.0)
    }

    pub fn into_item(self) -> Item {
        Item::Cookie(self)
    }
}

impl ItemBehavior for CookieItem {
    fn key(&self) -> &'static str {
        "cookie"
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
            crate::l10n::Language::Korean => "쿠키",
            crate::l10n::Language::English => "Cookie",
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
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::COOKIE)
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_cookie_item, || {
        crate::Rarity::Common
    });

fn generate_cookie_item(_rng: &mut dyn rand::RngCore) -> Item {
    CookieItem::standard().into_item()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::ItemDiscriminants;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generator_returns_standard_cookie() {
        let mut rng = StdRng::seed_from_u64(1);

        assert_eq!(
            generate_cookie_item(&mut rng),
            CookieItem::standard().into_item()
        );
    }

    #[test]
    fn cookie_has_common_rarity() {
        assert_eq!(
            CookieItem::standard().into_item().discriminant(),
            ItemDiscriminants::Cookie
        );
        assert_eq!(ItemDiscriminants::Cookie.rarity(), crate::Rarity::Common);
    }

    #[test]
    fn using_cookie_heals_five_health() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let max_hp = game_state.max_hp();
        game_state.hp = max_hp - 7.0;

        CookieItem::standard().use_item(&mut game_state);

        assert_eq!(game_state.hp, max_hp - 2.0);
    }
}
