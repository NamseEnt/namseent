use crate::game_state::item::Item;
use crate::l10n::{Language, Locale, LocalizedText, effect::EffectText};
use crate::theme::typography::TypographyBuilder;
use namui::*;

#[derive(Debug, Clone, State)]
pub enum ItemText {
    Name(crate::game_state::item::ItemKind),
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
            ItemText::Name(kind) => match kind {
                crate::game_state::item::ItemKind::RiceBall => {
                    builder.text("주먹밥");
                }
                crate::game_state::item::ItemKind::LumpSugar => {
                    builder.text("각설탕");
                }
                crate::game_state::item::ItemKind::Shield => {
                    builder.text("방어막");
                }
                crate::game_state::item::ItemKind::Painkiller => {
                    builder.text("진통제");
                }
                crate::game_state::item::ItemKind::GrantBarricades => {
                    builder.text("바리케이드");
                }
                crate::game_state::item::ItemKind::GrantCard { .. } => {
                    builder.text("급조카드");
                }
            },
            ItemText::Description(item) => {
                builder.l10n(
                    EffectText::Description(item.effect.clone()),
                    &Locale::KOREAN,
                );
            }
        }
    }

    fn apply_english<'b>(self, builder: &mut TypographyBuilder<'b>) {
        match self {
            ItemText::Name(kind) => match kind {
                crate::game_state::item::ItemKind::RiceBall => {
                    builder.text("Rice Ball");
                }
                crate::game_state::item::ItemKind::LumpSugar => {
                    builder.text("Lump Sugar");
                }
                crate::game_state::item::ItemKind::Shield => {
                    builder.text("Shield");
                }
                crate::game_state::item::ItemKind::Painkiller => {
                    builder.text("Painkiller");
                }
                crate::game_state::item::ItemKind::GrantBarricades => {
                    builder.text("Barricades");
                }
                crate::game_state::item::ItemKind::GrantCard { .. } => {
                    builder.text("Emergency Card");
                }
            },
            ItemText::Description(item) => {
                builder.l10n(
                    EffectText::Description(item.effect.clone()),
                    &Locale::ENGLISH,
                );
            }
        }
    }
}
