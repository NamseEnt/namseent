use crate::game_state::*;

pub(crate) fn apply_presentation_effects(game_state: &mut GameState, item: &item::Item) {
    record_history_event(game_state, item);
}

pub(super) fn record_history_event(game_state: &mut GameState, item: &item::Item) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::ItemUsed { item: item.clone() },
    );
}
