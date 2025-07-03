use super::super::get_highest_tower_template;
use crate::card::{Suit, Rank};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;
use super::make_card;

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
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Straight);
    assert_eq!(template.rank, Rank::Jack);
}
