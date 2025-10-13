use crate::*;
use namui::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
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
