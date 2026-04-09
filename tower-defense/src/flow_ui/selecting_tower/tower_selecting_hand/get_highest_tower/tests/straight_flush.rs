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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::King);
}

#[test]
fn test_straight_flush_with_removed_two_and_shorten_4cards_allows_ace_low() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ace),
        make_card(Suit::Hearts, Rank::Three),
        make_card(Suit::Hearts, Rank::Four),
        make_card(Suit::Hearts, Rank::Five),
        make_card(Suit::Hearts, Rank::Six),
    ];
    let upgrade_state = UpgradeState {
        removed_number_rank_count: 1,
        shorten_straight_flush_to_4_cards: true,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Six);
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert!(template.suit == Suit::Hearts || template.suit == Suit::Diamonds);
    assert_eq!(template.rank, Rank::King);
}

#[test]
fn test_straight_flush_with_removed_two_still_recognizes_included_two() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Two),
        make_card(Suit::Hearts, Rank::Three),
        make_card(Suit::Hearts, Rank::Four),
        make_card(Suit::Hearts, Rank::Five),
        make_card(Suit::Hearts, Rank::Six),
    ];
    let upgrade_state = UpgradeState {
        removed_number_rank_count: 1,
        ..UpgradeState::default()
    };
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::StraightFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Six);
}
