pub mod contract;

use super::{GameState, monster_spawn::start_spawn, tower::TowerTemplate};
use crate::{
    card::Card,
    game_state::{flow::contract::ContractFlow, hand::Hand},
    shop::Shop,
    *,
};

#[cfg(feature = "debug-tools")]
fn save_stage_snapshot(game_state: &GameState) {
    crate::game_state::debug_tools::state_snapshot::save_snapshot_from_state(game_state);
}

#[cfg(not(feature = "debug-tools"))]
fn save_stage_snapshot(_game_state: &GameState) {}

#[derive(Clone, Debug, State)]
#[allow(clippy::large_enum_variant)]
pub enum GameFlow {
    Initializing,
    Contract(ContractFlow),
    SelectingTower(SelectingTowerFlow),
    PlacingTower { hand: Hand<TowerTemplate> },
    Defense(DefenseFlow),
    Result { clear_rate: f32 },
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

#[derive(Clone, Debug, State)]
pub struct SelectingTowerFlow {
    pub hand: Hand<Card>,
    pub shop: Shop,
}

impl SelectingTowerFlow {
    pub fn new(game_state: &GameState) -> Self {
        let max_slots = (5 + game_state
            .stage_modifiers
            .get_card_selection_hand_max_slots_bonus())
        .saturating_sub(
            game_state
                .stage_modifiers
                .get_card_selection_hand_max_slots_penalty(),
        )
        .max(1);
        SelectingTowerFlow {
            hand: Hand::new((0..max_slots).map(|_| Card::new_random())),
            shop: Shop::new(game_state),
        }
    }

    fn update(&mut self) {
        self.hand.update();
        self.shop.update();
    }
}

#[derive(Clone, Debug, State)]
pub struct StageProgress {
    pub start_total_hp: f32,
    pub processed_hp: f32,
}

#[derive(Clone, Debug, State)]
pub struct DefenseFlow {
    pub stage_progress: StageProgress,
}

impl DefenseFlow {
    pub fn new(game_state: &GameState) -> Self {
        let start_total_hp =
            GameState::calculate_stage_total_hp(game_state.stage, &game_state.stage_modifiers);
        Self {
            stage_progress: StageProgress {
                start_total_hp,
                processed_hp: 0.0,
            },
        }
    }
}

impl GameState {
    pub fn goto_next_stage(&mut self) {
        contract::ContractFlow::step_all_contracts(&mut self.contracts);
        let contract_events = contract::ContractFlow::drain_all_events(&mut self.contracts);
        self.contracts.retain(|c| !c.is_expired());
        self.flow = GameFlow::Contract(contract::ContractFlow::new(contract_events));

        self.stage_modifiers.reset_stage_state();
        self.left_reroll_chance = self.max_reroll_chance();
        self.left_shop_refresh_chance = self.max_shop_refresh_chance();
        self.shield = 0.0;
        self.item_used = false;
        self.rerolled_count = 0;
        save_stage_snapshot(self);
    }

    pub fn goto_selecting_tower(&mut self) {
        self.flow = GameFlow::SelectingTower(SelectingTowerFlow::new(self));
        self.just_cleared_boss_stage = false;
    }

    pub fn goto_placing_tower(&mut self, tower_template: TowerTemplate) {
        let mut hand_items = vec![tower_template];

        // Drain all queued extra towers (barricades, special towers, etc.)
        for (tower_kind, suit, rank) in self.stage_modifiers.drain_extra_tower_cards() {
            hand_items.push(TowerTemplate::new(tower_kind, suit, rank));
        }

        let mut hand = Hand::new(hand_items);

        // Auto-select the first card (tower or barricade)
        let first_slot_id = hand.get_slot_id_by_index(0).unwrap();
        hand.select_slot(first_slot_id);

        self.flow = GameFlow::PlacingTower { hand };
    }

    pub fn goto_defense(&mut self) {
        self.flow = GameFlow::Defense(DefenseFlow::new(self));
        start_spawn(self);
    }

    pub fn goto_result(&mut self) {
        let clear_rate = self.calculate_clear_rate();

        // 남은 모든 적 제거 (패배 후에도 적들이 건물에 들어오는 것을 방지)
        self.monsters.clear();
        self.projectiles.clear();

        self.flow = GameFlow::Result { clear_rate };
    }
}
