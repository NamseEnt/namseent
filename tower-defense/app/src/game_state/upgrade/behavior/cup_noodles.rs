use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const CUP_NOODLES_MAX_HP_DECREASE: HealthDelta = HealthDelta::from_integer(2);
const CUP_NOODLES_HEAL_AMOUNT: Health = Health::from_integer(6);

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CupNoodlesUpgrade;

impl UpgradePresentation for CupNoodlesUpgrade {
    fn key(&self) -> &'static str {
        "cup_noodles"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::CUP_NOODLES)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Cup Noodles",
            crate::l10n::locale::Language::Korean => "컵라면",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .with_bold("Max Health")
                .static_text(" decreased by ")
                .with_bold(format!("{:.0}", CUP_NOODLES_MAX_HP_DECREASE))
                .static_text(", ")
                .with_bold("Health")
                .static_text(" recovered by ")
                .with_bold(format!("{:.0}", CUP_NOODLES_HEAL_AMOUNT))
                .static_text("."),
            crate::l10n::locale::Language::Korean => builder
                .with_bold("최대 체력")
                .static_text("을 ")
                .with_bold(format!("{:.0}", CUP_NOODLES_MAX_HP_DECREASE))
                .static_text(" 줄이고, ")
                .with_bold("체력을 ")
                .with_bold(format!("{:.0}", CUP_NOODLES_HEAL_AMOUNT))
                .static_text(" 회복합니다."),
        };
    }
}

impl CupNoodlesUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::CupNoodles(CupNoodlesUpgrade)
    }
}
