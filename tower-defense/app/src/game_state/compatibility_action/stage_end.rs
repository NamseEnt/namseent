use crate::game_state::*;

#[cfg(any(test, feature = "debug-tools"))]
pub(super) fn update_clear_metrics(game_state: &mut GameState, perfect_clear: bool) {
    if perfect_clear {
        game_state.record_event(
            crate::game_state::play_history::HistoryEventType::StagePerfectClear {
                stage: game_state.raw_core.progress().stage,
            },
        );
    }
    let mut raw = game_state.raw_core.state().clone();
    raw.update_clear_metrics(perfect_clear);
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw clear metrics must be restorable in headed adapter");
}

#[cfg(any(test, feature = "debug-tools"))]
pub(super) fn trigger_upgrades(
    game_state: &mut GameState,
    perfect_clear: bool,
    gold: usize,
    item_count: usize,
) {
    let mut raw = game_state.raw_core.state().clone();
    raw.trigger_stage_end_upgrades(perfect_clear, gold, item_count);
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw stage-end upgrades must be restorable in headed adapter");
}
