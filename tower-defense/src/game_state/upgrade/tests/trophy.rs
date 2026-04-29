use super::super::*;
use namui::OneZero;

#[test]
fn trophy_uses_perfect_clear_stacks_for_global_damage() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::Trophy(TrophyUpgrade { perfect_clear_stacks: 0,
         }),
        value: OneZero::default(),
    });
    state.record_perfect_clear();
    state.record_perfect_clear();

    let game_state = super::support::create_mock_game_state();
    let global_multiplier = state.global_tower_damage_multiplier(&game_state);

    assert!((global_multiplier - 3.0).abs() < f32::EPSILON);
}
