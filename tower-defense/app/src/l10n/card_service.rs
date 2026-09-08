use crate::l10n::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;

#[derive(Debug, Clone)]
pub enum CardServiceText {
    Name(crate::game_state::card_service::CardService),
    Description(crate::game_state::card_service::CardService),
}

impl LocalizedText for CardServiceText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl CardServiceText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            CardServiceText::Name(_) => builder.text("카드 서비스"),
            CardServiceText::Description(_) => {
                builder.text("덱 카드 선택 효과를 제공하는 카드 서비스입니다.")
            }
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            CardServiceText::Name(_) => builder.text("Card Service"),
            CardServiceText::Description(_) => {
                builder.text("A card service with deck selection effects.")
            }
        };
    }
}
