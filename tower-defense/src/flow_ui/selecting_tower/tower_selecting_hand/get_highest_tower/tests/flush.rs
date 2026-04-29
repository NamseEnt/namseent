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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let upgrade_state = state_with(vec![UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade)]);

    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let upgrade_state = state_with(vec![UpgradeKind::BlackWhite(crate::game_state::upgrade::BlackWhiteUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let upgrade_state = state_with(vec![
        UpgradeKind::BlackWhite(crate::game_state::upgrade::BlackWhiteUpgrade),
        UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade),
    ]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Flush);
    assert!(template.suit == Suit::Spades || template.suit == Suit::Clubs);
    assert_eq!(template.rank, Rank::Jack);
}
