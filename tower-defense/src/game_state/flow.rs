use super::{
    GameState,
    item::{generate_items, item_cost},
    monster_spawn::start_spawn,
    quest::generate_quests,
    tower::TowerTemplate,
    upgrade::{Upgrade, generate_upgrades_for_boss_reward},
};
use crate::{
    card::Card, quest_board::QuestBoardSlot, shop::ShopSlot, tower_placing_hand::PlacingTowerSlot,
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
    Result,
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
        self.left_reroll_chance = self.max_reroll_chance();
        self.shield = 0.0;
        self.item_used = false;

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
        self.left_shop_refresh_chance = self.max_shop_refresh_chance();
        let items = generate_items(self, self.max_shop_slot());
        for slot in self.shop_slots.iter_mut() {
            *slot = ShopSlot::Locked;
        }
        for (slot, item) in self.shop_slots.iter_mut().zip(items.into_iter()) {
            let cost = item_cost(&item.rarity, self.upgrade_state.shop_item_price_minus);
            *slot = ShopSlot::Item {
                item,
                cost,
                purchased: false,
            }
        }
    }
    fn renew_quest_board(&mut self) {
        self.left_quest_board_refresh_chance = self.max_quest_board_refresh_chance();
        let quests = generate_quests(self, self.max_quest_board_slot());
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

    pub fn goto_result(&mut self) {
        self.flow = GameFlow::Result;
    }
}
