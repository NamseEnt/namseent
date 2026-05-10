use super::Item;
use crate::game_state::GameState;

pub fn use_item(game_state: &mut GameState, item: &Item) {
    game_state.action(crate::game_state::GameStateAction::UseItem(item));
}
