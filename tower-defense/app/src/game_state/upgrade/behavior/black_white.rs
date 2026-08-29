use crate::card::Suit;

use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BlackWhiteUpgrade;

impl UpgradePresentation for BlackWhiteUpgrade {
    fn key(&self) -> &'static str {
        "black_white"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BLACK_WHITE)
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
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BlackWhite(BlackWhiteUpgrade)
    }
}
