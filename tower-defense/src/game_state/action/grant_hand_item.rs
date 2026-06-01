use crate::game_state::GameState;
use crate::hand::HandItem;

pub(super) fn apply(game_state: &mut GameState, item: HandItem) {
    game_state.hand.push(item);
}