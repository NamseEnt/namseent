use crate::game_state::upgrade::Upgrade;

#[test]
fn name_tag_applies_to_next_tower_and_consumes_it() {
    use super::support;

    let mut game_state = support::create_mock_game_state();
    game_state
        .upgrade_state
        .upgrade(crate::game_state::upgrade::NameTagUpgrade::into_upgrade(
            3.0,
        ));
    game_state.left_dice = 0;

    let template = crate::game_state::tower::TowerTemplate::new(
        crate::game_state::tower::TowerKind::High,
        crate::card::Suit::Spades,
        crate::card::Rank::Ace,
    );
    game_state.goto_placing_tower(template);

    assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
        if let Upgrade::NameTag(upgrade) = upgrade {
            (upgrade.damage_multiplier - 3.0).abs() < f32::EPSILON
        } else {
            false
        }
    }));

    let placing_slot_id = game_state
        .hand
        .get_slot_id_by_index(0)
        .expect("expected tower slot to be present");
    let placed_template = support::first_hand_tower_template(&game_state);
    let tower = crate::game_state::tower::Tower::new(
        &placed_template,
        crate::MapCoord::new(0, 0),
        game_state.now(),
    );
    game_state.place_tower(tower);
    game_state.hand.delete_slots(&[placing_slot_id]);

    let placed_tower = game_state
        .towers
        .iter()
        .next()
        .expect("expected tower placed");
    support::assert_tower_cached_damage_mul(placed_tower, 3.0);
}
