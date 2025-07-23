// --- Rich text 헬퍼 함수 (item 전용) ---
fn suit_icon(suit: Suit) -> String {
    Icon::new(IconKind::Suit { suit }).as_tag()
}

fn purple<T: std::fmt::Display>(value: T) -> String {
    format!("|purple|{value}|/purple|")
}

fn blue<T: std::fmt::Display>(value: T) -> String {
    format!("|blue|{value}|/blue|")
}

fn attack_damage_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackDamage);
    format!(
        "|attack_damage_color|{}{value}|/attack_damage_color|",
        icon.as_tag()
    )
}

fn heal_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::Health);
    format!("|gold_color|{}{value}|/gold_color|", icon.as_tag())
}

fn gold_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::Gold);
    format!("|gold_color|{}{value}|/gold_color|", icon.as_tag())
}

fn green<T: std::fmt::Display>(value: T) -> String {
    format!("|green|{value}|/green|")
}

fn red<T: std::fmt::Display>(value: T) -> String {
    format!("|red|{value}|/red|")
}
use super::{Language, Locale, LocalizedText};
use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconKind},
};
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
                    "반경 {}m 내 타워의 |attack_damage_color|{}공격력|/attack_damage_color|을 {} {:.1}초간 증가시킵니다",
                    blue(*radius),
                    Icon::new(IconKind::AttackDamage).as_tag(),
                    green(format!("+{amount:.0}")),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 {}m 내 타워의 |attack_damage_color|{}공격력|/attack_damage_color|을 {} {:.1}초간 증가시킵니다",
                    blue(*radius),
                    Icon::new(IconKind::AttackDamage).as_tag(),
                    green(format!("×{amount:.1}")),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 {}m 내 타워의 |attack_speed_color|{}공격 속도|/attack_speed_color|를 {} {:.1}초간 증가시킵니다",
                    blue(*radius),
                    Icon::new(IconKind::AttackSpeed).as_tag(),
                    green(format!("+{amount:.0}")),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 {}m 내 타워의 |attack_speed_color|{}공격 속도|/attack_speed_color|를 {} {:.1}초간 증가시킵니다",
                    blue(*radius),
                    Icon::new(IconKind::AttackSpeed).as_tag(),
                    green(format!("×{amount:.1}")),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackRangePlus {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 {}m 내 타워의 |attack_range_color|{}공격 범위|/attack_range_color|를 {} {:.1}초간 증가시킵니다",
                    blue(*radius),
                    Icon::new(IconKind::AttackRange).as_tag(),
                    green(format!("+{amount:.0}")),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 {}m 내 적의 |red|이동 속도|/red|를 |red|-{:.0}%|/red| {:.1}초간 감소시킵니다",
                    blue(*radius),
                    amount * 100.0,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage,
                    radius,
                } => format!(
                    "반경 {}m 내 적들에게 {} {} {} 피해를 입힙니다",
                    blue(*radius),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank)
                ),
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage,
                    radius,
                    duration,
                    ..
                } => format!(
                    "반경 {}m 내 적들에게 {} {} {} 지속 피해를 {:.1}초간 입힙니다",
                    blue(*radius),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank),
                    duration.as_secs_f32()
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
                    "두께 {}m 직선 범위 내 적들에게 {} {} {} 피해를 입힙니다",
                    blue(*thickness),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank)
                ),
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage,
                    thickness,
                    duration,
                    ..
                } => format!(
                    "두께 {}m 직선 범위 내 적들에게 {} {} {} 지속 피해를 {:.1}초간 입힙니다",
                    blue(*thickness),
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::ExtraReroll => {
                    "|blue|추가 리롤|/blue|을 획득합니다".to_string()
                }
                ItemKindTextVariant::Shield { amount } => {
                    format!("|blue|{amount:.0}|/blue| 피해를 흡수하는 방어막을 획득합니다",)
                }
                ItemKindTextVariant::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "받는 피해를 |green|{:.0}%|/green| 감소시킵니다 ({:.1}초간)",
                    (1.0 - damage_multiply) * 100.0,
                    duration.as_secs_f32()
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
                    "Increases |attack_damage_color|{}attack power|/attack_damage_color| by {} for towers within {}m radius for {:.1}s",
                    Icon::new(IconKind::AttackDamage).as_tag(),
                    green(format!("+{amount:.0}")),
                    blue(*radius),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_damage_color|{}attack power|/attack_damage_color| by {} for towers within {}m radius for {:.1}s",
                    Icon::new(IconKind::AttackDamage).as_tag(),
                    green(format!("×{amount:.1}")),
                    blue(*radius),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_speed_color|{}attack speed|/attack_speed_color| by {} for towers within {}m radius for {:.1}s",
                    Icon::new(IconKind::AttackSpeed).as_tag(),
                    green(format!("+{amount:.0}")),
                    blue(*radius),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_speed_color|{}attack speed|/attack_speed_color| by {} for towers within {}m radius for {:.1}s",
                    Icon::new(IconKind::AttackSpeed).as_tag(),
                    green(format!("×{amount:.1}")),
                    blue(*radius),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackRangePlus {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_range_color|{}attack range|/attack_range_color| by {} for towers within {}m radius for {:.1}s",
                    Icon::new(IconKind::AttackRange).as_tag(),
                    green(format!("+{amount:.0}")),
                    blue(*radius),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Decreases enemy {}movement speed{} by {} within {}m radius for {:.1}s",
                    "|red|",
                    "|/red|",
                    red(format!("-{:.0}%", amount * 100.0)),
                    blue(*radius),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage,
                    radius,
                    ..
                } => format!(
                    "Deals {} {} {} damage to enemies within {}m radius",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank),
                    blue(*radius)
                ),
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage,
                    radius,
                    duration,
                    ..
                } => format!(
                    "Deals {} {} {} damage over time to enemies within {}m radius for {:.1}s",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank),
                    blue(*radius),
                    duration.as_secs_f32()
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
                    "Deals {} {} {} damage to enemies in {}m thick beam",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank),
                    blue(*thickness)
                ),
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage,
                    thickness,
                    duration,
                    ..
                } => format!(
                    "Deals {} {} {} damage over time to enemies in {}m thick beam for {:.1}s",
                    attack_damage_icon(*damage),
                    suit_icon(**suit),
                    purple(rank),
                    blue(*thickness),
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::ExtraReroll => "Gain an |blue|extra reroll|/blue|".to_string(),
                ItemKindTextVariant::Shield { amount } => {
                    format!(
                        "Gain a shield that absorbs {} damage",
                        blue(format!("{amount:.0}"))
                    )
                }
                ItemKindTextVariant::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "Reduces damage taken by {} for {:.1}s",
                    green(format!("{:.0}%", (1.0 - damage_multiply) * 100.0)),
                    duration.as_secs_f32()
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
