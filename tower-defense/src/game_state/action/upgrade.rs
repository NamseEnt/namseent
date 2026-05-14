use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;
use crate::game_state::upgrade::Upgrade;
use crate::game_state::*;

pub(super) fn trigger_upgrades(game_state: &mut GameState, upgrade: Upgrade) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::UpgradeAcquired { upgrade });
}

pub(super) fn record_history_event(
    game_state: &mut GameState,
    upgrade: Upgrade,
    cost: Option<usize>,
) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::UpgradeAcquired { upgrade, cost },
    );
}
