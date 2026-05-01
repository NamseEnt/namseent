use super::super::*;

#[test]
fn tape_applies_enemy_speed_reduction_every_four_waves() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.stage = 3;

    game_state.upgrade(crate::game_state::upgrade::TapeUpgrade::into_upgrade(0));

    let effects_stage_3 = game_state.upgrade_state.stage_start_effects(3);
    assert_eq!(effects_stage_3.enemy_speed_multiplier, None);

    let effects_stage_4 = game_state.upgrade_state.stage_start_effects(4);
    assert_eq!(effects_stage_4.enemy_speed_multiplier, Some(0.75));

    let effects_stage_5 = game_state.upgrade_state.stage_start_effects(5);
    assert_eq!(effects_stage_5.enemy_speed_multiplier, None);

    let effects_stage_8 = game_state.upgrade_state.stage_start_effects(8);
    assert_eq!(effects_stage_8.enemy_speed_multiplier, Some(0.75));
}
