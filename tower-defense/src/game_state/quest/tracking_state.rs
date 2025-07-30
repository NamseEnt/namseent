use crate::{
    card::{Rank, Suit},
    game_state::{GameState, tower::TowerKind, quest::QuestRequirement},
    l10n::quest::QuestText,
};

#[derive(Debug)]
pub enum QuestTrackingState {
    BuildTowerRankNew {
        rank: Rank,
        target_count: usize,
        new_built_count: usize,
    },
    BuildTowerRank {
        rank: Rank,
        target_count: usize,
    },
    BuildTowerSuitNew {
        suit: Suit,
        target_count: usize,
        new_built_count: usize,
    },
    BuildTowerSuit {
        suit: Suit,
        target_count: usize,
    },
    BuildTowerHandNew {
        hand: TowerKind,
        target_count: usize,
        new_built_count: usize,
    },
    BuildTowerHand {
        hand: TowerKind,
        target_count: usize,
    },
    ClearBossRoundWithoutItems,
    DealDamageWithItems {
        target_damage: usize,
        dealt_damage: f32,
    },
    BuildTowersWithoutReroll {
        target_count: usize,
        built_count: usize,
    },
    UseReroll {
        target_count: usize,
        rolled_count: usize,
    },
    SpendGold {
        target_gold: usize,
        spent_gold: usize,
    },
    EarnGold {
        target_gold: usize,
        earned_gold: usize,
    },
}

impl QuestTrackingState {
    pub fn to_requirement(&self) -> QuestRequirement {
        match self {
            QuestTrackingState::BuildTowerRankNew { rank, target_count, .. } => {
                QuestRequirement::BuildTowerRankNew { rank: *rank, count: *target_count }
            }
            QuestTrackingState::BuildTowerRank { rank, target_count } => {
                QuestRequirement::BuildTowerRank { rank: *rank, count: *target_count }
            }
            QuestTrackingState::BuildTowerSuitNew { suit, target_count, .. } => {
                QuestRequirement::BuildTowerSuitNew { suit: *suit, count: *target_count }
            }
            QuestTrackingState::BuildTowerSuit { suit, target_count } => {
                QuestRequirement::BuildTowerSuit { suit: *suit, count: *target_count }
            }
            QuestTrackingState::BuildTowerHandNew { hand, target_count, .. } => {
                QuestRequirement::BuildTowerHandNew { hand: *hand, count: *target_count }
            }
            QuestTrackingState::BuildTowerHand { hand, target_count } => {
                QuestRequirement::BuildTowerHand { hand: *hand, count: *target_count }
            }
            QuestTrackingState::ClearBossRoundWithoutItems => {
                QuestRequirement::ClearBossRoundWithoutItems
            }
            QuestTrackingState::DealDamageWithItems { target_damage, .. } => {
                QuestRequirement::DealDamageWithItems { damage: *target_damage }
            }
            QuestTrackingState::BuildTowersWithoutReroll { target_count, .. } => {
                QuestRequirement::BuildTowersWithoutReroll { count: *target_count }
            }
            QuestTrackingState::UseReroll { target_count, .. } => {
                QuestRequirement::UseReroll { count: *target_count }
            }
            QuestTrackingState::SpendGold { target_gold, .. } => {
                QuestRequirement::SpendGold { gold: *target_gold }
            }
            QuestTrackingState::EarnGold { target_gold, .. } => {
                QuestRequirement::EarnGold { gold: *target_gold }
            }
        }
    }

    pub(crate) fn description(&self, game_state: &GameState) -> String {
        match self {
            QuestTrackingState::BuildTowerRankNew {
                rank,
                target_count,
                new_built_count,
            } => {
                game_state.text().quest(QuestText::BuildTowerRankNew {
                    rank: rank.to_string(),
                    count: *target_count,
                }) + &format!(" ({new_built_count}/{target_count})")
            }
            QuestTrackingState::BuildTowerRank { rank, target_count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.rank == *rank)
                    .count();
                game_state.text().quest(QuestText::BuildTowerRank {
                    rank: rank.to_string(),
                    count: *target_count,
                    current_count,
                })
            }
            QuestTrackingState::BuildTowerSuitNew {
                suit,
                target_count,
                new_built_count,
            } => {
                game_state.text().quest(QuestText::BuildTowerSuitNew {
                    suit: *suit,
                    count: *target_count,
                }) + &format!(" ({new_built_count}/{target_count})")
            }
            QuestTrackingState::BuildTowerSuit { suit, target_count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.suit == *suit)
                    .count();
                game_state.text().quest(QuestText::BuildTowerSuit {
                    suit: *suit,
                    count: *target_count,
                    current_count,
                })
            }
            QuestTrackingState::BuildTowerHandNew {
                hand,
                target_count,
                new_built_count,
            } => {
                game_state.text().quest(QuestText::BuildTowerHandNew {
                    hand: game_state.text().tower(hand.to_text()).to_string(),
                    count: *target_count,
                }) + &format!(" ({new_built_count}/{target_count})")
            }
            QuestTrackingState::BuildTowerHand { hand, target_count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.kind == *hand)
                    .count();
                game_state.text().quest(QuestText::BuildTowerHand {
                    hand: game_state.text().tower(hand.to_text()).to_string(),
                    count: *target_count,
                    current_count,
                })
            }
            QuestTrackingState::ClearBossRoundWithoutItems => game_state
                .text()
                .quest(QuestText::ClearBossRoundWithoutItems),
            QuestTrackingState::DealDamageWithItems {
                target_damage,
                dealt_damage,
            } => {
                game_state.text().quest(QuestText::DealDamageWithItems {
                    damage: *target_damage,
                }) + &format!(" ({dealt_damage}/{target_damage})")
            }
            QuestTrackingState::BuildTowersWithoutReroll {
                target_count,
                built_count,
            } => {
                game_state
                    .text()
                    .quest(QuestText::BuildTowersWithoutReroll {
                        count: *target_count,
                    })
                    + &format!(" ({built_count}/{target_count})")
            }
            QuestTrackingState::UseReroll {
                target_count,
                rolled_count,
            } => {
                game_state.text().quest(QuestText::UseReroll {
                    count: *target_count,
                }) + &format!(" ({rolled_count}/{target_count})")
            }
            QuestTrackingState::SpendGold {
                target_gold,
                spent_gold,
            } => {
                game_state
                    .text()
                    .quest(QuestText::SpendGold { gold: *target_gold })
                    + &format!(" ({spent_gold}/{target_gold})")
            }
            QuestTrackingState::EarnGold {
                target_gold,
                earned_gold,
            } => {
                game_state
                    .text()
                    .quest(QuestText::EarnGold { gold: *target_gold })
                    + &format!(" ({earned_gold}/{target_gold})")
            }
        }
    }
}