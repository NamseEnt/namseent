use super::super::*;
use namui::OneZero;

#[test]
fn stage_start_effects_applies_ice_cream_and_slot_machine() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::IceCream(IceCreamUpgrade { damage_multiplier: 3.0, waves_remaining: 5,
         }),
        value: OneZero::default(),
    });
    state.upgrade(Upgrade {
        kind: UpgradeKind::SlotMachine(SlotMachineUpgrade { next_round_dice: 10,
         }),
        value: OneZero::default(),
    });

    let effects = state.stage_start_effects(2);
    let effects_next_wave = state.stage_start_effects(3);

    assert_eq!(effects.damage_multiplier, 3.0);
    assert_eq!(effects.extra_dice, 10);
    assert_eq!(effects_next_wave.damage_multiplier, 3.0);
    assert_eq!(effects_next_wave.extra_dice, 0);
}
