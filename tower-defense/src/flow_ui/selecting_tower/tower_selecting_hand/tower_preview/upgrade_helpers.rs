use crate::{
    game_state::upgrade::{TowerSelectUpgradeTarget, TowerUpgradeTarget, UpgradeKind},
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

/// TowerUpgradeTarget에서 적절한 UpgradeKind를 생성하는 함수
pub fn create_upgrade_kind_for_target(
    target: &TowerUpgradeTarget,
    stat_type: UpgradeStatType,
    is_additive: bool,
    value: f32,
) -> UpgradeKind {
    match (target, stat_type, is_additive) {
        // Suit 기반 업그레이드
        (TowerUpgradeTarget::Suit { suit }, UpgradeStatType::Damage, false) => match suit {
            crate::card::Suit::Diamonds => UpgradeKind::Staff {
                damage_multiplier: value,
            },
            crate::card::Suit::Spades => UpgradeKind::LongSword {
                damage_multiplier: value,
            },
            crate::card::Suit::Hearts => UpgradeKind::Mace {
                damage_multiplier: value,
            },
            crate::card::Suit::Clubs => UpgradeKind::ClubSword {
                damage_multiplier: value,
            },
        },

        // EvenOdd 기반 업그레이드
        (TowerUpgradeTarget::EvenOdd { even }, UpgradeStatType::Damage, false) => {
            if *even {
                UpgradeKind::PairChopsticks {
                    damage_multiplier: value,
                }
            } else {
                UpgradeKind::SingleChopstick {
                    damage_multiplier: value,
                }
            }
        }

        // FaceNumber 기반 업그레이드
        (TowerUpgradeTarget::FaceNumber { face }, UpgradeStatType::Damage, false) => {
            if *face {
                UpgradeKind::Brush {
                    damage_multiplier: value,
                }
            } else {
                UpgradeKind::FountainPen {
                    damage_multiplier: value,
                }
            }
        }

        // 기타 경우는 기본값 반환 (이는 실제로는 발생하지 않아야 함)
        _ => UpgradeKind::Cat { add: 1 },
    }
}

/// TowerSelectUpgradeTarget에서 적절한 UpgradeKind를 생성하는 함수
pub fn create_tower_select_upgrade_kind(
    target: &TowerSelectUpgradeTarget,
    stat_type: UpgradeStatType,
    is_additive: bool,
    value: f32,
) -> UpgradeKind {
    match (target, stat_type, is_additive) {
        (TowerSelectUpgradeTarget::LowCard, UpgradeStatType::Damage, false) => {
            UpgradeKind::Tricycle {
                damage_multiplier: value,
            }
        }

        (TowerSelectUpgradeTarget::NoReroll, UpgradeStatType::Damage, false) => {
            UpgradeKind::PerfectPottery {
                damage_multiplier: value,
            }
        }

        (TowerSelectUpgradeTarget::Reroll, UpgradeStatType::Damage, false) => {
            UpgradeKind::BrokenPottery {
                damage_multiplier: value,
            }
        }

        // 기타 경우는 기본값 반환
        _ => UpgradeKind::Cat { add: 1 },
    }
}
