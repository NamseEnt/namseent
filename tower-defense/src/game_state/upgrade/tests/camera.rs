#[test]
fn camera_grants_gold_when_face_tower_is_placed() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    let initial_gold = game_state.gold;

    game_state
        .upgrade_state
        .upgrade(crate::game_state::upgrade::CameraUpgrade::into_upgrade());

    let face_tower_template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::King,
    );
    let face_tower = crate::game_state::tower::Tower::new(
        &face_tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    game_state.place_tower(face_tower);

    assert_eq!(game_state.gold, initial_gold + 50);
}

#[test]
fn camera_does_not_grant_gold_for_number_tower() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    let initial_gold = game_state.gold;

    game_state
        .upgrade_state
        .upgrade(crate::game_state::upgrade::CameraUpgrade::into_upgrade());

    let number_tower_template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ten,
    );
    let number_tower = crate::game_state::tower::Tower::new(
        &number_tower_template,
        crate::MapCoord::new(2, 0),
        game_state.now(),
    );
    game_state.place_tower(number_tower);

    assert_eq!(game_state.gold, initial_gold);
}
