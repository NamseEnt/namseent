use super::super::*;
use namui::OneZero;

#[test]
fn metronome_grants_extra_dice_every_two_waves() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::Metronome(MetronomeUpgrade { start_stage: None  }),
        value: OneZero::default(),
    });

    let effects_stage_1 = state.stage_start_effects(1);
    assert_eq!(effects_stage_1.extra_dice, 1);

    let effects_stage_2 = state.stage_start_effects(2);
    assert_eq!(effects_stage_2.extra_dice, 0);

    let effects_stage_3 = state.stage_start_effects(3);
    assert_eq!(effects_stage_3.extra_dice, 1);
}
