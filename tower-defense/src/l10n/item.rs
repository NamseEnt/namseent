use super::{Language, Locale, LocalizedText, rich_text_helpers::*};
use crate::card::{Rank, Suit};
use namui::Duration;

pub enum ItemKindText<'a> {
    Name(ItemKindTextVariant<'a>),
    Description(ItemKindTextVariant<'a>),
}

pub enum ItemKindTextVariant<'a> {
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
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        radius: f32,
    },
    RoundDamageOverTime {
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        radius: f32,
        duration: Duration,
    },
    Lottery {
        amount: f32,
        probability: f32,
    },
    LinearDamage {
        rank: &'a Rank,
        suit: &'a Suit,
        damage: f32,
        thickness: f32,
    },
    LinearDamageOverTime {
        rank: &'a Rank,
        suit: &'a Suit,
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

impl<'a> ItemKindText<'a> {
    pub(super) fn to_korean(&self) -> String {
        match self {
            ItemKindText::Name(variant) => match variant {
                ItemKindTextVariant::Heal { .. } => "치유".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff { .. } => "공격력 증가 버프".to_string(),
                ItemKindTextVariant::AttackPowerMultiplyBuff { .. } => {
                    "공격력 배수 버프".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff { .. } => {
                    "공격 속도 증가 버프".to_string()
                }
                ItemKindTextVariant::AttackSpeedMultiplyBuff { .. } => {
                    "공격 속도 배수 버프".to_string()
                }
                ItemKindTextVariant::AttackRangePlus { .. } => "공격 범위 증가".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff { .. } => {
                    "이동 속도 감소 디버프".to_string()
                }
                ItemKindTextVariant::RoundDamage { .. } => "범위 피해".to_string(),
                ItemKindTextVariant::RoundDamageOverTime { .. } => "지속 범위 피해".to_string(),
                ItemKindTextVariant::Lottery { .. } => "복권".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => "광선 피해".to_string(),
                ItemKindTextVariant::LinearDamageOverTime { .. } => "지속 광선 피해".to_string(),
                ItemKindTextVariant::ExtraReroll => "추가 리롤".to_string(),
                ItemKindTextVariant::Shield { .. } => "방어막".to_string(),
                ItemKindTextVariant::DamageReduction { .. } => "피해 감소".to_string(),
            },
            ItemKindText::Description(variant) => match variant {
                ItemKindTextVariant::Heal { amount } => {
                    format!("{} 체력을 회복합니다", heal_icon(format!("{amount:.0}")))
                }
                ItemKindTextVariant::AttackPowerPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "{}동안 반경 {} 내 타워의 {}을 {} 증가시킵니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    attack_damage_stat("공격력"),
                    additive_value(format!("{amount:.0}"))
                ),
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "{}동안 반경 {} 내 타워의 {}을 {} 증가시킵니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    attack_damage_stat("공격력"),
                    multiplier_value(format!("{amount:.1}"))
                ),
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "{}동안 반경 {} 내 타워의 {}를 {} 증가시킵니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    attack_speed_stat("공격 속도"),
                    additive_value(format!("{amount:.0}"))
                ),
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "{}동안 반경 {} 내 타워의 {}를 {} 증가시킵니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    attack_speed_stat("공격 속도"),
                    multiplier_value(format!("{amount:.1}"))
                ),
                ItemKindTextVariant::AttackRangePlus {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "{}동안 반경 {} 내 타워의 {}를 {} 증가시킵니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    attack_range_stat("공격 범위"),
                    additive_value(format!("{amount:.0}"))
                ),
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "{}동안 반경 {} 내 적의 {}를 {} 감소시킵니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    movement_speed_debuff_text("이동 속도"),
                    movement_speed_debuff_value(format!("-{:.0}%", amount * 100.0))
                ),
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage,
                    radius,
                } => format!(
                    "반경 {} 내 적들에게 {} {} {} 피해를 입힙니다",
                    range(format!("{radius}m")),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank)
                ),
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage,
                    radius,
                    duration,
                    ..
                } => format!(
                    "{}동안 반경 {} 내 적들에게 {} {} {} 지속 피해를 입힙니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    range(format!("{radius}m")),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank)
                ),
                ItemKindTextVariant::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% 확률로 {} 골드를 획득합니다",
                    probability * 100.0,
                    gold_icon(format!("{amount:.0}"))
                ),
                ItemKindTextVariant::LinearDamage {
                    rank,
                    suit,
                    damage,
                    thickness,
                    ..
                } => format!(
                    "두께 {} 직선 범위 내 적들에게 {} {} {} 피해를 입힙니다",
                    beam_thickness(format!("{thickness}m")),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank)
                ),
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage,
                    thickness,
                    duration,
                    ..
                } => format!(
                    "{}동안 두께 {} 직선 범위 내 적들에게 {} {} {} 지속 피해를 입힙니다",
                    time_duration(format!("{:.1}초", duration.as_secs_f32())),
                    beam_thickness(format!("{thickness}m")),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank)
                ),
                ItemKindTextVariant::ExtraReroll => {
                    format!("{}을 획득합니다", special_item_text("추가 리롤"))
                }
                ItemKindTextVariant::Shield { amount } => {
                    format!(
                        "{} 피해를 흡수하는 방어막을 획득합니다",
                        shield_value(format!("{amount:.0}"))
                    )
                }
                ItemKindTextVariant::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "받는 피해를 {} 감소시킵니다 ({})",
                    reduction_percentage(format!("{:.0}", (1.0 - damage_multiply) * 100.0)),
                    time_duration(format!("{:.1}초간", duration.as_secs_f32()))
                ),
            },
        }
    }
    pub(super) fn to_english(&self) -> String {
        match self {
            ItemKindText::Name(variant) => match variant {
                ItemKindTextVariant::Heal { .. } => "Heal".to_string(),
                ItemKindTextVariant::AttackPowerPlusBuff { .. } => {
                    "Attack Power Plus Buff".to_string()
                }
                ItemKindTextVariant::AttackPowerMultiplyBuff { .. } => {
                    "Attack Power Multiply Buff".to_string()
                }
                ItemKindTextVariant::AttackSpeedPlusBuff { .. } => {
                    "Attack Speed Plus Buff".to_string()
                }
                ItemKindTextVariant::AttackSpeedMultiplyBuff { .. } => {
                    "Attack Speed Multiply Buff".to_string()
                }
                ItemKindTextVariant::AttackRangePlus { .. } => "Attack Range Plus".to_string(),
                ItemKindTextVariant::MovementSpeedDebuff { .. } => {
                    "Movement Speed Debuff".to_string()
                }
                ItemKindTextVariant::RoundDamage { .. } => "Area Damage".to_string(),
                ItemKindTextVariant::RoundDamageOverTime { .. } => {
                    "Damage Over Time in Area".to_string()
                }
                ItemKindTextVariant::Lottery { .. } => "Lottery".to_string(),
                ItemKindTextVariant::LinearDamage { .. } => "Beam Damage".to_string(),
                ItemKindTextVariant::LinearDamageOverTime { .. } => {
                    "Damage Over Time in Beam".to_string()
                }
                ItemKindTextVariant::ExtraReroll => "Extra Reroll".to_string(),
                ItemKindTextVariant::Shield { .. } => "Shield".to_string(),
                ItemKindTextVariant::DamageReduction { .. } => "Damage Reduction".to_string(),
            },
            ItemKindText::Description(variant) => match variant {
                ItemKindTextVariant::Heal { amount } => {
                    format!("Restores {} health", heal_icon(format!("{amount:.0}")))
                }
                ItemKindTextVariant::AttackPowerPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases {} by {} for towers within {} radius for {}",
                    attack_damage_stat("attack power"),
                    additive_value(format!("{amount:.0}")),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases {} by {} for towers within {} radius for {}",
                    attack_damage_stat("attack power"),
                    multiplier_value(format!("{amount:.1}")),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases {} by {} for towers within {} radius for {}",
                    attack_speed_stat("attack speed"),
                    additive_value(format!("{amount:.0}")),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases {} by {} for towers within {} radius for {}",
                    attack_speed_stat("attack speed"),
                    multiplier_value(format!("{amount:.1}")),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::AttackRangePlus {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases {} by {} for towers within {} radius for {}",
                    attack_range_stat("attack range"),
                    additive_value(format!("{amount:.0}")),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Decreases enemy {} by {} within {} radius for {}",
                    movement_speed_debuff_text("movement speed"),
                    movement_speed_debuff_value(format!("-{:.0}%", amount * 100.0)),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage,
                    radius,
                    ..
                } => format!(
                    "Deals {} {} {} damage to enemies within {} radius",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank),
                    range(format!("{radius}m"))
                ),
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage,
                    radius,
                    duration,
                    ..
                } => format!(
                    "Deals {} {} {} damage over time to enemies within {} radius for {}",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank),
                    range(format!("{radius}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% chance to gain {} gold",
                    probability * 100.0,
                    gold_icon(format!("{amount:.0}"))
                ),
                ItemKindTextVariant::LinearDamage {
                    rank,
                    suit,
                    damage,
                    thickness,
                    ..
                } => format!(
                    "Deals {} {} {} damage to enemies in {} thick beam",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank),
                    beam_thickness(format!("{thickness}m"))
                ),
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage,
                    thickness,
                    duration,
                    ..
                } => format!(
                    "Deals {} {} {} damage over time to enemies in {} thick beam for {}",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    card_rank(rank),
                    beam_thickness(format!("{thickness}m")),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                ItemKindTextVariant::ExtraReroll => {
                    format!("Gain an {}", special_item_text("extra reroll"))
                }
                ItemKindTextVariant::Shield { amount } => {
                    format!(
                        "Gain a shield that absorbs {} damage",
                        shield_value(format!("{amount:.0}"))
                    )
                }
                ItemKindTextVariant::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "Reduces damage taken by {} for {}",
                    reduction_percentage(format!("{:.0}", (1.0 - damage_multiply) * 100.0)),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
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
