use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BackpackUpgrade {
    pub add: usize,
}

impl UpgradePresentation for BackpackUpgrade {
    fn key(&self) -> &'static str {
        "backpack"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BACKPACK)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("{}", self.add),
            crate::theme::palette::WHITE,
        )]
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Backpack",
            crate::l10n::locale::Language::Korean => "배낭",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.with_bold("The shop offers 1 more item")
            }
            crate::l10n::locale::Language::Korean => {
                builder.with_bold("상점에 상품이 하나 더 표시됩니다")
            }
        };
    }
}

impl BackpackUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Backpack(BackpackUpgrade { add })
    }
}
