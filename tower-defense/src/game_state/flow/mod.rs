use super::{
    GameState,
    monster_spawn::start_spawn,
    tower::TowerTemplate,
    upgrade::{Upgrade, generate_upgrades_for_boss_reward},
};
use crate::{card::Card, game_state::hand::Hand, shop::Shop};

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum GameFlow {
    Initializing,
    Contract,
    SelectingTower(SelectingTowerFlow),
    PlacingTower { hand: Hand<TowerTemplate> },
    Defense,
    SelectingUpgrade { upgrades: Vec<Upgrade> },
    Result,
}
impl GameFlow {
    pub(crate) fn update(&mut self) {
        match self {
            GameFlow::SelectingTower(selecting_tower) => {
                selecting_tower.update();
            }
            GameFlow::PlacingTower { hand } => {
                hand.update();
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectingTowerFlow {
    pub hand: Hand<Card>,
    pub shop: Shop,
}

impl SelectingTowerFlow {
    pub fn new(game_state: &GameState) -> Self {
        SelectingTowerFlow {
            hand: Hand::new((0..5).map(|_| Card::new_random())),
            shop: Shop::new(game_state),
        }
    }

    fn update(&mut self) {
        self.hand.update();
    }
}

impl GameState {
    pub fn goto_next_stage(&mut self) {
        self.flow = GameFlow::Contract;
        self.left_reroll_chance = self.max_reroll_chance();
        self.shield = 0.0;
        self.item_used = false;
        self.rerolled_count = 0;
    }

    pub fn goto_selecting_tower(&mut self) {
        self.flow = GameFlow::SelectingTower(SelectingTowerFlow::new(self));
    }

    pub fn goto_placing_tower(&mut self, tower_template: TowerTemplate) {
        let mut hand = Hand::new([
            tower_template,
            TowerTemplate::barricade(),
            TowerTemplate::barricade(),
            TowerTemplate::barricade(),
            TowerTemplate::barricade(),
        ]);

        // Auto-select the first card (tower or barricade)
        let first_slot_id = hand.get_slot_id_by_index(0).unwrap();
        hand.select_slot(first_slot_id);

        self.flow = GameFlow::PlacingTower { hand };
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
