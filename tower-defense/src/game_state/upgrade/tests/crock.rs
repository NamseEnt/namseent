#[test]
fn crock_increases_tower_damage_for_existing_towers() {
    use super::support;

    let mut game_state = support::create_mock_game_state();

    let tower_template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::FullHouse,
        crate::card::Suit::Hearts,
        crate::card::Rank::Queen,
    );
    let tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    game_state.place_tower(tower);

    let tower_id = game_state
        .towers
        .iter()
        .next()
        .expect("expected tower to be placed")
        .id();
    let before_damage = game_state
        .towers
        .iter()
        .find(|tower| tower.id() == tower_id)
        .expect("expected placed tower")
        .calculate_projectile_damage(&[], 1.0);

    game_state.gold = 2500;
    game_state.upgrade(crate::game_state::upgrade::CrockUpgrade::into_upgrade());

    let after_damage = {
        let tower = game_state
            .towers
            .iter()
            .find(|tower| tower.id() == tower_id)
            .expect("expected placed tower");
        let upgrade_bonuses = game_state
            .upgrade_state
            .tower_upgrade_damage_bonuses(&game_state);
        tower.calculate_projectile_damage(&upgrade_bonuses, 1.0)
    };

    assert!(before_damage > 0.0);
    assert!(after_damage > before_damage);
}
