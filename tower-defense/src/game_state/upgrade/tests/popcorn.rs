use super::super::*;

#[test]
fn popcorn_effect_decrements_over_waves_and_expires() {
    let mut state = UpgradeState::default();
    state.upgrade(crate::game_state::upgrade::PopcornUpgrade::into_upgrade(5.0, 5, 5));

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
    state.upgrade(crate::game_state::upgrade::PopcornUpgrade::into_upgrade(4.0, 4, 4));

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
