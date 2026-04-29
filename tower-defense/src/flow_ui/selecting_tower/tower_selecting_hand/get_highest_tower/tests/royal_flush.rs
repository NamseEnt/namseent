use super::super::get_highest_tower_template;
use super::make_card;
use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::{Upgrade, UpgradeKind, UpgradeState};

fn state_with(kinds: Vec<UpgradeKind>) -> UpgradeState {
    UpgradeState {
        upgrades: kinds
            .into_iter()
            .map(|kind| Upgrade {
                kind,
                value: crate::OneZero::default(),
            })
            .collect(),
        ..UpgradeState::default()
    }
}

#[test]
fn test_royal_flush() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Ace);
}

#[test]
fn test_royal_flush_4cards_without_upgrade() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_ne!(template.kind, TowerKind::RoyalFlush);
}

#[test]
fn test_royal_flush_4cards_with_upgrade() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Ace);
}

#[test]
fn test_royal_flush_skip_rank() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade)]);
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
fn test_royal_flush_skip_rank_and_shorten_4cards() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Hearts, Rank::Ace),
        make_card(Suit::Hearts, Rank::Jack),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade), UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Ace);
}

#[test]
fn test_royal_flush_treat_suits_as_same() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Diamonds, Rank::Jack),
        make_card(Suit::Hearts, Rank::Queen),
        make_card(Suit::Diamonds, Rank::King),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::BlackWhite(crate::game_state::upgrade::BlackWhiteUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert!(template.suit == Suit::Hearts || template.suit == Suit::Diamonds);
    assert_eq!(template.rank, Rank::Ace);
}

#[test]
fn test_royal_flush_treat_suits_as_same_and_shorten_4cards() {
    let cards = vec![
        make_card(Suit::Diamonds, Rank::Jack),
        make_card(Suit::Diamonds, Rank::Queen),
        make_card(Suit::Hearts, Rank::King),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::BlackWhite(crate::game_state::upgrade::BlackWhiteUpgrade), UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert!(template.suit == Suit::Hearts || template.suit == Suit::Diamonds);
    assert_eq!(template.rank, Rank::Ace);
}

#[test]
fn test_royal_flush_treat_suits_as_same_and_shorten_4cards_and_skip_rank_for_straight() {
    let cards = vec![
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Diamonds, Rank::Jack),
        make_card(Suit::Diamonds, Rank::Queen),
        make_card(Suit::Hearts, Rank::Ace),
    ];
    let upgrade_state =
        state_with(vec![UpgradeKind::BlackWhite(crate::game_state::upgrade::BlackWhiteUpgrade), UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade), UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert!(template.suit == Suit::Hearts || template.suit == Suit::Diamonds);
    assert_eq!(template.rank, Rank::Ace);
}
