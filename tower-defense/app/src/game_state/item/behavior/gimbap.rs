use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct GimbapItem {
    pub heal_amount: crate::Health,
    pub shield_amount: crate::Shield,
}

impl GimbapItem {
    pub fn new(heal_amount: crate::Health, shield_amount: crate::Shield) -> Self {
        Self {
            heal_amount,
            shield_amount,
        }
    }

    pub fn standard() -> Self {
        Self::new(
            crate::Health::from_integer(9),
            crate::Shield::from_integer(9),
        )
    }

    pub fn into_item(self) -> Item {
        Item::Gimbap(self)
    }
}

impl ItemBehavior for GimbapItem {
    fn key(&self) -> &'static str {
        "gimbap"
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "김밥",
            crate::l10n::Language::English => "Gimbap",
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
                    .static_text(" 회복하고, ")
                    .l10n(Word::Shield.name(), locale)
                    .static_text("을 ")
                    .with_bold(format!("{:.0}", self.shield_amount))
                    .static_text(" 획득합니다.");
            }
            crate::l10n::Language::English => {
                builder
                    .static_text("Recover ")
                    .with_bold(format!("{:.0}", self.heal_amount))
                    .static_text(" ")
                    .l10n(Word::Health.name(), locale)
                    .static_text(" and gain ")
                    .with_bold(format!("{:.0}", self.shield_amount))
                    .static_text(" ")
                    .l10n(Word::Shield.name(), locale)
                    .static_text(".");
            }
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::GIMBAP)
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            Word::Shield.tooltip_section(locale),
        ]
    }
}
