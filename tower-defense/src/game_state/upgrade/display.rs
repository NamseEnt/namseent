use super::UpgradeKind;

impl UpgradeKind {
    pub fn name(&self) -> String {
        match self {
            UpgradeKind::GoldEarnPlus => "골드 획득량 증가".to_string(),
            UpgradeKind::RankAttackDamagePlus { rank, .. } => format!("{rank} 공격력 증가"),
            UpgradeKind::RankAttackDamageMultiply { rank, .. } => {
                format!("{rank} 공격력 배수 증가")
            }
            UpgradeKind::RankAttackSpeedPlus { rank, .. } => format!("{rank} 공격 속도 증가"),
            UpgradeKind::RankAttackSpeedMultiply { rank, .. } => {
                format!("{rank} 공격 속도 배수 증가")
            }
            UpgradeKind::RankAttackRangePlus { rank, .. } => format!("{rank} 공격 범위 증가"),
            UpgradeKind::SuitAttackDamagePlus { suit, .. } => format!("{suit} 공격력 증가"),
            UpgradeKind::SuitAttackDamageMultiply { suit, .. } => {
                format!("{suit} 공격력 배수 증가")
            }
            UpgradeKind::SuitAttackSpeedPlus { suit, .. } => format!("{suit} 공격 속도 증가"),
            UpgradeKind::SuitAttackSpeedMultiply { suit, .. } => {
                format!("{suit} 공격 속도 배수 증가")
            }
            UpgradeKind::SuitAttackRangePlus { suit, .. } => format!("{suit} 공격 범위 증가"),
            UpgradeKind::HandAttackDamagePlus { .. } => "족보 공격력 증가".to_string(),
            UpgradeKind::HandAttackDamageMultiply { .. } => "족보 공격력 배수 증가".to_string(),
            UpgradeKind::HandAttackSpeedPlus { .. } => "족보 공격 속도 증가".to_string(),
            UpgradeKind::HandAttackSpeedMultiply { .. } => "족보 공격 속도 배수 증가".to_string(),
            UpgradeKind::HandAttackRangePlus { .. } => "족보 공격 범위 증가".to_string(),
            UpgradeKind::ShopSlotExpansion => "상점 슬롯 확장".to_string(),
            UpgradeKind::QuestSlotExpansion => "퀘스트 슬롯 확장".to_string(),
            UpgradeKind::QuestBoardExpansion => "퀘스트 게시판 확장".to_string(),
            UpgradeKind::RerollCountPlus => "리롤 횟수 증가".to_string(),
            UpgradeKind::LowCardTowerDamagePlus { .. } => "적은 카드 타워 공격력 증가".to_string(),
            UpgradeKind::LowCardTowerDamageMultiply { .. } => {
                "적은 카드 타워 공격력 배수 증가".to_string()
            }
            UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => {
                "적은 카드 타워 공격 속도 증가".to_string()
            }
            UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => {
                "적은 카드 타워 공격 속도 배수 증가".to_string()
            }
            UpgradeKind::LowCardTowerAttackRangePlus { .. } => {
                "적은 카드 타워 공격 범위 증가".to_string()
            }
            UpgradeKind::ShopItemPriceMinus => "상점 아이템 가격 감소".to_string(),
            UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수 증가".to_string(),
            UpgradeKind::QuestBoardRefreshPlus => "퀘스트 게시판 새로고침 횟수 증가".to_string(),
            UpgradeKind::NoRerollTowerAttackDamagePlus { .. } => {
                "리롤 안하면 공격력 증가".to_string()
            }
            UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => {
                "리롤 안하면 공격력 배수 증가".to_string()
            }
            UpgradeKind::NoRerollTowerAttackSpeedPlus { .. } => {
                "리롤 안하면 공격 속도 증가".to_string()
            }
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { .. } => {
                "리롤 안하면 공격 속도 배수 증가".to_string()
            }
            UpgradeKind::NoRerollTowerAttackRangePlus { .. } => {
                "리롤 안하면 공격 범위 증가".to_string()
            }
            UpgradeKind::EvenOddTowerAttackDamagePlus { .. } => {
                "짝수/홀수 타워 공격력 증가".to_string()
            }
            UpgradeKind::EvenOddTowerAttackDamageMultiply { .. } => {
                "짝수/홀수 타워 공격력 배수 증가".to_string()
            }
            UpgradeKind::EvenOddTowerAttackSpeedPlus { .. } => {
                "짝수/홀수 타워 공격 속도 증가".to_string()
            }
            UpgradeKind::EvenOddTowerAttackSpeedMultiply { .. } => {
                "짝수/홀수 타워 공격 속도 배수 증가".to_string()
            }
            UpgradeKind::EvenOddTowerAttackRangePlus { .. } => {
                "짝수/홀수 타워 공격 범위 증가".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { .. } => {
                "그림/숫자 카드 타워 공격력 증가".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { .. } => {
                "그림/숫자 카드 타워 공격력 배수 증가".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { .. } => {
                "그림/숫자 카드 타워 공격 속도 증가".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { .. } => {
                "그림/숫자 카드 타워 공격 속도 배수 증가".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { .. } => {
                "그림/숫자 카드 타워 공격 범위 증가".to_string()
            }
            UpgradeKind::ShortenStraightFlushTo4Cards => "스트레이트를 4장으로 단축".to_string(),
            UpgradeKind::SkipRankForStraight => "스트레이트에서 랭크 하나 건너뛰기".to_string(),
            UpgradeKind::TreatSuitsAsSame => "색이 같으면 같은 문양으로 취급".to_string(),
            UpgradeKind::RerollTowerAttackDamagePlus { .. } => {
                "리롤하면 타워 공격력 증가".to_string()
            }
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => {
                "리롤하면 타워 공격력 배수 증가".to_string()
            }
            UpgradeKind::RerollTowerAttackSpeedPlus { .. } => {
                "리롤하면 타워 공격 속도 증가".to_string()
            }
            UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => {
                "리롤하면 타워 공격 속도 배수 증가".to_string()
            }
            UpgradeKind::RerollTowerAttackRangePlus { .. } => {
                "리롤하면 타워 공격 범위 증가".to_string()
            }
        }
    }
    pub fn description(&self) -> String {
        match self {
            UpgradeKind::GoldEarnPlus => "골드 획득량이 증가합니다.".to_string(),
            UpgradeKind::RankAttackDamagePlus { rank, damage_plus } => {
                format!("{rank} 공격력이 {damage_plus:.0} 증가합니다.")
            }
            UpgradeKind::RankAttackDamageMultiply {
                rank,
                damage_multiplier,
            } => format!("{rank} 공격력이 {damage_multiplier:.1}배 증가합니다."),
            UpgradeKind::RankAttackSpeedPlus { rank, speed_plus } => {
                format!("{rank} 공격 속도가 {speed_plus:.0} 증가합니다.")
            }
            UpgradeKind::RankAttackSpeedMultiply {
                rank,
                speed_multiplier,
            } => format!("{rank} 공격 속도가 {speed_multiplier:.1}배 증가합니다."),
            UpgradeKind::RankAttackRangePlus { rank, range_plus } => {
                format!("{rank} 공격 범위가 {range_plus:.1} 증가합니다.")
            }
            UpgradeKind::SuitAttackDamagePlus { suit, damage_plus } => {
                format!("{suit} 공격력이 {damage_plus:.0} 증가합니다.")
            }
            UpgradeKind::SuitAttackDamageMultiply {
                suit,
                damage_multiplier,
            } => format!("{suit} 공격력이 {damage_multiplier:.1}배 증가합니다."),
            UpgradeKind::SuitAttackSpeedPlus { suit, speed_plus } => {
                format!("{suit} 공격 속도가 {speed_plus:.0} 증가합니다.")
            }
            UpgradeKind::SuitAttackSpeedMultiply {
                suit,
                speed_multiplier,
            } => format!("{suit} 공격 속도가 {speed_multiplier:.1}배 증가합니다."),
            UpgradeKind::SuitAttackRangePlus { suit, range_plus } => {
                format!("{suit} 공격 범위가 {range_plus:.1} 증가합니다.")
            }
            UpgradeKind::HandAttackDamagePlus { .. } => "족보 공격력이 증가합니다.".to_string(),
            UpgradeKind::HandAttackDamageMultiply { .. } => {
                "족보 공격력이 배수로 증가합니다.".to_string()
            }
            UpgradeKind::HandAttackSpeedPlus { .. } => "족보 공격 속도가 증가합니다.".to_string(),
            UpgradeKind::HandAttackSpeedMultiply { .. } => {
                "족보 공격 속도가 배수로 증가합니다.".to_string()
            }
            UpgradeKind::HandAttackRangePlus { .. } => "족보 공격 범위가 증가합니다.".to_string(),
            UpgradeKind::ShopSlotExpansion => "상점 슬롯이 확장됩니다.".to_string(),
            UpgradeKind::QuestSlotExpansion => "퀘스트 슬롯이 확장됩니다.".to_string(),
            UpgradeKind::QuestBoardExpansion => "퀘스트 게시판이 확장됩니다.".to_string(),
            UpgradeKind::RerollCountPlus => "리롤 횟수가 증가합니다.".to_string(),
            UpgradeKind::LowCardTowerDamagePlus { .. } => {
                "적은 카드 타워의 공격력이 증가합니다.".to_string()
            }
            UpgradeKind::LowCardTowerDamageMultiply { .. } => {
                "적은 카드 타워의 공격력이 배수로 증가합니다.".to_string()
            }
            UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => {
                "적은 카드 타워의 공격 속도가 증가합니다.".to_string()
            }
            UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => {
                "적은 카드 타워의 공격 속도가 배수로 증가합니다.".to_string()
            }
            UpgradeKind::LowCardTowerAttackRangePlus { .. } => {
                "적은 카드 타워의 공격 범위가 증가합니다.".to_string()
            }
            UpgradeKind::ShopItemPriceMinus => "상점 아이템 가격이 감소합니다.".to_string(),
            UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수가 증가합니다.".to_string(),
            UpgradeKind::QuestBoardRefreshPlus => {
                "퀘스트 게시판 새로고침 횟수가 증가합니다.".to_string()
            }
            UpgradeKind::NoRerollTowerAttackDamagePlus { .. } => {
                "리롤하지 않으면 타워 공격력이 증가합니다.".to_string()
            }
            UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => {
                "리롤하지 않으면 타워 공격력이 배수로 증가합니다.".to_string()
            }
            UpgradeKind::NoRerollTowerAttackSpeedPlus { .. } => {
                "리롤하지 않으면 타워 공격 속도가 증가합니다.".to_string()
            }
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { .. } => {
                "리롤하지 않으면 타워 공격 속도가 배수로 증가합니다.".to_string()
            }
            UpgradeKind::NoRerollTowerAttackRangePlus { .. } => {
                "리롤하지 않으면 타워 공격 범위가 증가합니다.".to_string()
            }
            UpgradeKind::EvenOddTowerAttackDamagePlus { .. } => {
                "짝수/홀수 타워의 공격력이 증가합니다.".to_string()
            }
            UpgradeKind::EvenOddTowerAttackDamageMultiply { .. } => {
                "짝수/홀수 타워의 공격력이 배수로 증가합니다.".to_string()
            }
            UpgradeKind::EvenOddTowerAttackSpeedPlus { .. } => {
                "짝수/홀수 타워의 공격 속도가 증가합니다.".to_string()
            }
            UpgradeKind::EvenOddTowerAttackSpeedMultiply { .. } => {
                "짝수/홀수 타워의 공격 속도가 배수로 증가합니다.".to_string()
            }
            UpgradeKind::EvenOddTowerAttackRangePlus { .. } => {
                "짝수/홀수 타워의 공격 범위가 증가합니다.".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { .. } => {
                "그림/숫자 카드 타워의 공격력이 증가합니다.".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { .. } => {
                "그림/숫자 카드 타워의 공격력이 배수로 증가합니다.".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { .. } => {
                "그림/숫자 카드 타워의 공격 속도가 증가합니다.".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { .. } => {
                "그림/숫자 카드 타워의 공격 속도가 배수로 증가합니다.".to_string()
            }
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { .. } => {
                "그림/숫자 카드 타워의 공격 범위가 증가합니다.".to_string()
            }
            UpgradeKind::ShortenStraightFlushTo4Cards => {
                "스트레이트 플러시를 4장으로 단축합니다.".to_string()
            }
            UpgradeKind::SkipRankForStraight => {
                "스트레이트에서 랭크 하나를 건너뜁니다.".to_string()
            }
            UpgradeKind::TreatSuitsAsSame => "같은 색의 문양을 동일하게 취급합니다.".to_string(),
            UpgradeKind::RerollTowerAttackDamagePlus { .. } => {
                "리롤하면 타워 공격력이 증가합니다.".to_string()
            }
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => {
                "리롤하면 타워 공격력이 배수로 증가합니다.".to_string()
            }
            UpgradeKind::RerollTowerAttackSpeedPlus { .. } => {
                "리롤하면 타워 공격 속도가 증가합니다.".to_string()
            }
            UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => {
                "리롤하면 타워 공격 속도가 배수로 증가합니다.".to_string()
            }
            UpgradeKind::RerollTowerAttackRangePlus { .. } => {
                "리롤하면 타워 공격 범위가 증가합니다.".to_string()
            }
        }
    }
}
