use tower_defense::{Card, Rank, Suit, TowerKind, UpgradeState};
use tower_defense::tower_selecting_hand::get_highest_tower::get_highest_tower_template;

fn make_card(suit: Suit, rank: Rank) -> Card {
    Card { suit, rank }
}

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

#[test]
fn test_one_pair() {
    let cards = vec![
        make_card(Suit::Spades, Rank::Ace),
        make_card(Suit::Hearts, Rank::Ace),
        make_card(Suit::Clubs, Rank::Seven),
        make_card(Suit::Diamonds, Rank::Nine),
        make_card(Suit::Spades, Rank::Eight),
    ];
    let upgrade_state = UpgradeState::default();
    let rerolled_count = 0;
    let template = get_highest_tower_template(&cards, &upgrade_state, rerolled_count);
    assert_eq!(template.kind, TowerKind::OnePair);
    assert_eq!(template.rank, Rank::Ace);
}
