use crate::{tower_placing_hand::PlacingTowerSlot, upgrade::Upgrade};

#[derive(Clone)]
pub enum GameFlow {
    SelectingTower,
    PlacingTower {
        placing_tower_slots: [PlacingTowerSlot; 5],
    },
    SelectingUpgrade {
        upgrades: [Upgrade; 3],
    },
}
