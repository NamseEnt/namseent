use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::modal::card_candidate::CardCandidateModal;
use crate::game_state::modal::deck::CardSelectionState;
use crate::game_state::{GameState, UserModal};

pub(crate) fn consume_headed(
    game_state: &mut GameState,
    events: impl IntoIterator<Item = td_core::CoreEvent>,
    presentation_instant: crate::PresentationInstant,
) -> Vec<(usize, usize)> {
    let mut defense_intros = Vec::new();
    for (event_index, event) in events.into_iter().enumerate() {
        match event {
            td_core::CoreEvent::CardServiceSelectionRequested {
                service_kind,
                step_counts,
                candidate_card_ids,
            } => game_state.open_card_service_selection_from_core_event(
                &service_kind,
                &step_counts,
                candidate_card_ids.as_deref(),
            ),
            td_core::CoreEvent::StageStarted { stage, card_count } => {
                crate::game_state::presentation_effect::apply_stage_start(
                    game_state, stage, card_count,
                );
            }
            td_core::CoreEvent::GameFinished { victory } => {
                crate::sound::play_game_end_sound_at(
                    if victory {
                        crate::sound::GameEndKind::Victory
                    } else {
                        crate::sound::GameEndKind::Defeat
                    },
                    presentation_instant,
                );
                crate::game_state::presentation_effect::apply_game_over(game_state);
            }
            td_core::CoreEvent::DefenseEnded {
                stage,
                perfect_clear,
                transition,
            } => {
                crate::game_state::presentation_effect::apply_stage_end(
                    game_state,
                    stage,
                    perfect_clear,
                );
                if matches!(
                    transition,
                    td_core::DefenseEndTransitionState::TreasureSelection
                ) {
                    game_state.discover_treasure_options();
                }
            }
            td_core::CoreEvent::BaseDamageApplied {
                amount,
                actual_amount,
            } => {
                crate::game_state::presentation_effect::apply_damage(
                    game_state,
                    crate::Damage::from_raw(amount),
                    crate::Damage::from_raw(actual_amount),
                );
            }
            td_core::CoreEvent::DamageApplied {
                amount, position, ..
            } => {
                let damage = crate::Damage::from_raw(amount);
                if !damage.is_zero() {
                    game_state.push_presentation_event(
                        crate::game_state::PresentationEvent::SpawnParticle(
                            crate::game_state::ParticleSpawnRequest::DamageText {
                                position: [
                                    position[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                                    position[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                                ],
                                damage: damage.as_f32(),
                            },
                        ),
                    );
                    game_state.push_presentation_event(
                        crate::game_state::PresentationEvent::PlaySoundCue {
                            cue: crate::game_state::SoundCue::Whoop,
                            position: Some([
                                position[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                                position[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            ]),
                            volume: crate::game_state::SoundVolume::Minimum,
                            max_duration_ms: None,
                        },
                    );
                }
            }
            td_core::CoreEvent::MonsterDefeated {
                position,
                monster_kind,
                reward,
                rotation_milliradians,
                ..
            } => {
                let position = [position[0], position[1]];
                let rotation_radians = rotation_milliradians as f32 / 1_000.0;
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::SpawnParticle(
                        crate::game_state::ParticleSpawnRequest::MonsterSoul {
                            position,
                            rotation_radians,
                        },
                    ),
                );
                if let Some(monster_kind) =
                    crate::game_state::MonsterKind::from_core_raw(monster_kind)
                {
                    game_state.push_presentation_event(
                        crate::game_state::PresentationEvent::SpawnParticle(
                            crate::game_state::ParticleSpawnRequest::MonsterCorpse {
                                position,
                                rotation_radians,
                                monster_kind,
                            },
                        ),
                    );
                }
                if reward > 0 {
                    let map_position =
                        crate::world::WorldCoord::new(position[0], position[1]).as_map_coord_f32();
                    game_state.push_presentation_event(
                        crate::game_state::PresentationEvent::SpawnGoldReward {
                            position: [map_position.x, map_position.y],
                            amount: reward,
                        },
                    );
                    crate::game_state::presentation_effect::apply_earn_gold_sound(
                        game_state, reward,
                    );
                }
            }
            td_core::CoreEvent::TowerAttack {
                tower_id,
                tower_kind: tower_kind_raw,
                start_xy,
                end_xy,
                attack_range_raw,
                ..
            } => {
                game_state.presentation_metadata.transition_tower_animation(
                    crate::TowerId::from_raw(tower_id),
                    crate::game_state::tower::AnimationKind::Attack,
                    game_state.sim_tick(),
                );
                if let Some(
                    tower_kind @ (crate::game_state::tower::TowerKind::High
                    | crate::game_state::tower::TowerKind::OnePair
                    | crate::game_state::tower::TowerKind::TwoPair
                    | crate::game_state::tower::TowerKind::ThreeOfAKind
                    | crate::game_state::tower::TowerKind::Straight
                    | crate::game_state::tower::TowerKind::StraightFlush
                    | crate::game_state::tower::TowerKind::RoyalFlush
                    | crate::game_state::tower::TowerKind::FourOfAKind
                    | crate::game_state::tower::TowerKind::Flush
                    | crate::game_state::tower::TowerKind::FullHouse),
                ) = crate::game_state::tower::TowerKind::from_core_raw(tower_kind_raw)
                {
                    game_state.push_presentation_event(
                        crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                            tower_kind,
                            start_xy: [
                                start_xy[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                                start_xy[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            ],
                            end_xy: [
                                end_xy[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                                end_xy[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            ],
                            attack_range_radius: attack_range_raw as f32
                                / crate::world::WORLD_UNITS_PER_TILE as f32,
                        },
                    );
                }
            }
            td_core::CoreEvent::TowerAttackHit {
                tower_kind,
                start_xy,
                position,
                chain_delay_ms,
                ..
            } => {
                if let Some(kind) = crate::game_state::tower::TowerKind::from_core_raw(tower_kind) {
                    let units_per_tile = crate::world::WORLD_UNITS_PER_TILE as f32;
                    game_state.push_presentation_event(
                        crate::game_state::PresentationEvent::SpawnMonsterHitVisualEffect {
                            source: crate::game_state::presentation_event::MonsterHitVisualSource::Tower {
                                kind,
                            },
                            start_xy: [
                                start_xy[0] as f32 / units_per_tile,
                                start_xy[1] as f32 / units_per_tile,
                            ],
                            impact_xy: [
                                position[0] as f32 / units_per_tile,
                                position[1] as f32 / units_per_tile,
                            ],
                            chain_delay_ms,
                        },
                    );
                }
            }
            td_core::CoreEvent::MonsterSpawned { .. } => {
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::AnimateBase(
                        crate::game_state::BaseAnimationEvent::EnemySpawn,
                    ),
                );
            }
            td_core::CoreEvent::DefenseStarted { stage } => {
                defense_intros.push((event_index, stage));
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::PlaySoundCue {
                        cue: crate::game_state::SoundCue::StartDefenseFanfare,
                        position: None,
                        volume: crate::game_state::SoundVolume::High,
                        max_duration_ms: Some(6_000),
                    },
                );
            }
            td_core::CoreEvent::TreasureSelected { upgrade } => {
                if let Some(upgrade) =
                    crate::game_state::upgrade::UpgradeWithId::from_core_state(upgrade)
                {
                    crate::game_state::presentation_effect::apply_upgrade(
                        game_state,
                        upgrade.upgrade,
                        None,
                    );
                }
            }
            td_core::CoreEvent::TreasureDiscarded { .. } => {}
        }
    }
    defense_intros
}

pub(crate) fn consume_headless(events: impl IntoIterator<Item = td_core::CoreEvent>) {
    for _event in events {}
}

pub(crate) fn card_service_selection(
    game_state: &GameState,
    service_kind: &str,
    step_counts: &[usize],
    candidate_card_ids: Option<&[usize]>,
) -> Option<UserModal> {
    let service = td_core::CardServiceKind::ALL
        .iter()
        .copied()
        .map(crate::game_state::card_service::CardServiceDiscriminants::from_core_kind)
        .find(|kind| kind.generate().key() == service_kind)?
        .generate();
    let mut steps = service.acquire_selection_steps(game_state.locale());
    if steps.len() != step_counts.len() {
        return None;
    }
    for (step, count) in steps.iter_mut().zip(step_counts) {
        step.count = *count;
    }
    if let Some(candidate_card_ids) = candidate_card_ids {
        Some(UserModal::CardCandidate(CardCandidateModal {
            card_service: service,
            candidate_card_ids: candidate_card_ids
                .iter()
                .copied()
                .map(crate::card::CardId::from_raw)
                .collect(),
        }))
    } else {
        Some(UserModal::Deck(crate::game_state::modal::deck::DeckModal {
            deck_kind: crate::game_state::modal::deck::DeckKind::Deck,
            selection: Some(CardSelectionState::new(steps, service)),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_service_event_uses_authoritative_step_counts() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let modal =
            card_service_selection(&game_state, "eraser", &[2], None).expect("eraser modal");
        let UserModal::Deck(deck) = modal else {
            panic!("expected deck modal");
        };
        assert_eq!(deck.selection.expect("selection").steps[0].count, 2);
    }

    #[test]
    fn card_service_event_with_candidates_uses_candidate_modal() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let modal = card_service_selection(&game_state, "tricycle", &[1], Some(&[0, 1, 2]))
            .expect("tricycle modal");

        let UserModal::CardCandidate(modal) = modal else {
            panic!("expected candidate modal");
        };
        assert_eq!(
            modal.candidate_card_ids,
            vec![
                crate::card::CardId::from_raw(0),
                crate::card::CardId::from_raw(1),
                crate::card::CardId::from_raw(2),
            ]
        );
    }

    #[test]
    fn invalid_card_service_event_is_ignored() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        assert!(card_service_selection(&game_state, "unknown", &[1], None).is_none());
        assert!(card_service_selection(&game_state, "eraser", &[1, 2], None).is_none());
    }

    #[test]
    fn stage_started_event_queues_stage_presentation_once() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::StageStarted {
                stage: 2,
                card_count: 3,
            }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert_eq!(effects.presentation_events.events.len(), 2);
        assert_eq!(effects.history_events.len(), 1);
    }

    #[test]
    fn game_finished_event_records_one_game_over_presentation() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::GameFinished { victory: true }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert_eq!(effects.history_events.len(), 1);
        assert!(effects.presentation_events.events.is_empty());
    }

    #[test]
    fn defense_ended_event_records_the_completed_stage() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::DefenseEnded {
                stage: 4,
                perfect_clear: true,
                transition: td_core::DefenseEndTransitionState::StartStage { stage: 5 },
            }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(matches!(
            effects.history_events.as_slice(),
            [crate::game_state::play_history::HistoryEvent {
                event_type: crate::game_state::play_history::HistoryEventType::StagePerfectClear {
                    stage: 4,
                },
                ..
            }]
        ));
    }

    #[test]
    fn defense_started_event_queues_start_fanfare() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::DefenseStarted { stage: 1 }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(matches!(
            effects.presentation_events.events.as_slice(),
            [crate::game_state::PresentationEvent::PlaySoundCue {
                cue: crate::game_state::SoundCue::StartDefenseFanfare,
                ..
            }]
        ));
    }

    #[test]
    fn monster_spawned_event_queues_enemy_spawn_animation() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::MonsterSpawned {
                monster_id: 3,
                monster_kind: 0,
                position: [0, 0],
            }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(matches!(
            effects.presentation_events.events.as_slice(),
            [crate::game_state::PresentationEvent::AnimateBase(
                crate::game_state::BaseAnimationEvent::EnemySpawn
            )]
        ));
    }

    #[test]
    fn base_damage_and_defeat_events_do_not_duplicate_game_over_history() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [
                td_core::CoreEvent::BaseDamageApplied {
                    amount: 10,
                    actual_amount: 10,
                },
                td_core::CoreEvent::GameFinished { victory: false },
            ],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert_eq!(
            effects
                .history_events
                .iter()
                .filter(|event| matches!(
                    event.event_type,
                    crate::game_state::play_history::HistoryEventType::GameOver
                ))
                .count(),
            1
        );
    }

    #[test]
    fn combat_events_only_damage_applied_creates_damage_text() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [
                td_core::CoreEvent::DamageApplied {
                    target_id: 1,
                    amount: 10,
                    position: [0, 0],
                },
                td_core::CoreEvent::MonsterDefeated {
                    monster_id: 1,
                    position: [0, 0],
                    monster_kind: 0,
                    reward: 0,
                    rotation_milliradians: 0,
                },
                td_core::CoreEvent::TowerAttack {
                    tower_id: 1,
                    target_id: 1,
                    tower_kind: 0,
                    start_xy: [0, 0],
                    end_xy: [0, 0],
                    attack_range_raw: 0,
                },
            ],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert_eq!(effects.presentation_events.events.len(), 4);
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnParticle(
                        crate::game_state::ParticleSpawnRequest::DamageText { damage, .. }
                    ) if *damage > 0.0
                ))
        );
        assert!(effects.history_events.is_empty());
    }

    #[test]
    fn monster_reward_queues_gold_particles_and_staggered_coin_sounds() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::MonsterDefeated {
                monster_id: 1,
                position: [1024, 2048],
                monster_kind: 0,
                reward: 3,
                rotation_milliradians: 0,
            }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        let gold_reward = effects
            .presentation_events
            .events
            .iter()
            .find_map(|event| match event {
                crate::game_state::PresentationEvent::SpawnGoldReward { position, amount } => {
                    Some((*position, *amount))
                }
                _ => None,
            })
            .expect("monster reward should queue gold particles");
        assert_eq!(gold_reward.1, 3);
        assert_eq!(
            gold_reward.0,
            [
                1024.0 / crate::world::WORLD_UNITS_PER_TILE as f32,
                2048.0 / crate::world::WORLD_UNITS_PER_TILE as f32,
            ]
        );
        let coin_sound_delays = effects
            .presentation_events
            .events
            .iter()
            .filter_map(|event| match event {
                crate::game_state::PresentationEvent::PlaySoundCueDelayed {
                    cue: crate::game_state::SoundCue::Coin,
                    delay_ms,
                    ..
                } => Some(*delay_ms),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(coin_sound_delays, vec![0, 100, 200]);
    }

    #[test]
    fn tower_attack_visual_presentation_aligns_with_tower_sprite_center() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        let units_per_tile = crate::world::WORLD_UNITS_PER_TILE;

        consume_headed(
            &mut game_state,
            [
                td_core::CoreEvent::TowerAttack {
                    tower_id: 1,
                    target_id: 2,
                    tower_kind: 1,
                    start_xy: [units_per_tile, 2 * units_per_tile],
                    end_xy: [3 * units_per_tile, 4 * units_per_tile],
                    attack_range_raw: 5 * units_per_tile,
                },
                td_core::CoreEvent::TowerAttack {
                    tower_id: 3,
                    target_id: 4,
                    tower_kind: 2,
                    start_xy: [5 * units_per_tile, 6 * units_per_tile],
                    end_xy: [7 * units_per_tile, 8 * units_per_tile],
                    attack_range_raw: 5 * units_per_tile,
                },
                td_core::CoreEvent::TowerAttack {
                    tower_id: 5,
                    target_id: 6,
                    tower_kind: 3,
                    start_xy: [9 * units_per_tile, 10 * units_per_tile],
                    end_xy: [11 * units_per_tile, 12 * units_per_tile],
                    attack_range_raw: 6 * units_per_tile,
                },
                td_core::CoreEvent::TowerAttack {
                    tower_id: 7,
                    target_id: 8,
                    tower_kind: crate::game_state::tower::TowerKind::ThreeOfAKind.to_core_raw(),
                    start_xy: [13 * units_per_tile, 14 * units_per_tile],
                    end_xy: [16 * units_per_tile, 17 * units_per_tile],
                    attack_range_raw: 7 * units_per_tile,
                },
            ],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                        tower_kind: crate::game_state::tower::TowerKind::High,
                        start_xy: [1.0, 2.0],
                        end_xy: [3.0, 4.0],
                        ..
                    }
                ))
        );
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                        tower_kind: crate::game_state::tower::TowerKind::ThreeOfAKind,
                        start_xy: [13.0, 14.0],
                        end_xy: [16.0, 17.0],
                        attack_range_radius: 7.0,
                    }
                ))
        );
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                        tower_kind: crate::game_state::tower::TowerKind::TwoPair,
                        start_xy: [9.0, 10.0],
                        end_xy: [11.0, 12.0],
                        ..
                    }
                ))
        );
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                        tower_kind: crate::game_state::tower::TowerKind::OnePair,
                        start_xy: [5.0, 6.0],
                        end_xy: [7.0, 8.0],
                        ..
                    }
                ))
        );
    }

    #[test]
    fn straight_tower_attack_visual_uses_core_range_endpoint() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        let units_per_tile = crate::world::WORLD_UNITS_PER_TILE;

        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::TowerAttack {
                tower_id: 9,
                target_id: 10,
                tower_kind: crate::game_state::tower::TowerKind::Straight.to_core_raw(),
                start_xy: [2 * units_per_tile, 3 * units_per_tile],
                end_xy: [11 * units_per_tile, 3 * units_per_tile],
                attack_range_raw: 9 * units_per_tile,
            }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                        tower_kind: crate::game_state::tower::TowerKind::Straight,
                        start_xy: [2.0, 3.0],
                        end_xy: [11.0, 3.0],
                        attack_range_radius: 9.0,
                    }
                ))
        );
    }

    #[test]
    fn royal_flush_attack_visual_uses_core_range_endpoint() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        let units_per_tile = crate::world::WORLD_UNITS_PER_TILE;

        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::TowerAttack {
                tower_id: 9,
                target_id: 10,
                tower_kind: crate::game_state::tower::TowerKind::RoyalFlush.to_core_raw(),
                start_xy: [2 * units_per_tile, 3 * units_per_tile],
                end_xy: [11 * units_per_tile, 3 * units_per_tile],
                attack_range_raw: 9 * units_per_tile,
            }],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnTowerAttackVisualEffect {
                        tower_kind: crate::game_state::tower::TowerKind::RoyalFlush,
                        start_xy: [2.0, 3.0],
                        end_xy: [11.0, 3.0],
                        attack_range_radius: 9.0,
                    }
                ))
        );
    }

    #[test]
    fn monster_hit_visual_presentation_converts_coordinates_to_tiles() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let _ = game_state.take_pending_action_effects();
        let units_per_tile = crate::world::WORLD_UNITS_PER_TILE;

        consume_headed(
            &mut game_state,
            [
                td_core::CoreEvent::TowerAttackHit {
                    tower_id: 7,
                    target_id: 9,
                    tower_kind: crate::game_state::tower::TowerKind::ThreeOfAKind.to_core_raw(),
                    start_xy: [13 * units_per_tile, 14 * units_per_tile],
                    position: [18 * units_per_tile, 19 * units_per_tile],
                    chain_delay_ms: 0,
                },
                td_core::CoreEvent::TowerAttackHit {
                    tower_id: 8,
                    target_id: 10,
                    tower_kind: crate::game_state::tower::TowerKind::Straight.to_core_raw(),
                    start_xy: [17 * units_per_tile, 18 * units_per_tile],
                    position: [26 * units_per_tile, 18 * units_per_tile],
                    chain_delay_ms: 0,
                },
                td_core::CoreEvent::TowerAttackHit {
                    tower_id: 11,
                    target_id: 12,
                    tower_kind: crate::game_state::tower::TowerKind::StraightFlush.to_core_raw(),
                    start_xy: [26 * units_per_tile, 18 * units_per_tile],
                    position: [29 * units_per_tile, 20 * units_per_tile],
                    chain_delay_ms: 20,
                },
            ],
            crate::PresentationInstant::zero(),
        );

        let effects = game_state.take_pending_action_effects();
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnMonsterHitVisualEffect {
                        source:
                            crate::game_state::presentation_event::MonsterHitVisualSource::Tower {
                                kind: crate::game_state::tower::TowerKind::ThreeOfAKind,
                            },
                        start_xy: [13.0, 14.0],
                        impact_xy: [18.0, 19.0],
                        ..
                    }
                ))
        );
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnMonsterHitVisualEffect {
                        source:
                            crate::game_state::presentation_event::MonsterHitVisualSource::Tower {
                                kind: crate::game_state::tower::TowerKind::StraightFlush,
                            },
                        start_xy: [26.0, 18.0],
                        impact_xy: [29.0, 20.0],
                        chain_delay_ms: 20,
                    }
                ))
        );
        assert!(
            effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnMonsterHitVisualEffect {
                        source:
                            crate::game_state::presentation_event::MonsterHitVisualSource::Tower {
                                kind: crate::game_state::tower::TowerKind::Straight,
                            },
                        start_xy: [17.0, 18.0],
                        impact_xy: [26.0, 18.0],
                        ..
                    }
                ))
        );
    }
}
