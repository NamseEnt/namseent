use crate::game_state::upgrade::Upgrade;
use crate::game_state::*;

pub(crate) fn record_history_event(
    game_state: &mut GameState,
    upgrade: Upgrade,
    cost: Option<usize>,
) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::UpgradeAcquired { upgrade, cost },
    );
}
