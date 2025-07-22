use super::{Language, Locale, LocalizedStaticText};

#[derive(Debug, Clone, Copy)]
pub enum TopBarText {
    Stage,
    LevelUp,
    RarityCommon,
    RarityRare,
    RarityEpic,
    RarityLegendary,
    To,
    Quest,
    Refresh,
    Accepted,
    Accept,
    Use,
    Remove,
    Settings,
    Close,
    Shop,
    SoldOut,
    UseTower,
}

impl LocalizedStaticText for TopBarText {
    fn localized_text(&self, locale: &Locale) -> &'static str {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl TopBarText {
    pub(super) fn to_korean(&self) -> &'static str {
        match self {
            TopBarText::Stage => "스테이지",
            TopBarText::LevelUp => "레벨업",
            TopBarText::RarityCommon => "커먼",
            TopBarText::RarityRare => "레어",
            TopBarText::RarityEpic => "에픽",
            TopBarText::RarityLegendary => "레전더리",
            TopBarText::To => ">>>",
            TopBarText::Quest => "퀘스트",
            TopBarText::Refresh => "새로고침",
            TopBarText::Accepted => "수락됨",
            TopBarText::Accept => "수락",
            TopBarText::Use => "사용",
            TopBarText::Remove => "X",
            TopBarText::Settings => "설정",
            TopBarText::Close => "닫기",
            TopBarText::Shop => "상점",
            TopBarText::SoldOut => "품절",
            TopBarText::UseTower => "타워 사용",
        }
    }

    pub(super) fn to_english(&self) -> &'static str {
        match self {
            TopBarText::Stage => "Stage",
            TopBarText::LevelUp => "Level Up",
            TopBarText::RarityCommon => "Common",
            TopBarText::RarityRare => "Rare",
            TopBarText::RarityEpic => "Epic",
            TopBarText::RarityLegendary => "Legendary",
            TopBarText::To => ">>>",
            TopBarText::Quest => "Quest",
            TopBarText::Refresh => "Refresh",
            TopBarText::Accepted => "Accepted",
            TopBarText::Accept => "Accept",
            TopBarText::Use => "Use",
            TopBarText::Remove => "X",
            TopBarText::Settings => "Settings",
            TopBarText::Close => "Close",
            TopBarText::Shop => "Shop",
            TopBarText::SoldOut => "Sold Out",
            TopBarText::UseTower => "Use Tower",
        }
    }
}
