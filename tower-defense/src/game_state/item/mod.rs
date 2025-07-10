pub mod effect_processor;
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
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        use crate::l10n::item::{ItemKindText, ItemKindTextVariant};
        match self {
            ItemKind::Heal { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::Heal))
            }
            ItemKind::AttackPowerPlusBuff { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::AttackPowerPlusBuff))
            }
            ItemKind::AttackPowerMultiplyBuff { .. } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::AttackPowerMultiplyBuff,
            )),
            ItemKind::AttackSpeedPlusBuff { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::AttackSpeedPlusBuff))
            }
            ItemKind::AttackSpeedMultiplyBuff { .. } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::AttackSpeedMultiplyBuff,
            )),
            ItemKind::AttackRangePlus { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::AttackRangePlus))
            }
            ItemKind::MovementSpeedDebuff { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::MovementSpeedDebuff))
            }
            ItemKind::RoundDamage { rank, suit, .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                }))
            }
            ItemKind::RoundDamageOverTime { rank, suit, .. } => text_manager.item(
                ItemKindText::Name(ItemKindTextVariant::RoundDamageOverTime { rank, suit }),
            ),
            ItemKind::Lottery { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::Lottery))
            }
            ItemKind::LinearDamage { rank, suit, .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::LinearDamage {
                    rank,
                    suit,
                }))
            }
            ItemKind::LinearDamageOverTime { rank, suit, .. } => text_manager.item(
                ItemKindText::Name(ItemKindTextVariant::LinearDamageOverTime { rank, suit }),
            ),
            ItemKind::ExtraReroll => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::ExtraReroll))
            }
            ItemKind::Shield { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::Shield))
            }
            ItemKind::DamageReduction { .. } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::DamageReduction))
            }
        }
    }
    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        use crate::l10n::item::{ItemKindText, ItemKindTextVariant};
        match self {
            ItemKind::Heal { .. } => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::Heal))
            }
            ItemKind::AttackPowerPlusBuff { .. } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackPowerPlusBuff,
            )),
            ItemKind::AttackPowerMultiplyBuff { .. } => text_manager.item(
                ItemKindText::Description(ItemKindTextVariant::AttackPowerMultiplyBuff),
            ),
            ItemKind::AttackSpeedPlusBuff { .. } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackSpeedPlusBuff,
            )),
            ItemKind::AttackSpeedMultiplyBuff { .. } => text_manager.item(
                ItemKindText::Description(ItemKindTextVariant::AttackSpeedMultiplyBuff),
            ),
            ItemKind::AttackRangePlus { .. } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackRangePlus,
            )),
            ItemKind::MovementSpeedDebuff { .. } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::MovementSpeedDebuff,
            )),
            ItemKind::RoundDamage { rank, suit, .. } => text_manager.item(
                ItemKindText::Description(ItemKindTextVariant::RoundDamage { rank, suit }),
            ),
            ItemKind::RoundDamageOverTime { rank, suit, .. } => text_manager.item(
                ItemKindText::Description(ItemKindTextVariant::RoundDamageOverTime { rank, suit }),
            ),
            ItemKind::Lottery { .. } => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::Lottery))
            }
            ItemKind::LinearDamage { rank, suit, .. } => text_manager.item(
                ItemKindText::Description(ItemKindTextVariant::LinearDamage { rank, suit }),
            ),
            ItemKind::LinearDamageOverTime { rank, suit, .. } => text_manager.item(
                ItemKindText::Description(ItemKindTextVariant::LinearDamageOverTime { rank, suit }),
            ),
            ItemKind::ExtraReroll => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::ExtraReroll))
            }
            ItemKind::Shield { .. } => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::Shield))
            }
            ItemKind::DamageReduction { .. } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::DamageReduction,
            )),
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
