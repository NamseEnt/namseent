use super::{
    item::{generate_item, Item},
    tower::TowerKind,
    GameState,
};
use crate::{
    card::{Rank, Suit, REVERSED_RANKS, SUITS},
    rarity::Rarity,
    upgrade::{generate_upgrade, Upgrade},
};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Quest {
    requirement: QuestRequirement,
    reward: QuestReward,
}

pub fn generate_quests(game_state: &GameState, amount: usize) -> Vec<Quest> {
    let rarity_table = generate_rarity_table(game_state.stage);
    let rarities = {
        let mut rarities = Vec::with_capacity(amount);
        for _ in 0..amount {
            let rarity = &rarity_table
                .choose_weighted(&mut rand::thread_rng(), |x| x.1)
                .unwrap()
                .0;
            rarities.push(*rarity);
        }
        rarities
    };

    let mut items = Vec::with_capacity(rarities.len());
    for rarity in rarities {
        let item = generate_quest(game_state, rarity);
        items.push(item);
    }
    items
}
fn generate_quest(game_state: &GameState, rarity: Rarity) -> Quest {
    let requirement = generate_quest_requirement(game_state, rarity);
    let reward = generate_quest_reward(game_state, rarity);
    Quest {
        requirement,
        reward,
    }
}
fn generate_rarity_table(stage: usize) -> Vec<(Rarity, f32)> {
    let rarity_weight = match stage {
        1..=4 => [0.9, 0.1, 0.0, 0.0],
        5..=9 => [0.75, 0.25, 0.0, 0.0],
        10..=14 => [0.55, 0.3, 0.15, 0.0],
        15..=19 => [0.45, 0.33, 0.2, 0.02],
        20..=24 => [0.25, 0.4, 0.3, 0.05],
        25..=29 => [0.19, 0.3, 0.35, 0.15],
        30..=34 => [0.16, 0.2, 0.35, 0.25],
        35..=39 => [0.09, 0.15, 0.3, 0.3],
        40..=50 => [0.05, 0.1, 0.3, 0.4],
        _ => panic!("Invalid stage: {}", stage),
    };
    let rarity_table = vec![
        (Rarity::Common, rarity_weight[0]),
        (Rarity::Rare, rarity_weight[1]),
        (Rarity::Epic, rarity_weight[2]),
        (Rarity::Legendary, rarity_weight[3]),
    ];
    rarity_table
}

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
fn generate_quest_requirement(game_state: &GameState, rarity: Rarity) -> QuestRequirement {
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

#[derive(Debug, Clone)]
enum QuestReward {
    Money { amount: usize },
    Item { item: Item },
    Upgrade { upgrade: Upgrade },
}
fn generate_quest_reward(game_state: &GameState, rarity: Rarity) -> QuestReward {
    match [(0, 0.2), (1, 0.3), (2, 0.5)]
        .choose_weighted(&mut thread_rng(), |x| x.1)
        .unwrap()
        .0
    {
        0 => QuestReward::Money {
            amount: thread_rng().gen_range(match rarity {
                Rarity::Common => 10..25,
                Rarity::Rare => 25..50,
                Rarity::Epic => 50..100,
                Rarity::Legendary => 100..500,
            }),
        },
        1 => QuestReward::Item {
            item: generate_item(rarity),
        },
        2 => QuestReward::Upgrade {
            upgrade: generate_upgrade(game_state, rarity),
        },
        _ => panic!("Invalid QuestReward"),
    }
}
