pub enum TopBarText {
    Hp,
    Gold,
    Stage,
    Level,
    LevelUp,
    RarityCommon,
    RarityRare,
    RarityEpic,
    RarityLegendary,
    To,
    Quest,
    Refresh,
    Locked,
    Accepted,
    Accept,
    Inventory,
    Use,
    Remove,
    Settings,
    Close,
    Shop,     // 상점
    SoldOut,  // 품절
    UseTower, // 타워 사용
}

impl TopBarText {
    pub fn to_korean(&self) -> &'static str {
        match self {
            TopBarText::Hp => "HP",
            TopBarText::Gold => "골드",
            TopBarText::Stage => "스테이지",
            TopBarText::Level => "레벨",
            TopBarText::LevelUp => "레벨업",
            TopBarText::RarityCommon => "커먼",
            TopBarText::RarityRare => "레어",
            TopBarText::RarityEpic => "에픽",
            TopBarText::RarityLegendary => "레전더리",
            TopBarText::To => ">>>",
            TopBarText::Quest => "퀘스트",
            TopBarText::Refresh => "새로고침",
            TopBarText::Locked => "잠김",
            TopBarText::Accepted => "수락됨",
            TopBarText::Accept => "수락",
            TopBarText::Inventory => "인벤토리",
            TopBarText::Use => "사용",
            TopBarText::Remove => "X",
            TopBarText::Settings => "설정",
            TopBarText::Close => "닫기",
            TopBarText::Shop => "상점",
            TopBarText::SoldOut => "품절",
            TopBarText::UseTower => "타워 사용",
        }
    }
    pub fn to_english(&self) -> &'static str {
        match self {
            TopBarText::Hp => "HP",
            TopBarText::Gold => "Gold",
            TopBarText::Stage => "Stage",
            TopBarText::Level => "Level",
            TopBarText::LevelUp => "Level Up",
            TopBarText::RarityCommon => "Common",
            TopBarText::RarityRare => "Rare",
            TopBarText::RarityEpic => "Epic",
            TopBarText::RarityLegendary => "Legendary",
            TopBarText::To => ">>>",
            TopBarText::Quest => "Quest",
            TopBarText::Refresh => "Refresh",
            TopBarText::Locked => "Locked",
            TopBarText::Accepted => "Accepted",
            TopBarText::Accept => "Accept",
            TopBarText::Inventory => "Inventory",
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
