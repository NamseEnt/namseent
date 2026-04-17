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
    let removed_number_rank_count = upgrade_state.removed_number_rank_count;

    if cards.len() < straight_card_count {
        return None;
    }

    let mut cards_ace_as_high = cards
        .iter()
        .map(|card| {
            let rank = if card.rank == Rank::Ace {
                Rank::Ace.ace_high_value()
            } else {
                card.rank.ordinal() + 1
            };
            (rank, card)
        })
        .collect::<Vec<_>>();
    cards_ace_as_high.sort_by_key(|a| a.0);
    let ace_high_removed_low = 2;
    let ace_high_removed_high = removed_number_rank_count + 1;
    if let Some((start_idx, end_idx, _)) = check_rank(
        &cards_ace_as_high,
        straight_card_count,
        skip_rank_for_straight,
        &|value| value >= ace_high_removed_low && value <= ace_high_removed_high,
    ) {
        let straight_slice = &cards_ace_as_high[start_idx..=end_idx];
        let ranks: Vec<usize> = straight_slice.iter().map(|(r, _)| *r).collect();
        let is_royal = if straight_card_count == 5 {
            [
                Rank::Ten.ordinal() + 1,
                Rank::Jack.ordinal() + 1,
                Rank::Queen.ordinal() + 1,
                Rank::King.ordinal() + 1,
                Rank::Ace.ace_high_value(),
            ]
            .iter()
            .all(|r| ranks.contains(r))
        } else if straight_card_count == 4 {
            if skip_rank_for_straight {
                // 10-J-Q-A, J-Q-K-A, 10-Q-K-A 등 4장 royal 허용 (랭크 건너뛰기 포함)
                let has_10 = ranks.contains(&(Rank::Ten.ordinal() + 1));
                let has_j = ranks.contains(&(Rank::Jack.ordinal() + 1));
                let has_q = ranks.contains(&(Rank::Queen.ordinal() + 1));
                let has_k = ranks.contains(&(Rank::King.ordinal() + 1));
                let has_a = ranks.contains(&Rank::Ace.ace_high_value());
                has_a && has_q && (has_10 || has_j) && (has_j || has_k)
            } else {
                // 연속된 4장 royal 확인: 10-J-Q-K 또는 J-Q-K-A
                let royal_sequences = [
                    [
                        Rank::Ten.ordinal() + 1,
                        Rank::Jack.ordinal() + 1,
                        Rank::Queen.ordinal() + 1,
                        Rank::King.ordinal() + 1,
                    ],
                    [
                        Rank::Jack.ordinal() + 1,
                        Rank::Queen.ordinal() + 1,
                        Rank::King.ordinal() + 1,
                        Rank::Ace.ace_high_value(),
                    ],
                ];
                royal_sequences
                    .iter()
                    .any(|sequence| sequence.iter().all(|r| ranks.contains(r)))
            }
        } else {
            false
        };
        return Some(StraightResult {
            royal: is_royal,
            top: *straight_slice.last().unwrap().1,
        });
    }

    let mut cards_ace_as_low = cards
        .iter()
        .map(|card| {
            let rank = if card.rank == Rank::Ace {
                0
            } else {
                card.rank.ordinal() + 1
            };
            (rank, card)
        })
        .collect::<Vec<_>>();
    cards_ace_as_low.sort_by_key(|a| a.0);
    let ace_low_removed_low = 1;
    let ace_low_removed_high = removed_number_rank_count;
    if let Some((start_idx, end_idx, _)) = check_rank(
        &cards_ace_as_low,
        straight_card_count,
        skip_rank_for_straight,
        &|value| value >= ace_low_removed_low && value <= ace_low_removed_high,
    ) {
        let straight_slice = &cards_ace_as_low[start_idx..=end_idx];
        return Some(StraightResult {
            royal: false,
            top: *straight_slice.last().unwrap().1,
        });
    }

    return None;

    fn check_rank(
        cards: &[(usize, &Card)],
        straight_card_count: usize,
        skip_rank: bool,
        is_removed_rank: &dyn Fn(usize) -> bool,
    ) -> Option<(usize, usize, usize)> {
        let mut count = 1;
        let mut skips = 0;
        let mut start = 0;
        for i in 1..cards.len() {
            let prev = cards[i - 1].0;
            let curr = cards[i].0;

            if curr == prev {
                continue;
            }

            let missing_non_removed_count = (prev + 1..curr)
                .filter(|value| !is_removed_rank(*value))
                .count();

            if missing_non_removed_count == 0 {
                count += 1;
            } else if skip_rank && skips + missing_non_removed_count <= 1 {
                count += 1;
                skips += missing_non_removed_count;
            } else {
                count = 1;
                skips = 0;
                start = i;
            }

            if count == straight_card_count {
                return Some((start, i, skips));
            }
        }
        None
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
