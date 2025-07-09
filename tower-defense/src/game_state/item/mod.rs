pub mod effect_processor;
mod generation;
mod usage;

use crate::{
    card::{Rank, Suit},
    l10n::item::ItemText,
    l10n::item::ItemTextLocale,
    l10n::upgrade::Locales,
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
    pub fn name(&self, locale: &Locales) -> String {
        match self {
            ItemKind::Heal { .. } => locale.item_text(ItemText::HealName),
            ItemKind::AttackPowerPlusBuff { .. } => locale.item_text(ItemText::AttackPowerPlusBuffName),
            ItemKind::AttackPowerMultiplyBuff { .. } => locale.item_text(ItemText::AttackPowerMultiplyBuffName),
            ItemKind::AttackSpeedPlusBuff { .. } => locale.item_text(ItemText::AttackSpeedPlusBuffName),
            ItemKind::AttackSpeedMultiplyBuff { .. } => locale.item_text(ItemText::AttackSpeedMultiplyBuffName),
            ItemKind::AttackRangePlus { .. } => locale.item_text(ItemText::AttackRangePlusName),
            ItemKind::MovementSpeedDebuff { .. } => locale.item_text(ItemText::MovementSpeedDebuffName),
            ItemKind::RoundDamage { .. } => locale.item_text(ItemText::RoundDamageName),
            ItemKind::RoundDamageOverTime { .. } => locale.item_text(ItemText::RoundDamageOverTimeName),
            ItemKind::Lottery { .. } => locale.item_text(ItemText::LotteryName),
            ItemKind::LinearDamage { .. } => locale.item_text(ItemText::LinearDamageName),
            ItemKind::LinearDamageOverTime { .. } => locale.item_text(ItemText::LinearDamageOverTimeName),
            ItemKind::ExtraReroll => locale.item_text(ItemText::ExtraRerollName),
            ItemKind::Shield { .. } => locale.item_text(ItemText::ShieldName),
            ItemKind::DamageReduction { .. } => locale.item_text(ItemText::DamageReductionName),
        }
    }
    pub fn description(&self, locale: &Locales) -> String {
        match self {
            ItemKind::Heal { amount } => locale.item_text(ItemText::HealDesc { amount: *amount }),
            ItemKind::AttackPowerPlusBuff { amount, duration, radius } => locale.item_text(ItemText::AttackPowerPlusBuffDesc { amount: *amount, duration: *duration, radius: *radius }),
            ItemKind::AttackPowerMultiplyBuff { amount, duration, radius } => locale.item_text(ItemText::AttackPowerMultiplyBuffDesc { amount: *amount, duration: *duration, radius: *radius }),
            ItemKind::AttackSpeedPlusBuff { amount, duration, radius } => locale.item_text(ItemText::AttackSpeedPlusBuffDesc { amount: *amount, duration: *duration, radius: *radius }),
            ItemKind::AttackSpeedMultiplyBuff { amount, duration, radius } => locale.item_text(ItemText::AttackSpeedMultiplyBuffDesc { amount: *amount, duration: *duration, radius: *radius }),
            ItemKind::AttackRangePlus { amount, duration, radius } => locale.item_text(ItemText::AttackRangePlusDesc { amount: *amount, duration: *duration, radius: *radius }),
            ItemKind::MovementSpeedDebuff { amount, duration, radius } => locale.item_text(ItemText::MovementSpeedDebuffDesc { amount: *amount, duration: *duration, radius: *radius }),
            ItemKind::RoundDamage { rank, suit, damage, radius } => locale.item_text(ItemText::RoundDamageDesc { rank, suit, damage: *damage, radius: *radius }),
            ItemKind::RoundDamageOverTime { rank, suit, damage, radius, duration } => locale.item_text(ItemText::RoundDamageOverTimeDesc { rank, suit, damage: *damage, radius: *radius, duration: *duration }),
            ItemKind::Lottery { amount, probability } => locale.item_text(ItemText::LotteryDesc { amount: *amount, probability: *probability }),
            ItemKind::LinearDamage { rank, suit, damage, thickness } => locale.item_text(ItemText::LinearDamageDesc { rank, suit, damage: *damage, thickness: *thickness }),
            ItemKind::LinearDamageOverTime { rank, suit, damage, thickness, duration } => locale.item_text(ItemText::LinearDamageOverTimeDesc { rank, suit, damage: *damage, thickness: *thickness, duration: *duration }),
            ItemKind::ExtraReroll => locale.item_text(ItemText::ExtraRerollDesc),
            ItemKind::Shield { amount } => locale.item_text(ItemText::ShieldDesc { amount: *amount }),
            ItemKind::DamageReduction { damage_multiply, duration } => locale.item_text(ItemText::DamageReductionDesc { damage_multiply: *damage_multiply, duration: *duration }),
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
