use crate::{
    icon::IconKind,
    l10n::{Language, Locale, LocalizedText, rich_text_helpers::RichTextHelpers, word::WordName},
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
            super::Word::Health => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::RED)
                    .with_icon_bold(IconKind::Health, "체력");
            }),
            super::Word::Gold => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::YELLOW)
                    .with_icon_bold(IconKind::Gold, "골드");
            }),
            super::Word::Dice => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::BLUE)
                    .with_icon_bold(IconKind::Refresh, "주사위");
            }),
            super::Word::Item => builder.with_style(|builder| {
                builder.with_icon_bold(IconKind::Item, "아이템");
            }),
            super::Word::Treasure => builder.with_style(|builder| {
                builder.with_icon_bold(IconKind::Treasure, "보물");
            }),
            super::Word::Shield => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::GREEN)
                    .with_icon_bold(IconKind::Shield, "보호막");
            }),
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self.0 {
            super::Word::Health => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::RED)
                    .with_icon_bold(IconKind::Refresh, "Health");
            }),
            super::Word::Gold => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::YELLOW)
                    .with_icon_bold(IconKind::Gold, "Gold");
            }),
            super::Word::Dice => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::BLUE)
                    .with_icon_bold(IconKind::Refresh, "Dice");
            }),
            super::Word::Item => builder.with_style(|builder| {
                builder.with_icon_bold(IconKind::Item, "Item");
            }),
            super::Word::Treasure => builder.with_style(|builder| {
                builder.with_icon_bold(IconKind::Refresh, "Treasure");
            }),
            super::Word::Shield => builder.with_style(|builder| {
                builder
                    .color(crate::theme::palette::GREEN)
                    .with_icon_bold(IconKind::Refresh, "Shield");
            }),
        };
    }
}
