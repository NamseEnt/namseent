use super::{Language, Locale, LocalizedText};
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
                ItemKindTextVariant::Heal { amount } => format!(
                    "|gold_color|icon<heal:16:16:16:1>{amount:.0}|/gold_color| 체력을 회복합니다",
                ),
                ItemKindTextVariant::AttackPowerPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 |blue|{:.0}|/blue|m 내 타워의 |attack_damage_color|icon<attack_damage:16:16:16:1>공격력|/attack_damage_color|을 |green|+{:.0}|/green| {:.1}초간 증가시킵니다",
                    radius,
                    amount,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 |blue|{:.0}|/blue|m 내 타워의 |attack_damage_color|icon<attack_damage:16:16:16:1>공격력|/attack_damage_color|을 |green|×{:.1}|/green| {:.1}초간 증가시킵니다",
                    radius,
                    amount,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 |blue|{:.0}|/blue|m 내 타워의 |attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도|/attack_speed_color|를 |green|+{:.0}|/green| {:.1}초간 증가시킵니다",
                    radius,
                    amount,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 |blue|{:.0}|/blue|m 내 타워의 |attack_speed_color|icon<attack_speed:16:16:16:1>공격 속도|/attack_speed_color|를 |green|×{:.1}|/green| {:.1}초간 증가시킵니다",
                    radius,
                    amount,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackRangePlus {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 |blue|{:.0}|/blue|m 내 타워의 |attack_range_color|icon<attack_range:16:16:16:1>공격 범위|/attack_range_color|를 |green|+{:.0}|/green| {:.1}초간 증가시킵니다",
                    radius,
                    amount,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "반경 |blue|{:.0}|/blue|m 내 적의 |red|이동 속도|/red|를 |red|-{:.0}%|/red| {:.1}초간 감소시킵니다",
                    radius,
                    amount * 100.0,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage,
                    radius,
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "반경 |blue|{radius:.0}|/blue|m 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>{damage:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| 피해를 입힙니다"
                    )
                }
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage,
                    radius,
                    duration,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "반경 |blue|{:.0}|/blue|m 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>{:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| 지속 피해를 {:.1}초간 입힙니다",
                        radius,
                        damage,
                        duration.as_secs_f32()
                    )
                }
                ItemKindTextVariant::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% 확률로 |gold_color|icon<gold:16:16:16:1>{:.0}|/gold_color| 골드를 획득합니다",
                    probability * 100.0,
                    amount
                ),
                ItemKindTextVariant::LinearDamage {
                    rank,
                    suit,
                    damage,
                    thickness,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "두께 |blue|{thickness:.0}|/blue|m 직선 범위 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>{damage:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| 피해를 입힙니다"
                    )
                }
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage,
                    thickness,
                    duration,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "두께 |blue|{:.0}|/blue|m 직선 범위 내 적들에게 |attack_damage_color|icon<attack_damage:16:16:16:1>{:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| 지속 피해를 {:.1}초간 입힙니다",
                        thickness,
                        damage,
                        duration.as_secs_f32()
                    )
                }
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
                ItemKindTextVariant::Heal { amount } => format!(
                    "Restores |gold_color|icon<heal:16:16:16:1>{amount:.0}|/gold_color| health",
                ),
                ItemKindTextVariant::AttackPowerPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_damage_color|icon<attack_damage:16:16:16:1>attack power|/attack_damage_color| by |green|+{:.0}|/green| for towers within |blue|{:.0}|/blue|m radius for {:.1}s",
                    amount,
                    radius,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_damage_color|icon<attack_damage:16:16:16:1>attack power|/attack_damage_color| by |green|×{:.1}|/green| for towers within |blue|{:.0}|/blue|m radius for {:.1}s",
                    amount,
                    radius,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_speed_color|icon<attack_speed:16:16:16:1>attack speed|/attack_speed_color| by |green|+{:.0}|/green| for towers within |blue|{:.0}|/blue|m radius for {:.1}s",
                    amount,
                    radius,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_speed_color|icon<attack_speed:16:16:16:1>attack speed|/attack_speed_color| by |green|×{:.1}|/green| for towers within |blue|{:.0}|/blue|m radius for {:.1}s",
                    amount,
                    radius,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::AttackRangePlus {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Increases |attack_range_color|icon<attack_range:16:16:16:1>attack range|/attack_range_color| by |green|+{:.0}|/green| for towers within |blue|{:.0}|/blue|m radius for {:.1}s",
                    amount,
                    radius,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount,
                    duration,
                    radius,
                } => format!(
                    "Decreases enemy |red|movement speed|/red| by |red|-{:.0}%|/red| within |blue|{:.0}|/blue|m radius for {:.1}s",
                    amount * 100.0,
                    radius,
                    duration.as_secs_f32()
                ),
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage,
                    radius,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>{damage:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| damage to enemies within |blue|{radius:.0}|/blue|m radius"
                    )
                }
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage,
                    radius,
                    duration,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>{:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| damage over time to enemies within |blue|{:.0}|/blue|m radius for {:.1}s",
                        damage,
                        radius,
                        duration.as_secs_f32()
                    )
                }
                ItemKindTextVariant::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% chance to gain |gold_color|icon<gold:16:16:16:1>{:.0}|/gold_color| gold",
                    probability * 100.0,
                    amount
                ),
                ItemKindTextVariant::LinearDamage {
                    rank,
                    suit,
                    damage,
                    thickness,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>{damage:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| damage to enemies in |blue|{thickness:.0}|/blue|m thick beam"
                    )
                }
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage,
                    thickness,
                    duration,
                    ..
                } => {
                    let suit_icon = match suit {
                        Suit::Spades => "icon<suit_spades:12:12:12:1>",
                        Suit::Hearts => "icon<suit_hearts:12:12:12:1>",
                        Suit::Diamonds => "icon<suit_diamonds:12:12:12:1>",
                        Suit::Clubs => "icon<suit_clubs:12:12:12:1>",
                    };
                    format!(
                        "Deals |attack_damage_color|icon<attack_damage:16:16:16:1>{:.0}|/attack_damage_color| {suit_icon}|purple|{rank}|/purple| damage over time to enemies in |blue|{:.0}|/blue|m thick beam for {:.1}s",
                        damage,
                        thickness,
                        duration.as_secs_f32()
                    )
                }
                ItemKindTextVariant::ExtraReroll => "Gain an |blue|extra reroll|/blue|".to_string(),
                ItemKindTextVariant::Shield { amount } => {
                    format!("Gain a shield that absorbs |blue|{amount:.0}|/blue| damage",)
                }
                ItemKindTextVariant::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "Reduces damage taken by |green|{:.0}%|/green| for {:.1}s",
                    (1.0 - damage_multiply) * 100.0,
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
