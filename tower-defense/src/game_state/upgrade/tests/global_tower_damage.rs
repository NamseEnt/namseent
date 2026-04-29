use super::super::*;
use namui::OneZero;

#[test]
fn global_damage_treasures_combine_additively() {
    let mut game_state = super::support::create_mock_game_state();
    game_state.gold = 2000;

    game_state.upgrade(Upgrade {
        kind: UpgradeKind::Crock(CrockUpgrade),
        value: OneZero::default(),
    });
    game_state.upgrade(Upgrade {
        kind: UpgradeKind::Trophy(TrophyUpgrade { perfect_clear_stacks: 0,
         }),
        value: OneZero::default(),
    });
    game_state.upgrade_state.record_perfect_clear();

    assert_eq!(
        game_state
            .upgrade_state
            .global_tower_damage_multiplier(&game_state),
        4.0
    );
}
