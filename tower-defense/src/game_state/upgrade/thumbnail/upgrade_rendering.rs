use crate::{
    game_state::upgrade::UpgradeKind,
    icon::{Icon, IconKind, IconSize},
    thumbnail::{
        ThumbnailComposer,
        constants::OVERLAY_SIZE_RATIO,
        overlay_rendering::OverlayPosition,
    },
};
use namui::*;

impl UpgradeKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            UpgradeKind::Magnet => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .add_plus_overlay()
                .build(),

            UpgradeKind::CainSword { .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                crate::card::Suit::Diamonds,
                StatType::Damage,
                OperationType::Multiply,
            ),
            UpgradeKind::LongSword { .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                crate::card::Suit::Spades,
                StatType::Damage,
                OperationType::Multiply,
            ),
            UpgradeKind::Mace { .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                crate::card::Suit::Hearts,
                StatType::Damage,
                OperationType::Multiply,
            ),
            UpgradeKind::ClubSword { .. } => create_suit_stat_upgrade_thumbnail(
                width_height,
                crate::card::Suit::Clubs,
                StatType::Damage,
                OperationType::Multiply,
            ),

            UpgradeKind::Backpack => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shop)
                .add_expansion_indicator("+")
                .build(),

            UpgradeKind::DiceBundle => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Refresh)
                .add_plus_overlay()
                .build(),

            UpgradeKind::Spoon { .. } => create_condition_stat_upgrade_thumbnail(
                width_height,
                ConditionType::LowCard,
                StatType::Damage,
                OperationType::Multiply,
            ),

            UpgradeKind::EnergyDrink => Icon::new(IconKind::Shop)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .to_rendering_tree(),

            UpgradeKind::PerfectPottery { .. } => create_condition_stat_upgrade_thumbnail(
                width_height,
                ConditionType::NoReroll,
                StatType::Damage,
                OperationType::Multiply,
            ),

            UpgradeKind::SingleChopstick { .. } => create_even_odd_stat_upgrade_thumbnail(
                width_height,
                false,
                StatType::Damage,
                OperationType::Multiply,
            ),
            UpgradeKind::PairChopsticks { .. } => create_even_odd_stat_upgrade_thumbnail(
                width_height,
                true,
                StatType::Damage,
                OperationType::Multiply,
            ),

            UpgradeKind::FountainPen { .. } => create_face_number_stat_upgrade_thumbnail(
                width_height,
                false,
                StatType::Damage,
                OperationType::Multiply,
            ),
            UpgradeKind::Brush { .. } => create_face_number_stat_upgrade_thumbnail(
                width_height,
                true,
                StatType::Damage,
                OperationType::Multiply,
            ),

            UpgradeKind::FourLeafClover => ThumbnailComposer::new(width_height)
                .with_default_tower()
                .add_shortcut_indicator("4")
                .build(),
            UpgradeKind::Rabbit => ThumbnailComposer::new(width_height)
                .with_default_tower()
                .add_skip_indicator()
                .build(),
            UpgradeKind::BlackWhite => ThumbnailComposer::new(width_height)
                .with_default_tower()
                .add_same_suits_indicator()
                .build(),

            UpgradeKind::BrokenPottery { .. } => create_condition_stat_upgrade_thumbnail(
                width_height,
                ConditionType::Reroll,
                StatType::Damage,
                OperationType::Multiply,
            ),
        }
    }
}

// 스탯 타입을 정의하는 열거형
#[derive(Clone, Copy, State)]
enum StatType {
    Damage,
    Speed,
    Range,
}

impl StatType {
    fn to_icon_kind(self) -> IconKind {
        match self {
            StatType::Damage => IconKind::Damage,
            StatType::Speed => IconKind::Damage,
            StatType::Range => IconKind::Damage,
        }
    }
}

// 연산 타입을 정의하는 열거형
#[derive(Clone, Copy, State)]
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
#[derive(Clone, Copy, State)]
enum ConditionType {
    LowCard,
    NoReroll,
    Reroll,
}

// 랭크 기반 스탯 업그레이드 썸네일 생성 함수
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    let with_condition = match condition_type {
        ConditionType::LowCard => ThumbnailComposer::new(width_height).add_low_card_indicator(),
        ConditionType::NoReroll => ThumbnailComposer::new(width_height).add_no_reroll_indicator(),
        ConditionType::Reroll => ThumbnailComposer::new(width_height).add_reroll_indicator(),
    };

    with_condition
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

// 짝수/홀수 기반 스탯 업그레이드 썸네일 생성 함수
fn create_even_odd_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    even: bool,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_even_odd_indicator(even)
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

// 페이스/숫자 기반 스탯 업그레이드 썸네일 생성 함수
fn create_face_number_stat_upgrade_thumbnail(
    width_height: Wh<Px>,
    face: bool,
    stat_type: StatType,
    operation_type: OperationType,
) -> RenderingTree {
    ThumbnailComposer::new(width_height)
        .with_default_tower()
        .add_face_number_indicator(face)
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
