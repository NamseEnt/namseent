use crate::card::{Card, Rank, Suit};
use crate::game_state::upgrade::UpgradeState;
use std::collections::HashMap;

pub struct StraightResult {
    pub royal: bool,
    pub top: Card,
}

pub struct FlushResult {
    pub suit: Suit,
}

pub fn check_straight(cards: &[Card], upgrade_state: &UpgradeState) -> Option<StraightResult> {
    let straight_card_count = match upgrade_state.shorten_straight_flush_to_4_cards {
        true => 4,
        false => 5,
    };
    let skip_rank_for_straight = upgrade_state.skip_rank_for_straight;

    if cards.len() < straight_card_count {
        return None;
    }

    let mut cards_ace_as_high = cards
        .iter()
        .map(|card| {
            let mut rank = card.rank as usize;
            if rank == 0 {
                rank = Rank::King as usize + 1;
            }
            (rank, card)
        })
        .collect::<Vec<_>>();
    cards_ace_as_high.sort_by(|a, b| a.0.cmp(&b.0));
    let straight = check_rank(
        &cards_ace_as_high,
        straight_card_count,
        skip_rank_for_straight,
    );
    if straight {
        return Some(StraightResult {
            royal: cards_ace_as_high
                .iter()
                .any(|(rank, _)| *rank == Rank::Ace as usize),
            top: *cards_ace_as_high.last().unwrap().1,
        });
    }

    let mut cards_ace_as_low = cards
        .iter()
        .map(|card| (card.rank as usize, card))
        .collect::<Vec<_>>();
    cards_ace_as_low.sort_by(|a, b| a.0.cmp(&b.0));
    let straight = check_rank(
        &cards_ace_as_low,
        straight_card_count,
        skip_rank_for_straight,
    );
    if straight {
        return Some(StraightResult {
            royal: false,
            top: *cards_ace_as_low.last().unwrap().1,
        });
    }

    return None;

    fn check_rank(cards: &[(usize, &Card)], straight_card_count: usize, skip_rank: bool) -> bool {
        let mut count = 1;
        let mut skips = 0;
        for i in 1..cards.len() {
            if cards[i].0 == cards[i - 1].0 + 1 {
                count += 1;
            } else if skip_rank && cards[i].0 == cards[i - 1].0 + 2 && skips == 0 {
                count += 1;
                skips += 1;
            } else {
                count = 1;
                skips = 0;
            }
            if count == straight_card_count {
                return true;
            }
        }
        false
    }
}

pub fn check_flush(cards: &[Card], upgrade_state: &UpgradeState) -> Option<FlushResult> {
    let flush_card_count = match upgrade_state.shorten_straight_flush_to_4_cards {
        true => 4,
        false => 5,
    };
    let treat_suits_as_same = upgrade_state.treat_suits_as_same;

    if cards.len() < flush_card_count {
        return None;
    }

    let mut suit_map = HashMap::new();
    for card in cards {
        let suit = if treat_suits_as_same {
            match card.suit {
                Suit::Clubs | Suit::Spades => Suit::Spades,
                Suit::Hearts | Suit::Diamonds => Suit::Hearts,
            }
        } else {
            card.suit
        };
        suit_map.entry(suit).or_insert_with(Vec::new).push(card);
    }
    for (suit, cards) in suit_map {
        if cards.len() >= flush_card_count {
            return Some(FlushResult { suit });
        }
    }
    None
}

pub fn count_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank).or_insert_with(Vec::new).push(*card);
    }
    map
}
