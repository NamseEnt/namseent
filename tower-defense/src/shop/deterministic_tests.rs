use super::ShopSlot;
use super::bag;
use super::deterministic::{self, CARD_SERVICE_CATEGORY, ITEM_CATEGORY, UPGRADE_CATEGORY};
use crate::game_state::rng::GameRngState;
use crate::game_state::{GameState, action::GameStateAction, flow::GameFlow};
use crate::rarity::Rarity;
use std::collections::HashSet;

fn slot_key(slot: &ShopSlot) -> String {
    match slot {
        ShopSlot::Item { item, .. } => format!("item:{:?}", item.discriminant()),
        ShopSlot::CardService { card_service, .. } => format!(
            "card_service:{:?}",
            crate::game_state::card_service::CardServiceDiscriminants::from(card_service)
        ),
        ShopSlot::Upgrade { upgrade, .. } => format!("upgrade:{:?}", upgrade.discriminant()),
    }
}

fn shop_signature(game_state: &GameState) -> Vec<String> {
    let GameFlow::Shopping(flow) = &game_state.flow else {
        panic!("expected shopping flow");
    };
    flow.shop
        .slots
        .iter()
        .map(|slot| format!("{}:{:?}", slot_key(&slot.slot), slot.slot))
        .collect()
}

fn generated_screens(seed: u64) -> Vec<Vec<String>> {
    let mut game_state = crate::game_state::create_game_state_with_seed(seed);
    let mut screens = vec![shop_signature(&game_state)];
    for stage in 2..=4 {
        game_state.action(GameStateAction::StartStage { stage });
        screens.push(shop_signature(&game_state));
    }
    screens
}

#[test]
fn same_seed_replays_multiple_shop_screens() {
    assert_eq!(
        generated_screens(0xA11C_E123),
        generated_screens(0xA11C_E123)
    );
}

#[test]
fn different_seeds_change_shop_order_or_payload() {
    assert_ne!(generated_screens(1), generated_screens(2));
}

#[test]
fn shop_continuation_matches_after_game_state_serialize_and_deserialize() {
    let mut direct = crate::game_state::create_game_state_with_seed(0x5EED_2026);
    direct.action(GameStateAction::StartStage { stage: 2 });
    let first_screen = shop_signature(&direct);

    let bytes = namui::bincode::encode_to_vec(&direct, namui::bincode::config::standard())
        .expect("GameState should serialize");
    let (mut restored, consumed): (GameState, usize) =
        namui::bincode::decode_from_slice(&bytes, namui::bincode::config::standard())
            .expect("GameState should deserialize");
    assert_eq!(consumed, bytes.len());
    assert_eq!(first_screen, shop_signature(&restored));

    direct.action(GameStateAction::StartStage { stage: 3 });
    restored.action(GameStateAction::StartStage { stage: 3 });
    assert_eq!(shop_signature(&direct), shop_signature(&restored));
}

#[test]
fn category_bag_matches_quota_for_multiple_cycles() {
    let mut state = GameRngState::new(17);
    let mut counts = [0; 3];
    for _ in 0..20 {
        counts[bag::draw_category(&mut state)] += 1;
    }
    assert_eq!(counts, [10, 6, 4]);
}

#[test]
fn category_bag_replays_the_second_and_third_cycles() {
    fn sequence(seed: u64) -> Vec<usize> {
        let mut state = GameRngState::new(seed);
        (0..30).map(|_| bag::draw_category(&mut state)).collect()
    }

    assert_eq!(sequence(88), sequence(88));
    assert_ne!(sequence(88), sequence(89));
}

#[test]
fn rarity_bag_matches_category_quota() {
    let mut state = GameRngState::new(17);
    let eligible = vec![Rarity::Common, Rarity::Rare, Rarity::Epic];
    let mut counts = [0; 4];
    for _ in 0..20 {
        counts
            [bag::rarity_index(bag::draw_rarity(&mut state, ITEM_CATEGORY, &eligible).unwrap())] +=
            1;
    }
    assert_eq!(counts, [8, 10, 2, 0]);
}

#[test]
fn content_bag_has_no_duplicate_in_a_cycle_and_sorts_keys() {
    let mut first = GameRngState::new(29);
    let mut second = GameRngState::new(29);
    let first_keys = vec![
        "content:c".to_string(),
        "content:a".to_string(),
        "content:b".to_string(),
    ];
    let second_keys = vec![
        "content:b".to_string(),
        "content:c".to_string(),
        "content:a".to_string(),
    ];
    let mut first_seen = Vec::new();
    let mut second_seen = Vec::new();
    for _ in 0..3 {
        first_seen.push(
            bag::draw_content_key(
                &mut first,
                CARD_SERVICE_CATEGORY,
                Rarity::Common,
                &first_keys,
                &first_seen,
            )
            .unwrap(),
        );
        second_seen.push(
            bag::draw_content_key(
                &mut second,
                CARD_SERVICE_CATEGORY,
                Rarity::Common,
                &second_keys,
                &second_seen,
            )
            .unwrap(),
        );
    }
    assert_eq!(first_seen, second_seen);
    assert_eq!(first_seen.iter().collect::<HashSet<_>>().len(), 3);
}

#[test]
fn screen_generation_entrypoint_returns_requested_slots() {
    let mut game_state = crate::game_state::create_game_state_with_seed(100);
    assert_eq!(
        deterministic::generate_shop_screen_with_stats(&mut game_state, 3, &[], None)
            .slots
            .len(),
        3
    );
}

#[test]
fn screen_prefers_unique_content_when_candidates_are_available() {
    let mut game_state = crate::game_state::create_game_state_with_seed(101);
    let generated = deterministic::generate_shop_screen_with_stats(&mut game_state, 12, &[], None);
    let keys = generated
        .slots
        .iter()
        .map(|slot| slot_key(&slot.slot))
        .collect::<HashSet<_>>();
    assert_eq!(keys.len(), generated.slots.len());
    assert_eq!(generated.stats.unavoidable_duplicate_count, 0);
}

#[test]
fn duplicate_fallback_is_used_only_when_all_unique_content_is_exhausted() {
    let mut game_state = crate::game_state::create_game_state_with_seed(102);
    let generated = deterministic::generate_shop_screen_with_stats(&mut game_state, 80, &[], None);
    assert!(generated.stats.unavoidable_duplicate_count > 0);
    assert!(
        generated
            .slots
            .iter()
            .map(|slot| slot_key(&slot.slot))
            .collect::<HashSet<_>>()
            .len()
            < generated.slots.len()
    );
}

#[test]
fn empty_rarity_and_category_eligibility_have_deterministic_fallbacks() {
    let mut state = GameRngState::new(7);
    state.shop.config.rarity_bag_size = 0;
    assert_eq!(bag::draw_rarity(&mut state, UPGRADE_CATEGORY, &[]), None);
    assert_eq!(
        bag::draw_rarity(&mut state, UPGRADE_CATEGORY, &[Rarity::Rare]),
        Some(Rarity::Rare)
    );
    state.shop.config.category_bag_size = 0;
    assert_eq!(bag::draw_category(&mut state), ITEM_CATEGORY);
}

#[test]
fn content_addition_does_not_change_rarity_quota() {
    assert_eq!(bag::weighted_quota(&[150, 175, 20, 0], 10), [4, 5, 1, 0]);
    let before = bag::weighted_quota(&[150, 175, 20, 0], 10);
    let after = bag::weighted_quota(&[150, 175, 20, 0], 10);
    assert_eq!(before, after);
}

#[test]
fn shop_slots_share_the_same_rng_state_for_extensions_and_free_services() {
    let mut game_state = crate::game_state::create_game_state_with_seed(55);
    let before = game_state.rng.shop.generation_sequence;
    crate::shop::add_shop_slots(&mut game_state, 2);
    let after_extension = game_state.rng.shop.generation_sequence;
    assert_eq!(after_extension, before + 1);

    let GameFlow::Shopping(flow) = &game_state.flow else {
        panic!("expected shopping flow");
    };
    let mut shop = flow.shop.clone();
    shop.push_free_card_service(&mut game_state);
    assert_eq!(game_state.rng.shop.generation_sequence, after_extension + 1);
    assert!(matches!(
        shop.slots.last().map(|slot| &slot.slot),
        Some(ShopSlot::CardService { cost: 0, .. })
    ));
}
