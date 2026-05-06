#[test]
fn metronome_grants_extra_dice_every_two_waves() {
    let mut state = crate::game_state::upgrade::UpgradeState::default();
    state.upgrade(crate::game_state::upgrade::MetronomeUpgrade::into_upgrade());

    let (effects_stage_1, _) = state.stage_start_effects(1);
    assert_eq!(effects_stage_1.extra_dice, 1);

    let (effects_stage_2, _) = state.stage_start_effects(2);
    assert_eq!(effects_stage_2.extra_dice, 0);

    let (effects_stage_3, _) = state.stage_start_effects(3);
    assert_eq!(effects_stage_3.extra_dice, 1);
}
