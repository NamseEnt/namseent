use super::{QuestState, QuestTrackingState};
use crate::{
    card::{Rank, Suit},
    game_state::{GameState, tower::TowerKind},
};

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
    let requirement = quest.tracking.to_requirement();
    game_state.complete_quest(requirement, quest.reward);
}
