#[test]
fn ice_cream_effect_applies_to_placed_tower_and_expires_after_waves() {
    use super::support;
    use crate::game_state::flow::DefenseFlow;
    use crate::game_state::tower::TowerTemplate;

    let mut game_state = support::create_mock_game_state();
    game_state.flow = crate::game_state::GameFlow::Defense(DefenseFlow::new(&game_state));
    let upgrade = crate::game_state::upgrade::IceCreamUpgrade::into_upgrade(3.0, 2);
    game_state.upgrade(upgrade);

    let tower_template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Hearts,
        crate::card::Rank::Two,
    );
    let tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    game_state.place_tower(tower);

    let placed_tower = game_state
        .towers
        .iter()
        .next()
        .expect("expected tower placed");
    let base_damage = placed_tower.calculate_projectile_damage(&[], 1.0);
    let boosted_damage = placed_tower.cached_upgrade_damage();

    assert!(boosted_damage > base_damage);
    assert!((boosted_damage / base_damage - 3.0).abs() < f32::EPSILON);

    crate::game_state::tick::defense_end::check_defense_end(&mut game_state);
    let first_tower_after_second_wave = game_state
        .towers
        .iter()
        .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
        .expect("expected tower to still exist after first stage");
    let second_boosted_damage = first_tower_after_second_wave.cached_upgrade_damage();
    assert!((second_boosted_damage / base_damage - 3.0).abs() < f32::EPSILON);

    game_state.flow = crate::game_state::GameFlow::Defense(DefenseFlow::new(&game_state));
    crate::game_state::tick::defense_end::check_defense_end(&mut game_state);

    let expired_tower = game_state
        .towers
        .iter()
        .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
        .expect("expected tower to still exist after second stage");
    let expired_damage = expired_tower.cached_upgrade_damage();

    assert!((expired_damage / base_damage - 1.0).abs() < f32::EPSILON);
}
