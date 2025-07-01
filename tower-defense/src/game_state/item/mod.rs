mod effect_processor;
mod generation;
mod usage;

use crate::{
    card::{Rank, Suit},
    rarity::Rarity,
};
pub use generation::*;
use namui::*;
pub use usage::*;

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub rarity: Rarity,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemKind {
    Heal {
        amount: f32,
    },
    AttackPowerPlusBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackPowerMultiplyBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackSpeedPlusBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackSpeedMultiplyBuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackRangePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    MovementSpeedDebuff {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    RoundDamage {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
    },
    RoundDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
        duration: Duration,
    },
    Lottery {
        amount: f32,
        probability: f32,
    },
    LinearDamage {
        rank: Rank,
        suit: Suit,
        damage: f32,
        thickness: f32,
    },
    LinearDamageOverTime {
        rank: Rank,
        suit: Suit,
        damage: f32,
        thickness: f32,
        duration: Duration,
    },
    ExtraReroll,
    Shield {
        amount: f32,
    },
    DamageReduction {
        damage_multiply: f32,
        duration: Duration,
    },
}

impl ItemKind {
    pub fn name(&self) -> &'static str {
        match self {
            ItemKind::Heal { .. } => "치유",
            ItemKind::AttackPowerPlusBuff { .. } => "공격력 증가 버프",
            ItemKind::AttackPowerMultiplyBuff { .. } => "공격력 배수 버프",
            ItemKind::AttackSpeedPlusBuff { .. } => "공격 속도 증가 버프",
            ItemKind::AttackSpeedMultiplyBuff { .. } => "공격 속도 배수 버프",
            ItemKind::AttackRangePlus { .. } => "공격 범위 증가",
            ItemKind::MovementSpeedDebuff { .. } => "이동 속도 감소 디버프",
            ItemKind::RoundDamage { .. } => "범위 피해",
            ItemKind::RoundDamageOverTime { .. } => "지속 범위 피해",
            ItemKind::Lottery { .. } => "복권",
            ItemKind::LinearDamage { .. } => "광선 피해",
            ItemKind::LinearDamageOverTime { .. } => "지속 광선 피해",
            ItemKind::ExtraReroll => "추가 리롤",
            ItemKind::Shield { .. } => "방어막",
            ItemKind::DamageReduction { .. } => "피해 감소",
        }
    }
    pub fn description(&self) -> String {
        match self {
            ItemKind::Heal { amount } => format!("체력을 {amount} 회복합니다"),
            ItemKind::AttackPowerPlusBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackPowerMultiplyBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackSpeedPlusBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackSpeedMultiplyBuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::AttackRangePlus {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 사거리를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::MovementSpeedDebuff {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 적들의 이동 속도를 {amount}만큼 감소시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::RoundDamage {
                rank,
                suit,
                damage,
                radius,
            } => format!("{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."),
            ItemKind::RoundDamageOverTime {
                rank,
                suit,
                damage,
                radius,
                duration,
            } => format!(
                "{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 지속 피해를 입힙니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::Lottery {
                amount,
                probability,
            } => format!("{probability}% 확률로 {amount}의 보상을 획득합니다"),
            ItemKind::LinearDamage {
                rank,
                suit,
                damage,
                thickness,
            } => format!(
                "{thickness} 두께의 직선 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."
            ),
            ItemKind::LinearDamageOverTime {
                rank,
                suit,
                damage,
                thickness,
                duration,
            } => format!(
                "{thickness} 두께의 직선 범위 내 적들에게 {damage}만큼의 {suit}{rank} 지속 피해를 입힙니다. {duration:?} 동안 지속됩니다"
            ),
            ItemKind::ExtraReroll => "추가 리롤을 획득합니다".to_string(),
            ItemKind::Shield { amount } => {
                format!("이번 라운드에 피해를 {amount}흡수하는 방어막을 획득합니다.")
            }
            ItemKind::DamageReduction {
                damage_multiply: amount,
                duration,
            } => {
                format!("{duration:?} 동안 받는 피해를 {amount}만큼 감소시킵니다")
            }
        }
    }
}

pub fn item_cost(rarity: &Rarity, shop_item_price_minus: usize) -> usize {
    (match rarity {
        Rarity::Common => 25,
        Rarity::Rare => 50,
        Rarity::Epic => 75,
        Rarity::Legendary => 100,
    } - shop_item_price_minus)
}
