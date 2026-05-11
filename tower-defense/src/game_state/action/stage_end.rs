use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, *};

pub(super) fn update_clear_metrics(game_state: &mut GameState, perfect_clear: bool) {
    if perfect_clear {
        game_state.record_event(
            crate::game_state::play_history::HistoryEventType::StagePerfectClear {
                stage: game_state.stage,
            },
        );
        game_state.metrics.current_consecutive_perfect_clears += 1;
        game_state.metrics.max_consecutive_perfect_clears = game_state
            .metrics
            .max_consecutive_perfect_clears
            .max(game_state.metrics.current_consecutive_perfect_clears);
    } else {
        game_state.metrics.current_consecutive_perfect_clears = 0;
    }
}

pub(super) fn trigger_upgrades(
    game_state: &mut GameState,
    perfect_clear: bool,
    gold: usize,
    item_count: usize,
) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::StageEnd {
        perfect_clear,
        gold,
        item_count,
    });
}
