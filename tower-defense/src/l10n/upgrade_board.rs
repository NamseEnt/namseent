use super::{Language, Locale, LocalizedRichText, LocalizedText};
use crate::{theme::typography::TypographyBuilder, *};

#[derive(Debug, Clone, State)]
pub enum UpgradeBoardText {
    Title,
    GoldEarnPlus { amount: usize },
    ShopSlotExpand { amount: usize },
    RerollChancePlus { amount: usize },
    ShopItemPriceMinus { amount: usize },
    ShopRefreshChancePlus { amount: usize },
    ShortenStraightFlushTo4Cards,
    SkipRankForStraight,
    TreatSuitsAsSame,
    TowerSelectLowCard { amount: usize },
    TowerSelectNoReroll,
    TowerSelectReroll,
    TowerUpgradeRank { name: String },
    TowerUpgradeSuit { name: String },
    TowerUpgradeKind { name: String },
    TowerUpgradeEvenOdd { name: String },
    TowerUpgradeFaceNumber { name: String },
    DamagePlus { amount: f32 },
    DamageMultiplier { amount: f32 },
    SpeedPlus { amount: f32 },
    SpeedMultiplier { amount: f32 },
    RangePlus { amount: f32 },
}

impl LocalizedText for UpgradeBoardText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.text_korean(),
            Language::English => self.text_english(),
        }
    }
}

impl LocalizedRichText for UpgradeBoardText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        builder.text(self.localized_text(locale))
    }
}

impl UpgradeBoardText {
    fn text_korean(&self) -> String {
        match self {
            UpgradeBoardText::Title => "강화 정보".to_string(),
            UpgradeBoardText::GoldEarnPlus { amount } => {
                format!("몬스터 처치 시 💰 {amount}를 추가로 얻습니다")
            }
            UpgradeBoardText::ShopSlotExpand { amount } => {
                format!("[Shop] 상점 슬롯이 {amount}개 증가합니다")
            }
            UpgradeBoardText::RerollChancePlus { amount } => {
                format!("[Refresh] 리롤 기회가 {amount}개 증가합니다")
            }
            UpgradeBoardText::ShopItemPriceMinus { amount } => {
                format!("[Shop] 상점 아이템 가격이 {amount} 감소합니다")
            }
            UpgradeBoardText::ShopRefreshChancePlus { amount } => {
                format!("[Shop] 상점 새로고침 기회가 {amount}개 증가합니다")
            }
            UpgradeBoardText::ShortenStraightFlushTo4Cards => {
                "스트레이트와 플러시를 4장으로 줄입니다".to_string()
            }
            UpgradeBoardText::SkipRankForStraight => {
                "스트레이트를 만들 때 랭크 하나를 건너뛸 수 있습니다".to_string()
            }
            UpgradeBoardText::TreatSuitsAsSame => {
                "색이 같으면 같은 문양으로 취급합니다".to_string()
            }
            UpgradeBoardText::TowerSelectLowCard { amount } => {
                format!("카드 {amount}개 이하로 타워를 만들 때 타워의")
            }
            UpgradeBoardText::TowerSelectNoReroll => {
                "리롤을 하지 않고 타워를 만들 때 타워의".to_string()
            }
            UpgradeBoardText::TowerSelectReroll => "리롤을 할 때 마다 타워의".to_string(),
            UpgradeBoardText::TowerUpgradeRank { name } => format!("랭크가 {name}인 타워의"),
            UpgradeBoardText::TowerUpgradeSuit { name } => format!("문양이 {name}인 타워의"),
            UpgradeBoardText::TowerUpgradeKind { name } => format!("{name} 타워의"),
            UpgradeBoardText::TowerUpgradeEvenOdd { name } => format!("{name} 타워의"),
            UpgradeBoardText::TowerUpgradeFaceNumber { name } => format!("{name} 타워의"),
            UpgradeBoardText::DamagePlus { amount } => {
                format!("공격력이 +{amount:.1} 증가합니다")
            }
            UpgradeBoardText::DamageMultiplier { amount } => {
                format!("공격력이 x{amount:.1} 증가합니다")
            }
            UpgradeBoardText::SpeedPlus { amount } => {
                format!("공격 속도가 +{amount:.1} 증가합니다")
            }
            UpgradeBoardText::SpeedMultiplier { amount } => {
                format!("공격 속도가 x{amount:.1} 증가합니다")
            }
            UpgradeBoardText::RangePlus { amount } => {
                format!("사정거리가 +{amount:.1} 증가합니다")
            }
        }
    }

    fn text_english(&self) -> String {
        match self {
            UpgradeBoardText::Title => "Upgrade Information".to_string(),
            UpgradeBoardText::GoldEarnPlus { amount } => {
                format!("Earn an additional 💰 {amount} when defeating monsters")
            }
            UpgradeBoardText::ShopSlotExpand { amount } => {
                format!("[Shop] Increases shop slots by {amount}")
            }
            UpgradeBoardText::RerollChancePlus { amount } => {
                format!("[Refresh] Increases reroll chances by {amount}")
            }
            UpgradeBoardText::ShopItemPriceMinus { amount } => {
                format!("[Shop] Decreases shop item prices by {amount}")
            }
            UpgradeBoardText::ShopRefreshChancePlus { amount } => {
                format!("[Shop] Increases shop refresh chances by {amount}")
            }
            UpgradeBoardText::ShortenStraightFlushTo4Cards => {
                "Shortens straight and flush to 4 cards".to_string()
            }
            UpgradeBoardText::SkipRankForStraight => {
                "Skip one rank when creating a straight".to_string()
            }
            UpgradeBoardText::TreatSuitsAsSame => {
                "Treats same colors as the same pattern".to_string()
            }
            UpgradeBoardText::TowerSelectLowCard { amount } => {
                format!("When creating a tower with {amount} or fewer cards, the tower's")
            }
            UpgradeBoardText::TowerSelectNoReroll => {
                "When creating a tower without rerolling, the tower's".to_string()
            }
            UpgradeBoardText::TowerSelectReroll => "Each time you reroll, the tower's".to_string(),
            UpgradeBoardText::TowerUpgradeRank { name } => format!("For towers with rank {name},"),
            UpgradeBoardText::TowerUpgradeSuit { name } => format!("For towers with suit {name},"),
            UpgradeBoardText::TowerUpgradeKind { name } => format!("For {name} towers,"),
            UpgradeBoardText::TowerUpgradeEvenOdd { name } => format!("For {name} towers,"),
            UpgradeBoardText::TowerUpgradeFaceNumber { name } => format!("For {name} towers,"),
            UpgradeBoardText::DamagePlus { amount } => {
                format!("Attack Damage increases by +{amount:.1}")
            }
            UpgradeBoardText::DamageMultiplier { amount } => {
                format!("Attack Damage increases by x{amount:.1}")
            }
            UpgradeBoardText::SpeedPlus { amount } => {
                format!("Attack Speed increases by +{amount:.1}")
            }
            UpgradeBoardText::SpeedMultiplier { amount } => {
                format!("Attack Speed increases by x{amount:.1}")
            }
            UpgradeBoardText::RangePlus { amount } => {
                format!("Attack Range increases by +{amount:.1}")
            }
        }
    }
}
