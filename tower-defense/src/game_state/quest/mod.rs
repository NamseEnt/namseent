pub mod requirement;
pub mod reward;
mod tracking;

use super::{GameState, mutate_game_state};
use crate::rarity::Rarity;
use rand::seq::SliceRandom;
use requirement::{QuestRequirement, generate_quest_requirement};
use reward::{QuestReward, generate_quest_reward};
pub use tracking::*;

#[derive(Debug, Clone)]
pub struct Quest {
    pub requirement: QuestRequirement,
    pub reward: QuestReward,
}

impl Quest {
    pub fn to_state(&self) -> QuestState {
        QuestState {
            tracking: match self.requirement {
                QuestRequirement::BuildTowerRankNew { rank, count } => {
                    QuestTrackingState::BuildTowerRankNew {
                        rank,
                        target_count: count,
                        new_built_count: 0,
                    }
                }
                QuestRequirement::BuildTowerRank { rank, count } => {
                    QuestTrackingState::BuildTowerRank {
                        rank,
                        target_count: count,
                    }
                }
                QuestRequirement::BuildTowerSuitNew { suit, count } => {
                    QuestTrackingState::BuildTowerSuitNew {
                        suit,
                        target_count: count,
                        new_built_count: 0,
                    }
                }
                QuestRequirement::BuildTowerSuit { suit, count } => {
                    QuestTrackingState::BuildTowerSuit {
                        suit,
                        target_count: count,
                    }
                }
                QuestRequirement::BuildTowerHandNew { hand, count } => {
                    QuestTrackingState::BuildTowerHandNew {
                        hand,
                        target_count: count,
                        new_built_count: 0,
                    }
                }
                QuestRequirement::BuildTowerHand { hand, count } => {
                    QuestTrackingState::BuildTowerHand {
                        hand,
                        target_count: count,
                    }
                }
                QuestRequirement::ClearBossRoundWithoutItems => {
                    QuestTrackingState::ClearBossRoundWithoutItems
                }
                QuestRequirement::DealDamageWithItems { damage } => {
                    QuestTrackingState::DealDamageWithItems {
                        target_damage: damage,
                        dealt_damage: 0.0,
                    }
                }
                QuestRequirement::BuildTowersWithoutReroll { count } => {
                    QuestTrackingState::BuildTowersWithoutReroll {
                        target_count: count,
                        built_count: 0,
                    }
                }
                QuestRequirement::UseReroll { count } => QuestTrackingState::UseReroll {
                    target_count: count,
                    rolled_count: 0,
                },
                QuestRequirement::SpendGold { gold } => QuestTrackingState::SpendGold {
                    target_gold: gold,
                    spent_gold: 0,
                },
                QuestRequirement::EarnGold { gold } => QuestTrackingState::EarnGold {
                    target_gold: gold,
                    earned_gold: 0,
                },
            },
            reward: self.reward,
        }
    }
}

#[derive(Debug)]
pub struct QuestState {
    pub tracking: QuestTrackingState,
    pub reward: QuestReward,
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
    let requirement = generate_quest_requirement(rarity);
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

pub fn cancel_quest(quest_index: usize) {
    mutate_game_state(move |game_state| {
        game_state.quest_states.remove(quest_index);
    });
}
