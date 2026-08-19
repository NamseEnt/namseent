use crate::{Health, game_state::GameState};

pub(super) fn apply(game_state: &mut GameState, amount: Health) {
    game_state.hp = game_state
        .hp
        .saturating_add(amount)
        .min(game_state.max_hp());
}
