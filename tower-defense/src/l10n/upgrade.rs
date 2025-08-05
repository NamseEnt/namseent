use super::{Language, Locale, LocalizedText, rich_text_helpers::*};

pub enum UpgradeKindText<'a> {
    Name(&'a crate::game_state::upgrade::UpgradeKind),
    Description(&'a crate::game_state::upgrade::UpgradeKind),
}

impl LocalizedText for UpgradeKindText<'_> {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

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

    pub fn to_english(&self) -> String {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => "Gold Income Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamagePlus { rank, .. } => format!("{rank} Card Attack Damage Increase"),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, .. } => format!("{rank} Card Attack Damage Multiplier"),
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedPlus { rank, .. } => format!("{rank} Card Attack Speed Increase"),
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedMultiply { rank, .. } => format!("{rank} Card Attack Speed Multiplier"),
                crate::game_state::upgrade::UpgradeKind::RankAttackRangePlus { rank, .. } => format!("{rank} Card Attack Range Increase"),
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamagePlus { suit, .. } => format!("{} Card Attack Damage Increase", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, .. } => format!("{} Card Attack Damage Multiplier", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedPlus { suit, .. } => format!("{} Card Attack Speed Increase", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedMultiply { suit, .. } => format!("{} Card Attack Speed Multiplier", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::SuitAttackRangePlus { suit, .. } => format!("{} Card Attack Range Increase", suit_icon(*suit)),
                crate::game_state::upgrade::UpgradeKind::HandAttackDamagePlus { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{tower_name} Attack Damage Increase")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{tower_name} Attack Damage Multiplier")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedPlus { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{tower_name} Attack Speed Increase")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{tower_name} Attack Speed Multiplier")
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackRangePlus { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{tower_name} Attack Range Increase")
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => "Shop Slot Expansion".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestSlotExpansion => "Quest Slot Expansion".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardExpansion => "Quest Board Expansion".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => "Reroll Count Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamagePlus { .. } => "Low Card Tower Attack Damage Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { .. } => "Low Card Tower Attack Damage Multiplier".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => "Low Card Tower Attack Speed Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => "Low Card Tower Attack Speed Multiplier".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackRangePlus { .. } => "Low Card Tower Attack Range Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => "Shop Item Price Discount".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => "Shop Refresh Count Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardRefreshPlus => "Quest Board Refresh Count Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamagePlus { .. } => "No Reroll Tower Attack Damage Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => "No Reroll Tower Attack Damage Multiplier".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedPlus { .. } => "No Reroll Tower Attack Speed Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedMultiply { .. } => "No Reroll Tower Attack Speed Multiplier".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackRangePlus { .. } => "No Reroll Tower Attack Range Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamagePlus { even, .. } => {
                    if *even { "Even Card Attack Damage Increase" } else { "Odd Card Attack Damage Increase" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                    if *even { "Even Card Attack Damage Multiplier" } else { "Odd Card Attack Damage Multiplier" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedPlus { even, .. } => {
                    if *even { "Even Card Attack Speed Increase" } else { "Odd Card Attack Speed Increase" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, .. } => {
                    if *even { "Even Card Attack Speed Multiplier" } else { "Odd Card Attack Speed Multiplier" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackRangePlus { even, .. } => {
                    if *even { "Even Card Attack Range Increase" } else { "Odd Card Attack Range Increase" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, .. } => {
                    if *face { "Face Card Attack Damage Increase" } else { "Number Card Attack Damage Increase" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                    if *face { "Face Card Attack Damage Multiplier" } else { "Number Card Attack Damage Multiplier" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, .. } => {
                    if *face { "Face Card Attack Speed Increase" } else { "Number Card Attack Speed Increase" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, .. } => {
                    if *face { "Face Card Attack Speed Multiplier" } else { "Number Card Attack Speed Multiplier" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, .. } => {
                    if *face { "Face Card Attack Range Increase" } else { "Number Card Attack Range Increase" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => "Shorten Straight Flush to 4 Cards".to_string(),
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => "Skip Rank for Straight".to_string(),
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => "Treat All Suits as Same".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamagePlus { .. } => "Reroll Tower Attack Damage Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { .. } => "Reroll Tower Attack Damage Multiplier".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedPlus { .. } => "Reroll Tower Attack Speed Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => "Reroll Tower Attack Speed Multiplier".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackRangePlus { .. } => "Reroll Tower Attack Range Increase".to_string(),
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => "Increases gold earned when defeating monsters.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamagePlus { rank, damage_plus } => {
                    format!("{} increases by {} for towers made with {rank} cards.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                    format!("{} increases by {} for towers made with {rank} cards.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedPlus { rank, speed_plus } => {
                    format!("{} increases by {} for towers made with {rank} cards.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackSpeedMultiply { rank, speed_multiplier } => {
                    format!("{} increases by {} for towers made with {rank} cards.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackRangePlus { rank, range_plus } => {
                    format!("{} increases by {} for towers made with {rank} cards.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamagePlus { suit, damage_plus } => {
                    format!("{} increases by {} for towers made with {} cards.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")), suit_icon(*suit))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                    format!("{} increases by {} for towers made with {} cards.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")), suit_icon(*suit))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedPlus { suit, speed_plus } => {
                    format!("{} increases by {} for towers made with {} cards.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")), suit_icon(*suit))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackSpeedMultiply { suit, speed_multiplier } => {
                    format!("{} increases by {} for towers made with {} cards.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")), suit_icon(*suit))
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackRangePlus { suit, range_plus } => {
                    format!("{} increases by {} for towers made with {} cards.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")), suit_icon(*suit))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamagePlus { tower_kind, damage_plus } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{} increases by {} for {tower_name} towers.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{} increases by {} for {tower_name} towers.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedPlus { tower_kind, speed_plus } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{} increases by {} for {tower_name} towers.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackSpeedMultiply { tower_kind, speed_multiplier } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{} increases by {} for {tower_name} towers.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackRangePlus { tower_kind, range_plus } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{} increases by {} for {tower_name} towers.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => "Adds 1 slot available for purchase in the shop.".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestSlotExpansion => "Adds 1 quest inventory slot.".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardExpansion => "Adds 1 quest displayed on the quest board.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => "Increases the number of rerolls available each round by 1.".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamagePlus { damage_plus } => {
                    format!("{} increases by {} for towers made with 3 or fewer cards.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                    format!("{} increases by {} for towers made with 3 or fewer cards.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedPlus { speed_plus } => {
                    format!("{} increases by {} for towers made with 3 or fewer cards.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackSpeedMultiply { speed_multiplier } => {
                    format!("{} increases by {} for towers made with 3 or fewer cards.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerAttackRangePlus { range_plus } => {
                    format!("{} increases by {} for towers made with 3 or fewer cards.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => "Shop item prices are discounted.".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => "Shop refresh count increases by 1.".to_string(),
                crate::game_state::upgrade::UpgradeKind::QuestBoardRefreshPlus => "Quest board refresh count increases by 1.".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamagePlus { damage_plus } => {
                    format!("{} increases by {} for towers made without rerolling.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                    format!("{} increases by {} for towers made without rerolling.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedPlus { speed_plus } => {
                    format!("{} increases by {} for towers made without rerolling.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackSpeedMultiply { speed_multiplier } => {
                    format!("{} increases by {} for towers made without rerolling.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackRangePlus { range_plus } => {
                    format!("{} increases by {} for towers made without rerolling.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamagePlus { even, damage_plus } => {
                    let card_type = if *even { "even" } else { "odd" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                    let card_type = if *even { "even" } else { "odd" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedPlus { even, speed_plus } => {
                    let card_type = if *even { "even" } else { "odd" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, speed_multiplier } => {
                    let card_type = if *even { "even" } else { "odd" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackRangePlus { even, range_plus } => {
                    let card_type = if *even { "even" } else { "odd" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, damage_plus } => {
                    let card_type = if *face { "face" } else { "number" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                    let card_type = if *face { "face" } else { "number" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, speed_plus } => {
                    let card_type = if *face { "face" } else { "number" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, speed_multiplier } => {
                    let card_type = if *face { "face" } else { "number" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, range_plus } => {
                    let card_type = if *face { "face" } else { "number" };
                    format!("{} increases by {} for towers made with {card_type} cards.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => "Allows making straight flush with 4 cards.".to_string(),
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => "Allows skipping one rank when making a straight.".to_string(),
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => "Treats all suits as the same.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamagePlus { damage_plus } => {
                    format!("{} increases by {} for towers made after rerolling.", attack_damage_stat("Attack Damage"), additive_value(format!("{damage_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                    format!("{} increases by {} for towers made after rerolling.", attack_damage_stat("Attack Damage"), multiplier_value(format!("{damage_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedPlus { speed_plus } => {
                    format!("{} increases by {} for towers made after rerolling.", attack_speed_stat("Attack Speed"), additive_value(format!("{speed_plus:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackSpeedMultiply { speed_multiplier } => {
                    format!("{} increases by {} for towers made after rerolling.", attack_speed_stat("Attack Speed"), multiplier_value(format!("{speed_multiplier:.1}")))
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackRangePlus { range_plus } => {
                    format!("{} increases by {} for towers made after rerolling.", attack_range_stat("Attack Range"), additive_value(format!("{range_plus:.1}")))
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

    fn get_english_tower_name(tower_kind: &crate::game_state::tower::TowerKind) -> &'static str {
        match tower_kind {
            crate::game_state::tower::TowerKind::Barricade => "Barricade",
            crate::game_state::tower::TowerKind::High => "High Card",
            crate::game_state::tower::TowerKind::OnePair => "One Pair",
            crate::game_state::tower::TowerKind::TwoPair => "Two Pair",
            crate::game_state::tower::TowerKind::ThreeOfAKind => "Three of a Kind",
            crate::game_state::tower::TowerKind::Straight => "Straight",
            crate::game_state::tower::TowerKind::Flush => "Flush",
            crate::game_state::tower::TowerKind::FullHouse => "Full House",
            crate::game_state::tower::TowerKind::FourOfAKind => "Four of a Kind",
            crate::game_state::tower::TowerKind::StraightFlush => "Straight Flush",
            crate::game_state::tower::TowerKind::RoyalFlush => "Royal Flush",
        }
    }
}

// 기존 Template 구조체들은 하위 호환성을 위해 유지
#[derive(Debug, Clone, Copy)]
pub enum TowerUpgradeTarget {
    Tower(crate::game_state::upgrade::TowerUpgradeTarget),
    TowerSelect(crate::game_state::upgrade::TowerSelectUpgradeTarget),
}

#[derive(Debug, Clone, Copy)]
pub enum WhatUpgrade {
    Damage,
    Speed,
    Range,
}

#[derive(Debug, Clone, Copy)]
pub enum AddOrMultiply {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub enum Template {
    TowerUpgrade {
        target: TowerUpgradeTarget,
        what_upgrade: WhatUpgrade,
        add_or_multiply: AddOrMultiply,
        how_much: f32,
    },
}

impl LocalizedText for Template {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl Template {
    pub fn to_korean(&self) -> String {
        "레거시 템플릿".to_string() // 간단한 구현
    }

    pub fn to_english(&self) -> String {
        "Legacy template".to_string()
    }

    pub fn from_kind(_kind: &crate::game_state::upgrade::UpgradeKind, _is_name: bool) -> Self {
        // 간단한 기본값 반환
        Template::TowerUpgrade {
            target: TowerUpgradeTarget::Tower(
                crate::game_state::upgrade::TowerUpgradeTarget::Rank {
                    rank: crate::card::Rank::Ace,
                },
            ),
            what_upgrade: WhatUpgrade::Damage,
            add_or_multiply: AddOrMultiply::Add,
            how_much: 0.0,
        }
    }
}
