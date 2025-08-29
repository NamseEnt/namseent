use super::super::get_highest_tower_template;
use super::make_card;
use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;

#[test]
fn test_high_card() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Ten),
        make_card(Suit::Clubs, Rank::Seven),
        make_card(Suit::Diamonds, Rank::Nine),
        make_card(Suit::Spades, Rank::Eight),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::High);
    assert_eq!(template.rank, Rank::Ace);
}
