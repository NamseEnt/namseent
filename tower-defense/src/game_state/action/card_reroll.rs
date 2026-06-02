use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;
use crate::game_state::GameState;

pub(super) fn trigger_upgrades(game_state: &mut GameState) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::CardReroll);
}
