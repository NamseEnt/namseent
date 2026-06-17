use crate::Suit;

use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BlackWhiteUpgrade;

impl UpgradeBehavior for BlackWhiteUpgrade {
    fn key(&self) -> &'static str {
        "black_white"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::BLACK_WHITE,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::CACHE | UpgradeUpdateFlags::REVISION
    }

    fn treat_suits_as_same(&self) -> bool {
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Black & White",
            crate::l10n::locale::Language::Korean => "흑백",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        _locale: &crate::l10n::Locale,
    ) {
        builder
            .card_suit(Suit::Spades)
            .static_text("=")
            .card_suit(Suit::Clubs)
            .static_text(", ")
            .card_suit(Suit::Hearts)
            .static_text("=")
            .card_suit(Suit::Diamonds);
    }
}

impl BlackWhiteUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BlackWhite(BlackWhiteUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    current_and_max,
    UpgradeDefinition::rarity_legendary,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BlackWhiteUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.treat_suits_as_same() as usize, 1))
}
