use super::super::get_highest_tower_template;
use crate::card::{Suit, Rank};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;
use super::make_card;

#[test]
fn test_three_of_a_kind() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Ace),
        make_card(Suit::Clubs, Rank::Ace),
        make_card(Suit::Diamonds, Rank::Ten),
        make_card(Suit::Spades, Rank::Seven),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::ThreeOfAKind);
    assert_eq!(template.rank, Rank::Ace);
}
