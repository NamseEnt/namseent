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

impl TowerKindText {
    fn to_korean(&self) -> &'static str {
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
}

pub trait TowerKindTextLocale {
    fn tower_kind_text(&self, text: &TowerKindText) -> &'static str;
}

impl TowerKindTextLocale for crate::l10n::upgrade::Locales {
    fn tower_kind_text(&self, text: &TowerKindText) -> &'static str {
        match self {
            crate::l10n::upgrade::Locales::KoKR(_) => text.to_korean(),
            // 다국어 확장 시 여기에 추가
        }
    }
}
