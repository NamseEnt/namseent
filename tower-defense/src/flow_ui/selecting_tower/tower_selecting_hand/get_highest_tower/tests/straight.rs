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
    let upgrade_state = state_with(vec![UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let upgrade_state = state_with(vec![UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
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
    let upgrade_state = state_with(vec![UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade), UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Jack);
}

#[test]
fn test_straight_with_removed_two_allows_ace_low() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Three),
        make_card(Suit::Clubs, Rank::Four),
        make_card(Suit::Diamonds, Rank::Five),
        make_card(Suit::Spades, Rank::Six),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::Eraser(crate::game_state::upgrade::EraserUpgrade { add: 1  })]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Six);
}

#[test]
fn test_straight_with_removed_two_and_shorten_4cards_allows_ace_low() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Three),
        make_card(Suit::Clubs, Rank::Four),
        make_card(Suit::Diamonds, Rank::Five),
        make_card(Suit::Spades, Rank::Six),
    ];
    let upgrade_state = state_with(vec![
        UpgradeKind::Eraser(crate::game_state::upgrade::EraserUpgrade { add: 1  }),
        UpgradeKind::FourLeafClover(crate::game_state::upgrade::FourLeafCloverUpgrade),
        UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade),
    ]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Six);
}

#[test]
fn test_straight_with_removed_two_and_three_allows_ace_low() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Four),
        make_card(Suit::Clubs, Rank::Five),
        make_card(Suit::Diamonds, Rank::Six),
        make_card(Suit::Spades, Rank::Seven),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::Eraser(crate::game_state::upgrade::EraserUpgrade { add: 2  })]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Seven);
}

#[test]
fn test_straight_with_removed_two_still_recognizes_included_two() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Two),
        make_card(Suit::Hearts, Rank::Three),
        make_card(Suit::Clubs, Rank::Four),
        make_card(Suit::Diamonds, Rank::Five),
        make_card(Suit::Spades, Rank::Six),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::Eraser(crate::game_state::upgrade::EraserUpgrade { add: 1  })]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Six);
}

#[test]
fn test_straight_with_removed_two_and_skip_rank_allows_ace_four_five_six_seven() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Four),
        make_card(Suit::Clubs, Rank::Five),
        make_card(Suit::Diamonds, Rank::Six),
        make_card(Suit::Spades, Rank::Seven),
    ];
    let upgrade_state = state_with(vec![UpgradeKind::Eraser(crate::game_state::upgrade::EraserUpgrade { add: 1  }), UpgradeKind::Rabbit(crate::game_state::upgrade::RabbitUpgrade)]);
    let rerolled_count = 0;
    let template = get_highest_tower_template(
        &cards,
        &upgrade_state,
        rerolled_count,
        &crate::config::GameConfig::default_config(),
    );
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Seven);
}
