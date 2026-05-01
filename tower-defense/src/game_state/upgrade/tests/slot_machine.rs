use super::super::*;

#[test]
fn stage_start_effects_applies_ice_cream_and_slot_machine() {
    let mut state = UpgradeState::default();
    state.upgrade(crate::game_state::upgrade::IceCreamUpgrade::into_upgrade(3.0, 5));
    state.upgrade(crate::game_state::upgrade::SlotMachineUpgrade::into_upgrade(10));

    let effects = state.stage_start_effects(2);
    let effects_next_wave = state.stage_start_effects(3);

    assert_eq!(effects.damage_multiplier, 3.0);
    assert_eq!(effects.extra_dice, 10);
    assert_eq!(effects_next_wave.damage_multiplier, 3.0);
    assert_eq!(effects_next_wave.extra_dice, 0);
}
