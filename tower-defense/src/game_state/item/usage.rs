use super::Item;
use crate::game_state::GameState;

pub fn use_item(game_state: &mut GameState, item: &Item) {
    game_state.use_item(item);
}
