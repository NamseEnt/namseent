use crate::{
    l10n::{Language, Locale, LocalizedText, word::WordName},
    theme::typography::TypographyBuilder,
};

impl LocalizedText for WordName {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl WordName {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self.0 {
            super::Word::Health => builder.static_text("체력"),
            super::Word::Gold => builder.static_text("골드"),
            super::Word::Dice => builder.static_text("주사위"),
            super::Word::Deck => builder.static_text("덱"),
            super::Word::Encyclopedia => builder.static_text("백과사전"),
            super::Word::Item => builder.static_text("아이템"),
            super::Word::Treasure => builder.static_text("보물"),
            super::Word::Shield => builder.static_text("보호막"),
            super::Word::PerfectClear => builder.static_text("퍼펙트 클리어"),
            super::Word::CardService => builder.static_text("카드 서비스"),
            super::Word::Polish(_) => builder.static_text("연마"),
            super::Word::Engraving(engraving) => match engraving {
                Some(engraving) => {
                    engraving.l10n_name(builder, &Locale::KOREAN);
                    builder
                }
                None => builder.static_text("각인"),
            },
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self.0 {
            super::Word::Health => builder.static_text("Health"),
            super::Word::Gold => builder.static_text("Gold"),
            super::Word::Dice => builder.static_text("Dice"),
            super::Word::Deck => builder.static_text("Deck"),
            super::Word::Encyclopedia => builder.static_text("Encyclopedia"),
            super::Word::Item => builder.static_text("Item"),
            super::Word::Treasure => builder.static_text("Treasure"),
            super::Word::Shield => builder.static_text("Shield"),
            super::Word::PerfectClear => builder.static_text("Perfect clear"),
            super::Word::CardService => builder.static_text("Card Service"),
            super::Word::Polish(_) => builder.static_text("Polish"),
            super::Word::Engraving(engraving) => match engraving {
                Some(engraving) => {
                    engraving.l10n_name(builder, &Locale::ENGLISH);
                    builder
                }
                None => builder.static_text("Engraving"),
            },
        };
    }
}
