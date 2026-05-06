#[test]
fn trophy_uses_perfect_clear_stacks_for_global_damage() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    game_state
        .upgrade_state
        .upgrade(crate::game_state::upgrade::TrophyUpgrade::into_upgrade());

    let tower_template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Hearts,
        crate::card::Rank::Two,
    );
    let tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    let before_damage = tower.calculate_projectile_damage(&[], 1.0);

    game_state.handle_upgrade_trigger(crate::game_state::upgrade::UpgradeTriggerEvent::StageEnd {
        perfect_clear: true,
        gold: 0,
        item_count: 0,
    });
    game_state.handle_upgrade_trigger(crate::game_state::upgrade::UpgradeTriggerEvent::StageEnd {
        perfect_clear: true,
        gold: 0,
        item_count: 0,
    });

    let upgrade_bonuses = game_state
        .upgrade_state
        .tower_upgrade_damage_bonuses(&game_state);
    let after_damage = tower.calculate_projectile_damage(&upgrade_bonuses, 1.0);

    assert!(after_damage > before_damage);
    assert!((after_damage / before_damage - 3.0).abs() < f32::EPSILON);
}

#[test]
fn trophy_perfect_clear_increments_perfect_clear_stacks() {
    use super::support;

    let mut gs = support::create_mock_game_state();
    gs.flow = crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
    gs.upgrade_state
        .upgrade(crate::game_state::upgrade::TrophyUpgrade::into_upgrade());

    crate::game_state::tick::defense_end::check_defense_end(&mut gs);

    assert!(gs.upgrade_state.upgrades.iter().any(|upgrade| {
        matches!(upgrade, crate::game_state::upgrade::Upgrade::Trophy(trophy) if trophy.perfect_clear_stacks == 1)
    }));
}
