use crate::game_state::GameState;
use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;

pub(super) fn trigger_upgrades(game_state: &mut GameState) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::CardReroll);
}
