use crate::game_state::GameState;

pub(super) fn apply(game_state: &mut GameState, amount: f32) {
    game_state.shield += amount;
}
