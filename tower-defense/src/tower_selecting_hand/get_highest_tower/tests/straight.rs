use super::super::get_highest_tower_template;
use super::make_card;
use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;

#[test]
fn test_straight() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Nine),
        make_card(Suit::Diamonds, Rank::Ten),
        make_card(Suit::Spades, Rank::Jack),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Jack);
}

#[test]
fn test_straight_4cards_without_upgrade() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Nine),
        make_card(Suit::Diamonds, Rank::Ten),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_ne!(template.kind, TowerKind::Straight);
}

#[test]
fn test_straight_4cards_with_upgrade() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Nine),
        make_card(Suit::Diamonds, Rank::Ten),
    ];
    let upgrade_state = UpgradeState {
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Ten);
}

#[test]
fn test_straight_skip_rank() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Nine),
        make_card(Suit::Diamonds, Rank::Jack),
    ];
    let upgrade_state = UpgradeState {
        skip_rank_for_straight: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::High);
    assert_eq!(template.rank, Rank::Jack);
}

#[test]
fn test_straight_skip_rank_and_shorten_4cards() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Jack),
        make_card(Suit::Diamonds, Rank::Nine),
    ];
    let upgrade_state = UpgradeState {
        skip_rank_for_straight: true,
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Jack);
}
