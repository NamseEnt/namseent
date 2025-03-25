use crate::{
    card::{Rank, Suit},
    game_state::{GameState, mutate_game_state, tower::TowerKind},
};

pub enum QuestTrackingState {
    // [a-k]타워를 n개 새로 건설하세요
    BuildTowerRankNew {
        rank: Rank,
        target_count: usize,
        new_built_count: usize,
    },
    // [a-k]타워를 n개 건설하세요. 이미 있는 경우 즉시 완료
    BuildTowerRank {
        rank: Rank,
        target_count: usize,
    },
    // [하트|스페이드|클로버|다이아몬드]타워를 n개 새로 건설하세요
    BuildTowerSuitNew {
        suit: Suit,
        target_count: usize,
        new_built_count: usize,
    },
    // [하트|스페이드|클로버|다이아몬드]타워를 n개 건설하세요
    BuildTowerSuit {
        suit: Suit,
        target_count: usize,
    },
    // [High|OnePair|TwoPair|ThreeOfAKind|Straight|Flush|FullHouse|FourOfAKind|StraightFlush|RoyalFlush|]타워를 n개 새로 건설하세요
    BuildTowerHandNew {
        hand: TowerKind,
        target_count: usize,
        new_built_count: usize,
    },
    // [High|OnePair|TwoPair|ThreeOfAKind|Straight|Flush|FullHouse|FourOfAKind|StraightFlush|RoyalFlush|]타워를 n개 건설하세요
    BuildTowerHand {
        hand: TowerKind,
        target_count: usize,
    },
    // 아이템을 사용하지않고 보스라운드 클리어
    ClearBossRoundWithoutItems,
    // 아이템을 사용해 n피해 입히기
    DealDamageWithItems {
        target_damage: usize,
        dealt_damage: usize,
    },
    // 리롤하지않고 타워 n개 만들기
    BuildTowersWithoutReroll {
        target_count: usize,
        built_count: usize,
    },
    // 리롤 n회 사용하기
    UseReroll {
        target_count: usize,
        rolled_count: usize,
    },
    // n골드 사용하기
    SpendGold {
        target_gold: usize,
        spent_gold: usize,
    },
    // n골드 획득하기
    EarnGold {
        target_gold: usize,
        earned_gold: usize,
    },
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
        damage: usize,
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
    struct RemoveQuest {
        index: usize,
        completed: bool,
    }
    let mut remove_quests = vec![];
    for (quest_index, quest_state) in game_state.quest_states.iter_mut().enumerate() {
        match quest_state {
            &mut QuestTrackingState::BuildTowerRankNew {
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
            &mut QuestTrackingState::BuildTowerRank { rank, target_count } => {
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
            &mut QuestTrackingState::BuildTowerSuitNew {
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
            &mut QuestTrackingState::BuildTowerSuit { suit, target_count } => {
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
            &mut QuestTrackingState::BuildTowerHandNew {
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
            &mut QuestTrackingState::BuildTowerHand { hand, target_count } => {
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
            &mut QuestTrackingState::ClearBossRoundWithoutItems => match event {
                QuestTriggerEvent::ClearBossRound => remove_quests.push(RemoveQuest {
                    index: quest_index,
                    completed: true,
                }),
                QuestTriggerEvent::UseItem => {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: false,
                    });
                }
                _ => continue,
            },
            &mut QuestTrackingState::DealDamageWithItems {
                target_damage,
                ref mut dealt_damage,
            } => {
                let QuestTriggerEvent::DealDamageWithItem { damage } = event else {
                    continue;
                };
                *dealt_damage += damage;
                if *dealt_damage >= target_damage {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: true,
                    });
                }
            }
            &mut QuestTrackingState::BuildTowersWithoutReroll {
                target_count,
                ref mut built_count,
            } => match event {
                QuestTriggerEvent::BuildTower { .. } => {
                    *built_count += 1;
                    if *built_count >= target_count {
                        remove_quests.push(RemoveQuest {
                            index: quest_index,
                            completed: true,
                        });
                    }
                }
                QuestTriggerEvent::Reroll => {
                    remove_quests.push(RemoveQuest {
                        index: quest_index,
                        completed: false,
                    });
                }
                _ => continue,
            },
            &mut QuestTrackingState::UseReroll {
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
            &mut QuestTrackingState::SpendGold {
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
            &mut QuestTrackingState::EarnGold {
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

fn on_quest_failed(game_state: &mut GameState, quest: QuestTrackingState) {
    todo!()
}
fn on_quest_completed(game_state: &mut GameState, quest: QuestTrackingState) {
    todo!()
}
