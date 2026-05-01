use crate::{
    game_state::upgrade::{TowerSelectUpgradeTarget, TowerUpgradeTarget, Upgrade},
    *,
};

#[derive(Debug, Clone, Copy, State)]
pub enum UpgradeStatType {
    Damage,
    Speed,
    Range,
}

#[derive(Debug, Clone, State)]
pub enum UpgradeTargetType {
    Tower(TowerUpgradeTarget),
    TowerSelect(TowerSelectUpgradeTarget),
}

/// TowerUpgradeTarget에서 적절한 UpgradeType를 생성하는 함수
pub fn create_upgrade_kind_for_target(
    target: &TowerUpgradeTarget,
    stat_type: UpgradeStatType,
    is_additive: bool,
    value: f32,
) -> Upgrade {
    match (target, stat_type, is_additive) {
        // Suit 기반 업그레이드
        (TowerUpgradeTarget::Suit { suit }, UpgradeStatType::Damage, false) => match suit {
            crate::card::Suit::Diamonds => crate::game_state::upgrade::StaffUpgrade::into_upgrade(value),
            crate::card::Suit::Spades => crate::game_state::upgrade::LongSwordUpgrade::into_upgrade(value),
            crate::card::Suit::Hearts => crate::game_state::upgrade::MaceUpgrade::into_upgrade(value),
            crate::card::Suit::Clubs => crate::game_state::upgrade::ClubSwordUpgrade::into_upgrade(value),
        },

        // EvenOdd 기반 업그레이드
        (TowerUpgradeTarget::EvenOdd { even }, UpgradeStatType::Damage, false) => {
            if *even {
                crate::game_state::upgrade::PairChopsticksUpgrade::into_upgrade(value)
            } else {
                crate::game_state::upgrade::SingleChopstickUpgrade::into_upgrade(value)
            }
        }

        // FaceNumber 기반 업그레이드
        (TowerUpgradeTarget::FaceNumber { face }, UpgradeStatType::Damage, false) => {
            if *face {
                crate::game_state::upgrade::BrushUpgrade::into_upgrade(value)
            } else {
                crate::game_state::upgrade::FountainPenUpgrade::into_upgrade(value)
            }
        }

        // 기타 경우는 기본값 반환 (이는 실제로는 발생하지 않아야 함)
        _ => crate::game_state::upgrade::CatUpgrade::into_upgrade(1),
    }
}

/// TowerSelectUpgradeTarget에서 적절한 UpgradeType를 생성하는 함수
pub fn create_tower_select_upgrade_kind(
    target: &TowerSelectUpgradeTarget,
    stat_type: UpgradeStatType,
    is_additive: bool,
    value: f32,
) -> Upgrade {
    match (target, stat_type, is_additive) {
        (TowerSelectUpgradeTarget::LowCard, UpgradeStatType::Damage, false) => crate::game_state::upgrade::TricycleUpgrade::into_upgrade(value),
        (TowerSelectUpgradeTarget::NoReroll, UpgradeStatType::Damage, false) => {
            crate::game_state::upgrade::PerfectPotteryUpgrade::into_upgrade(value)
        }
        (TowerSelectUpgradeTarget::Reroll, UpgradeStatType::Damage, false) => {
            crate::game_state::upgrade::BrokenPotteryUpgrade::into_upgrade(value)
        }
        // 기타 경우는 기본값 반환
        _ => crate::game_state::upgrade::CatUpgrade::into_upgrade(1),
    }
}
