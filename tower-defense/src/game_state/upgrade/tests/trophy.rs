use super::super::*;

#[test]
fn trophy_uses_perfect_clear_stacks_for_global_damage() {
    let mut state = UpgradeState::default();
    state.upgrade(crate::game_state::upgrade::TrophyUpgrade::into_upgrade());
    state.record_perfect_clear();
    state.record_perfect_clear();

    let game_state = super::support::create_mock_game_state();
    let global_multiplier = state.global_tower_damage_multiplier(&game_state);

    assert!((global_multiplier - 3.0).abs() < f32::EPSILON);
}

#[test]
fn trophy_perfect_clear_increments_perfect_clear_stacks() {
    let mut gs = super::support::create_mock_game_state();
    gs.flow = GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
    gs.upgrade_state
        .upgrade(crate::game_state::upgrade::TrophyUpgrade::into_upgrade());

    tick::defense_end::check_defense_end(&mut gs);

    assert!(gs.upgrade_state.upgrades.iter().any(|upgrade| {
        matches!(upgrade, Upgrade::Trophy(..)) && upgrade.perfect_clear_stacks().unwrap_or(0) == 1
    }));
}
