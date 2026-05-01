use super::super::*;

#[test]
fn gift_box_awards_gold_per_item_on_stage_end() {
    let mut gs = super::support::create_mock_game_state();
    gs.flow = GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
    gs.items = vec![
        crate::game_state::item::Item {
            kind: ItemKind::LumpSugar,
            effect: crate::game_state::item::Effect::ExtraDice,
        },
        crate::game_state::item::Item {
            kind: ItemKind::LumpSugar,
            effect: crate::game_state::item::Effect::ExtraDice,
        },
    ];
    gs.upgrade_state.upgrade(crate::game_state::upgrade::GiftBoxUpgrade::into_upgrade());

    tick::defense_end::check_defense_end(&mut gs);

    assert_eq!(gs.gold, gs.config.player.starting_gold + 20);
}
