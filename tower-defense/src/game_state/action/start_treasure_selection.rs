use crate::game_state::{
    GameState,
    flow::{GameFlow, TreasureSelectionFlow},
};

pub(super) fn set_treasure_selection_flow(game_state: &mut GameState) {
    game_state.flow = GameFlow::TreasureSelection(TreasureSelectionFlow::new(game_state));
}
