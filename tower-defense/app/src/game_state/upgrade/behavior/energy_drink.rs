use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EnergyDrinkUpgrade {
    pub add: usize,
}

impl UpgradePresentation for EnergyDrinkUpgrade {
    fn key(&self) -> &'static str {
        "energy_drink"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::ENERGY_DRINK)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("-{}", self.add),
            crate::theme::palette::YELLOW,
        )]
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Energy Drink",
            crate::l10n::locale::Language::Korean => "에너지드링크",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Shop price ")
                .with_bold(format!("-{}", self.add)),
            crate::l10n::locale::Language::Korean => builder
                .static_text("상점 가격 ")
                .with_bold(format!("-{}", self.add)),
        };
    }
}

impl EnergyDrinkUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::EnergyDrink(EnergyDrinkUpgrade { add })
    }
}
