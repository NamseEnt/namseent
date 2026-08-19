use crate::{
    card::{Card, Rank, Suit},
    game_state::upgrade::UpgradeState,
};
use std::collections::HashMap;

pub struct StraightResult {
    pub royal: bool,
    pub top: Card,
    pub cards: Vec<Card>,
}

pub struct FlushResult {
    pub suit: Suit,
}

pub fn flush_groups(cards: &[Card], upgrade_state: &UpgradeState) -> Vec<(Suit, Vec<Card>)> {
    let flush_card_count = match upgrade_state.shorten_straight_flush_to_4_cards() {
        true => 4,
        false => 5,
    };
    let treat_suits_as_same = upgrade_state.treat_suits_as_same();

    if cards.len() < flush_card_count {
        return Vec::new();
    }

    let mut suit_map = HashMap::<Suit, Vec<Card>>::new();
    for card in cards {
        let suit = if treat_suits_as_same {
            match card.suit {
                Suit::Clubs | Suit::Spades => Suit::Spades,
                Suit::Hearts | Suit::Diamonds => Suit::Hearts,
            }
        } else {
            card.suit
        };
        suit_map.entry(suit).or_default().push(*card);
    }

    suit_map
        .into_iter()
        .filter(|(_, cards)| cards.len() >= flush_card_count)
        .collect()
}

pub fn check_straight(cards: &[Card], upgrade_state: &UpgradeState) -> Option<StraightResult> {
    let straight_card_count = match upgrade_state.shorten_straight_flush_to_4_cards() {
        true => 4,
        false => 5,
    };
    let skip_rank_for_straight = upgrade_state.skip_rank_for_straight();

    if cards.len() < straight_card_count {
        return None;
    }

    let mut best = None;
    for ace_value in [false, true] {
        let mut cards_by_value = HashMap::<usize, Vec<&Card>>::new();
        for card in cards {
            let value = if ace_value {
                if card.rank == Rank::Ace {
                    Rank::Ace.ace_high_value()
                } else {
                    card.rank.ordinal() + 1
                }
            } else if card.rank == Rank::Ace {
                0
            } else {
                card.rank.ordinal() + 1
            };
            cards_by_value.entry(value).or_default().push(card);
        }

        let mut values = cards_by_value.keys().copied().collect::<Vec<_>>();
        values.sort_unstable();
        if values.len() < straight_card_count {
            continue;
        }

        for window in values.windows(straight_card_count) {
            let missing_count = window
                .last()
                .unwrap()
                .saturating_sub(*window.first().unwrap())
                .saturating_sub(straight_card_count - 1);
            if missing_count > usize::from(skip_rank_for_straight) {
                continue;
            }

            let selected_cards = window
                .iter()
                .map(|value| {
                    cards_by_value
                        .get(value)
                        .unwrap()
                        .iter()
                        .max_by_key(|card| **card)
                        .map(|card| **card)
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let candidate = StraightResult {
                royal: is_royal(window, straight_card_count, skip_rank_for_straight),
                top: *selected_cards.last().unwrap(),
                cards: selected_cards,
            };

            if best
                .as_ref()
                .is_none_or(|(_, best_value)| *best_value < *window.last().unwrap())
            {
                best = Some((candidate, *window.last().unwrap()));
            }
        }
    }

    fn is_royal(ranks: &[usize], straight_card_count: usize, _skip_rank: bool) -> bool {
        let ten = Rank::Ten.ordinal() + 1;
        let jack = Rank::Jack.ordinal() + 1;
        let queen = Rank::Queen.ordinal() + 1;
        let king = Rank::King.ordinal() + 1;
        let ace = Rank::Ace.ace_high_value();
        let royal_ranks = [ten, jack, queen, king, ace];

        if straight_card_count == 5 {
            return royal_ranks.iter().all(|rank| ranks.contains(rank));
        }
        straight_card_count == 4 && ranks.iter().all(|rank| royal_ranks.contains(rank))
    }

    best.map(|(result, _)| result)
}

pub fn check_flush(cards: &[Card], upgrade_state: &UpgradeState) -> Option<FlushResult> {
    flush_groups(cards, upgrade_state)
        .into_iter()
        .max_by_key(|(_, cards)| cards.iter().map(|card| card.rank.ordinal()).max())
        .map(|(suit, _)| FlushResult { suit })
}

pub fn count_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank).or_insert_with(Vec::new).push(*card);
    }
    map
}
