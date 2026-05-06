#[test]
fn slot_machine_grants_extra_dice_on_stage_start_only_once() {
    use super::support;
    use crate::game_state::upgrade::UpgradeTriggerEvent;

    let mut game_state = support::create_mock_game_state();
    game_state.upgrade(crate::game_state::upgrade::SlotMachineUpgrade::into_upgrade(10));

    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::StageStart { stage: 1 });
    assert_eq!(
        game_state.left_dice,
        game_state.max_dice_chance() + 10,
        "slot machine should add extra dice on the first stage start",
    );

    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::StageStart { stage: 2 });
    assert_eq!(
        game_state.left_dice,
        game_state.max_dice_chance(),
        "slot machine should only apply extra dice once",
    );
}
