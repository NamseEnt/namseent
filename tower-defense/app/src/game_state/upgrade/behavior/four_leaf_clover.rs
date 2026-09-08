use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FourLeafCloverUpgrade;

impl UpgradePresentation for FourLeafCloverUpgrade {
    fn key(&self) -> &'static str {
        "four_leaf_clover"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::FOUR_LEAF_CLOVER)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Four Leaf Clover",
            crate::l10n::locale::Language::Korean => "네잎클로버",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Enable straight flush with 4 cards",
            crate::l10n::locale::Language::Korean => {
                "스트레이트와 플러시를 4장으로 만들 수 있습니다"
            }
        });
    }
}

impl FourLeafCloverUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::FourLeafClover(FourLeafCloverUpgrade)
    }
}
