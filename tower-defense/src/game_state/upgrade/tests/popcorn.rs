use super::super::*;
use namui::OneZero;

#[test]
fn popcorn_effect_decrements_over_waves_and_expires() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::Popcorn(PopcornUpgrade { max_multiplier: 5.0, duration: 5, waves_remaining: 5,
         }),
        value: OneZero::default(),
    });

    for remaining in (1..=5).rev() {
        let effects = state.stage_start_effects(1);
        assert_eq!(effects.damage_multiplier, remaining as f32);
    }

    let effects = state.stage_start_effects(6);
    assert_eq!(effects.damage_multiplier, 1.0);
}

#[test]
fn popcorn_uses_configured_max_multiplier_and_duration() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::Popcorn(PopcornUpgrade { max_multiplier: 4.0, duration: 4, waves_remaining: 4,
         }),
        value: OneZero::default(),
    });

    let first = state.stage_start_effects(1);
    let second = state.stage_start_effects(2);
    let third = state.stage_start_effects(3);
    let fourth = state.stage_start_effects(4);
    let fifth = state.stage_start_effects(5);

    assert!((first.damage_multiplier - 4.0).abs() < f32::EPSILON);
    assert!((second.damage_multiplier - 3.0).abs() < f32::EPSILON);
    assert!((third.damage_multiplier - 2.0).abs() < f32::EPSILON);
    assert!((fourth.damage_multiplier - 1.0).abs() < f32::EPSILON);
    assert!((fifth.damage_multiplier - 1.0).abs() < f32::EPSILON);
}
