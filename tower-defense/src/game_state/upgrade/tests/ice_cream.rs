use super::super::*;
use namui::OneZero;

#[test]
fn ice_cream_effect_expires_after_five_waves() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::IceCream(IceCreamUpgrade { damage_multiplier: 3.0, waves_remaining: 5,
         }),
        value: OneZero::default(),
    });

    for expected in (1..=5).rev() {
        let effects = state.stage_start_effects(expected);
        assert_eq!(effects.damage_multiplier, 3.0);
    }

    let effects = state.stage_start_effects(6);
    assert_eq!(effects.damage_multiplier, 1.0);
}

#[test]
fn ice_cream_uses_configured_multiplier_and_duration() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::IceCream(IceCreamUpgrade { damage_multiplier: 2.5, waves_remaining: 2,
         }),
        value: OneZero::default(),
    });

    let first = state.stage_start_effects(1);
    let second = state.stage_start_effects(2);
    let third = state.stage_start_effects(3);

    assert!((first.damage_multiplier - 2.5).abs() < f32::EPSILON);
    assert!((second.damage_multiplier - 2.5).abs() < f32::EPSILON);
    assert!((third.damage_multiplier - 1.0).abs() < f32::EPSILON);
}
