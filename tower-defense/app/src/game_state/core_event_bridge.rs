use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::modal::deck::CardSelectionState;
use crate::game_state::{GameState, UserModal};

pub(crate) fn consume_headed(
    game_state: &mut GameState,
    events: impl IntoIterator<Item = td_core::CoreEvent>,
    presentation_instant: crate::PresentationInstant,
) {
    for event in events {
        match event {
            td_core::CoreEvent::CardServiceSelectionRequested {
                service_kind,
                step_counts,
            } => {
                game_state.open_card_service_selection_from_core_event(&service_kind, &step_counts)
            }
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
                crate::game_state::presentation_effect::apply_earn_gold_sound(game_state, reward);
            }
            td_core::CoreEvent::TowerAttack {
                tower_id,
                attack_ids,
                projectile_attack_ids,
                ..
            } => {
                game_state.presentation_metadata.transition_tower_animation(
                    crate::TowerId::from_raw(tower_id),
                    crate::game_state::tower::AnimationKind::Attack,
                    game_state.sim_tick(),
                );
                let tower_events =
                    tower_attack_presentation_events(game_state.raw_core_state(), &attack_ids);
                for event in tower_events {
                    game_state.push_presentation_event(event);
                }
                let projectile_events = {
                    let raw_state = game_state.raw_core.state();
                    let headed_attacks = game_state.presentation_in_flight_attacks();
                    let presentation_metadata = &mut game_state.presentation_metadata;
                    projectile_attack_ids
                        .iter()
                        .filter_map(|attack_id| {
                            projectile_spawn_presentation_event(
                                raw_state,
                                &headed_attacks,
                                presentation_metadata,
                                *attack_id,
                            )
                        })
                        .collect::<Vec<_>>()
                };
                for event in projectile_events {
                    game_state.push_presentation_event(event);
                }
            }
            td_core::CoreEvent::ProjectileMoved {
                attack_id,
                start_xy,
                end_xy,
            } => {
                let Some(metadata) = game_state
                    .presentation_metadata
                    .projectile(crate::AttackId::from_raw(attack_id))
                else {
                    continue;
                };
                let dx = (end_xy[0] - start_xy[0]) as f32;
                let dy = (end_xy[1] - start_xy[1]) as f32;
                let moved_distance =
                    (dx * dx + dy * dy).sqrt() / crate::world::WORLD_UNITS_PER_TILE as f32;
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::SyncProjectileTrailState {
                        projectile_id: crate::AttackId::from_raw(attack_id),
                        trail: metadata.trail,
                        start_xy: [
                            start_xy[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            start_xy[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                        ],
                        end_xy: [
                            end_xy[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            end_xy[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                        ],
                        moved_distance,
                        dt_secs: 1.0 / crate::world::SIM_TICKS_PER_SECOND as f32,
                    },
                );
            }
            td_core::CoreEvent::TimedAttackExecuted { position } => {
                let position = [
                    position[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                    position[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                ];
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::PlaySoundCue {
                        cue: crate::game_state::SoundCue::KnifeSlash,
                        position: Some(position),
                        volume: crate::game_state::SoundVolume::Low,
                        max_duration_ms: None,
                    },
                );
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::PlaySoundCueDelayed {
                        cue: crate::game_state::SoundCue::KnifeSlash,
                        position: Some(position),
                        volume: crate::game_state::SoundVolume::Low,
                        delay_ms: 45,
                    },
                );
            }
            td_core::CoreEvent::ProjectileHit {
                attack_id,
                position,
            } => {
                let attack_id = crate::AttackId::from_raw(attack_id);
                let Some(hit_effect) = game_state
                    .presentation_metadata
                    .projectile(attack_id)
                    .map(|metadata| metadata.hit_effect)
                else {
                    continue;
                };
                game_state
                    .presentation_metadata
                    .remove_projectile(attack_id);
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::SpawnProjectileHitEffect(
                        hit_effect,
                        [
                            position[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            position[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                        ],
                    ),
                );
            }
            td_core::CoreEvent::MonsterSpawned { .. } => {
                game_state.push_presentation_event(
                    crate::game_state::PresentationEvent::AnimateBase(
                        crate::game_state::BaseAnimationEvent::EnemySpawn,
                    ),
                );
            }
            td_core::CoreEvent::DefenseStarted { .. } => {
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
        }
    }
}

fn projectile_spawn_presentation_event(
    raw_state: &td_core::CoreState,
    headed_attacks: &[crate::game_state::attack::InFlightAttack],
    presentation_metadata: &mut crate::game_state::presentation_metadata::PresentationMetadataStore,
    attack_id: u64,
) -> Option<crate::game_state::PresentationEvent> {
    let raw_attack = raw_state
        .in_flight_attacks()
        .iter()
        .find(|attack| attack.id == attack_id)?;
    let td_core::InFlightAttackKindState::Spatial(spatial) = &raw_attack.kind else {
        return None;
    };
    let headed_attack = headed_attacks
        .iter()
        .find(|attack| attack.id.raw() == attack_id)?;
    let crate::game_state::attack::InFlightAttackKind::Spatial(presentation) = &headed_attack.kind
    else {
        return None;
    };
    let start_xy = [
        spatial.position[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
        spatial.position[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
    ];
    let end_xy = [
        (spatial.position[0] + spatial.velocity[0]) as f32
            / crate::world::WORLD_UNITS_PER_TILE as f32,
        (spatial.position[1] + spatial.velocity[1]) as f32
            / crate::world::WORLD_UNITS_PER_TILE as f32,
    ];
    let projectile_kind = presentation.projectile_kind;
    let trail = presentation.trail;
    let hit_effect = presentation.hit_effect;
    presentation_metadata.insert_projectile(
        crate::AttackId::from_raw(attack_id),
        projectile_kind,
        trail,
        hit_effect,
    );
    Some(crate::game_state::PresentationEvent::SpawnProjectileTrail {
        trail,
        start_xy,
        end_xy,
        count: 1,
    })
}

fn tower_attack_presentation_events(
    state: &td_core::CoreState,
    attack_ids: &[u64],
) -> Vec<crate::game_state::PresentationEvent> {
    let mut events = Vec::new();
    for attack in state
        .in_flight_attacks()
        .iter()
        .filter(|attack| attack_ids.contains(&attack.id))
    {
        match &attack.kind {
            td_core::InFlightAttackKindState::Laser(laser) => {
                let start = [
                    laser.start_xy[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                    laser.start_xy[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                ];
                let end = [
                    laser.end_xy[0] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                    laser.end_xy[1] as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                ];
                events.push(crate::game_state::PresentationEvent::PlaySoundCue {
                    cue: crate::game_state::SoundCue::RedLaserShot,
                    position: Some(start),
                    volume: crate::game_state::SoundVolume::Minimum,
                    max_duration_ms: None,
                });
                events.push(crate::game_state::PresentationEvent::PlaySoundCue {
                    cue: crate::game_state::SoundCue::RedLaserShot,
                    position: Some(end),
                    volume: crate::game_state::SoundVolume::Minimum,
                    max_duration_ms: None,
                });
                events.push(crate::game_state::PresentationEvent::SpawnLaserBeam(
                    (start[0], start[1]),
                    (end[0], end[1]),
                ));
            }
            td_core::InFlightAttackKindState::Spatial(_) => {}
            td_core::InFlightAttackKindState::Timed(timed) => {
                let Some(source_tower) = attack.source_tower else {
                    continue;
                };
                let Some(monster) = state
                    .monsters()
                    .iter()
                    .find(|monster| monster.id == timed.target_monster_id)
                else {
                    continue;
                };
                let target_xy = [
                    (monster.move_on_route.map_coord[0] + td_core::WORLD_UNITS_PER_TILE / 2) as f32
                        / crate::world::WORLD_UNITS_PER_TILE as f32,
                    (monster.move_on_route.map_coord[1] + td_core::WORLD_UNITS_PER_TILE / 2) as f32
                        / crate::world::WORLD_UNITS_PER_TILE as f32,
                ];
                events.push(
                    crate::game_state::PresentationEvent::SpawnRoyalStraightFlushVisual {
                        tower_id: crate::TowerId::from_raw(source_tower.tower_id),
                        target_xy,
                        target_monster_id: crate::MonsterId::from_raw(timed.target_monster_id),
                        sim_tick: crate::SimTick::from_ticks(state.sim_tick().ticks()),
                    },
                );
            }
        }
    }
    events
}

pub(crate) fn consume_headless(events: impl IntoIterator<Item = td_core::CoreEvent>) {
    for _event in events {}
}

pub(crate) fn card_service_selection(
    game_state: &GameState,
    service_kind: &str,
    step_counts: &[usize],
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
    Some(UserModal::Deck(crate::game_state::modal::deck::DeckModal {
        deck_kind: crate::game_state::modal::deck::DeckKind::Deck,
        selection: Some(CardSelectionState::new(steps, service)),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_service_event_uses_authoritative_step_counts() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let modal = card_service_selection(&game_state, "eraser", &[2]).expect("eraser modal");
        let UserModal::Deck(deck) = modal else {
            panic!("expected deck modal");
        };
        assert_eq!(deck.selection.expect("selection").steps[0].count, 2);
    }

    #[test]
    fn invalid_card_service_event_is_ignored() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        assert!(card_service_selection(&game_state, "unknown", &[1]).is_none());
        assert!(card_service_selection(&game_state, "eraser", &[1, 2]).is_none());
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
                    attack_kind: 0,
                    attack_ids: vec![2],
                    projectile_attack_ids: Vec::new(),
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
    fn tower_attack_laser_presentation_uses_raw_attack_coordinates() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let mut raw_state = game_state.authoritative_core_state();
        raw_state
            .edit_snapshot(|parts| {
                parts.next_entity_id = td_core::EntityIdAllocator::from_next_id(10_000);
                parts.in_flight_attacks.push(td_core::InFlightAttackState {
                    id: 9,
                    damage_raw: 1,
                    source_tower: None,
                    kind: td_core::InFlightAttackKindState::Laser(td_core::LaserAttackState {
                        start_xy: [1_000_000, 2_000_000],
                        end_xy: [3_000_000, 4_000_000],
                        created_at: 1,
                        target_monster_id: 2,
                    }),
                    on_hit_splashes: Vec::new(),
                });
            })
            .expect("test attack edit must preserve a valid snapshot");
        let events = tower_attack_presentation_events(&raw_state, &[9]);

        assert!(events.iter().any(|event| matches!(
            event,
            crate::game_state::PresentationEvent::SpawnLaserBeam((1.0, 2.0), (3.0, 4.0))
        )));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::PlaySoundCue { .. }
                ))
                .count(),
            2
        );
    }

    #[test]
    fn timed_tower_attack_queues_royal_straight_flush_visual() {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let mut raw_state = game_state.authoritative_core_state();
        let route = raw_state.route().clone();
        raw_state
            .edit_snapshot(|parts| {
                parts.next_entity_id = td_core::EntityIdAllocator::from_next_id(10_000);
                parts.monsters.push(td_core::MonsterState {
                    id: 2,
                    move_on_route: td_core::MoveOnRouteState {
                        route,
                        route_index: 0,
                        route_progress_raw: 0,
                        map_coord: [0, 0],
                        velocity_raw: 0,
                        movement_remainder: 0,
                        motion_revision: 0,
                    },
                    kind: 0,
                    hp_raw: 1,
                    max_hp_raw: 1,
                    stage_progress_counted: false,
                    skills: Vec::new(),
                    status_effects: Vec::new(),
                    damage_raw: 0,
                    reward: 0,
                })
            })
            .expect("test monster edit must preserve a valid snapshot");
        raw_state
            .edit_snapshot(|parts| {
                parts.in_flight_attacks.push(td_core::InFlightAttackState {
                    id: 9,
                    damage_raw: 1,
                    source_tower: Some(td_core::AttackSourceState {
                        tower_id: 1,
                        tower_kind: 10,
                        rank: None,
                        suit: None,
                    }),
                    kind: td_core::InFlightAttackKindState::Timed(td_core::TimedAttackState {
                        target_monster_id: 2,
                        execute_at: 10,
                    }),
                    on_hit_splashes: Vec::new(),
                })
            })
            .expect("test attack edit must preserve a valid snapshot");

        let events = tower_attack_presentation_events(&raw_state, &[9]);
        assert!(events.iter().any(|event| matches!(
            event,
            crate::game_state::PresentationEvent::SpawnRoyalStraightFlushVisual {
                tower_id,
                target_xy: [x, y],
                target_monster_id,
                ..
            } if *tower_id == crate::TowerId::from_raw(1)
                && *target_monster_id == crate::MonsterId::from_raw(2)
                && *x == 0.5
                && *y == 0.5
        )));
    }

    #[test]
    fn projectile_events_use_metadata_registered_by_tower_attack() {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        let attack_id = crate::AttackId::from_raw(9);
        let spatial_state = td_core::SpatialAttackState {
            position: [1_000_000, 2_000_000],
            target_monster_id: 2,
            velocity: [100_000, 200_000],
            behavior: td_core::SpatialAttackBehaviorState::Direct,
            movement_remainder: 0,
            stable_key: 9,
        };
        let headed_attack = crate::game_state::attack::InFlightAttack {
            id: attack_id,
            damage: crate::Damage::from_raw(1),
            source_tower: None,
            kind: crate::game_state::attack::InFlightAttackKind::Spatial(
                crate::game_state::attack::SpatialAttack::from_core_state(
                    spatial_state,
                    crate::game_state::projectile::ProjectileKind::Cards00,
                    crate::game_state::projectile::ProjectileTrail::Sparkle,
                    crate::game_state::attack::ProjectileHitEffect::CardBurst,
                ),
            ),
            on_hit_splashes: Vec::new(),
        };
        game_state.in_flight_attacks.push(headed_attack);
        game_state.sync_raw_core_from_projection();
        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::TowerAttack {
                tower_id: 1,
                target_id: 2,
                attack_kind: 0,
                attack_ids: vec![attack_id.raw()],
                projectile_attack_ids: vec![attack_id.raw()],
            }],
            crate::PresentationInstant::zero(),
        );
        game_state.in_flight_attacks.clear();

        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::ProjectileMoved {
                attack_id: attack_id.raw(),
                start_xy: [1_000_000, 2_000_000],
                end_xy: [1_100_000, 2_200_000],
            }],
            crate::PresentationInstant::zero(),
        );
        let moved_effects = game_state.take_pending_action_effects();
        assert!(
            moved_effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SyncProjectileTrailState {
                        projectile_id,
                        trail: crate::game_state::projectile::ProjectileTrail::Sparkle,
                        ..
                    } if *projectile_id == attack_id
                ))
        );

        consume_headed(
            &mut game_state,
            [td_core::CoreEvent::ProjectileHit {
                attack_id: attack_id.raw(),
                position: [1_100_000, 2_200_000],
            }],
            crate::PresentationInstant::zero(),
        );
        let hit_effects = game_state.take_pending_action_effects();
        assert!(
            hit_effects
                .presentation_events
                .events
                .iter()
                .any(|event| matches!(
                    event,
                    crate::game_state::PresentationEvent::SpawnProjectileHitEffect(
                        crate::game_state::attack::ProjectileHitEffect::CardBurst,
                        _,
                    )
                ))
        );
        assert!(
            game_state
                .presentation_metadata
                .projectile(attack_id)
                .is_none()
        );
    }
}
