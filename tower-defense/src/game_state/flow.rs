use super::{
    GameState,
    monster_spawn::start_spawn,
    quest::generate_quests,
    tower::TowerTemplate,
    upgrade::{Upgrade, generate_upgrades_for_boss_reward},
};
use crate::{game_state::shop::initialize_shop, quest_board::QuestBoardSlot};

#[derive(Clone, Debug)]
pub enum GameFlow {
    Initializing,
    SelectingTower,
    PlacingTower,
    Defense,
    SelectingUpgrade { upgrades: Vec<Upgrade> },
    Result,
}

impl GameState {
    pub fn goto_selecting_tower(&mut self) {
        self.flow = GameFlow::SelectingTower;
        self.left_reroll_chance = self.max_reroll_chance();
        self.shield = 0.0;
        self.item_used = false;
        self.rerolled_count = 0;
        self.hand.clear();
        self.hand.add_random_cards(5);

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
        initialize_shop(self);
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
        self.flow = GameFlow::PlacingTower;
        self.hand.clear();
        self.hand
            .add_tower_template_with_barricades(tower_template, 4);
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
