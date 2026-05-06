#[test]
fn membership_card_grants_free_shop_next_stage() {
    use super::support;
    use crate::game_state::GameFlow;
    use crate::game_state::effect::Effect;
    use crate::game_state::item::ItemKind;
    use crate::shop::ShopSlot;

    let mut game_state = support::create_mock_game_state();
    game_state.upgrade(crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade());

    game_state.handle_upgrade_trigger(
        crate::game_state::upgrade::UpgradeTriggerEvent::StageStart { stage: 3 },
    );
    assert!(game_state.stage_modifiers.is_free_shop_this_stage());

    game_state.goto_selecting_tower();
    let initial_gold = game_state.gold;

    let slot_id = if let GameFlow::SelectingTower(flow) = &mut game_state.flow {
        flow.shop
            .slots
            .iter()
            .find_map(|slot_data| match &slot_data.slot {
                ShopSlot::Item { .. } if !slot_data.purchased => Some(slot_data.id),
                _ => None,
            })
            .expect("expected at least one item slot in shop")
    } else {
        panic!("expected selecting tower flow");
    };

    game_state.purchase_shop_item(slot_id);
    assert_eq!(game_state.gold, initial_gold);
    assert!(
        game_state
            .items
            .iter()
            .any(|item| item.kind == ItemKind::LumpSugar)
            || game_state
                .items
                .iter()
                .any(|item| item.effect == Effect::ExtraDice)
    );
}
