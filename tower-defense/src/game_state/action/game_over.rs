use crate::game_state::{GameState, flow::GameFlow};

pub(super) fn clear_active_entities(game_state: &mut GameState) {
    game_state.monsters.clear();
    game_state.in_flight_attacks.clear();
}

pub(super) fn record_history_event(game_state: &mut GameState) {
    game_state.record_event(crate::game_state::play_history::HistoryEventType::GameOver);
}

pub(super) fn set_result_flow(game_state: &mut GameState) {
    let clear_rate = game_state.calculate_clear_rate();
    game_state.flow = GameFlow::Result { clear_rate };
}
