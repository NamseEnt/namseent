#[test]
fn piggy_bank_awards_gold_on_stage_end_with_enough_gold() {
    use super::support;

    let mut gs = support::create_mock_game_state();
    gs.flow = crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
    gs.gold = 500;
    gs.upgrade_state
        .upgrade(crate::game_state::upgrade::PiggyBankUpgrade::into_upgrade());

    crate::game_state::tick::defense_end::check_defense_end(&mut gs);

    assert_eq!(gs.gold, 550);
}
