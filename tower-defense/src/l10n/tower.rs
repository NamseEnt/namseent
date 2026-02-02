use super::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;
use crate::*;

#[derive(Debug, Clone, Copy, State)]
pub enum TowerKindText {
    Barricade,
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl LocalizedText for TowerKindText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl TowerKindText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            TowerKindText::Barricade => "바리케이드",
            TowerKindText::High => "하이카드",
            TowerKindText::OnePair => "원페어",
            TowerKindText::TwoPair => "투페어",
            TowerKindText::ThreeOfAKind => "트리플",
            TowerKindText::Straight => "스트레이트",
            TowerKindText::Flush => "플러쉬",
            TowerKindText::FullHouse => "풀하우스",
            TowerKindText::FourOfAKind => "포카드",
            TowerKindText::StraightFlush => "스트레이트 플러쉬",
            TowerKindText::RoyalFlush => "로열 플러쉬",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            TowerKindText::Barricade => "Barricade",
            TowerKindText::High => "High",
            TowerKindText::OnePair => "One Pair",
            TowerKindText::TwoPair => "Two Pair",
            TowerKindText::ThreeOfAKind => "Three of a Kind",
            TowerKindText::Straight => "Straight",
            TowerKindText::Flush => "Flush",
            TowerKindText::FullHouse => "Full House",
            TowerKindText::FourOfAKind => "Four of a Kind",
            TowerKindText::StraightFlush => "Straight Flush",
            TowerKindText::RoyalFlush => "Royal Flush",
        }
    }
}
