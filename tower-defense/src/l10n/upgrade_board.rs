use super::{rich_text_helpers::RichTextHelpers, Language, Locale, LocalizedText};
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
    fn apply_to_builder<'a>(
        self,
        builder: &mut TypographyBuilder<'a>,
        locale: &Locale,
    ) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl UpgradeBoardText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeBoardText::Title => {
                builder.text("강화 정보");
            }
            UpgradeBoardText::GoldEarnPlus { amount } => {
                builder
                    .text("몬스터 처치 시 ");
                builder.with_gold_icon(format!("{amount}"));
                builder
                    .text("를 추가로 얻습니다");
            }
            UpgradeBoardText::ShopSlotExpand { amount } => {
                builder.text(format!("[Shop] 상점 슬롯이 {amount}개 증가합니다"));
            }
            UpgradeBoardText::RerollChancePlus { amount } => {
                builder.text(format!("[Refresh] 리롤 기회가 {amount}개 증가합니다"));
            }
            UpgradeBoardText::ShopItemPriceMinus { amount } => {
                builder.text(format!("[Shop] 상점 아이템 가격이 {amount} 감소합니다"));
            }
            UpgradeBoardText::ShopRefreshChancePlus { amount } => {
                builder.text(format!(
                    "[Shop] 상점 새로고침 기회가 {amount}개 증가합니다"
                ));
            }
            UpgradeBoardText::ShortenStraightFlushTo4Cards => {
                builder.text("스트레이트와 플러시를 4장으로 줄입니다");
            }
            UpgradeBoardText::SkipRankForStraight => {
                builder.text("스트레이트를 만들 때 랭크 하나를 건너뛸 수 있습니다");
            }
            UpgradeBoardText::TreatSuitsAsSame => {
                builder.text("색이 같으면 같은 문양으로 취급합니다");
            }
            UpgradeBoardText::TowerSelectLowCard { amount } => {
                builder.text(format!("카드 {amount}개 이하로 타워를 만들 때 타워의"));
            }
            UpgradeBoardText::TowerSelectNoReroll => {
                builder.text("리롤을 하지 않고 타워를 만들 때 타워의");
            }
            UpgradeBoardText::TowerSelectReroll => {
                builder.text("리롤을 할 때 마다 타워의");
            }
            UpgradeBoardText::TowerUpgradeRank { name } => {
                builder.text(format!("랭크가 {name}인 타워의"));
            }
            UpgradeBoardText::TowerUpgradeSuit { name } => {
                builder.text(format!("문양이 {name}인 타워의"));
            }
            UpgradeBoardText::TowerUpgradeKind { name } => {
                builder.text(format!("{name} 타워의"));
            }
            UpgradeBoardText::TowerUpgradeEvenOdd { name } => {
                builder.text(format!("{name} 타워의"));
            }
            UpgradeBoardText::TowerUpgradeFaceNumber { name } => {
                builder.text(format!("{name} 타워의"));
            }
            UpgradeBoardText::DamagePlus { amount } => {
                builder.text(format!("공격력이 +{amount:.1} 증가합니다"));
            }
            UpgradeBoardText::DamageMultiplier { amount } => {
                builder.text(format!("공격력이 x{amount:.1} 증가합니다"));
            }
            UpgradeBoardText::SpeedPlus { amount } => {
                builder.text(format!("공격 속도가 +{amount:.1} 증가합니다"));
            }
            UpgradeBoardText::SpeedMultiplier { amount } => {
                builder.text(format!("공격 속도가 x{amount:.1} 증가합니다"));
            }
            UpgradeBoardText::RangePlus { amount } => {
                builder.text(format!("사정거리가 +{amount:.1} 증가합니다"));
            }
        }
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeBoardText::Title => {
                builder.text("Upgrade Information");
            }
            UpgradeBoardText::GoldEarnPlus { amount } => {
                builder
                    .text("Earn an additional ");
                builder.with_gold_icon(format!("{amount}"));
                builder
                    .text(" when defeating monsters");
            }
            UpgradeBoardText::ShopSlotExpand { amount } => {
                builder.text(format!("[Shop] Increases shop slots by {amount}"));
            }
            UpgradeBoardText::RerollChancePlus { amount } => {
                builder.text(format!("[Refresh] Increases reroll chances by {amount}"));
            }
            UpgradeBoardText::ShopItemPriceMinus { amount } => {
                builder.text(format!("[Shop] Decreases shop item prices by {amount}"));
            }
            UpgradeBoardText::ShopRefreshChancePlus { amount } => {
                builder.text(format!("[Shop] Increases shop refresh chances by {amount}"));
            }
            UpgradeBoardText::ShortenStraightFlushTo4Cards => {
                builder.text("Shortens straight and flush to 4 cards");
            }
            UpgradeBoardText::SkipRankForStraight => {
                builder.text("Skip one rank when creating a straight");
            }
            UpgradeBoardText::TreatSuitsAsSame => {
                builder.text("Treats same colors as the same pattern");
            }
            UpgradeBoardText::TowerSelectLowCard { amount } => {
                builder.text(format!(
                    "When creating a tower with {amount} or fewer cards, the tower's"
                ));
            }
            UpgradeBoardText::TowerSelectNoReroll => {
                builder.text("When creating a tower without rerolling, the tower's");
            }
            UpgradeBoardText::TowerSelectReroll => {
                builder.text("Each time you reroll, the tower's");
            }
            UpgradeBoardText::TowerUpgradeRank { name } => {
                builder.text(format!("For towers with rank {name},"));
            }
            UpgradeBoardText::TowerUpgradeSuit { name } => {
                builder.text(format!("For towers with suit {name},"));
            }
            UpgradeBoardText::TowerUpgradeKind { name } => {
                builder.text(format!("For {name} towers,"));
            }
            UpgradeBoardText::TowerUpgradeEvenOdd { name } => {
                builder.text(format!("For {name} towers,"));
            }
            UpgradeBoardText::TowerUpgradeFaceNumber { name } => {
                builder.text(format!("For {name} towers,"));
            }
            UpgradeBoardText::DamagePlus { amount } => {
                builder.text(format!("Attack Damage increases by +{amount:.1}"));
            }
            UpgradeBoardText::DamageMultiplier { amount } => {
                builder.text(format!("Attack Damage increases by x{amount:.1}"));
            }
            UpgradeBoardText::SpeedPlus { amount } => {
                builder.text(format!("Attack Speed increases by +{amount:.1}"));
            }
            UpgradeBoardText::SpeedMultiplier { amount } => {
                builder.text(format!("Attack Speed increases by x{amount:.1}"));
            }
            UpgradeBoardText::RangePlus { amount } => {
                builder.text(format!("Attack Range increases by +{amount:.1}"));
            }
        }
    }
}
