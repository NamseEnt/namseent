use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => {
                    builder.static_text("골드 수입 증가");
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, .. } => {
                    builder
                        .with_card_rank(format!("{rank}"))
                        .static_text(" 카드 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, .. } => {
                    builder
                        .with_suit_color(format!("{:?}", suit), *suit)
                        .static_text(" 카드 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    builder
                        .text(tower_name)
                        .static_text(" ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => {
                    builder.static_text("상점 슬롯 확장");
                },
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => {
                    builder.static_text("리롤 횟수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { .. } => {
                    builder
                        .static_text("로우카드 타워 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => {
                    builder.static_text("상점 아이템 가격 할인");
                },
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => {
                    builder.static_text("상점 새로고침 횟수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => {
                    builder
                        .static_text("무리롤 타워 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    builder
                        .text(card_type)
                        .static_text(" 카드 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    builder
                        .text(card_type)
                        .static_text(" 카드 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => {
                    builder.static_text("스트레이트 플러시 4장 단축");
                },
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => {
                    builder.static_text("스트레이트 랭크 건너뛰기");
                },
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => {
                    builder.static_text("모든 무늬 동일 취급");
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { .. } => {
                    builder
                        .static_text("리롤 타워 ")
                        .with_attack_damage_stat("공격력")
                        .static_text(" 배수 증가");
                },
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => {
                    builder
                        .static_text("몬스터를 처치할 때 얻는 ")
                        .with_gold_icon("골드")
                        .static_text("가 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                    builder
                        .with_card_rank(format!("{rank}"))
                        .static_text(" 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                    builder
                        .with_suit_color(format!("{:?}", suit), *suit)
                        .static_text(" 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                    let tower_name = Self::get_tower_name(tower_kind);
                    builder
                        .text(tower_name)
                        .static_text(" 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => {
                    builder
                        .static_text("상점에서 구매할 수 있는 슬롯이 ")
                        .with_positive_effect("1개")
                        .static_text(" 추가됩니다.");
                },
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => {
                    builder
                        .static_text("매 라운드마다 사용할 수 있는 리롤 횟수가 ")
                        .with_positive_effect("1회")
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                    builder
                        .static_text("3장 이하로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => {
                    builder.static_text("상점 아이템의 가격이 할인됩니다.");
                },
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => {
                    builder
                        .static_text("상점 새로고침 횟수가 ")
                        .with_positive_effect("1회")
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                    builder
                        .static_text("리롤하지 않고 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                    let card_type = if *even { "짝수" } else { "홀수" };
                    builder
                        .text(card_type)
                        .static_text(" 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                    let card_type = if *face { "페이스" } else { "숫자" };
                    builder
                        .text(card_type)
                        .static_text(" 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => {
                    builder.static_text("스트레이트 플러시를 4장으로 만들 수 있게 됩니다.");
                },
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => {
                    builder.static_text("스트레이트에서 한 랭크를 건너뛸 수 있게 됩니다.");
                },
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => {
                    builder.static_text("모든 무늬를 같은 것으로 취급합니다.");
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                    builder
                        .static_text("리롤하고 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
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
