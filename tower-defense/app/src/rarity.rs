//! Headed palette/localization adapter around `td_core::Rarity`.

use crate::*;
use namui::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
#[repr(u8)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl From<Rarity> for td_core::Rarity {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::Common => Self::Common,
            Rarity::Rare => Self::Rare,
            Rarity::Epic => Self::Epic,
            Rarity::Legendary => Self::Legendary,
        }
    }
}

impl From<td_core::Rarity> for Rarity {
    fn from(rarity: td_core::Rarity) -> Self {
        match rarity {
            td_core::Rarity::Common => Self::Common,
            td_core::Rarity::Rare => Self::Rare,
            td_core::Rarity::Epic => Self::Epic,
            td_core::Rarity::Legendary => Self::Legendary,
        }
    }
}
impl Rarity {
    pub const fn color(&self) -> Color {
        match self {
            Rarity::Common => palette::COMMON,
            Rarity::Rare => palette::RARE,
            Rarity::Epic => palette::EPIC,
            Rarity::Legendary => palette::LEGENDARY,
        }
    }
}
