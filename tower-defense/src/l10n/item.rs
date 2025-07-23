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
                ItemKindTextVariant::Heal => "|gold_color|icon<heal:16:16:16:1>치유|/gold_color|".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "|attack_damage_color|icon<attack_damage:16:16:16:1>공격력 증가 버프|/attack_damage_color|".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => "|attack_damage_color|icon<attack_damage:16:16:16:1>공격력 배수 버프|/attack_damage_color|".to_string(),
                ItemKindTextVariant::AttackSpeedPlusBuff => "|attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도 증가 버프|/attack_speed_color|".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => "|attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도 배수 버프|/attack_speed_color|".to_string(),
                ItemKindTextVariant::AttackRangePlus => "|attack_range_color|icon<attack_range:16:16:16:1>공격 범위 증가|/attack_range_color|".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "|red|이동 속도 감소 디버프|/red|".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => "|attack_damage_color|icon<attack_damage:16:16:16:1>범위 피해|/attack_damage_color|".to_string(),
                ItemKindTextVariant::RoundDamageOverTime { .. } => "|attack_damage_color|icon<attack_damage:16:16:16:1>지속 범위 피해|/attack_damage_color|".to_string(),
                ItemKindTextVariant::Lottery => "|gold_color|icon<gold:16:16:16:1>복권|/gold_color|".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => "|attack_damage_color|icon<attack_damage:16:16:16:1>광선 피해|/attack_damage_color|".to_string(),
                ItemKindTextVariant::LinearDamageOverTime { .. } => "|attack_damage_color|icon<attack_damage:16:16:16:1>지속 광선 피해|/attack_damage_color|".to_string(),
                ItemKindTextVariant::ExtraReroll => "|blue|추가 리롤|/blue|".to_string(),
                ItemKindTextVariant::Shield => "|blue|방어막|/blue|".to_string(),
                ItemKindTextVariant::DamageReduction => "|blue|피해 감소|/blue|".to_string(),
            },
            ItemKindText::Description(variant) => match variant {
                ItemKindTextVariant::Heal => "|gold_color|icon<heal:16:16:16:1>체력|/gold_color|을 회복합니다".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "|attack_damage_color|icon<attack_damage:16:16:16:1>공격력|/attack_damage_color|을 증가시킵니다".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => {
                    "|attack_damage_color|icon<attack_damage:16:16:16:1>공격력|/attack_damage_color|을 배수로 증가시킵니다".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff => "|attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도|/attack_speed_color|를 증가시킵니다".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => {
                    "|attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도|/attack_speed_color|를 배수로 증가시킵니다".to_string()
                }
                ItemKindTextVariant::AttackRangePlus => "|attack_range_color|icon<attack_range:16:16:16:1>공격 범위|/attack_range_color|를 증가시킵니다".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "|red|이동 속도|/red|를 감소시킵니다".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => {
                    "범위 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>피해|/attack_damage_color|를 입힙니다".to_string()
                }
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "범위 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>지속 피해|/attack_damage_color|를 입힙니다".to_string()
                }
                ItemKindTextVariant::Lottery => "확률적으로 |gold_color|icon<gold:16:16:16:1>보상|/gold_color|을 획득합니다".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => {
                    "직선 범위 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>피해|/attack_damage_color|를 입힙니다".to_string()
                }
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "직선 범위 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>지속 피해|/attack_damage_color|를 입힙니다".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "|blue|추가 리롤|/blue|을 획득합니다".to_string(),
                ItemKindTextVariant::Shield => "|blue|피해를 흡수하는 방어막|/blue|을 획득합니다".to_string(),
                ItemKindTextVariant::DamageReduction => "받는 |red|피해를 감소|/red|시킵니다".to_string(),
            },
        }
    }
    pub(super) fn to_english(&self) -> String {
        match self {
            ItemKindText::Name(variant) => match variant {
                ItemKindTextVariant::Heal => "|gold_color|icon<heal:16:16:16:1>Heal|/gold_color|".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "|attack_damage_color|icon<attack_damage:16:16:16:1>Attack Power Plus Buff|/attack_damage_color|".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => {
                    "|attack_damage_color|icon<attack_damage:16:16:16:1>Attack Power Multiply Buff|/attack_damage_color|".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff => "|attack_speed_color|icon<attack_speed:16:16:16:1>Attack Speed Plus Buff|/attack_speed_color|".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => {
                    "|attack_speed_color|icon<attack_speed:16:16:16:1>Attack Speed Multiply Buff|/attack_speed_color|".to_string()
                }
                ItemKindTextVariant::AttackRangePlus => "|attack_range_color|icon<attack_range:16:16:16:1>Attack Range Plus|/attack_range_color|".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "|red|Movement Speed Debuff|/red|".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => "|attack_damage_color|icon<attack_damage:16:16:16:1>Area Damage|/attack_damage_color|".to_string(),
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "|attack_damage_color|icon<attack_damage:16:16:16:1>Damage Over Time in Area|/attack_damage_color|".to_string()
                }
                ItemKindTextVariant::Lottery => "|gold_color|icon<gold:16:16:16:1>Lottery|/gold_color|".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => "|attack_damage_color|icon<attack_damage:16:16:16:1>Beam Damage|/attack_damage_color|".to_string(),
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "|attack_damage_color|icon<attack_damage:16:16:16:1>Damage Over Time in Beam|/attack_damage_color|".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "|blue|Extra Reroll|/blue|".to_string(),
                ItemKindTextVariant::Shield => "|blue|Shield|/blue|".to_string(),
                ItemKindTextVariant::DamageReduction => "|blue|Damage Reduction|/blue|".to_string(),
            },
            ItemKindText::Description(variant) => match variant {
                ItemKindTextVariant::Heal => "|gold_color|icon<heal:16:16:16:1>Restores health|/gold_color|".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff => "|attack_damage_color|icon<attack_damage:16:16:16:1>Increases attack power|/attack_damage_color|".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff => {
                    "|attack_damage_color|icon<attack_damage:16:16:16:1>Increases attack power by a factor|/attack_damage_color|".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff => "|attack_speed_color|icon<attack_speed:16:16:16:1>Increases attack speed|/attack_speed_color|".to_string(),
                ItemKindTextVariant::AttackSpeedMultiplyBuff => {
                    "|attack_speed_color|icon<attack_speed:16:16:16:1>Increases attack speed by a factor|/attack_speed_color|".to_string()
                }
                ItemKindTextVariant::AttackRangePlus => "|attack_range_color|icon<attack_range:16:16:16:1>Increases attack range|/attack_range_color|".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff => "|red|Decreases movement speed|/red|".to_string(),
                ItemKindTextVariant::RoundDamage { .. } => {
                    "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>area damage|/attack_damage_color| to enemies".to_string()
                }
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>damage over time|/attack_damage_color| in area".to_string()
                }
                ItemKindTextVariant::Lottery => "Chance to gain |gold_color|icon<gold:16:16:16:1>rewards|/gold_color|".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => {
                    "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>beam damage|/attack_damage_color| to enemies".to_string()
                }
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>damage over time|/attack_damage_color| in beam".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "Gain an |blue|extra reroll|/blue|".to_string(),
                ItemKindTextVariant::Shield => "Gain a |blue|shield that absorbs damage|/blue|".to_string(),
                ItemKindTextVariant::DamageReduction => "|blue|Reduces damage taken|/blue|".to_string(),
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
