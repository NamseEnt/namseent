use super::{Locale, Language, LocalizedStaticText};

#[derive(Debug, Clone, Copy)]
pub enum UpgradeBoardText {
    Title,
    GoldEarnPlus,
    ShopSlotExpand,
    QuestSlotExpand,
    QuestBoardSlotExpand,
    RerollChancePlus,
    ShopItemPriceMinus,
    ShopRefreshChancePlus,
    QuestBoardRefreshChancePlus,
    ShortenStraightFlushTo4Cards,
    SkipRankForStraight,
    TreatSuitsAsSame,
    TowerSelectLowCard,
    TowerSelectNoReroll,
    TowerSelectReroll,
    TowerUpgradeRank,
    TowerUpgradeSuit,
    TowerUpgradeKind,
    TowerUpgradeEvenOdd,
    TowerUpgradeFaceNumber,
    DamagePlus,
    DamageMultiplier,
    SpeedPlus,
    SpeedMultiplier,
    RangePlus,
}

impl LocalizedStaticText for UpgradeBoardText {
    fn localized_text(&self, locale: &Locale) -> &'static str {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl UpgradeBoardText {
    pub(super) fn to_korean(&self) -> &'static str {
        match self {
            UpgradeBoardText::Title => "강화 정보",
            UpgradeBoardText::GoldEarnPlus => "몬스터 처치 시 {amount}골드를 추가로 얻습니다",
            UpgradeBoardText::ShopSlotExpand => "상점 슬롯이 {amount}개 증가합니다",
            UpgradeBoardText::QuestSlotExpand => "퀘스트 슬롯이 {amount}개 증가합니다",
            UpgradeBoardText::QuestBoardSlotExpand => "퀘스트 게시판 슬롯이 {amount}개 증가합니다",
            UpgradeBoardText::RerollChancePlus => "리롤 기회가 {amount}개 증가합니다",
            UpgradeBoardText::ShopItemPriceMinus => "상점 아이템 가격이 {amount} 감소합니다",
            UpgradeBoardText::ShopRefreshChancePlus => "상점 새로고침 기회가 {amount}개 증가합니다",
            UpgradeBoardText::QuestBoardRefreshChancePlus => {
                "퀘스트 게시판 새로고침 기회가 {amount}개 증가합니다"
            }
            UpgradeBoardText::ShortenStraightFlushTo4Cards => {
                "스트레이트와 플러시를 4장으로 줄입니다"
            }
            UpgradeBoardText::SkipRankForStraight => {
                "스트레이트를 만들 때 랭크 하나를 건너뛸 수 있습니다"
            }
            UpgradeBoardText::TreatSuitsAsSame => "색이 같으면 같은 문양으로 취급합니다",
            UpgradeBoardText::TowerSelectLowCard => "카드 {amount}개 이하로 타워를 만들 때 타워의",
            UpgradeBoardText::TowerSelectNoReroll => "리롤을 하지 않고 타워를 만들 때 타워의",
            UpgradeBoardText::TowerSelectReroll => "리롤을 할 때 마다 타워의",
            UpgradeBoardText::TowerUpgradeRank => "랭크가 {name}인 타워의",
            UpgradeBoardText::TowerUpgradeSuit => "문양이 {name}인 타워의",
            UpgradeBoardText::TowerUpgradeKind => "{name} 타워의",
            UpgradeBoardText::TowerUpgradeEvenOdd => "{name} 타워의",
            UpgradeBoardText::TowerUpgradeFaceNumber => "{name} 타워의",
            UpgradeBoardText::DamagePlus => "공격력이 {amount}만큼 증가합니다",
            UpgradeBoardText::DamageMultiplier => "공격력이 {amount}배 증가합니다",
            UpgradeBoardText::SpeedPlus => "공격 속도가 {amount}만큼 증가합니다",
            UpgradeBoardText::SpeedMultiplier => "공격 속도가 {amount}배 증가합니다",
            UpgradeBoardText::RangePlus => "사정거리가 {amount}만큼 증가합니다",
        }
    }

    pub(super) fn to_english(&self) -> &'static str {
        match self {
            UpgradeBoardText::Title => "Upgrade Information",
            UpgradeBoardText::GoldEarnPlus => "Earn an additional {amount} gold when defeating monsters",
            UpgradeBoardText::ShopSlotExpand => "Increases shop slots by {amount}",
            UpgradeBoardText::QuestSlotExpand => "Increases quest slots by {amount}",
            UpgradeBoardText::QuestBoardSlotExpand => "Increases quest board slots by {amount}",
            UpgradeBoardText::RerollChancePlus => "Increases reroll chances by {amount}",
            UpgradeBoardText::ShopItemPriceMinus => "Decreases shop item prices by {amount}",
            UpgradeBoardText::ShopRefreshChancePlus => "Increases shop refresh chances by {amount}",
            UpgradeBoardText::QuestBoardRefreshChancePlus => {
                "Increases quest board refresh chances by {amount}"
            }
            UpgradeBoardText::ShortenStraightFlushTo4Cards => {
                "Shortens straight and flush to 4 cards"
            }
            UpgradeBoardText::SkipRankForStraight => {
                "Skip one rank when creating a straight"
            }
            UpgradeBoardText::TreatSuitsAsSame => "Treats same colors as the same pattern",
            UpgradeBoardText::TowerSelectLowCard => "When creating a tower with {amount} or fewer cards, the tower's",
            UpgradeBoardText::TowerSelectNoReroll => "When creating a tower without rerolling, the tower's",
            UpgradeBoardText::TowerSelectReroll => "Each time you reroll, the tower's",
            UpgradeBoardText::TowerUpgradeRank => "For towers with rank {name},",
            UpgradeBoardText::TowerUpgradeSuit => "For towers with suit {name},",
            UpgradeBoardText::TowerUpgradeKind => "For {name} towers,",
            UpgradeBoardText::TowerUpgradeEvenOdd => "For {name} towers,",
            UpgradeBoardText::TowerUpgradeFaceNumber => "For {name} towers,",
            UpgradeBoardText::DamagePlus => "Increases attack power by {amount}",
            UpgradeBoardText::DamageMultiplier => "Increases attack power by a factor of {amount}",
            UpgradeBoardText::SpeedPlus => "Increases attack speed by {amount}",
            UpgradeBoardText::SpeedMultiplier => "Increases attack speed by a factor of {amount}",
            UpgradeBoardText::RangePlus => "Increases range by {amount}",
        }
    }
}
