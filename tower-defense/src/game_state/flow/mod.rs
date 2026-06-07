use super::GameState;
use crate::{shop::Shop, *};

#[derive(Clone, Debug, State)]
#[allow(clippy::large_enum_variant)]
pub enum GameFlow {
    Initializing,
    Shopping(ShoppingFlow),
    SelectingTower(SelectingTowerFlow),
    PlacingTower,
    Defense(DefenseFlow),
    TreasureSelection(TreasureSelectionFlow),
    Result { clear_rate: f32 },
}

#[derive(Clone, Debug, State)]
pub struct TreasureSelectionFlow {
    pub options: Vec<crate::game_state::upgrade::Upgrade>,
    pub pending_selection: Option<usize>,
}

impl TreasureSelectionFlow {
    pub fn new(game_state: &GameState) -> Self {
        let options = (0..3)
            .map(|_| crate::game_state::upgrade::generate_boss_reward_upgrade(game_state))
            .collect();
        TreasureSelectionFlow {
            options,
            pending_selection: None,
        }
    }

    fn update(&mut self) {}
}
impl GameFlow {
    pub(crate) fn update(&mut self) {
        match self {
            GameFlow::Shopping(shopping_flow) => shopping_flow.update(),
            GameFlow::SelectingTower(selecting_tower) => selecting_tower.update(),
            GameFlow::TreasureSelection(treasure_flow) => treasure_flow.update(),
            _ => {}
        }
    }
}

#[derive(Clone, Debug, State)]
pub struct ShoppingFlow {
    pub shop: Shop,
}

impl ShoppingFlow {
    pub fn new(game_state: &GameState) -> Self {
        let shop = Shop::new(game_state);
        ShoppingFlow { shop }
    }

    fn update(&mut self) {
        self.shop.update();
    }
}

#[derive(Clone, Debug, State)]
pub struct SelectingTowerFlow {}

impl SelectingTowerFlow {
    pub fn new(_game_state: &GameState) -> Self {
        SelectingTowerFlow {}
    }

    fn update(&mut self) {}
}

#[derive(Clone, Debug, State)]
pub struct DefenseFlow {
    pub stage_progress: StageProgress,
    pub took_damage: bool,
}

impl DefenseFlow {
    pub fn new(game_state: &GameState) -> Self {
        let start_total_hp = GameState::calculate_stage_total_hp(
            game_state.stage,
            &game_state.config,
            &game_state.stage_modifiers,
        );
        Self {
            stage_progress: StageProgress {
                start_total_hp,
                processed_hp: 0.0,
            },
            took_damage: false,
        }
    }
}

#[derive(Clone, Debug, State)]
pub struct StageProgress {
    pub start_total_hp: f32,
    pub processed_hp: f32,
}
