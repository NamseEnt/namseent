use super::super::get_highest_tower_template;
use super::make_card;
use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;

#[test]
fn test_straight_flush() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Nine),
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::King);
}

#[test]
fn test_straight_flush_4cards_without_upgrade() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_ne!(template.kind, TowerKind::StraightFlush);
}

#[test]
fn test_straight_flush_4cards_with_upgrade() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
    ];
    let upgrade_state = UpgradeState {
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::King);
}

#[test]
fn test_straight_flush_skip_rank() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = UpgradeState {
        skip_rank_for_straight: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::High);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Ace);
}

#[test]
fn test_straight_flush_treat_suits_as_same() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Nine),
        make_card(Suit::Diamonds, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Diamonds, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
    ];
    let upgrade_state = UpgradeState {
        treat_suits_as_same: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert!(template.suit == Suit::Hearts || template.suit == Suit::Diamonds);
    assert_eq!(template.rank, Rank::King);
}

#[test]
fn test_straight_flush_treat_suits_as_same_and_shorten_4cards() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Diamonds, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Diamonds, Rank::King),
    ];
    let upgrade_state = UpgradeState {
        treat_suits_as_same: true,
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert!(template.suit == Suit::Hearts || template.suit == Suit::Diamonds);
    assert_eq!(template.rank, Rank::King);
}
