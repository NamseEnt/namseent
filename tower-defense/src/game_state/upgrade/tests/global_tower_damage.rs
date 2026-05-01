use super::super::*;

#[test]
fn global_damage_treasures_combine_additively() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.gold = 2000;

    game_state.upgrade(crate::game_state::upgrade::CrockUpgrade::into_upgrade());
    game_state.upgrade(crate::game_state::upgrade::TrophyUpgrade::into_upgrade());
    game_state.upgrade_state.record_perfect_clear();

    assert_eq!(
        game_state
            .upgrade_state
            .global_tower_damage_multiplier(&game_state),
        4.0
    );
}
