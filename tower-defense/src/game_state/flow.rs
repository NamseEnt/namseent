use super::GameState;
use crate::{card::Card, tower_placing_hand::PlacingTowerSlot, upgrade::Upgrade};

#[derive(Clone)]
pub enum GameFlow {
    SelectingTower {
        cards: [Card; 5],
    },
    PlacingTower {
        placing_tower_slots: [PlacingTowerSlot; 5],
    },
    SelectingUpgrade {
        upgrades: [Upgrade; 3],
    },
}
impl GameFlow {
    pub fn new_selecting_tower() -> Self {
        Self::SelectingTower {
            cards: [
                Card::new_random(),
                Card::new_random(),
                Card::new_random(),
                Card::new_random(),
                Card::new_random(),
            ],
        }
    }
}

impl GameState {
    pub fn goto_selecting_tower(&mut self) {
        self.flow = GameFlow::new_selecting_tower();
    }
}
