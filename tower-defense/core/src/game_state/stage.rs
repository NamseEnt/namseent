pub const STAGES_PER_ACT: usize = 10;
pub const MAX_STAGE_COUNT: usize = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageKind {
    Treasure,
    Normal,
    Boss,
}

pub const fn act_for_stage(stage: usize) -> Option<usize> {
    if stage == 0 {
        None
    } else {
        Some((stage - 1) / STAGES_PER_ACT + 1)
    }
}

pub const fn stage_in_act(stage: usize) -> Option<usize> {
    if stage == 0 {
        None
    } else {
        Some((stage - 1) % STAGES_PER_ACT + 1)
    }
}

pub const fn stage_kind(stage: usize) -> Option<StageKind> {
    if stage == 0 || stage > MAX_STAGE_COUNT {
        return None;
    }

    if (stage - 1).is_multiple_of(STAGES_PER_ACT) {
        Some(StageKind::Treasure)
    } else if stage.is_multiple_of(STAGES_PER_ACT) {
        Some(StageKind::Boss)
    } else {
        Some(StageKind::Normal)
    }
}

pub const fn is_treasure_stage(stage: usize) -> bool {
    matches!(stage_kind(stage), Some(StageKind::Treasure))
}

pub const fn is_boss_stage(stage: usize) -> bool {
    matches!(stage_kind(stage), Some(StageKind::Boss))
}

pub const fn is_normal_stage(stage: usize) -> bool {
    matches!(stage_kind(stage), Some(StageKind::Normal))
}

pub const fn treasure_stage_for_act(act: usize) -> Option<usize> {
    if act == 0 {
        None
    } else {
        let stage = (act - 1) * STAGES_PER_ACT + 1;
        if stage <= MAX_STAGE_COUNT {
            Some(stage)
        } else {
            None
        }
    }
}

pub const fn boss_stage_for_act(act: usize) -> Option<usize> {
    if act == 0 {
        None
    } else {
        let stage = act * STAGES_PER_ACT;
        if stage <= MAX_STAGE_COUNT {
            Some(stage)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_every_stage_in_an_act() {
        assert_eq!(stage_kind(1), Some(StageKind::Treasure));
        assert!(matches!(stage_kind(2), Some(StageKind::Normal)));
        assert!(matches!(stage_kind(9), Some(StageKind::Normal)));
        assert_eq!(stage_kind(10), Some(StageKind::Boss));
    }

    #[test]
    fn classifies_all_treasure_and_boss_stages() {
        assert_eq!(
            (1..=MAX_STAGE_COUNT)
                .filter(|&stage| is_treasure_stage(stage))
                .collect::<Vec<_>>(),
            vec![1, 11, 21, 31, 41]
        );
        assert_eq!(
            (1..=MAX_STAGE_COUNT)
                .filter(|&stage| is_boss_stage(stage))
                .collect::<Vec<_>>(),
            vec![10, 20, 30, 40, 50]
        );
    }

    #[test]
    fn derives_act_and_local_stage() {
        assert_eq!(act_for_stage(1), Some(1));
        assert_eq!(stage_in_act(1), Some(1));
        assert_eq!(act_for_stage(11), Some(2));
        assert_eq!(stage_in_act(20), Some(10));
        assert_eq!(treasure_stage_for_act(5), Some(41));
        assert_eq!(boss_stage_for_act(5), Some(50));
    }

    #[test]
    fn rejects_zero_and_stages_after_the_run() {
        assert_eq!(stage_kind(0), None);
        assert_eq!(stage_kind(MAX_STAGE_COUNT + 1), None);
        assert!(!is_treasure_stage(51));
        assert!(!is_boss_stage(51));
    }

    #[test]
    fn core_starts_with_treasure_and_opens_next_act_after_boss() {
        let config = crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 20,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: Vec::new(),
            },
            monsters: crate::MonsterConfigState {
                stats: vec![crate::MonsterConfigEntryState {
                    kind: 0,
                    base_hp_raw: 1_000,
                    velocity_mul_raw: crate::RATIO_SCALE,
                    damage_raw: 100,
                    reward: 1,
                }],
                stage_waves: vec![
                    crate::StageWaveState {
                        stage: 1,
                        entries: vec![crate::StageWaveEntryState { kind: 0, count: 1 }],
                    },
                    crate::StageWaveState {
                        stage: 10,
                        entries: vec![crate::StageWaveEntryState { kind: 0, count: 1 }],
                    },
                ],
            },
        };
        let mut state = crate::CoreState::new_initial(config, 7);

        assert!(matches!(
            state.flow(),
            crate::GameFlowState::TreasureSelection { .. }
        ));
        state
            .select_treasure(0)
            .expect("the first treasure should be selectable");
        state.start_stage(10);
        state.force_start_defense();
        state
            .edit_snapshot(|parts| {
                parts.monster_spawn.monster_queue.clear();
                parts.monster_spawn.next_spawn_tick = None;
                parts.monsters.clear();
            })
            .expect("empty completed defense state should be valid");

        let defense_end = state
            .resolve_defense_end()
            .expect("cleared boss defense should resolve");
        assert_eq!(
            defense_end.transition,
            crate::DefenseEndTransitionState::TreasureSelection
        );
        state.apply_defense_end_transition(defense_end.transition);
        assert_eq!(state.progress().stage, 11);
        assert!(matches!(
            state.flow(),
            crate::GameFlowState::TreasureSelection { .. }
        ));
    }
}
