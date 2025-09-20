pub mod effect_processor;
pub mod generation;
mod thumbnail;
mod usage;

use crate::rarity::Rarity;
use namui::*;
pub use usage::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub rarity: Rarity,
    pub value: OneZero,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemKind {
    Heal {
        amount: f32,
    },
    Lottery {
        amount: f32,
        probability: f32,
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
            ItemKind::Lottery {
                amount,
                probability,
            } => text_manager.item(ItemKindText::Name(ItemKindTextVariant::Lottery {
                amount: *amount,
                probability: *probability,
            })),
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
            ItemKind::Lottery {
                amount,
                probability,
            } => text_manager.item(ItemKindText::Description(ItemKindTextVariant::Lottery {
                amount: *amount,
                probability: *probability,
            })),
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
