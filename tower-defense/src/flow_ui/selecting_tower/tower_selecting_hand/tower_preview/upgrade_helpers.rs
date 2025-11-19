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
        // Rank 기반 업그레이드
        (TowerUpgradeTarget::Rank { rank }, UpgradeStatType::Damage, false) => {
            UpgradeKind::RankAttackDamageMultiply {
                rank: *rank,
                damage_multiplier: value,
            }
        }
        (TowerUpgradeTarget::Rank { rank }, UpgradeStatType::Range, true) => {
            UpgradeKind::RankAttackRangePlus {
                rank: *rank,
                range_plus: value,
            }
        }

        // Suit 기반 업그레이드
        (TowerUpgradeTarget::Suit { suit }, UpgradeStatType::Damage, false) => {
            UpgradeKind::SuitAttackDamageMultiply {
                suit: *suit,
                damage_multiplier: value,
            }
        }
        (TowerUpgradeTarget::Suit { suit }, UpgradeStatType::Range, true) => {
            UpgradeKind::SuitAttackRangePlus {
                suit: *suit,
                range_plus: value,
            }
        }

        // TowerKind 기반 업그레이드
        (TowerUpgradeTarget::TowerKind { tower_kind }, UpgradeStatType::Damage, false) => {
            UpgradeKind::HandAttackDamageMultiply {
                tower_kind: *tower_kind,
                damage_multiplier: value,
            }
        }
        (TowerUpgradeTarget::TowerKind { tower_kind }, UpgradeStatType::Range, true) => {
            UpgradeKind::HandAttackRangePlus {
                tower_kind: *tower_kind,
                range_plus: value,
            }
        }

        // EvenOdd 기반 업그레이드
        (TowerUpgradeTarget::EvenOdd { even }, UpgradeStatType::Damage, false) => {
            UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even: *even,
                damage_multiplier: value,
            }
        }
        (TowerUpgradeTarget::EvenOdd { even }, UpgradeStatType::Range, true) => {
            UpgradeKind::EvenOddTowerAttackRangePlus {
                even: *even,
                range_plus: value,
            }
        }

        // FaceNumber 기반 업그레이드
        (TowerUpgradeTarget::FaceNumber { face }, UpgradeStatType::Damage, false) => {
            UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                face: *face,
                damage_multiplier: value,
            }
        }
        (TowerUpgradeTarget::FaceNumber { face }, UpgradeStatType::Range, true) => {
            UpgradeKind::FaceNumberCardTowerAttackRangePlus {
                face: *face,
                range_plus: value,
            }
        }

        // 기타 경우는 기본값 반환 (이는 실제로는 발생하지 않아야 함)
        _ => UpgradeKind::GoldEarnPlus,
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
            UpgradeKind::LowCardTowerDamageMultiply {
                damage_multiplier: value,
            }
        }
        (TowerSelectUpgradeTarget::LowCard, UpgradeStatType::Range, true) => {
            UpgradeKind::LowCardTowerAttackRangePlus { range_plus: value }
        }

        (TowerSelectUpgradeTarget::NoReroll, UpgradeStatType::Damage, false) => {
            UpgradeKind::NoRerollTowerAttackDamageMultiply {
                damage_multiplier: value,
            }
        }
        (TowerSelectUpgradeTarget::NoReroll, UpgradeStatType::Range, true) => {
            UpgradeKind::NoRerollTowerAttackRangePlus { range_plus: value }
        }

        (TowerSelectUpgradeTarget::Reroll, UpgradeStatType::Damage, false) => {
            UpgradeKind::RerollTowerAttackDamageMultiply {
                damage_multiplier: value,
            }
        }
        (TowerSelectUpgradeTarget::Reroll, UpgradeStatType::Range, true) => {
            UpgradeKind::RerollTowerAttackRangePlus { range_plus: value }
        }

        // 기타 경우는 기본값 반환
        _ => UpgradeKind::GoldEarnPlus,
    }
}
