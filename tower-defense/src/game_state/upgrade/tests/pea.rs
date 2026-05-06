#[test]
fn pea_increases_max_hp_and_fully_heals() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    game_state.hp = 1.0;

    game_state.upgrade(crate::game_state::upgrade::PeaUpgrade::into_upgrade());

    assert_eq!(game_state.upgrade_state.max_hp_plus(), 10);
    assert!((game_state.max_hp() - (game_state.config.player.max_hp + 10.0)).abs() < f32::EPSILON);
    assert!((game_state.hp - game_state.max_hp()).abs() < f32::EPSILON);
}
