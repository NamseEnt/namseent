use super::{item::generate_items, GameState};
use crate::{
    card::Card, rarity::Rarity, shop::ShopSlot, tower_placing_hand::PlacingTowerSlot,
    upgrade::Upgrade,
};

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

        let items = generate_items(self, self.max_shop_slot);
        for slot in self.shop_slots.iter_mut() {
            *slot = ShopSlot::Locked;
        }
        for (slot, item) in self.shop_slots.iter_mut().zip(items.into_iter()) {
            let cost = match item.rarity {
                Rarity::Common => 25,
                Rarity::Rare => 50,
                Rarity::Epic => 100,
                Rarity::Legendary => 250,
            };
            *slot = ShopSlot::Item {
                item,
                cost,
                purchased: false,
            }
        }
    }
}
