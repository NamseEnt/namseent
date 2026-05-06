#[test]
fn demolition_hammer_stage_end_stores_removed_tower_damage_bonus() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    let upgrade = crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(2.0);
    game_state.upgrade_state.upgrade(upgrade);

    let tower_template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Hearts,
        crate::card::Rank::Two,
    );
    let first_tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    let second_tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(2, 0),
        game_state.now(),
    );

    game_state.place_tower(first_tower);
    game_state.place_tower(second_tower);

    let first_id = game_state
        .towers
        .iter()
        .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
        .expect("expected first tower placed")
        .id();
    let second_id = game_state
        .towers
        .iter()
        .find(|tower| tower.left_top == crate::MapCoord::new(2, 0))
        .expect("expected second tower placed")
        .id();
    assert!(game_state.remove_tower(first_id));
    assert!(game_state.remove_tower(second_id));

    game_state.handle_upgrade_trigger(crate::game_state::upgrade::UpgradeTriggerEvent::StageEnd {
        perfect_clear: false,
        gold: game_state.gold,
        item_count: game_state.items.len(),
    });

    let upgrade_bonuses = game_state
        .upgrade_state
        .tower_upgrade_damage_bonuses(&game_state);

    assert_eq!(upgrade_bonuses.len(), 1);
    assert!((upgrade_bonuses[0].bonus_pct - 4.0).abs() < f32::EPSILON);
    assert!(matches!(
        game_state.upgrade_state.upgrades[0],
        crate::game_state::upgrade::Upgrade::DemolitionHammer(..)
    ));
    if let crate::game_state::upgrade::Upgrade::DemolitionHammer(upgrade) =
        game_state.upgrade_state.upgrades[0]
    {
        assert_eq!(upgrade.removed_tower_count, 0);
    }
}

#[test]
fn demolition_hammer_uses_configured_damage_multiplier() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    let upgrade = crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(1.25);
    game_state.upgrade_state.upgrade(upgrade);

    let tower_template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Hearts,
        crate::card::Rank::Two,
    );
    let first_tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    let second_tower = crate::game_state::tower::Tower::new(
        &tower_template,
        crate::MapCoord::new(2, 0),
        game_state.now(),
    );

    game_state.place_tower(first_tower);
    game_state.place_tower(second_tower);

    let first_id = game_state
        .towers
        .iter()
        .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
        .expect("expected first tower placed")
        .id();
    let second_id = game_state
        .towers
        .iter()
        .find(|tower| tower.left_top == crate::MapCoord::new(2, 0))
        .expect("expected second tower placed")
        .id();
    assert!(game_state.remove_tower(first_id));
    assert!(game_state.remove_tower(second_id));

    game_state.handle_upgrade_trigger(crate::game_state::upgrade::UpgradeTriggerEvent::StageEnd {
        perfect_clear: false,
        gold: game_state.gold,
        item_count: game_state.items.len(),
    });

    let upgrade_bonuses = game_state
        .upgrade_state
        .tower_upgrade_damage_bonuses(&game_state);

    assert_eq!(upgrade_bonuses.len(), 1);
    assert!((upgrade_bonuses[0].bonus_pct - 2.5).abs() < f32::EPSILON);
}
