use super::super::*;
use namui::OneZero;

#[test]
fn crock_increases_global_tower_damage_with_gold() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.gold = 2500;

    game_state.upgrade(Upgrade {
        kind: UpgradeKind::Crock(CrockUpgrade),
        value: OneZero::default(),
    });

    assert_eq!(
        game_state
            .upgrade_state
            .global_tower_damage_multiplier(&game_state),
        3.0
    );
}
