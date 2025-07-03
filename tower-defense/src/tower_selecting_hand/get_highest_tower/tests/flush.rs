use super::super::get_highest_tower_template;
use crate::card::{Suit, Rank};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;
use super::make_card;

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
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::Flush);
    assert_eq!(template.suit, Suit::Spades);
    assert_eq!(template.rank, Rank::Queen);
}
