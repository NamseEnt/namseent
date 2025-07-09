pub mod effect_processor;
mod generation;
mod usage;

use crate::{
    card::{Rank, Suit},
    l10n::item::ItemText,
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
    pub fn name(&self) -> String {
        match self {
            ItemKind::Heal { .. } => ItemText::HealName.to_korean(),
            ItemKind::AttackPowerPlusBuff { .. } => ItemText::AttackPowerPlusBuffName.to_korean(),
            ItemKind::AttackPowerMultiplyBuff { .. } => ItemText::AttackPowerMultiplyBuffName.to_korean(),
            ItemKind::AttackSpeedPlusBuff { .. } => ItemText::AttackSpeedPlusBuffName.to_korean(),
            ItemKind::AttackSpeedMultiplyBuff { .. } => ItemText::AttackSpeedMultiplyBuffName.to_korean(),
            ItemKind::AttackRangePlus { .. } => ItemText::AttackRangePlusName.to_korean(),
            ItemKind::MovementSpeedDebuff { .. } => ItemText::MovementSpeedDebuffName.to_korean(),
            ItemKind::RoundDamage { .. } => ItemText::RoundDamageName.to_korean(),
            ItemKind::RoundDamageOverTime { .. } => ItemText::RoundDamageOverTimeName.to_korean(),
            ItemKind::Lottery { .. } => ItemText::LotteryName.to_korean(),
            ItemKind::LinearDamage { .. } => ItemText::LinearDamageName.to_korean(),
            ItemKind::LinearDamageOverTime { .. } => ItemText::LinearDamageOverTimeName.to_korean(),
            ItemKind::ExtraReroll => ItemText::ExtraRerollName.to_korean(),
            ItemKind::Shield { .. } => ItemText::ShieldName.to_korean(),
            ItemKind::DamageReduction { .. } => ItemText::DamageReductionName.to_korean(),
        }
    }
    pub fn description(&self) -> String {
        match self {
            ItemKind::Heal { amount } => ItemText::HealDesc { amount: *amount }.to_korean(),
            ItemKind::AttackPowerPlusBuff { amount, duration, radius } => ItemText::AttackPowerPlusBuffDesc { amount: *amount, duration: *duration, radius: *radius }.to_korean(),
            ItemKind::AttackPowerMultiplyBuff { amount, duration, radius } => ItemText::AttackPowerMultiplyBuffDesc { amount: *amount, duration: *duration, radius: *radius }.to_korean(),
            ItemKind::AttackSpeedPlusBuff { amount, duration, radius } => ItemText::AttackSpeedPlusBuffDesc { amount: *amount, duration: *duration, radius: *radius }.to_korean(),
            ItemKind::AttackSpeedMultiplyBuff { amount, duration, radius } => ItemText::AttackSpeedMultiplyBuffDesc { amount: *amount, duration: *duration, radius: *radius }.to_korean(),
            ItemKind::AttackRangePlus { amount, duration, radius } => ItemText::AttackRangePlusDesc { amount: *amount, duration: *duration, radius: *radius }.to_korean(),
            ItemKind::MovementSpeedDebuff { amount, duration, radius } => ItemText::MovementSpeedDebuffDesc { amount: *amount, duration: *duration, radius: *radius }.to_korean(),
            ItemKind::RoundDamage { rank, suit, damage, radius } => ItemText::RoundDamageDesc { rank, suit, damage: *damage, radius: *radius }.to_korean(),
            ItemKind::RoundDamageOverTime { rank, suit, damage, radius, duration } => ItemText::RoundDamageOverTimeDesc { rank, suit, damage: *damage, radius: *radius, duration: *duration }.to_korean(),
            ItemKind::Lottery { amount, probability } => ItemText::LotteryDesc { amount: *amount, probability: *probability }.to_korean(),
            ItemKind::LinearDamage { rank, suit, damage, thickness } => ItemText::LinearDamageDesc { rank, suit, damage: *damage, thickness: *thickness }.to_korean(),
            ItemKind::LinearDamageOverTime { rank, suit, damage, thickness, duration } => ItemText::LinearDamageOverTimeDesc { rank, suit, damage: *damage, thickness: *thickness, duration: *duration }.to_korean(),
            ItemKind::ExtraReroll => ItemText::ExtraRerollDesc.to_korean(),
            ItemKind::Shield { amount } => ItemText::ShieldDesc { amount: *amount }.to_korean(),
            ItemKind::DamageReduction { damage_multiply, duration } => ItemText::DamageReductionDesc { damage_multiply: *damage_multiply, duration: *duration }.to_korean(),
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
