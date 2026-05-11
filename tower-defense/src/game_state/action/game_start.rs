use crate::game_state::GameState;

pub(super) fn record_history_event(game_state: &mut GameState) {
    game_state.record_event(crate::game_state::play_history::HistoryEventType::GameStart);
}
