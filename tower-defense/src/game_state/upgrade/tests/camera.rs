use super::super::*;
use crate::game_state::tower::{Tower, TowerTemplate};
use namui::OneZero;

#[test]
fn camera_upgrade_sets_active_flag() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::Camera(CameraUpgrade),
        value: OneZero::default(),
    });

    assert!(state.has_camera());
}

#[test]
fn camera_grants_gold_when_face_tower_is_placed() {
    let mut game_state = super::support::create_mock_game_state();
    let initial_gold = game_state.gold;

    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::Camera(CameraUpgrade),
        value: OneZero::default(),
    });

    let face_tower_template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::King,
    );
    let face_tower = Tower::new(
        &face_tower_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    game_state.place_tower(face_tower);

    assert_eq!(game_state.gold, initial_gold + 50);
}

#[test]
fn camera_does_not_grant_gold_for_number_tower() {
    let mut game_state = super::support::create_mock_game_state();
    let initial_gold = game_state.gold;

    game_state.upgrade_state.upgrade(Upgrade {
        kind: UpgradeKind::Camera(CameraUpgrade),
        value: OneZero::default(),
    });

    let number_tower_template = TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ten,
    );
    let number_tower = Tower::new(
        &number_tower_template,
        crate::MapCoord::new(2, 0),
        game_state.now(),
    );
    game_state.place_tower(number_tower);

    assert_eq!(game_state.gold, initial_gold);
}
