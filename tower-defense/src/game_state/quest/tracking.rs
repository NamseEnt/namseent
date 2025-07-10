use crate::{
    card::{Rank, Suit},
    game_state::{GameState, tower::TowerKind},
    l10n::quest::QuestText,
};

use super::QuestState;

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
                }) + &format!(" ({}/{})", new_built_count, target_count)
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
                    suit: suit.to_string(),
                    count: *target_count,
                }) + &format!(" ({}/{})", new_built_count, target_count)
            }
            QuestTrackingState::BuildTowerSuit { suit, target_count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.suit == *suit)
                    .count();
                game_state.text().quest(QuestText::BuildTowerSuit {
                    suit: suit.to_string(),
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
                    hand: hand.to_string(),
                    count: *target_count,
                }) + &format!(" ({}/{})", new_built_count, target_count)
            }
            QuestTrackingState::BuildTowerHand { hand, target_count } => {
                let current_count = game_state
                    .towers
                    .iter()
                    .filter(|tower| tower.kind == *hand)
                    .count();
                game_state.text().quest(QuestText::BuildTowerHand {
                    hand: hand.to_string(),
                    count: *target_count,
                    current_count,
                })
            }
            QuestTrackingState::ClearBossRoundWithoutItems => game_state
                .text().quest(QuestText::ClearBossRoundWithoutItems),
            QuestTrackingState::DealDamageWithItems {
                target_damage,
                dealt_damage,
            } => {
                game_state
                    .text().quest(QuestText::DealDamageWithItems {
                        damage: *target_damage,
                    })
                    + &format!(" ({}/{})", dealt_damage, target_damage)
            }
            QuestTrackingState::BuildTowersWithoutReroll {
                target_count,
                built_count,
            } => {
                game_state
                    .text().quest(QuestText::BuildTowersWithoutReroll {
                        count: *target_count,
                    })
                    + &format!(" ({}/{})", built_count, target_count)
            }
            QuestTrackingState::UseReroll {
                target_count,
                rolled_count,
            } => {
                game_state.text().quest(QuestText::UseReroll {
                    count: *target_count,
                }) + &format!(" ({}/{})", rolled_count, target_count)
            }
            QuestTrackingState::SpendGold {
                target_gold,
                spent_gold,
            } => {
                game_state
                    .text().quest(QuestText::SpendGold { gold: *target_gold })
                    + &format!(" ({}/{})", spent_gold, target_gold)
            }
            QuestTrackingState::EarnGold {
                target_gold,
                earned_gold,
            } => {
                game_state
                    .text().quest(QuestText::EarnGold { gold: *target_gold })
                    + &format!(" ({}/{})", earned_gold, target_gold)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// NOTE: Please call this event after the actual event has been processed.
/// For example, if the event is "BuildTower", call this function after the tower has been built.
pub enum QuestTriggerEvent {
    BuildTower {
        rank: Rank,
        suit: Suit,
        hand: TowerKind,
    },
    ClearBossRound,
    UseItem,
    DealDamageWithItem {
        damage: f32,
    },
    Reroll,
    SpendGold {
        gold: usize,
    },
    EarnGold {
        gold: usize,
    },
}

pub fn on_quest_trigger_event(game_state: &mut GameState, event: QuestTriggerEvent) {
    let rerolled = game_state.rerolled();
    struct RemoveQuest {
        index: usize,
        completed: bool,
    }
    let mut remove_quests = vec![];
    for (quest_index, quest_state) in game_state.quest_states.iter_mut().enumerate() {
        match quest_state.tracking {
            QuestTrackingState::BuildTowerRankNew {
                rank,
                target_count,
                ref mut new_built_count,
            } => {
                let QuestTriggerEvent::BuildTower {
                    rank: event_rank, ..
                } = event
                else {
                    continue;
                };
                if rank == event_rank {
                    *new_built_count += 1;
                    if *new_built_count >= target_count {
                        remove_quests.push(RemoveQuest {
                            index: quest_index,
                            completed: true,
                        });
                    }
                }
            }
            QuestTrackingState::BuildTowerRank { rank, target_count } => {
                let QuestTriggerEvent::BuildTower {
                    rank: event_rank, ..
                } = event
                else {
                    continue;
                };

                if rank == event_rank
                    && target_count
                        == game_state
                            .towers
                            .iter()
                            .filter(|tower| tower.rank == rank)
                            .count()
                {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            QuestTrackingState::BuildTowerSuitNew {
                suit,
                target_count,
                ref mut new_built_count,
            } => {
                let QuestTriggerEvent::BuildTower {
                    suit: event_suit, ..
                } = event
                else {
                    continue;
                };
                if suit == event_suit {
                    *new_built_count += 1;
                    if *new_built_count >= target_count {
                        remove_quests.push(RemoveQuest {
                            index: quest_index,
                            completed: true,
                        });
                    }
                }
            }
            QuestTrackingState::BuildTowerSuit { suit, target_count } => {
                let QuestTriggerEvent::BuildTower {
                    suit: event_suit, ..
                } = event
                else {
                    continue;
                };
                if suit == event_suit
                    && target_count
                        == game_state
                            .towers
                            .iter()
                            .filter(|tower| tower.suit == suit)
                            .count()
                {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            QuestTrackingState::BuildTowerHandNew {
                hand,
                target_count,
                ref mut new_built_count,
            } => {
                let QuestTriggerEvent::BuildTower {
                    hand: event_hand, ..
                } = event
                else {
                    continue;
                };
                if hand == event_hand {
                    *new_built_count += 1;
                    if *new_built_count >= target_count {
                        remove_quests.push(RemoveQuest {
                            index: quest_index,
                            completed: true,
                        });
                    }
                }
            }
            QuestTrackingState::BuildTowerHand { hand, target_count } => {
                let QuestTriggerEvent::BuildTower {
                    hand: event_hand, ..
                } = event
                else {
                    continue;
                };
                if hand == event_hand
                    && target_count
                        == game_state
                            .towers
                            .iter()
                            .filter(|tower| tower.kind == hand)
                            .count()
                {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            QuestTrackingState::ClearBossRoundWithoutItems => match event {
                QuestTriggerEvent::ClearBossRound => remove_quests.push(RemoveQuest {
                    index: quest_index,
                    completed: true,
                }),
                _ => continue,
            },
            QuestTrackingState::DealDamageWithItems {
                target_damage,
                ref mut dealt_damage,
            } => {
                let QuestTriggerEvent::DealDamageWithItem { damage } = event else {
                    continue;
                };
                *dealt_damage += damage;
                if *dealt_damage as usize >= target_damage {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            QuestTrackingState::BuildTowersWithoutReroll {
                target_count,
                ref mut built_count,
            } => match event {
                QuestTriggerEvent::BuildTower { .. } => {
                    if rerolled {
                        continue;
                    }
                    *built_count += 1;
                    if *built_count >= target_count {
                        remove_quests.push(RemoveQuest {
                            index: quest_index,
                            completed: true,
                        });
                    }
                }
                _ => continue,
            },
            QuestTrackingState::UseReroll {
                target_count,
                ref mut rolled_count,
            } => {
                let QuestTriggerEvent::Reroll = event else {
                    continue;
                };
                *rolled_count += 1;
                if *rolled_count >= target_count {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            QuestTrackingState::SpendGold {
                target_gold,
                ref mut spent_gold,
            } => {
                let QuestTriggerEvent::SpendGold { gold } = event else {
                    continue;
                };
                *spent_gold += gold;
                if *spent_gold >= target_gold {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            QuestTrackingState::EarnGold {
                target_gold,
                ref mut earned_gold,
            } => {
                let QuestTriggerEvent::EarnGold { gold } = event else {
                    continue;
                };
                *earned_gold += gold;
                if *earned_gold >= target_gold {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
        }
    }
    for remove_quest in remove_quests.into_iter().rev() {
        let quest = game_state.quest_states.remove(remove_quest.index);
        if remove_quest.completed {
            on_quest_completed(game_state, quest);
        } else {
            on_quest_failed(game_state, quest);
        }
    }
}

fn on_quest_failed(_game_state: &mut GameState, _quest: QuestState) {
    unimplemented!("All quests are not failable for now")
}
fn on_quest_completed(game_state: &mut GameState, quest: QuestState) {
    match quest.reward {
        super::reward::QuestReward::Money { amount } => {
            game_state.earn_gold(amount);
        }
        super::reward::QuestReward::Item { item } => {
            game_state.items.push(item);
        }
        super::reward::QuestReward::Upgrade { upgrade } => {
            game_state.upgrade_state.upgrade(upgrade);
        }
    }
}
