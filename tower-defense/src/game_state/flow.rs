use super::{
    item::generate_items, monster_spawn::start_spawn, quest::generate_quests, tower::TowerTemplate,
    GameState,
};
use crate::{
    card::Card,
    quest_board::QuestBoardSlot,
    rarity::Rarity,
    shop::ShopSlot,
    tower_placing_hand::PlacingTowerSlot,
    upgrade::{generate_upgrades_for_boss_reward, Upgrade},
};

#[derive(Clone)]
pub enum GameFlow {
    SelectingTower {
        cards: [Card; 5],
    },
    PlacingTower {
        placing_tower_slots: [PlacingTowerSlot; 5],
    },
    Defense,
    SelectingUpgrade {
        upgrades: Vec<Upgrade>,
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

        match self.in_even_stage() {
            true => {
                self.renew_shop();
            }
            false => {
                self.renew_quest_board();
            }
        }
    }
    fn renew_shop(&mut self) {
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
    fn renew_quest_board(&mut self) {
        let quests = generate_quests(self, self.max_quest_board_slot);
        for slot in self.quest_board_slots.iter_mut() {
            *slot = QuestBoardSlot::Locked;
        }
        for (slot, quest) in self.quest_board_slots.iter_mut().zip(quests.into_iter()) {
            *slot = QuestBoardSlot::Quest {
                quest,
                accepted: false,
            }
        }
    }

    pub fn goto_placing_tower(&mut self, tower_template: TowerTemplate) {
        self.flow = GameFlow::PlacingTower {
            placing_tower_slots: [
                PlacingTowerSlot::Tower { tower_template },
                PlacingTowerSlot::barricade(),
                PlacingTowerSlot::barricade(),
                PlacingTowerSlot::barricade(),
                PlacingTowerSlot::barricade(),
            ],
        };
    }

    pub fn goto_defense(&mut self) {
        self.flow = GameFlow::Defense;
        start_spawn(self);
    }

    pub fn goto_selecting_upgrade(&mut self) {
        let upgrades = generate_upgrades_for_boss_reward(self, 3);
        self.flow = GameFlow::SelectingUpgrade { upgrades };
    }
}
