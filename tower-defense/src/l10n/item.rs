use super::{Language, Locale, LocalizedText};
use crate::card::{Rank, Suit};

pub enum ItemKindText<'a> {
    Name(ItemKindTextVariant<'a>),
    Description(ItemKindTextVariant<'a>),
}

pub enum ItemKindTextVariant<'a> {
    Heal,
    AttackPowerPlusBuff,
    AttackPowerMultiplyBuff,
    AttackSpeedPlusBuff,
    AttackSpeedMultiplyBuff,
    AttackRangePlus,
    MovementSpeedDebuff,
    RoundDamage { rank: &'a Rank, suit: &'a Suit },
    RoundDamageOverTime { rank: &'a Rank, suit: &'a Suit },
    Lottery,
    LinearDamage { rank: &'a Rank, suit: &'a Suit },
    LinearDamageOverTime { rank: &'a Rank, suit: &'a Suit },
    ExtraReroll,
    Shield,
    DamageReduction,
}

impl<'a> ItemKindText<'a> {
    pub(super) fn to_korean(&self) -> String {
        match self {
            ItemKindText::Name(variant) => match variant {
                ItemKindTextVariant::Heal => "치유".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "공격력 증가 버프".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => "공격력 배수 버프".to_string(),
                ItemKindTextVariant::AttackSpeedPlusBuff => "공격 속도 증가 버프".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => "공격 속도 배수 버프".to_string(),
                ItemKindTextVariant::AttackRangePlus => "공격 범위 증가".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "이동 속도 감소 디버프".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => "범위 피해".to_string(),
                ItemKindTextVariant::RoundDamageOverTime { .. } => "지속 범위 피해".to_string(),
                ItemKindTextVariant::Lottery => "복권".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => "광선 피해".to_string(),
                ItemKindTextVariant::LinearDamageOverTime { .. } => "지속 광선 피해".to_string(),
                ItemKindTextVariant::ExtraReroll => "추가 리롤".to_string(),
                ItemKindTextVariant::Shield => "방어막".to_string(),
                ItemKindTextVariant::DamageReduction => "피해 감소".to_string(),
            },
            ItemKindText::Description(variant) => match variant {
                ItemKindTextVariant::Heal => "체력을 회복합니다".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "공격력을 증가시킵니다".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => {
                    "공격력을 배수로 증가시킵니다".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff => "공격 속도를 증가시킵니다".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => {
                    "공격 속도를 배수로 증가시킵니다".to_string()
                }
                ItemKindTextVariant::AttackRangePlus => "공격 범위를 증가시킵니다".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "이동 속도를 감소시킵니다".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => {
                    "범위 내 적들에게 피해를 입힙니다".to_string()
                }
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "범위 내 적들에게 지속 피해를 입힙니다".to_string()
                }
                ItemKindTextVariant::Lottery => "확률적으로 보상을 획득합니다".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => {
                    "직선 범위 내 적들에게 피해를 입힙니다".to_string()
                }
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "직선 범위 내 적들에게 지속 피해를 입힙니다".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "추가 리롤을 획득합니다".to_string(),
                ItemKindTextVariant::Shield => "피해를 흡수하는 방어막을 획득합니다".to_string(),
                ItemKindTextVariant::DamageReduction => "받는 피해를 감소시킵니다".to_string(),
            },
        }
    }
    pub(super) fn to_english(&self) -> String {
        match self {
            ItemKindText::Name(variant) => match variant {
                ItemKindTextVariant::Heal => "Heal".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "Attack Power Plus Buff".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => {
                    "Attack Power Multiply Buff".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff => "Attack Speed Plus Buff".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => {
                    "Attack Speed Multiply Buff".to_string()
                }
                ItemKindTextVariant::AttackRangePlus => "Attack Range Plus".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "Movement Speed Debuff".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => "Area Damage".to_string(),
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "Damage Over Time in Area".to_string()
                }
                ItemKindTextVariant::Lottery => "Lottery".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => "Beam Damage".to_string(),
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "Damage Over Time in Beam".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "Extra Reroll".to_string(),
                ItemKindTextVariant::Shield => "Shield".to_string(),
                ItemKindTextVariant::DamageReduction => "Damage Reduction".to_string(),
            },
            ItemKindText::Description(variant) => match variant {
                ItemKindTextVariant::Heal => "Restores health".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "Increases attack power".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => {
                    "Increases attack power by a factor".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff => "Increases attack speed".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => {
                    "Increases attack speed by a factor".to_string()
                }
                ItemKindTextVariant::AttackRangePlus => "Increases attack range".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "Decreases movement speed".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => {
                    "Deals area damage to enemies".to_string()
                }
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "Deals damage over time in area".to_string()
                }
                ItemKindTextVariant::Lottery => "Chance to gain rewards".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => {
                    "Deals beam damage to enemies".to_string()
                }
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "Deals damage over time in beam".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "Gain an extra reroll".to_string(),
                ItemKindTextVariant::Shield => "Gain a shield that absorbs damage".to_string(),
                ItemKindTextVariant::DamageReduction => "Reduces damage taken".to_string(),
            },
        }
    }
}

impl<'a> LocalizedText for ItemKindText<'a> {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}
