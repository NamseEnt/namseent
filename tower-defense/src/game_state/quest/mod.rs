mod icon;
pub mod requirement;
pub mod reward;
mod tracking_state;
mod trigger_event;

use super::{GameState, mutate_game_state};
use crate::rarity::Rarity;
pub use requirement::QuestRequirement;
use requirement::generate_quest_requirement;
pub use reward::QuestReward;
use reward::generate_quest_reward;
pub use tracking_state::QuestTrackingState;
pub use trigger_event::{QuestTriggerEvent, on_quest_trigger_event};

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
            reward: self.reward.clone(),
        }
    }
}

#[derive(Debug)]
pub struct QuestState {
    pub tracking: QuestTrackingState,
    pub reward: QuestReward,
}

pub fn generate_quests(game_state: &GameState, amount: usize) -> Vec<Quest> {
    let rarities = (0..amount).map(|_| game_state.generate_rarity(Default::default()));

    rarities
        .map(|rarity| generate_quest(game_state, rarity))
        .collect()
}
fn generate_quest(game_state: &GameState, rarity: Rarity) -> Quest {
    let requirement = generate_quest_requirement(rarity);
    let reward = generate_quest_reward(game_state, rarity);
    Quest {
        requirement,
        reward,
    }
}

pub fn cancel_quest(quest_index: usize) {
    mutate_game_state(move |game_state| {
        game_state.quest_states.remove(quest_index);
    });
}
