use super::super::get_highest_tower_template;
use super::make_card;
use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;

#[test]
fn test_flush() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Spades, Rank::Eight),
        make_card(Suit::Spades, Rank::Nine),
        make_card(Suit::Spades, Rank::Ten),
        make_card(Suit::Spades, Rank::Queen),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Flush);
    assert_eq!(template.suit, Suit::Spades);
    assert_eq!(template.rank, Rank::Queen);
}

#[test]
fn test_flush_4cards_without_upgrade() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Spades, Rank::Eight),
        make_card(Suit::Spades, Rank::Nine),
        make_card(Suit::Spades, Rank::Ten),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_ne!(template.kind, TowerKind::Flush);
}

#[test]
fn test_flush_4cards_with_upgrade() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Spades, Rank::Eight),
        make_card(Suit::Spades, Rank::Nine),
        make_card(Suit::Spades, Rank::Jack),
    ];
    let upgrade_state = UpgradeState {
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };

    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Flush);
    assert_eq!(template.suit, Suit::Spades);
    assert_eq!(template.rank, Rank::Jack);
}

#[test]
fn test_flush_treat_suits_as_same() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Clubs, Rank::Eight),
        make_card(Suit::Spades, Rank::Nine),
        make_card(Suit::Clubs, Rank::Ten),
        make_card(Suit::Spades, Rank::Queen),
    ];
    let upgrade_state = UpgradeState {
        treat_suits_as_same: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Flush);
    assert!(template.suit == Suit::Spades || template.suit == Suit::Clubs);
    assert_eq!(template.rank, Rank::Queen);
}

#[test]
fn test_flush_treat_suits_as_same_and_shorten_4cards() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Clubs, Rank::Eight),
        make_card(Suit::Spades, Rank::Nine),
        make_card(Suit::Clubs, Rank::Jack),
    ];
    let upgrade_state = UpgradeState {
        treat_suits_as_same: true,
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Flush);
    assert!(template.suit == Suit::Spades || template.suit == Suit::Clubs);
    assert_eq!(template.rank, Rank::Jack);
}
