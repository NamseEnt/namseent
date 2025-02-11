use crate::card::{Rank, Suit};
use namui::*;

#[derive(Debug, Clone)]
pub enum Item {
    Heal {
        amount: f32,
    },
    TowerDamagePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerDamageMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerSpeedPlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerSpeedMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerRangePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    WeakenMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    SlowdownMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    Attack {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
    },
}
impl Item {
    pub fn name(&self) -> &'static str {
        match self {
            Item::Heal { .. } => "회복",
            Item::TowerDamagePlus { .. } => "타워 공격력 증가",
            Item::TowerDamageMultiply { .. } => "타워 공격력 증가",
            Item::TowerSpeedPlus { .. } => "타워 공격 속도 증가",
            Item::TowerSpeedMultiply { .. } => "타워 공격 속도 증가",
            Item::TowerRangePlus { .. } => "타워 사거리 증가",
            Item::WeakenMultiply { .. } => "적 공격력 약화",
            Item::SlowdownMultiply { .. } => "적 슬로우",
            Item::Attack { .. } => "범위공격",
        }
    }
    pub fn description(&self) -> String {
        match self {
            Item::Heal { amount } => format!("체력을 {amount} 회복합니다"),
            Item::TowerDamagePlus { amount, duration, radius } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            Item::TowerDamageMultiply { amount, duration, radius } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            Item::TowerSpeedPlus { amount, duration, radius } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            Item::TowerSpeedMultiply { amount, duration, radius } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            Item::TowerRangePlus { amount, duration, radius } => format!(
                "{radius} 범위 내 타워들의 사거리를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            Item::WeakenMultiply { amount, duration, radius } => format!(
                "{radius} 범위 내 적들의 공격력을 {amount}배 만큼 약화시킵니다. {duration:?} 동안 지속됩니다"
            ),
            Item::SlowdownMultiply { amount, duration, radius } => format!(
                "{radius} 범위 내 적들의 이동 속도를 {amount}배 만큼 느리게 합니다. {duration:?} 동안 지속됩니다"
            ),
            Item::Attack { rank, suit, damage, radius } => format!(
                "{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."
            ),
        }
    }
}

pub fn use_item(item_index: usize) -> &'static Item {
    todo!()
}
