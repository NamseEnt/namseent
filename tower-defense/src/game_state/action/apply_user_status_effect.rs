use crate::game_state::user_status_effect::UserStatusEffect;
use crate::game_state::GameState;

pub(super) fn apply(game_state: &mut GameState, status_effect: UserStatusEffect) {
    game_state.user_status_effects.push(status_effect);
}