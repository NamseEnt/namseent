use crate::card::{Card, Rank, Suit, REVERSED_RANKS};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerKind {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

pub struct TowerBlueprint {
    pub kind: TowerKind,
    pub suit: Option<Suit>,
    pub rank: Option<Rank>,
}

pub fn get_highest_tower(cards: &[Card]) -> TowerBlueprint {
    let straight_result = check_straight(&cards);
    let flush_result = check_flush(&cards);

    if let (Some(straight_result), Some(flush_result)) = (&straight_result, &flush_result) {
        if straight_result.royal {
            return TowerBlueprint {
                kind: TowerKind::RoyalFlush,
                suit: Some(flush_result.suit),
                rank: Some(Rank::Ace),
            };
        }
        return TowerBlueprint {
            kind: TowerKind::StraightFlush,
            suit: Some(flush_result.suit),
            rank: Some(straight_result.top.rank),
        };
    }

    let rank_map = count_rank(&cards);
    let mut triple_cards = None;
    let mut pair_high_cards = None;
    let mut pair_low_cards = None;

    for rank in REVERSED_RANKS {
        let Some(cards) = rank_map.get(&rank) else {
            continue;
        };
        if cards.len() == 4 {
            let mut cards = cards.clone();
            cards.sort();
            let top = cards.last().unwrap();
            return TowerBlueprint {
                kind: TowerKind::FourOfAKind,
                suit: Some(top.suit),
                rank: Some(top.rank),
            };
        }

        if cards.len() == 3 && triple_cards.is_none() {
            triple_cards = Some(cards.clone());
        } else if cards.len() == 2 {
            if pair_high_cards.is_none() {
                pair_high_cards = Some(cards.clone());
            } else if pair_low_cards.is_none() {
                pair_low_cards = Some(cards.clone());
            }
        }
    }

    if let (Some(triple_cards), Some(pair_high_cards)) = (&triple_cards, &pair_high_cards) {
        let mut cards = triple_cards
            .into_iter()
            .chain(pair_high_cards)
            .collect::<Vec<_>>();
        cards.sort();
        let top = cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::FullHouse,
            suit: Some(top.suit),
            rank: Some(top.rank),
        };
    }

    if let Some(flush_result) = flush_result {
        let mut cards = cards.to_vec();
        cards.sort();
        let top = cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::Flush,
            suit: Some(flush_result.suit),
            rank: Some(top.rank),
        };
    }

    if let Some(straight_result) = straight_result {
        return TowerBlueprint {
            kind: TowerKind::Straight,
            suit: Some(straight_result.top.suit),
            rank: Some(straight_result.top.rank),
        };
    }

    if let Some(mut triple_cards) = triple_cards {
        triple_cards.sort();
        let top = triple_cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::ThreeOfAKind,
            suit: Some(top.suit),
            rank: Some(top.rank),
        };
    }

    if let (Some(pair_high_cards), Some(pair_low_cards)) = (&pair_high_cards, &pair_low_cards) {
        let mut cards = pair_high_cards
            .into_iter()
            .chain(pair_low_cards)
            .collect::<Vec<_>>();
        cards.sort();
        let top = cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::TwoPair,
            suit: Some(top.suit),
            rank: Some(top.rank),
        };
    }

    if let Some(mut cards) = pair_high_cards {
        cards.sort();
        let top = cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::OnePair,
            suit: Some(top.suit),
            rank: Some(top.rank),
        };
    }

    let mut cards = cards.to_vec();
    cards.sort();
    let top = cards.last().unwrap();

    TowerBlueprint {
        kind: TowerKind::High,
        suit: Some(top.suit),
        rank: Some(top.rank),
    }
}

struct StraightResult {
    royal: bool,
    top: Card,
}
fn check_straight(cards: &[Card]) -> Option<StraightResult> {
    if cards.len() != 5 {
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
    let straight = check_rank(&cards_ace_as_high);
    if straight {
        return Some(StraightResult {
            royal: true,
            top: *cards_ace_as_high.last().unwrap().1,
        });
    }

    let mut cards_ace_as_low = cards
        .iter()
        .map(|card| (card.rank as usize, card))
        .collect::<Vec<_>>();
    cards_ace_as_low.sort_by(|a, b| a.0.cmp(&b.0));
    let straight = check_rank(&cards_ace_as_low);
    if straight {
        return Some(StraightResult {
            royal: false,
            top: *cards_ace_as_low.last().unwrap().1,
        });
    }

    return None;

    fn check_rank(cards: &[(usize, &Card)]) -> bool {
        let mut prev = cards[0];
        for (rank, card) in cards.iter().skip(1) {
            if *rank != prev.0 + 1 {
                return false;
            }
            prev = (*rank, card);
        }
        true
    }
}

struct FlushResult {
    suit: Suit,
}
fn check_flush(cards: &[Card]) -> Option<FlushResult> {
    if cards.len() != 5 {
        return None;
    }
    let suit = cards[0].suit;
    for card in cards.iter().skip(1) {
        if card.suit != suit {
            return None;
        }
    }
    Some(FlushResult { suit })
}

fn count_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank).or_insert_with(Vec::new).push(*card);
    }
    map
}
