use super::super::get_highest_tower_template;
use crate::card::{Suit, Rank};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;
use super::make_card;

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
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::RoyalFlush);
    assert_eq!(template.suit, Suit::Hearts);
    assert_eq!(template.rank, Rank::Ace);
}
