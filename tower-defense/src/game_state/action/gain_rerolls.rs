use crate::game_state::GameState;

pub(super) fn apply(game_state: &mut GameState, amount: usize) {
    game_state.left_dice += amount;
}
