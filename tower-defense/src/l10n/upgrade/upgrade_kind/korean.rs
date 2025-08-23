use crate::l10n::{rich_text_helpers::*, upgrade::UpgradeKindText};

impl UpgradeKindText<'_> {
    pub fn to_korean(&self) -> String {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => "골드 수입 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamagePlus { rank, .. } => format!("{rank} 카드 공격력 증가"),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, .. } => format!("{rank} 카드 공격력 배수 증가"),
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedPlus { rank, .. } => format!("{rank} 카드 공격 속도 증가"),
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedMultiply { rank, .. } => format!("{rank} 카드 공격 속도 배수 증가"),
                crate::game_state::upgrade::UpgradeKind::RankAttackRangePlus { rank, .. } => format!("{rank} 카드 사거리 증가"),
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamagePlus { suit, .. } => format!("{} 카드 공격력 증가", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, .. } => format!("{} 카드 공격력 배수 증가", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedPlus { suit, .. } => format!("{} 카드 공격 속도 증가", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedMultiply { suit, .. } => format!("{} 카드 공격 속도 배수 증가", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackRangePlus { suit, .. } => format!("{} 카드 사거리 증가", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::HandAttackDamagePlus { tower_kind, .. } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 공격력 증가")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 공격력 배수 증가")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedPlus { tower_kind, .. } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 공격 속도 증가")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 공격 속도 배수 증가")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackRangePlus { tower_kind, .. } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 사거리 증가")
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => "상점 슬롯 확장".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestSlotExpansion => "퀘스트 슬롯 확장".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardExpansion => "퀘스트 보드 확장".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => "리롤 횟수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamagePlus { .. } => "로우카드 타워 공격력 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { .. } => "로우카드 타워 공격력 배수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => "로우카드 타워 공격 속도 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => "로우카드 타워 공격 속도 배수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackRangePlus { .. } => "로우카드 타워 사거리 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => "상점 아이템 가격 할인".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardRefreshPlus => "퀘스트 보드 새로고침 횟수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamagePlus { .. } => "무리롤 타워 공격력 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => "무리롤 타워 공격력 배수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedPlus { .. } => "무리롤 타워 공격 속도 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedMultiply { .. } => "무리롤 타워 공격 속도 배수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackRangePlus { .. } => "무리롤 타워 사거리 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamagePlus { even, .. } => {
                    if *even { "짝수 카드 공격력 증가" } else { "홀수 카드 공격력 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                    if *even { "짝수 카드 공격력 배수 증가" } else { "홀수 카드 공격력 배수 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedPlus { even, .. } => {
                    if *even { "짝수 카드 공격 속도 증가" } else { "홀수 카드 공격 속도 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, .. } => {
                    if *even { "짝수 카드 공격 속도 배수 증가" } else { "홀수 카드 공격 속도 배수 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackRangePlus { even, .. } => {
                    if *even { "짝수 카드 사거리 증가" } else { "홀수 카드 사거리 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, .. } => {
                    if *face { "페이스 카드 공격력 증가" } else { "숫자 카드 공격력 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                    if *face { "페이스 카드 공격력 배수 증가" } else { "숫자 카드 공격력 배수 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, .. } => {
                    if *face { "페이스 카드 공격 속도 증가" } else { "숫자 카드 공격 속도 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, .. } => {
                    if *face { "페이스 카드 공격 속도 배수 증가" } else { "숫자 카드 공격 속도 배수 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, .. } => {
                    if *face { "페이스 카드 사거리 증가" } else { "숫자 카드 사거리 증가" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => "스트레이트 플러시 4장 단축".to_string(),
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => "스트레이트 랭크 건너뛰기".to_string(),
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => "모든 무늬 동일 취급".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamagePlus { .. } => "리롤 타워 공격력 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { .. } => "리롤 타워 공격력 배수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedPlus { .. } => "리롤 타워 공격 속도 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => "리롤 타워 공격 속도 배수 증가".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackRangePlus { .. } => "리롤 타워 사거리 증가".to_string(),
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => "몬스터를 처치할 때 얻는 골드가 증가합니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamagePlus { rank, damage_plus } => {
                    format!("{rank} 카드로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                    format!("{rank} 카드로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedPlus { rank, speed_plus } => {
                    format!("{rank} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedMultiply { rank, speed_multiplier } => {
                    format!("{rank} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackRangePlus { rank, range_plus } => {
                    format!("{rank} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamagePlus { suit, damage_plus } => {
                    format!("{} 카드로 만든 타워의 {}이 {} 증가합니다.", suit_icon(*suit), attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                    format!("{} 카드로 만든 타워의 {}이 {} 증가합니다.", suit_icon(*suit), attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedPlus { suit, speed_plus } => {
                    format!("{} 카드로 만든 타워의 {}가 {} 증가합니다.", suit_icon(*suit), attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedMultiply { suit, speed_multiplier } => {
                    format!("{} 카드로 만든 타워의 {}가 {} 증가합니다.", suit_icon(*suit), attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackRangePlus { suit, range_plus } => {
                    format!("{} 카드로 만든 타워의 {}가 {} 증가합니다.", suit_icon(*suit), attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamagePlus { tower_kind, damage_plus } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedPlus { tower_kind, speed_plus } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedMultiply { tower_kind, speed_multiplier } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackRangePlus { tower_kind, range_plus } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    format!("{tower_name} 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => "상점에서 구매할 수 있는 슬롯이 1개 추가됩니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestSlotExpansion => "퀘스트 인벤토리 슬롯이 1개 추가됩니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardExpansion => "퀘스트 보드에 표시되는 퀘스트가 1개 추가됩니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => "매 라운드마다 사용할 수 있는 리롤 횟수가 1회 증가합니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamagePlus { damage_plus } => {
                    format!("3장 이하로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                    format!("3장 이하로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedPlus { speed_plus } => {
                    format!("3장 이하로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedMultiply { speed_multiplier } => {
                    format!("3장 이하로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackRangePlus { range_plus } => {
                    format!("3장 이하로 만든 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => "상점 아이템의 가격이 할인됩니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => "상점 새로고침 횟수가 1회 증가합니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardRefreshPlus => "퀘스트 보드 새로고침 횟수가 1회 증가합니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamagePlus { damage_plus } => {
                    format!("리롤하지 않고 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                    format!("리롤하지 않고 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedPlus { speed_plus } => {
                    format!("리롤하지 않고 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedMultiply { speed_multiplier } => {
                    format!("리롤하지 않고 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackRangePlus { range_plus } => {
                    format!("리롤하지 않고 만든 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamagePlus { even, damage_plus } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    format!("{card_type} 카드로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    format!("{card_type} 카드로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedPlus { even, speed_plus } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    format!("{card_type} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, speed_multiplier } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    format!("{card_type} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackRangePlus { even, range_plus } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    format!("{card_type} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, damage_plus } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    format!("{card_type} 카드로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    format!("{card_type} 카드로 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, speed_plus } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    format!("{card_type} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, speed_multiplier } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    format!("{card_type} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, range_plus } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    format!("{card_type} 카드로 만든 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => "스트레이트 플러시를 4장으로 만들 수 있게 됩니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => "스트레이트에서 한 랭크를 건너뛸 수 있게 됩니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => "모든 무늬를 같은 것으로 취급합니다.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamagePlus { damage_plus } => {
                    format!("리롤하고 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                    format!("리롤하고 만든 타워의 {}이 {} 증가합니다.", attack_damage_stat("공격력"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedPlus { speed_plus } => {
                    format!("리롤하고 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedMultiply { speed_multiplier } => {
                    format!("리롤하고 만든 타워의 {}가 {} 증가합니다.", attack_speed_stat("공격 속도"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackRangePlus { range_plus } => {
                    format!("리롤하고 만든 타워의 {}가 {} 증가합니다.", attack_range_stat("사거리"), additive_value(format!("{range_plus:.1}")))
                },
            }
        }
    }

    fn get_tower_name(tower_kind: &crate::game_state::tower::TowerKind) -> &'static str {
        match tower_kind {
            crate::game_state::tower::TowerKind::Barricade => "바리케이드",
            crate::game_state::tower::TowerKind::High => "하이카드",
            crate::game_state::tower::TowerKind::OnePair => "원페어",
            crate::game_state::tower::TowerKind::TwoPair => "투페어",
            crate::game_state::tower::TowerKind::ThreeOfAKind => "트리플",
            crate::game_state::tower::TowerKind::Straight => "스트레이트",
            crate::game_state::tower::TowerKind::Flush => "플러쉬",
            crate::game_state::tower::TowerKind::FullHouse => "풀하우스",
            crate::game_state::tower::TowerKind::FourOfAKind => "포카드",
            crate::game_state::tower::TowerKind::StraightFlush => "스트레이트 플러쉬",
            crate::game_state::tower::TowerKind::RoyalFlush => "로열 플러쉬",
        }
    }
}
