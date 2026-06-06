use crate::game_state::{
    GameState,
    flow::{GameFlow, SelectingTowerFlow},
};

pub(super) fn set_selecting_tower_flow(game_state: &mut GameState) {
    game_state.flow = GameFlow::SelectingTower(SelectingTowerFlow {});
}
