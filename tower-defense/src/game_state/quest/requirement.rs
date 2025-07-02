use crate::{
    card::{Rank, Suit, random_rank, random_suit},
    game_state::{GameState, tower::TowerKind},
    rarity::Rarity,
};
use rand::{Rng, seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy)]
pub enum QuestRequirement {
    BuildTowerRankNew { rank: Rank, count: usize },
    BuildTowerRank { rank: Rank, count: usize },
    BuildTowerSuitNew { suit: Suit, count: usize },
    BuildTowerSuit { suit: Suit, count: usize },
    BuildTowerHandNew { hand: TowerKind, count: usize },
    BuildTowerHand { hand: TowerKind, count: usize },
    ClearBossRoundWithoutItems,
    DealDamageWithItems { damage: usize },
    BuildTowersWithoutReroll { count: usize },
    UseReroll { count: usize },
    SpendGold { gold: usize },
    EarnGold { gold: usize },
}

impl QuestRequirement {
    pub fn description(self, game_state: &GameState) -> String {
        match self {
            QuestRequirement::BuildTowerRankNew { rank, count } => {
                format!("{rank}타워를 {count}개 새로 건설하세요.")
            }
            QuestRequirement::BuildTowerHand { hand, count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.kind == hand)
                    .count();
                format!(
                    "{hand}타워를 {count}개 소유하세요. ({current_count}/{count})"
                )
            }
            QuestRequirement::BuildTowerHandNew { hand, count } => {
                format!("{hand}타워를 {count}개 새로 건설하세요.")
            }
            QuestRequirement::BuildTowerRank { rank, count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.rank == rank)
                    .count();
                format!(
                    "{rank}타워를 {count}개 소유하세요. ({current_count}/{count})"
                )
            }
            QuestRequirement::BuildTowerSuitNew { suit, count } => {
                format!("{suit}타워를 {count}개 새로 건설하세요.")
            }
            QuestRequirement::BuildTowerSuit { suit, count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.suit == suit)
                    .count();
                format!(
                    "{suit}타워를 {count}개 소유하세요. ({current_count}/{count})"
                )
            }
            QuestRequirement::ClearBossRoundWithoutItems => {
                "아이템을 사용하지않고 보스라운드 클리어".to_string()
            }
            QuestRequirement::DealDamageWithItems { damage } => {
                format!("아이템을 사용해 {damage}피해 입히기")
            }
            QuestRequirement::BuildTowersWithoutReroll { count } => {
                format!("리롤하지않고 타워 {count}개 만들기")
            }
            QuestRequirement::UseReroll { count } => {
                format!("리롤 {count}회 사용하기")
            }
            QuestRequirement::SpendGold { gold } => {
                format!("{gold}골드 사용하기")
            }
            QuestRequirement::EarnGold { gold } => {
                format!("{gold}골드 획득하기")
            }
        }
    }
}
pub(super) fn generate_quest_requirement(rarity: Rarity) -> QuestRequirement {
    match thread_rng().gen_range(0..12) {
        0 => QuestRequirement::BuildTowerRankNew {
            rank: random_rank(),
            count: match rarity {
                Rarity::Common => 1,
                Rarity::Rare => 1,
                Rarity::Epic => 2,
                Rarity::Legendary => 3,
            },
        },
        1 => QuestRequirement::BuildTowerRank {
            rank: random_rank(),
            count: thread_rng().gen_range(match rarity {
                Rarity::Common => 1..=2,
                Rarity::Rare => 1..=3,
                Rarity::Epic => 2..=4,
                Rarity::Legendary => 3..=5,
            }),
        },
        2 => QuestRequirement::BuildTowerSuitNew {
            suit: random_suit(),
            count: thread_rng().gen_range(match rarity {
                Rarity::Common => 1..=3,
                Rarity::Rare => 1..=4,
                Rarity::Epic => 2..=4,
                Rarity::Legendary => 2..=5,
            }),
        },
        3 => QuestRequirement::BuildTowerSuit {
            suit: random_suit(),
            count: thread_rng().gen_range(match rarity {
                Rarity::Common => 1..=4,
                Rarity::Rare => 2..=6,
                Rarity::Epic => 4..=8,
                Rarity::Legendary => 5..=10,
            }),
        },
        4 => {
            const TABLE: [[usize; 10]; 4] = [
                // High OnePair TwoPair ThreeOfAKind Straight Flush FullHouse FourOfAKind StraightFlush RoyalFlush
                [2, 2, 2, 1, 0, 0, 0, 0, 0, 0], // Common
                [0, 0, 2, 1, 1, 1, 1, 0, 0, 0], // Rare
                [0, 0, 0, 2, 1, 1, 1, 1, 0, 0], // Epic
                [0, 0, 0, 0, 1, 1, 1, 1, 1, 1], // Legendary
            ];
            let hand = get_random_quest_requirement_target_kind(rarity);
            QuestRequirement::BuildTowerHandNew {
                hand,
                count: TABLE[rarity as usize][hand as usize - 1],
            }
        }
        5 => {
            const TABLE: [[usize; 10]; 4] = [
                // High OnePair TwoPair ThreeOfAKind Straight Flush FullHouse FourOfAKind StraightFlush RoyalFlush
                [3, 3, 3, 1, 0, 0, 0, 0, 0, 0], // Common
                [0, 0, 4, 3, 2, 2, 1, 0, 0, 0], // Rare
                [0, 0, 0, 4, 3, 2, 1, 1, 0, 0], // Epic
                [0, 0, 0, 0, 4, 3, 3, 2, 1, 1], // Legendary
            ];
            let hand = get_random_quest_requirement_target_kind(rarity);
            QuestRequirement::BuildTowerHand {
                hand: get_random_quest_requirement_target_kind(rarity),
                count: TABLE[rarity as usize][hand as usize - 1],
            }
        }
        6 => QuestRequirement::ClearBossRoundWithoutItems,
        7 => QuestRequirement::DealDamageWithItems {
            damage: thread_rng().gen_range(match rarity {
                Rarity::Common => 100..=150,
                Rarity::Rare => 300..=450,
                Rarity::Epic => 1000..=1250,
                Rarity::Legendary => 3000..=3750,
            }),
        },
        8 => QuestRequirement::BuildTowersWithoutReroll {
            count: match rarity {
                Rarity::Common => 1,
                Rarity::Rare => 2,
                Rarity::Epic => 3,
                Rarity::Legendary => 4,
            },
        },
        9 => QuestRequirement::UseReroll {
            count: thread_rng().gen_range(match rarity {
                Rarity::Common => 2..=3,
                Rarity::Rare => 3..=5,
                Rarity::Epic => 5..=8,
                Rarity::Legendary => 8..=12,
            }),
        },
        10 => QuestRequirement::SpendGold {
            gold: thread_rng().gen_range(match rarity {
                Rarity::Common => 25..=50,
                Rarity::Rare => 50..=150,
                Rarity::Epic => 150..=500,
                Rarity::Legendary => 500..=750,
            }),
        },
        11 => QuestRequirement::EarnGold {
            gold: thread_rng().gen_range(match rarity {
                Rarity::Common => 50..=100,
                Rarity::Rare => 100..=250,
                Rarity::Epic => 250..=750,
                Rarity::Legendary => 750..=1000,
            }),
        },
        _ => unreachable!("Invalid QuestRequirement"),
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
