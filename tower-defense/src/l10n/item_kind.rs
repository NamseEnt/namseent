use crate::l10n::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;
use namui::*;

#[derive(Debug, Clone, State)]
pub enum ItemKindText {
    Name(crate::game_state::item::ItemKind),
    Description(crate::game_state::item::ItemKind),
}

impl LocalizedText for ItemKindText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl ItemKindText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            ItemKindText::Name(kind) => match kind {
                crate::game_state::item::ItemKind::RiceCake => {
                    builder.text("찹쌀떡");
                }
                crate::game_state::item::ItemKind::EmergencyDice => {
                    builder.text("비상용 주사위");
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
                    builder.text("비상용 카드");
                }
            },
            ItemKindText::Description(kind) => match kind {
                crate::game_state::item::ItemKind::RiceCake => {
                    builder.text("체력을 회복하는 찹쌀떡");
                }
                crate::game_state::item::ItemKind::EmergencyDice => {
                    builder.text("주사위 기회를 추가로 제공합니다");
                }
                crate::game_state::item::ItemKind::Shield => {
                    builder.text("피해를 흡수하는 방어막을 생성합니다");
                }
                crate::game_state::item::ItemKind::Painkiller => {
                    builder.text("일시적으로 피해를 감소시킵니다");
                }
                crate::game_state::item::ItemKind::GrantBarricades => {
                    builder.text("바리케이드 타워 카드를 추가로 제공합니다");
                }
                crate::game_state::item::ItemKind::GrantCard { card } => {
                    builder.text(format!("패에 {}{} 카드를 추가합니다", card.suit, card.rank));
                }
            },
        }
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            ItemKindText::Name(kind) => match kind {
                crate::game_state::item::ItemKind::RiceCake => {
                    builder.text("Rice Cake");
                }
                crate::game_state::item::ItemKind::EmergencyDice => {
                    builder.text("Emergency Dice");
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
            ItemKindText::Description(kind) => match kind {
                crate::game_state::item::ItemKind::RiceCake => {
                    builder.text("Heals your HP.");
                }
                crate::game_state::item::ItemKind::EmergencyDice => {
                    builder.text("Grants an extra reroll chance.");
                }
                crate::game_state::item::ItemKind::Shield => {
                    builder.text("Grants a damage-absorbing shield.");
                }
                crate::game_state::item::ItemKind::Painkiller => {
                    builder.text("Reduces incoming damage temporarily.");
                }
                crate::game_state::item::ItemKind::GrantBarricades => {
                    builder.text("Grants barricade tower cards.");
                }
                crate::game_state::item::ItemKind::GrantCard { card } => {
                    builder.text(format!("Add {}{} to your hand.", card.rank, card.suit));
                }
            },
        }
    }
}
