use crate::{
    card::{Rank, Suit, REVERSED_RANKS, SUITS},
    game_state::{tower::TowerKind, GameState},
    rarity::Rarity,
};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub enum QuestRequirement {
    OwnTowerRank {
        rank: Rank,
        count: usize,
    },
    OwnTowerSuit {
        suit: Suit,
        count: usize,
    },
    OwnTowerHand {
        kind: TowerKind,
        count: usize,
    },
    BuildTowerRank {
        rank: Rank,
        count: usize,
        offset: usize,
    },
    BuildTowerSuit {
        suit: Suit,
        count: usize,
        offset: usize,
    },
    BuildTowerHand {
        kind: TowerKind,
        count: usize,
        offset: usize,
    },
}
impl QuestRequirement {
    pub fn description(&self, game_state: &GameState) -> String {
        match self {
            QuestRequirement::OwnTowerRank { rank, count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.rank == *rank)
                    .count();
                format!(
                    "{}타워를 {}개 소유하세요. ({}/{})",
                    count, rank, current_count, count
                )
            }
            QuestRequirement::OwnTowerSuit { suit, count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.suit == *suit)
                    .count();
                format!(
                    "{}타워를 {}개 소유하세요. ({}/{})",
                    count, suit, current_count, count
                )
            }
            QuestRequirement::OwnTowerHand { kind, count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.kind == *kind)
                    .count();
                format!(
                    "{}타워를 {}개 소유하세요. ({}/{})",
                    kind, count, current_count, count
                )
            }
            QuestRequirement::BuildTowerRank { rank, count, .. } => {
                format!("{}타워를 {}개 건설하세요.", rank, count)
            }
            QuestRequirement::BuildTowerSuit { suit, count, .. } => {
                format!("{}타워를 {}개 건설하세요.", suit, count)
            }
            QuestRequirement::BuildTowerHand { kind, count, .. } => {
                format!("{}타워를 {}개 건설하세요.", kind, count)
            }
        }
    }
}
pub(super) fn generate_quest_requirement(
    game_state: &GameState,
    rarity: Rarity,
) -> QuestRequirement {
    match thread_rng().gen_range(0..6) {
        0 => {
            let rank = REVERSED_RANKS.choose(&mut thread_rng()).unwrap().clone();
            let offset = game_state
                .towers
                .iter()
                .filter(|tower| tower.rank == rank)
                .count();
            QuestRequirement::BuildTowerRank {
                rank,
                count: match rarity {
                    Rarity::Common => 1,
                    Rarity::Rare => 2,
                    Rarity::Epic => 3,
                    Rarity::Legendary => 4,
                },
                offset,
            }
        }
        1 => QuestRequirement::OwnTowerRank {
            rank: REVERSED_RANKS.choose(&mut thread_rng()).unwrap().clone(),
            count: thread_rng().gen_range(match rarity {
                Rarity::Common => 3..=4,
                Rarity::Rare => 5..=6,
                Rarity::Epic => 7..=8,
                Rarity::Legendary => 9..=10,
            }),
        },
        2 => {
            let suit = SUITS.choose(&mut thread_rng()).unwrap().clone();
            let offset = game_state
                .towers
                .iter()
                .filter(|tower| tower.suit == suit)
                .count();
            QuestRequirement::BuildTowerSuit {
                suit,
                count: thread_rng().gen_range(match rarity {
                    Rarity::Common => 3..=4,
                    Rarity::Rare => 5..=6,
                    Rarity::Epic => 7..=8,
                    Rarity::Legendary => 9..=10,
                }),
                offset,
            }
        }
        3 => QuestRequirement::OwnTowerSuit {
            suit: SUITS.choose(&mut thread_rng()).unwrap().clone(),
            count: thread_rng().gen_range(match rarity {
                Rarity::Common => 5..=8,
                Rarity::Rare => 9..=11,
                Rarity::Epic => 12..=16,
                Rarity::Legendary => 17..=20,
            }),
        },
        4 => {
            let kind = get_random_quest_requirement_target_kind(rarity);
            let offset = game_state
                .towers
                .iter()
                .filter(|tower| tower.kind == kind)
                .count();
            QuestRequirement::BuildTowerHand {
                kind,
                count: match rarity {
                    Rarity::Common => match kind {
                        TowerKind::High => 3,
                        TowerKind::OnePair => 2,
                        TowerKind::TwoPair => 2,
                        TowerKind::ThreeOfAKind => 1,
                        _ => 0,
                    },
                    Rarity::Rare => match kind {
                        TowerKind::TwoPair => 3,
                        TowerKind::ThreeOfAKind => 2,
                        TowerKind::Straight => 2,
                        TowerKind::Flush => 1,
                        TowerKind::FullHouse => 1,
                        _ => 0,
                    },
                    Rarity::Epic => match kind {
                        TowerKind::ThreeOfAKind => 3,
                        TowerKind::Straight => 3,
                        TowerKind::Flush => 2,
                        TowerKind::FullHouse => 2,
                        TowerKind::FourOfAKind => 2,
                        _ => 0,
                    },
                    Rarity::Legendary => match kind {
                        TowerKind::Straight => 4,
                        TowerKind::Flush => 3,
                        TowerKind::FullHouse => 3,
                        TowerKind::FourOfAKind => 3,
                        TowerKind::StraightFlush => 2,
                        TowerKind::RoyalFlush => 1,
                        _ => 0,
                    },
                },
                offset,
            }
        }
        5 => {
            let kind = get_random_quest_requirement_target_kind(rarity);
            QuestRequirement::OwnTowerHand {
                kind,
                count: match rarity {
                    Rarity::Common => match kind {
                        TowerKind::High => 6,
                        TowerKind::OnePair => 6,
                        TowerKind::TwoPair => 4,
                        TowerKind::ThreeOfAKind => 3,
                        _ => 0,
                    },
                    Rarity::Rare => match kind {
                        TowerKind::TwoPair => 6,
                        TowerKind::ThreeOfAKind => 5,
                        TowerKind::Straight => 4,
                        TowerKind::Flush => 5,
                        TowerKind::FullHouse => 5,
                        _ => 0,
                    },
                    Rarity::Epic => match kind {
                        TowerKind::ThreeOfAKind => 7,
                        TowerKind::Straight => 6,
                        TowerKind::Flush => 6,
                        TowerKind::FullHouse => 6,
                        TowerKind::FourOfAKind => 5,
                        _ => 0,
                    },
                    Rarity::Legendary => match kind {
                        TowerKind::Straight => 8,
                        TowerKind::Flush => 7,
                        TowerKind::FullHouse => 7,
                        TowerKind::FourOfAKind => 7,
                        TowerKind::StraightFlush => 3,
                        TowerKind::RoyalFlush => 2,
                        _ => 0,
                    },
                },
            }
        }
        _ => panic!("Invalid QuestRequirement"),
    }
}
fn get_random_quest_requirement_target_kind(rarity: Rarity) -> TowerKind {
    let kind_weights = match rarity {
        Rarity::Common => [
            (TowerKind::High, 0.5),
            (TowerKind::OnePair, 0.1),
            (TowerKind::TwoPair, 0.05),
            (TowerKind::ThreeOfAKind, 0.02),
            (TowerKind::Straight, 0.0),
            (TowerKind::Flush, 0.0),
            (TowerKind::FullHouse, 0.0),
            (TowerKind::FourOfAKind, 0.0),
            (TowerKind::StraightFlush, 0.0),
            (TowerKind::RoyalFlush, 0.0),
        ],
        Rarity::Rare => [
            (TowerKind::High, 0.0),
            (TowerKind::OnePair, 0.0),
            (TowerKind::TwoPair, 0.05),
            (TowerKind::ThreeOfAKind, 0.02),
            (TowerKind::Straight, 0.005),
            (TowerKind::Flush, 0.003),
            (TowerKind::FullHouse, 0.0025),
            (TowerKind::FourOfAKind, 0.0),
            (TowerKind::StraightFlush, 0.0),
            (TowerKind::RoyalFlush, 0.0),
        ],
        Rarity::Epic => [
            (TowerKind::High, 0.0),
            (TowerKind::OnePair, 0.0),
            (TowerKind::TwoPair, 0.0),
            (TowerKind::ThreeOfAKind, 0.02),
            (TowerKind::Straight, 0.005),
            (TowerKind::Flush, 0.003),
            (TowerKind::FullHouse, 0.0025),
            (TowerKind::FourOfAKind, 0.002),
            (TowerKind::StraightFlush, 0.0),
            (TowerKind::RoyalFlush, 0.0),
        ],
        Rarity::Legendary => [
            (TowerKind::High, 0.0),
            (TowerKind::OnePair, 0.0),
            (TowerKind::TwoPair, 0.0),
            (TowerKind::ThreeOfAKind, 0.0),
            (TowerKind::Straight, 0.005),
            (TowerKind::Flush, 0.003),
            (TowerKind::FullHouse, 0.0025),
            (TowerKind::FourOfAKind, 0.002),
            (TowerKind::StraightFlush, 0.00001),
            (TowerKind::RoyalFlush, 0.0001),
        ],
    };
    kind_weights
        .choose_weighted(&mut thread_rng(), |x| x.1)
        .unwrap()
        .0
}
