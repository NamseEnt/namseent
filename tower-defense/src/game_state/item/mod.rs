pub mod effect_processor;
pub mod generation;
pub mod icon;
mod usage;

use crate::{
    card::{Rank, Suit},
    rarity::Rarity,
};
use namui::*;
pub use usage::*;

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub rarity: Rarity,
    pub value: OneZero,
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
            ItemKind::Heal { amount } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::Heal {
                    amount: *amount,
                }))
            }
            ItemKind::AttackPowerPlusBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::AttackPowerPlusBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackPowerMultiplyBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackSpeedPlusBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackSpeedMultiplyBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackRangePlus {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Name(ItemKindTextVariant::AttackRangePlus {
                amount: *amount,
                duration: *duration,
                radius: *radius,
            })),
            ItemKind::MovementSpeedDebuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::RoundDamage {
                rank,
                suit,
                damage,
                radius,
            } => text_manager.item(ItemKindText::Name(ItemKindTextVariant::RoundDamage {
                rank,
                suit,
                damage: *damage,
                radius: *radius,
            })),
            ItemKind::RoundDamageOverTime {
                rank,
                suit,
                damage,
                radius,
                duration,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage: *damage,
                    radius: *radius,
                    duration: *duration,
                },
            )),
            ItemKind::Lottery {
                amount,
                probability,
            } => text_manager.item(ItemKindText::Name(ItemKindTextVariant::Lottery {
                amount: *amount,
                probability: *probability,
            })),
            ItemKind::LinearDamage {
                rank,
                suit,
                damage,
                thickness,
            } => text_manager.item(ItemKindText::Name(ItemKindTextVariant::LinearDamage {
                rank,
                suit,
                damage: *damage,
                thickness: *thickness,
            })),
            ItemKind::LinearDamageOverTime {
                rank,
                suit,
                damage,
                thickness,
                duration,
            } => text_manager.item(ItemKindText::Name(
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage: *damage,
                    thickness: *thickness,
                    duration: *duration,
                },
            )),
            ItemKind::ExtraReroll => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::ExtraReroll))
            }
            ItemKind::Shield { amount } => {
                text_manager.item(ItemKindText::Name(ItemKindTextVariant::Shield {
                    amount: *amount,
                }))
            }
            ItemKind::DamageReduction {
                damage_multiply,
                duration,
            } => text_manager.item(ItemKindText::Name(ItemKindTextVariant::DamageReduction {
                damage_multiply: *damage_multiply,
                duration: *duration,
            })),
        }
    }
    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        use crate::l10n::item::{ItemKindText, ItemKindTextVariant};
        match self {
            ItemKind::Heal { amount } => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::Heal {
                    amount: *amount,
                }))
            }
            ItemKind::AttackPowerPlusBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackPowerPlusBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackPowerMultiplyBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackPowerMultiplyBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackSpeedPlusBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackSpeedPlusBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackSpeedMultiplyBuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackSpeedMultiplyBuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::AttackRangePlus {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::AttackRangePlus {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::MovementSpeedDebuff {
                amount,
                duration,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::MovementSpeedDebuff {
                    amount: *amount,
                    duration: *duration,
                    radius: *radius,
                },
            )),
            ItemKind::RoundDamage {
                rank,
                suit,
                damage,
                radius,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::RoundDamage {
                    rank,
                    suit,
                    damage: *damage,
                    radius: *radius,
                },
            )),
            ItemKind::RoundDamageOverTime {
                rank,
                suit,
                damage,
                radius,
                duration,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::RoundDamageOverTime {
                    rank,
                    suit,
                    damage: *damage,
                    radius: *radius,
                    duration: *duration,
                },
            )),
            ItemKind::Lottery {
                amount,
                probability,
            } => text_manager.item(ItemKindText::Description(ItemKindTextVariant::Lottery {
                amount: *amount,
                probability: *probability,
            })),
            ItemKind::LinearDamage {
                rank,
                suit,
                damage,
                thickness,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::LinearDamage {
                    rank,
                    suit,
                    damage: *damage,
                    thickness: *thickness,
                },
            )),
            ItemKind::LinearDamageOverTime {
                rank,
                suit,
                damage,
                thickness,
                duration,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::LinearDamageOverTime {
                    rank,
                    suit,
                    damage: *damage,
                    thickness: *thickness,
                    duration: *duration,
                },
            )),
            ItemKind::ExtraReroll => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::ExtraReroll))
            }
            ItemKind::Shield { amount } => {
                text_manager.item(ItemKindText::Description(ItemKindTextVariant::Shield {
                    amount: *amount,
                }))
            }
            ItemKind::DamageReduction {
                damage_multiply,
                duration,
            } => text_manager.item(ItemKindText::Description(
                ItemKindTextVariant::DamageReduction {
                    damage_multiply: *damage_multiply,
                    duration: *duration,
                },
            )),
        }
    }
}
