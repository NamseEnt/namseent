use crate::{Shield, game_state::GameState};

pub(super) fn apply(game_state: &mut GameState, amount: Shield) {
    game_state.shield = game_state.shield.saturating_add(amount);
}
