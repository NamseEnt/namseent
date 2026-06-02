use crate::game_state::item::Item;
use crate::l10n::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;
use namui::*;

#[derive(Debug, Clone, State)]
pub enum ItemText {
    Name(Item),
    Description(Item),
}

impl LocalizedText for ItemText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl ItemText {
    fn apply_korean<'b>(self, builder: &mut TypographyBuilder<'b>) {
        match self {
            ItemText::Name(item) => item.l10n_name(builder, &Locale::KOREAN),
            ItemText::Description(item) => {
                item.l10n_description(builder, &Locale::KOREAN);
            }
        }
    }

    fn apply_english<'b>(self, builder: &mut TypographyBuilder<'b>) {
        match self {
            ItemText::Name(item) => item.l10n_name(builder, &Locale::ENGLISH),
            ItemText::Description(item) => {
                item.l10n_description(builder, &Locale::ENGLISH);
            }
        }
    }
}
