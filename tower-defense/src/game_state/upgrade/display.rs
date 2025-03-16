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
            UpgradeKind::HandAttackDamagePlus { tower_kind, .. } => {
                format!("{tower_kind} 족보 공격력 증가")
            }
            UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                format!("{tower_kind} 족보 공격력 배수 증가")
            }
            UpgradeKind::HandAttackSpeedPlus { tower_kind, .. } => {
                format!("{tower_kind} 족보 공격 속도 증가")
            }
            UpgradeKind::HandAttackSpeedMultiply { tower_kind, .. } => {
                format!("{tower_kind} 족보 공격 속도 배수 증가")
            }
            UpgradeKind::HandAttackRangePlus { tower_kind, .. } => {
                format!("{tower_kind} 족보 공격 범위 증가")
            }
            UpgradeKind::ShopSlotExpansion => "상점 슬롯 확장".to_string(),
            UpgradeKind::QuestSlotExpansion => "퀘스트 슬롯 확장".to_string(),
            UpgradeKind::QuestBoardExpansion => "퀘스트 게시판 확장".to_string(),
            UpgradeKind::RerollCountPlus => "리롤 횟수 증가".to_string(),
            UpgradeKind::LowCardTowerDamagePlus { .. } => "3장 이하 타워 공격력 증가".to_string(),
            UpgradeKind::LowCardTowerDamageMultiply { .. } => {
                "3장 이하 타워 공격력 배수 증가".to_string()
            }
            UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => {
                "3장 이하 타워 공격 속도 증가".to_string()
            }
            UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => {
                "3장 이하 타워 공격 속도 배수 증가".to_string()
            }
            UpgradeKind::LowCardTowerAttackRangePlus { .. } => {
                "3장 이하 타워 공격 범위 증가".to_string()
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
            UpgradeKind::EvenOddTowerAttackDamagePlus { even, .. } => {
                format!("{} 타워 공격력 증가", if *even { "짝수" } else { "홀수" })
            }
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                format!(
                    "{} 타워 공격력 배수 증가",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::EvenOddTowerAttackSpeedPlus { even, .. } => {
                format!(
                    "{} 타워 공격 속도 증가",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, .. } => {
                format!(
                    "{} 타워 공격 속도 배수 증가",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::EvenOddTowerAttackRangePlus { even, .. } => {
                format!(
                    "{} 타워 공격 범위 증가",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, .. } => {
                format!(
                    "{} 카드 타워 공격력 증가",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                format!(
                    "{} 카드 타워 공격력 배수 증가",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, .. } => {
                format!(
                    "{} 카드 타워 공격 속도 증가",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, .. } => {
                format!(
                    "{} 카드 타워 공격 속도 배수 증가",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, .. } => {
                format!(
                    "{} 카드 타워 공격 범위 증가",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::ShortenStraightFlushTo4Cards => {
                "스트레이트와 플러시를 4장으로 단축".to_string()
            }
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
            UpgradeKind::HandAttackDamagePlus {
                tower_kind,
                damage_plus,
            } => {
                format!("{tower_kind}타워 공격력이 {damage_plus:.0} 증가합니다.")
            }
            UpgradeKind::HandAttackDamageMultiply {
                tower_kind,
                damage_multiplier,
            } => format!("{tower_kind}타워 공격력이 {damage_multiplier:.1}배 증가합니다."),
            UpgradeKind::HandAttackSpeedPlus {
                tower_kind,
                speed_plus,
            } => {
                format!("{tower_kind}타워 공격 속도가 {speed_plus:.0} 증가합니다.")
            }
            UpgradeKind::HandAttackSpeedMultiply {
                tower_kind,
                speed_multiplier,
            } => format!("{tower_kind}타워 공격 속도가 {speed_multiplier:.1}배 증가합니다."),
            UpgradeKind::HandAttackRangePlus {
                tower_kind,
                range_plus,
            } => {
                format!("{tower_kind}타워 공격 범위가 {range_plus:.1} 증가합니다.")
            }
            UpgradeKind::ShopSlotExpansion => "상점 슬롯이 확장됩니다.".to_string(),
            UpgradeKind::QuestSlotExpansion => "퀘스트 슬롯이 확장됩니다.".to_string(),
            UpgradeKind::QuestBoardExpansion => "퀘스트 게시판이 확장됩니다.".to_string(),
            UpgradeKind::RerollCountPlus => "리롤 횟수가 증가합니다.".to_string(),
            UpgradeKind::LowCardTowerDamagePlus { damage_plus } => {
                format!(
                    "3장 이하로 타워를 만들 때 타워의 공격력이 {damage_plus:.0} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                format!(
                    "3장 이하로 타워를 만들 때 타워의 공격력이 {damage_multiplier:.1}배 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::LowCardTowerAttackSpeedPlus { speed_plus } => {
                format!(
                    "3장 이하로 타워를 만들 때 타워의 공격 속도가 {speed_plus:.0} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::LowCardTowerAttackSpeedMultiply { speed_multiplier } => {
                format!(
                    "3장 이하로 타워를 만들 때 타워의 공격 속도가 {speed_multiplier:.1}배 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::LowCardTowerAttackRangePlus { range_plus } => {
                format!(
                    "3장 이하로 타워를 만들 때 타워의 공격 범위가 {range_plus:.1} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::ShopItemPriceMinus => "상점 아이템 가격이 감소합니다.".to_string(),
            UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수가 증가합니다.".to_string(),
            UpgradeKind::QuestBoardRefreshPlus => {
                "퀘스트 게시판 새로고침 횟수가 증가합니다.".to_string()
            }
            UpgradeKind::NoRerollTowerAttackDamagePlus { damage_plus } => {
                format!(
                    "리롤하지 않고 타워를 만들면 타워의 공격력이 {damage_plus:.0} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                format!(
                    "리롤하지 않고 타워를 만들면 타워의 공격력이 {damage_multiplier:.1}배 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::NoRerollTowerAttackSpeedPlus { speed_plus } => {
                format!(
                    "리롤하지 않고 타워를 만들면 타워의 공격 속도가 {speed_plus:.0} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { speed_multiplier } => {
                format!(
                    "리롤하지 않고 타워를 만들면 타워의 공격 속도가 {speed_multiplier:.1}배 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::NoRerollTowerAttackRangePlus { range_plus } => {
                format!(
                    "리롤하지 않고 타워를 만들면 타워의 공격 범위가 {range_plus:.1} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::EvenOddTowerAttackDamagePlus { even, damage_plus } => {
                format!(
                    "{} 타워의 공격력이 {damage_plus:.0} 증가합니다.",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even,
                damage_multiplier,
            } => format!(
                "{} 타워의 공격력이 {damage_multiplier:.1}배 증가합니다.",
                if *even { "짝수" } else { "홀수" }
            ),
            UpgradeKind::EvenOddTowerAttackSpeedPlus { even, speed_plus } => {
                format!(
                    "{} 타워의 공격 속도가 {speed_plus:.0} 증가합니다.",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::EvenOddTowerAttackSpeedMultiply {
                even,
                speed_multiplier,
            } => format!(
                "{} 타워의 공격 속도가 {speed_multiplier:.1}배 증가합니다.",
                if *even { "짝수" } else { "홀수" }
            ),
            UpgradeKind::EvenOddTowerAttackRangePlus { even, range_plus } => {
                format!(
                    "{} 타워의 공격 범위가 {range_plus:.1} 증가합니다.",
                    if *even { "짝수" } else { "홀수" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, damage_plus } => {
                format!(
                    "{} 카드 타워의 공격력이 {damage_plus:.0} 증가합니다.",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                face,
                damage_multiplier,
            } => format!(
                "{} 카드 타워의 공격력이 {damage_multiplier:.1}배 증가합니다.",
                if *face { "그림" } else { "숫자" }
            ),
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, speed_plus } => {
                format!(
                    "{} 카드 타워의 공격 속도가 {speed_plus:.0} 증가합니다.",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply {
                face,
                speed_multiplier,
            } => format!(
                "{} 카드 타워의 공격 속도가 {speed_multiplier:.1}배 증가합니다.",
                if *face { "그림" } else { "숫자" }
            ),
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, range_plus } => {
                format!(
                    "{} 카드 타워의 공격 범위가 {range_plus:.1} 증가합니다.",
                    if *face { "그림" } else { "숫자" }
                )
            }
            UpgradeKind::ShortenStraightFlushTo4Cards => {
                "스트레이트와 플러시를 4장으로 단축합니다.".to_string()
            }
            UpgradeKind::SkipRankForStraight => {
                "스트레이트를 만들 때 랭크 하나를 건너뛸 수 있습니다.".to_string()
            }
            UpgradeKind::TreatSuitsAsSame => "같은 색의 문양을 동일하게 취급합니다.".to_string(),
            UpgradeKind::RerollTowerAttackDamagePlus { damage_plus } => {
                format!(
                    "리롤할 때마다 타워의 공격력이 {damage_plus:.0} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                format!(
                    "리롤할 때마다 타워의 공격력이 {damage_multiplier:.1}배 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::RerollTowerAttackSpeedPlus { speed_plus } => {
                format!(
                    "리롤할 때마다 타워의 공격 속도가 {speed_plus:.0} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::RerollTowerAttackSpeedMultiply { speed_multiplier } => {
                format!(
                    "리롤할 때마다 타워의 공격 속도가 {speed_multiplier:.1}배 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
            UpgradeKind::RerollTowerAttackRangePlus { range_plus } => {
                format!(
                    "리롤할 때마다 타워의 공격 범위가 {range_plus:.1} 증가합니다. 이 효과는 타워를 새로 만들 때 적용됩니다."
                )
            }
        }
    }
}
