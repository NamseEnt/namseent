use super::UpgradeKind;
use crate::l10n::upgrade::UpgradeKindText;
use crate::l10n::LocalizedText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKind {
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        let mut builder = TypographyBuilder::new();
        UpgradeKindText::Name(self).apply_to_builder(&mut builder, &text_manager.locale());
        // TODO: 실제로 렌더링된 텍스트 추출 (지금은 임시로 description 기반)
        // builder를 렌더링해서 plain text를 추출하는 것이 필요할 수 있음
        self.name_fallback(text_manager)
    }

    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        let mut builder = TypographyBuilder::new();
        UpgradeKindText::Description(self).apply_to_builder(&mut builder, &text_manager.locale());
        // TODO: 실제로 렌더링된 텍스트 추출 (지금은 임시로 description 기반)
        self.description_fallback(text_manager)
    }

    // 임시 fallback 구현
    fn name_fallback(&self, text_manager: &crate::l10n::TextManager) -> String {
        use crate::l10n::Language;
        match text_manager.language() {
            Language::Korean => self.name_korean(),
            Language::English => self.name_english(),
        }
    }

    fn description_fallback(&self, text_manager: &crate::l10n::TextManager) -> String {
        use crate::l10n::Language;
        match text_manager.language() {
            Language::Korean => self.description_korean(),
            Language::English => self.description_english(),
        }
    }

    fn name_korean(&self) -> String {
        match self {
            UpgradeKind::GoldEarnPlus => "골드 수입 증가".to_string(),
            UpgradeKind::RankAttackDamageMultiply { rank, .. } => format!("{rank} 카드 공격력 배수 증가"),
            UpgradeKind::SuitAttackDamageMultiply { suit, .. } => format!("{:?} 카드 공격력 배수 증가", suit),
            UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                let tower_name = tower_kind.to_text().to_korean();
                format!("{tower_name} 공격력 배수 증가")
            },
            UpgradeKind::ShopSlotExpansion => "상점 슬롯 확장".to_string(),
            UpgradeKind::RerollCountPlus => "리롤 횟수 증가".to_string(),
            UpgradeKind::LowCardTowerDamageMultiply { .. } => "로우카드 타워 공격력 배수 증가".to_string(),
            UpgradeKind::ShopItemPriceMinus => "상점 아이템 가격 할인".to_string(),
            UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수 증가".to_string(),
            UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => "무리롤 타워 공격력 배수 증가".to_string(),
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                if *even { "짝수 카드 공격력 배수 증가" } else { "홀수 카드 공격력 배수 증가" }.to_string()
            },
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                if *face { "페이스 카드 공격력 배수 증가" } else { "숫자 카드 공격력 배수 증가" }.to_string()
            },
            UpgradeKind::ShortenStraightFlushTo4Cards => "스트레이트 플러시 4장 단축".to_string(),
            UpgradeKind::SkipRankForStraight => "스트레이트 랭크 건너뛰기".to_string(),
            UpgradeKind::TreatSuitsAsSame => "모든 무늬 동일 취급".to_string(),
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => "리롤 타워 공격력 배수 증가".to_string(),
        }
    }

    fn description_korean(&self) -> String {
        match self {
            UpgradeKind::GoldEarnPlus => "몬스터를 처치할 때 얻는 골드가 증가합니다.".to_string(),
            UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                format!("{rank} 카드로 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
            UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                format!("{:?} 카드로 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.", suit)
            },
            UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                let tower_name = tower_kind.to_text().to_korean();
                format!("{tower_name} 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
            UpgradeKind::ShopSlotExpansion => "상점에서 구매할 수 있는 슬롯이 1개 추가됩니다.".to_string(),
            UpgradeKind::RerollCountPlus => "매 라운드마다 사용할 수 있는 리롤 횟수가 1회 증가합니다.".to_string(),
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                format!("3장 이하로 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
            UpgradeKind::ShopItemPriceMinus => "상점 아이템의 가격이 할인됩니다.".to_string(),
            UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수가 1회 증가합니다.".to_string(),
            UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                format!("리롤하지 않고 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                let card_type = if *even { "짝수" } else { "홀수" };
                format!("{card_type} 카드로 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                let card_type = if *face { "페이스" } else { "숫자" };
                format!("{card_type} 카드로 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
            UpgradeKind::ShortenStraightFlushTo4Cards => "스트레이트 플러시를 4장으로 만들 수 있게 됩니다.".to_string(),
            UpgradeKind::SkipRankForStraight => "스트레이트에서 한 랭크를 건너뛸 수 있게 됩니다.".to_string(),
            UpgradeKind::TreatSuitsAsSame => "모든 무늬를 같은 것으로 취급합니다.".to_string(),
            UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                format!("리롤하고 만든 타워의 공격력이 x{damage_multiplier:.1} 증가합니다.")
            },
        }
    }

    fn name_english(&self) -> String {
        match self {
            UpgradeKind::GoldEarnPlus => "Gold Income Increase".to_string(),
            UpgradeKind::RankAttackDamageMultiply { rank, .. } => format!("{rank} Card Attack Damage Multiply"),
            UpgradeKind::SuitAttackDamageMultiply { suit, .. } => format!("{:?} Card Attack Damage Multiply", suit),
            UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                let tower_name = tower_kind.to_text().to_english();
                format!("{tower_name} Attack Damage Multiplier")
            },
            UpgradeKind::ShopSlotExpansion => "Shop Slot Expansion".to_string(),
            UpgradeKind::RerollCountPlus => "Reroll Count Increase".to_string(),
            UpgradeKind::LowCardTowerDamageMultiply { .. } => "Low Card Tower Attack Damage Multiply".to_string(),
            UpgradeKind::ShopItemPriceMinus => "Shop Item Price Discount".to_string(),
            UpgradeKind::ShopRefreshPlus => "Shop Refresh Count Increase".to_string(),
            UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => "No Reroll Tower Attack Damage Multiply".to_string(),
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                if *even { "Even Card Attack Damage Multiplier" } else { "Odd Card Attack Damage Multiplier" }.to_string()
            },
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                if *face { "Face Card Attack Damage Multiplier" } else { "Number Card Attack Damage Multiplier" }.to_string()
            },
            UpgradeKind::ShortenStraightFlushTo4Cards => "Shorten Straight Flush to 4 Cards".to_string(),
            UpgradeKind::SkipRankForStraight => "Skip Rank for Straight".to_string(),
            UpgradeKind::TreatSuitsAsSame => "Treat All Suits as Same".to_string(),
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => "Reroll Tower Attack Damage Multiply".to_string(),
        }
    }

    fn description_english(&self) -> String {
        match self {
            UpgradeKind::GoldEarnPlus => "Increases gold earned when defeating monsters.".to_string(),
            UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {rank} cards.")
            },
            UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {:?} cards.", suit)
            },
            UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                let tower_name = tower_kind.to_text().to_english();
                format!("Attack Damage increases by x{damage_multiplier:.1} for {tower_name} towers.")
            },
            UpgradeKind::ShopSlotExpansion => "Adds 1 slot available for purchase in the shop.".to_string(),
            UpgradeKind::RerollCountPlus => "Increases the number of rerolls available each round by 1.".to_string(),
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with 3 or fewer cards.")
            },
            UpgradeKind::ShopItemPriceMinus => "Shop item prices are discounted.".to_string(),
            UpgradeKind::ShopRefreshPlus => "Shop refresh count increases by 1.".to_string(),
            UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made without rerolling.")
            },
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                let card_type = if *even { "even" } else { "odd" };
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {card_type} cards.")
            },
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                let card_type = if *face { "face" } else { "number" };
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {card_type} cards.")
            },
            UpgradeKind::ShortenStraightFlushTo4Cards => "Allows making straight flush with 4 cards.".to_string(),
            UpgradeKind::SkipRankForStraight => "Allows skipping one rank when making a straight.".to_string(),
            UpgradeKind::TreatSuitsAsSame => "Treats all suits as the same.".to_string(),
            UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                format!("Attack Damage increases by x{damage_multiplier:.1} for towers made after rerolling.")
            },
        }
    }
}

