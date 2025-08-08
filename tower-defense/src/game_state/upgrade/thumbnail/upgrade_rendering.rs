use crate::{
    game_state::upgrade::UpgradeKind,
    icon::{Icon, IconKind, IconSize},
    thumbnail::{
        ThumbnailComposer, constants::OVERLAY_SIZE_RATIO, overlay_rendering::OverlayPosition,
    },
};
use namui::*;

impl UpgradeKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            // 골드 관련 업그레이드
            UpgradeKind::GoldEarnPlus => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .add_plus_overlay()
                .build(),

            // 랭크 기반 업그레이드들 - 공격력 관련
            UpgradeKind::RankAttackDamagePlus { rank, .. } => create_rank_stat_upgrade_thumbnail(
                width_height,
                *rank,
                StatType::Damage,
                OperationType::Plus,
            ),
            UpgradeKind::RankAttackDamageMultiply { rank, .. } => {
                create_rank_stat_upgrade_thumbnail(
                    width_height,
                    *rank,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::RankAttackSpeedPlus { rank, .. } => create_rank_stat_upgrade_thumbnail(
                width_height,
                *rank,
                StatType::Speed,
                OperationType::Plus,
            ),
            UpgradeKind::RankAttackSpeedMultiply { rank, .. } => {
                create_rank_stat_upgrade_thumbnail(
                    width_height,
                    *rank,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::RankAttackRangePlus { rank, .. } => create_rank_stat_upgrade_thumbnail(
                width_height,
                *rank,
                StatType::Range,
                OperationType::Plus,
            ),

            // 슈트 기반 업그레이드들 - 공격력 관련
            UpgradeKind::SuitAttackDamagePlus { suit, .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                *suit,
                StatType::Damage,
                OperationType::Plus,
            ),
            UpgradeKind::SuitAttackDamageMultiply { suit, .. } => {
                create_suit_stat_upgrade_thumbnail(
                    width_height,
                    *suit,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::SuitAttackSpeedPlus { suit, .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                *suit,
                StatType::Speed,
                OperationType::Plus,
            ),
            UpgradeKind::SuitAttackSpeedMultiply { suit, .. } => {
                create_suit_stat_upgrade_thumbnail(
                    width_height,
                    *suit,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::SuitAttackRangePlus { suit, .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                *suit,
                StatType::Range,
                OperationType::Plus,
            ),

            // 핸드 기반 업그레이드들 - 공격력 관련
            UpgradeKind::HandAttackDamagePlus { tower_kind, .. } => {
                create_hand_stat_upgrade_thumbnail(
                    width_height,
                    *tower_kind,
                    StatType::Damage,
                    OperationType::Plus,
                )
            }
            UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                create_hand_stat_upgrade_thumbnail(
                    width_height,
                    *tower_kind,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::HandAttackSpeedPlus { tower_kind, .. } => {
                create_hand_stat_upgrade_thumbnail(
                    width_height,
                    *tower_kind,
                    StatType::Speed,
                    OperationType::Plus,
                )
            }
            UpgradeKind::HandAttackSpeedMultiply { tower_kind, .. } => {
                create_hand_stat_upgrade_thumbnail(
                    width_height,
                    *tower_kind,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::HandAttackRangePlus { tower_kind, .. } => {
                create_hand_stat_upgrade_thumbnail(
                    width_height,
                    *tower_kind,
                    StatType::Range,
                    OperationType::Plus,
                )
            }

            // 확장 관련 업그레이드들
            UpgradeKind::ShopSlotExpansion => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shop)
                .add_expansion_indicator("+")
                .build(),
            UpgradeKind::QuestSlotExpansion => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Quest)
                .add_expansion_indicator("+")
                .build(),
            UpgradeKind::QuestBoardExpansion => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Quest)
                .add_expansion_indicator("Board")
                .build(),

            // 리롤 관련 업그레이드들
            UpgradeKind::RerollCountPlus => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .add_plus_overlay()
                .build(),

            // 낮은 카드 관련 업그레이드들
            UpgradeKind::LowCardTowerDamagePlus { .. } => create_condition_stat_upgrade_thumbnail(
                width_height,
                ConditionType::LowCard,
                StatType::Damage,
                OperationType::Plus,
            ),
            UpgradeKind::LowCardTowerDamageMultiply { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::LowCard,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::LowCardTowerAttackSpeedPlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::LowCard,
                    StatType::Speed,
                    OperationType::Plus,
                )
            }
            UpgradeKind::LowCardTowerAttackSpeedMultiply { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::LowCard,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::LowCardTowerAttackRangePlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::LowCard,
                    StatType::Range,
                    OperationType::Plus,
                )
            }

            // 상점 관련 업그레이드들
            UpgradeKind::ShopItemPriceMinus => Icon::new(IconKind::Shop)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .to_rendering_tree(),
            UpgradeKind::ShopRefreshPlus => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shop)
                .add_reroll_indicator()
                .build(),

            // 퀘스트 보드 리프레시
            UpgradeKind::QuestBoardRefreshPlus => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Quest)
                .add_reroll_indicator()
                .build(),

            // 리롤 없음 관련 업그레이드들
            UpgradeKind::NoRerollTowerAttackDamagePlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::NoReroll,
                    StatType::Damage,
                    OperationType::Plus,
                )
            }
            UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::NoReroll,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::NoRerollTowerAttackSpeedPlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::NoReroll,
                    StatType::Speed,
                    OperationType::Plus,
                )
            }
            UpgradeKind::NoRerollTowerAttackSpeedMultiply { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::NoReroll,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::NoRerollTowerAttackRangePlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::NoReroll,
                    StatType::Range,
                    OperationType::Plus,
                )
            }

            // 짝수/홀수 관련 업그레이드들
            UpgradeKind::EvenOddTowerAttackDamagePlus { even, .. } => {
                create_even_odd_stat_upgrade_thumbnail(
                    width_height,
                    *even,
                    StatType::Damage,
                    OperationType::Plus,
                )
            }
            UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                create_even_odd_stat_upgrade_thumbnail(
                    width_height,
                    *even,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::EvenOddTowerAttackSpeedPlus { even, .. } => {
                create_even_odd_stat_upgrade_thumbnail(
                    width_height,
                    *even,
                    StatType::Speed,
                    OperationType::Plus,
                )
            }
            UpgradeKind::EvenOddTowerAttackSpeedMultiply { even, .. } => {
                create_even_odd_stat_upgrade_thumbnail(
                    width_height,
                    *even,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::EvenOddTowerAttackRangePlus { even, .. } => {
                create_even_odd_stat_upgrade_thumbnail(
                    width_height,
                    *even,
                    StatType::Range,
                    OperationType::Plus,
                )
            }

            // 페이스/숫자 카드 관련 업그레이드들
            UpgradeKind::FaceNumberCardTowerAttackDamagePlus { face, .. } => {
                create_face_number_stat_upgrade_thumbnail(
                    width_height,
                    *face,
                    StatType::Damage,
                    OperationType::Plus,
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                create_face_number_stat_upgrade_thumbnail(
                    width_height,
                    *face,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedPlus { face, .. } => {
                create_face_number_stat_upgrade_thumbnail(
                    width_height,
                    *face,
                    StatType::Speed,
                    OperationType::Plus,
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackSpeedMultiply { face, .. } => {
                create_face_number_stat_upgrade_thumbnail(
                    width_height,
                    *face,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::FaceNumberCardTowerAttackRangePlus { face, .. } => {
                create_face_number_stat_upgrade_thumbnail(
                    width_height,
                    *face,
                    StatType::Range,
                    OperationType::Plus,
                )
            }

            // 특수 카드 게임 규칙 업그레이드들
            UpgradeKind::ShortenStraightFlushTo4Cards => ThumbnailComposer::new(width_height)
                .with_default_tower()
                .add_shortcut_indicator("4")
                .build(),
            UpgradeKind::SkipRankForStraight => ThumbnailComposer::new(width_height)
                .with_default_tower()
                .add_skip_indicator()
                .build(),
            UpgradeKind::TreatSuitsAsSame => ThumbnailComposer::new(width_height)
                .with_default_tower()
                .add_same_suits_indicator()
                .build(),

            // 리롤 관련 타워 업그레이드들
            UpgradeKind::RerollTowerAttackDamagePlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::Reroll,
                    StatType::Damage,
                    OperationType::Plus,
                )
            }
            UpgradeKind::RerollTowerAttackDamageMultiply { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::Reroll,
                    StatType::Damage,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::RerollTowerAttackSpeedPlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::Reroll,
                    StatType::Speed,
                    OperationType::Plus,
                )
            }
            UpgradeKind::RerollTowerAttackSpeedMultiply { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::Reroll,
                    StatType::Speed,
                    OperationType::Multiply,
                )
            }
            UpgradeKind::RerollTowerAttackRangePlus { .. } => {
                create_condition_stat_upgrade_thumbnail(
                    width_height,
                    ConditionType::Reroll,
                    StatType::Range,
                    OperationType::Plus,
                )
            }
        }
    }
}

// 스탯 타입을 정의하는 열거형
#[derive(Clone, Copy)]
enum StatType {
    Damage,
    Speed,
    Range,
}

impl StatType {
    fn to_icon_kind(self) -> IconKind {
        match self {
            StatType::Damage => IconKind::AttackDamage,
            StatType::Speed => IconKind::AttackSpeed,
            StatType::Range => IconKind::AttackRange,
        }
    }
}

// 연산 타입을 정의하는 열거형
#[derive(Clone, Copy)]
enum OperationType {
    Plus,
    Multiply,
}

impl OperationType {
    fn to_icon_kind(self) -> IconKind {
        match self {
            OperationType::Plus => IconKind::Add,
            OperationType::Multiply => IconKind::Multiply,
        }
    }
}

// 조건 타입을 정의하는 열거형
#[derive(Clone, Copy)]
enum ConditionType {
    LowCard,
    NoReroll,
    Reroll,
}

// 랭크 기반 스탯 업그레이드 썸네일 생성 함수
fn create_rank_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    rank: crate::card::Rank,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_rank_overlay(rank)
        .add_icon_overlay(
            stat_type.to_icon_kind(),
            OverlayPosition::BottomLeft,
            OVERLAY_SIZE_RATIO,
        )
        .add_icon_overlay(
            operation_type.to_icon_kind(),
            OverlayPosition::TopRight,
            OVERLAY_SIZE_RATIO,
        )
        .build()
}

// 슈트 기반 스탯 업그레이드 썸네일 생성 함수
fn create_suit_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    suit: crate::card::Suit,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_suit_overlay(suit)
        .add_icon_overlay(
            stat_type.to_icon_kind(),
            OverlayPosition::BottomLeft,
            OVERLAY_SIZE_RATIO,
        )
        .add_icon_overlay(
            operation_type.to_icon_kind(),
            OverlayPosition::TopRight,
            OVERLAY_SIZE_RATIO,
        )
        .build()
}

// 핸드 기반 스탯 업그레이드 썸네일 생성 함수
fn create_hand_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    tower_kind: crate::game_state::tower::TowerKind,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_tower_image(tower_kind)
        .add_icon_overlay(
            stat_type.to_icon_kind(),
            OverlayPosition::BottomRight,
            OVERLAY_SIZE_RATIO,
        )
        .add_icon_overlay(
            operation_type.to_icon_kind(),
            OverlayPosition::TopRight,
            OVERLAY_SIZE_RATIO,
        )
        .build()
}

// 조건 기반 스탯 업그레이드 썸네일 생성 함수
fn create_condition_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    condition_type: ConditionType,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    let mut composer = ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_icon_overlay(
            stat_type.to_icon_kind(),
            OverlayPosition::BottomRight,
            OVERLAY_SIZE_RATIO,
        )
        .add_icon_overlay(
            operation_type.to_icon_kind(),
            OverlayPosition::TopRight,
            OVERLAY_SIZE_RATIO,
        );

    match condition_type {
        ConditionType::LowCard => composer = composer.add_low_card_indicator(),
        ConditionType::NoReroll => composer = composer.add_no_reroll_indicator(),
        ConditionType::Reroll => composer = composer.add_reroll_indicator(),
    }

    composer.build()
}

// 짝수/홀수 기반 스탯 업그레이드 썸네일 생성 함수
fn create_even_odd_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    is_even: bool,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_even_odd_indicator(is_even)
        .add_icon_overlay(
            stat_type.to_icon_kind(),
            OverlayPosition::BottomRight,
            OVERLAY_SIZE_RATIO,
        )
        .add_icon_overlay(
            operation_type.to_icon_kind(),
            OverlayPosition::TopRight,
            OVERLAY_SIZE_RATIO,
        )
        .build()
}

// 페이스/숫자 카드 기반 스탯 업그레이드 썸네일 생성 함수
fn create_face_number_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    is_face: bool,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_face_number_indicator(is_face)
        .add_icon_overlay(
            stat_type.to_icon_kind(),
            OverlayPosition::BottomRight,
            OVERLAY_SIZE_RATIO,
        )
        .add_icon_overlay(
            operation_type.to_icon_kind(),
            OverlayPosition::TopRight,
            OVERLAY_SIZE_RATIO,
        )
        .build()
}
