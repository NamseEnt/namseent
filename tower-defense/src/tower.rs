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

#[derive(Debug, Clone)]
pub struct TowerBlueprint {
    pub kind: TowerKind,
    pub suit: Option<Suit>,
    pub rank: Option<Rank>,
    pub effects: Vec<TowerEffectBlueprint>,
}
impl TowerBlueprint {
    pub fn calculate_damage(&self) -> usize {
        let mut damage = match self.kind {
            TowerKind::High => 5,
            TowerKind::OnePair => 25,
            TowerKind::TwoPair => 50,
            TowerKind::ThreeOfAKind => 125,
            TowerKind::Straight => 250,
            TowerKind::Flush => 375,
            TowerKind::FullHouse => 1000,
            TowerKind::FourOfAKind => 1250,
            TowerKind::StraightFlush => 7500,
            TowerKind::RoyalFlush => 15000,
        } as f32;
        for effect in &self.effects {
            match effect {
                TowerEffectBlueprint::TopCardBonus { bonus_damage, .. } => {
                    damage += *bonus_damage as f32;
                }
                _ => {}
            }
        }
        damage as usize
    }
}

pub fn get_highest_tower(cards: &[Card]) -> TowerBlueprint {
    let mut highest_tower = highest_tower(cards);
    inject_effects(&mut highest_tower);
    highest_tower
}

fn highest_tower(cards: &[Card]) -> TowerBlueprint {
    let straight_result = check_straight(&cards);
    let flush_result = check_flush(&cards);

    if let (Some(straight_result), Some(flush_result)) = (&straight_result, &flush_result) {
        if straight_result.royal {
            return TowerBlueprint {
                kind: TowerKind::RoyalFlush,
                suit: Some(flush_result.suit),
                rank: Some(Rank::Ace),
                effects: vec![],
            };
        }
        return TowerBlueprint {
            kind: TowerKind::StraightFlush,
            suit: Some(flush_result.suit),
            rank: Some(straight_result.top.rank),
            effects: vec![],
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
                effects: vec![],
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
            effects: vec![],
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
            effects: vec![],
        };
    }

    if let Some(straight_result) = straight_result {
        return TowerBlueprint {
            kind: TowerKind::Straight,
            suit: Some(straight_result.top.suit),
            rank: Some(straight_result.top.rank),
            effects: vec![],
        };
    }

    if let Some(mut triple_cards) = triple_cards {
        triple_cards.sort();
        let top = triple_cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::ThreeOfAKind,
            suit: Some(top.suit),
            rank: Some(top.rank),
            effects: vec![],
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
            effects: vec![],
        };
    }

    if let Some(mut cards) = pair_high_cards {
        cards.sort();
        let top = cards.last().unwrap();
        return TowerBlueprint {
            kind: TowerKind::OnePair,
            suit: Some(top.suit),
            rank: Some(top.rank),
            effects: vec![],
        };
    }

    let mut cards = cards.to_vec();
    cards.sort();
    let top = cards.last().unwrap();

    TowerBlueprint {
        kind: TowerKind::High,
        suit: Some(top.suit),
        rank: Some(top.rank),
        effects: vec![],
    }
}

fn inject_effects(tower: &mut TowerBlueprint) {
    let hand_ranking_effect = match tower.kind {
        TowerKind::High => None,
        TowerKind::OnePair => Some(TowerEffectBlueprint::Bounty { bonus_gold: 1 }),
        TowerKind::TwoPair => Some(TowerEffectBlueprint::Bounty { bonus_gold: 2 }),
        TowerKind::ThreeOfAKind => Some(TowerEffectBlueprint::Drag {
            range: 4,
            drag: 0.9,
        }),
        TowerKind::Straight => None,
        TowerKind::Flush => None,
        TowerKind::FullHouse => Some(TowerEffectBlueprint::Haste {
            range: 2,
            haste: 2.0,
        }),
        TowerKind::FourOfAKind => Some(TowerEffectBlueprint::Drag {
            range: 4,
            drag: 0.75,
        }),
        TowerKind::StraightFlush => None,
        TowerKind::RoyalFlush => Some(TowerEffectBlueprint::Empower {
            range: 6,
            empower: 2.0,
        }),
    };
    if let Some(effect) = hand_ranking_effect {
        tower.effects.push(effect);
    }

    if let Some(rank) = tower.rank {
        let top_card_effect = TowerEffectBlueprint::top_card_bonus(rank);
        tower.effects.push(top_card_effect);
    }

    // TODO: Inject effects from upgrades
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TowerEffectBlueprint {
    TopCardBonus { rank: Rank, bonus_damage: usize },
    Bounty { bonus_gold: usize },
    Drag { range: usize, drag: f32 },
    Haste { range: usize, haste: f32 },
    Empower { range: usize, empower: f32 },
}
impl TowerEffectBlueprint {
    fn top_card_bonus(rank: Rank) -> Self {
        let bonus_damage = match rank {
            Rank::Seven => 1,
            Rank::Eight => 2,
            Rank::Nine => 3,
            Rank::Ten => 4,
            Rank::Jack => 6,
            Rank::Queen => 8,
            Rank::King => 10,
            Rank::Ace => 15,
        };
        TowerEffectBlueprint::TopCardBonus { rank, bonus_damage }
    }
}
