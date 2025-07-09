use crate::card::{Rank, Suit};
use namui::Duration;

pub enum ItemText<'a> {
    HealName,
    HealDesc {
        amount: f32,
    },
    AttackPowerPlusBuffName,
    AttackPowerPlusBuffDesc {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackPowerMultiplyBuffName,
    AttackPowerMultiplyBuffDesc {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackSpeedPlusBuffName,
    AttackSpeedPlusBuffDesc {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackSpeedMultiplyBuffName,
    AttackSpeedMultiplyBuffDesc {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    AttackRangePlusName,
    AttackRangePlusDesc {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    MovementSpeedDebuffName,
    MovementSpeedDebuffDesc {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    RoundDamageName,
    RoundDamageDesc {
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        radius: f32,
    },
    RoundDamageOverTimeName,
    RoundDamageOverTimeDesc {
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        radius: f32,
        duration: Duration,
    },
    LotteryName,
    LotteryDesc {
        amount: f32,
        probability: f32,
    },
    LinearDamageName,
    LinearDamageDesc {
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        thickness: f32,
    },
    LinearDamageOverTimeName,
    LinearDamageOverTimeDesc {
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        thickness: f32,
        duration: Duration,
    },
    ExtraRerollName,
    ExtraRerollDesc,
    ShieldName,
    ShieldDesc {
        amount: f32,
    },
    DamageReductionName,
    DamageReductionDesc {
        damage_multiply: f32,
        duration: Duration,
    },
}

impl<'a> ItemText<'a> {
    pub fn to_korean(&self) -> String {
        match self {
            ItemText::HealName => "치유".to_string(),
            ItemText::HealDesc { amount } => format!("체력을 {amount} 회복합니다"),
            ItemText::AttackPowerPlusBuffName => "공격력 증가 버프".to_string(),
            ItemText::AttackPowerPlusBuffDesc {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::AttackPowerMultiplyBuffName => "공격력 배수 버프".to_string(),
            ItemText::AttackPowerMultiplyBuffDesc {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격력을 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::AttackSpeedPlusBuffName => "공격 속도 증가 버프".to_string(),
            ItemText::AttackSpeedPlusBuffDesc {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::AttackSpeedMultiplyBuffName => "공격 속도 배수 버프".to_string(),
            ItemText::AttackSpeedMultiplyBuffDesc {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 공격 속도를 {amount}배 만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::AttackRangePlusName => "공격 범위 증가".to_string(),
            ItemText::AttackRangePlusDesc {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 타워들의 사거리를 {amount}만큼 증가시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::MovementSpeedDebuffName => "이동 속도 감소 디버프".to_string(),
            ItemText::MovementSpeedDebuffDesc {
                amount,
                duration,
                radius,
            } => format!(
                "{radius} 범위 내 적들의 이동 속도를 {amount}만큼 감소시킵니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::RoundDamageName => "범위 피해".to_string(),
            ItemText::RoundDamageDesc {
                rank,
                suit,
                damage,
                radius,
            } => format!("{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."),
            ItemText::RoundDamageOverTimeName => "지속 범위 피해".to_string(),
            ItemText::RoundDamageOverTimeDesc {
                rank,
                suit,
                damage,
                radius,
                duration,
            } => format!(
                "{radius} 범위 내 적들에게 {damage}만큼의 {suit}{rank} 지속 피해를 입힙니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::LotteryName => "복권".to_string(),
            ItemText::LotteryDesc {
                amount,
                probability,
            } => format!("{probability}% 확률로 {amount}의 보상을 획득합니다"),
            ItemText::LinearDamageName => "광선 피해".to_string(),
            ItemText::LinearDamageDesc {
                rank,
                suit,
                damage,
                thickness,
            } => format!(
                "{thickness} 두께의 직선 범위 내 적들에게 {damage}만큼의 {suit}{rank} 피해를 입힙니다."
            ),
            ItemText::LinearDamageOverTimeName => "지속 광선 피해".to_string(),
            ItemText::LinearDamageOverTimeDesc {
                rank,
                suit,
                damage,
                thickness,
                duration,
            } => format!(
                "{thickness} 두께의 직선 범위 내 적들에게 {damage}만큼의 {suit}{rank} 지속 피해를 입힙니다. {duration:?} 동안 지속됩니다"
            ),
            ItemText::ExtraRerollName => "추가 리롤".to_string(),
            ItemText::ExtraRerollDesc => "추가 리롤을 획득합니다".to_string(),
            ItemText::ShieldName => "방어막".to_string(),
            ItemText::ShieldDesc { amount } => {
                format!("이번 라운드에 피해를 {amount}흡수하는 방어막을 획득합니다.")
            }
            ItemText::DamageReductionName => "피해 감소".to_string(),
            ItemText::DamageReductionDesc {
                damage_multiply,
                duration,
            } => format!("{duration:?} 동안 받는 피해를 {damage_multiply}만큼 감소시킵니다"),
        }
    }
}
