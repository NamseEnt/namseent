#[test]
fn spanner_keeps_shield_across_stage_transition() {
    use super::support;

    let mut gs = support::create_mock_game_state();
    gs.shield = 50.0;
    gs.upgrade_state
        .upgrade(crate::game_state::upgrade::SpannerUpgrade::into_upgrade());

    gs.goto_next_stage();

    assert_eq!(gs.shield, 50.0);
}
