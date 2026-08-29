use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct LumpSugarItem {
    pub reroll_amount: usize,
}

impl LumpSugarItem {
    pub fn new(reroll_amount: usize) -> Self {
        Self { reroll_amount }
    }

    pub fn standard() -> Self {
        Self::new(1)
    }

    pub fn into_item(self) -> Item {
        Item::LumpSugar(self)
    }
}

impl ItemBehavior for LumpSugarItem {
    fn key(&self) -> &'static str {
        "lump_sugar"
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "각설탕",
            crate::l10n::Language::English => "Lump Sugar",
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
                    .l10n(Word::Dice.name(), locale)
                    .with_bold(format!(" +{}", self.reroll_amount));
            }
            crate::l10n::Language::English => {
                builder
                    .l10n(Word::Dice.name(), locale)
                    .with_bold(format!(" +{}", self.reroll_amount));
            }
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::LUMP_SUGAR)
    }
}
