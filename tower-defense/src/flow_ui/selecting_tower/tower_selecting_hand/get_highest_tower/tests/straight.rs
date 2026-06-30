use super::super::get_highest_tower_template;
use super::make_card;
use crate::game_state::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::{Upgrade, UpgradeState};

fn state_with(upgrades: Vec<Upgrade>) -> UpgradeState {
    UpgradeState::with_upgrades(upgrades)
}

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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank.unwrap(), Rank::Jack);
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
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let upgrade_state = state_with(vec![
        crate::game_state::upgrade::FourLeafCloverUpgrade::into_upgrade(),
    ]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank.unwrap(), Rank::Ten);
}

#[test]
fn test_straight_skip_rank() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Nine),
        make_card(Suit::Diamonds, Rank::Jack),
    ];
    let upgrade_state = state_with(vec![
        crate::game_state::upgrade::RabbitUpgrade::into_upgrade(),
    ]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::High);
    assert_eq!(template.rank.unwrap(), Rank::Jack);
}

#[test]
fn test_straight_skip_rank_and_shorten_4cards() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Seven),
        make_card(Suit::Hearts, Rank::Eight),
        make_card(Suit::Clubs, Rank::Jack),
        make_card(Suit::Diamonds, Rank::Nine),
    ];
    let upgrade_state = state_with(vec![
        crate::game_state::upgrade::RabbitUpgrade::into_upgrade(),
        crate::game_state::upgrade::FourLeafCloverUpgrade::into_upgrade(),
    ]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank.unwrap(), Rank::Jack);
}
