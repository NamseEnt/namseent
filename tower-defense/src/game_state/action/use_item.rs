use crate::game_state::*;

pub(super) fn can_use(game_state: &GameState, item: &item::Item) -> bool {
    item.can_use(game_state).is_ok()
}

pub(super) fn mark_as_used(game_state: &mut GameState) {
    game_state.item_used = true;
}

pub(super) fn apply_effect(game_state: &mut GameState, item: &item::Item) {
    item.use_item(game_state);
}

pub(super) fn record_history_event(game_state: &mut GameState, item: &item::Item) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::ItemUsed { item: item.clone() },
    );
}
