use super::*;
use crate::PresentationInstant;
use crate::game_state::attack::{InFlightAttack, InFlightAttackKind, ProjectileHitEffect};
use crate::game_state::effect_event::{GameEffectEvent, ParticleSpawnRequest};
use crate::game_state::projectile::{PROJECTILE_COLLISION_RADIUS, ProjectileBehavior};
use crate::{Damage, FixedRatio, MonsterId, RatioProduct, WorldCoord, segment_hits_point};
use std::collections::HashMap;

pub fn update_in_flight_attacks(
    game_state: &mut GameState,
    presentation_instant: PresentationInstant,
) {
    game_state.in_flight_attacks.sort_by_key(|attack| attack.id);
    process_timed_attacks(game_state, presentation_instant);
    process_laser_attacks(game_state, presentation_instant);
    move_spatial_attacks(game_state, presentation_instant);
}

fn process_timed_attacks(game_state: &mut GameState, presentation_instant: PresentationInstant) {
    let sim_tick = game_state.sim_tick();
    let mut due: Vec<InFlightAttack> = Vec::new();

    game_state.in_flight_attacks.retain(|attack| {
        if let InFlightAttackKind::Timed(timed) = &attack.kind
            && timed.execute_at <= sim_tick
        {
            due.push(attack.clone());
            return false;
        }
        true
    });

    let monster_index_by_id: HashMap<_, _> = game_state
        .monsters
        .iter()
        .enumerate()
        .map(|(i, m)| (m.id(), i))
        .collect();

    let mut hits = Vec::new();
    for attack in due {
        let InFlightAttackKind::Timed(timed) = &attack.kind else {
            continue;
        };
        let Some(&target_idx) = monster_index_by_id.get(&timed.target_monster_id) else {
            continue;
        };
        let target_xy = game_state.monsters[target_idx].center_world_xy();
        let target_xy_presentation = target_xy.as_map_coord_f32();

        game_state.effect_events.push(GameEffectEvent::PlaySound(
            crate::sound::EmitSoundParams::one_shot(
                crate::sound::deterministic_knife_slash(),
                crate::sound::SoundGroup::Sfx,
                crate::sound::VolumePreset::Low,
                crate::sound::SpatialMode::Spatial {
                    position: target_xy_presentation,
                },
            ),
        ));
        let delay_ms = 45_i64;
        game_state
            .effect_events
            .push(GameEffectEvent::PlaySoundDelayed(
                crate::sound::EmitSoundParams::one_shot(
                    crate::sound::deterministic_knife_slash(),
                    crate::sound::SoundGroup::Sfx,
                    crate::sound::VolumePreset::Low,
                    crate::sound::SpatialMode::Spatial {
                        position: target_xy_presentation,
                    },
                ),
                Duration::from_millis(delay_ms),
            ));

        if !attack.damage.is_zero() {
            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::DamageText(
                        crate::game_state::field_particle::DamageTextParticle::new(
                            target_xy_presentation,
                            attack.damage.as_f32(),
                            presentation_instant.as_namui(),
                        ),
                    ),
                ));
        }

        hits.push(MonsterHit {
            target_idx,
            damage: attack.damage,
            at_xy: target_xy,
            source_tower: attack.source_tower,
            on_hit_splashes: attack.on_hit_splashes.clone(),
        });
    }

    apply_monster_damage_and_remove_dead(game_state, hits, presentation_instant);
}

fn process_laser_attacks(game_state: &mut GameState, presentation_instant: PresentationInstant) {
    let mut due: Vec<InFlightAttack> = Vec::new();

    game_state.in_flight_attacks.retain(|attack| {
        if matches!(&attack.kind, InFlightAttackKind::Laser(_)) {
            due.push(attack.clone());
            return false;
        }
        true
    });

    let monster_index_by_id: HashMap<_, _> = game_state
        .monsters
        .iter()
        .enumerate()
        .map(|(i, m)| (m.id(), i))
        .collect();

    let mut hits = Vec::new();
    for attack in due {
        let InFlightAttackKind::Laser(beam) = &attack.kind else {
            continue;
        };

        // 레이저 빔 시각 이펙트 (데미지 적용 전에 발행)
        game_state.effect_events.push(GameEffectEvent::PlaySound(
            crate::sound::EmitSoundParams::one_shot(
                crate::sound::deterministic_red_laser_shot(),
                crate::sound::SoundGroup::Sfx,
                crate::sound::VolumePreset::Minimum,
                crate::sound::SpatialMode::Spatial {
                    position: beam.start_xy.as_map_coord_f32(),
                },
            ),
        ));
        game_state.effect_events.push(GameEffectEvent::PlaySound(
            crate::sound::EmitSoundParams::one_shot(
                crate::sound::deterministic_red_laser_shot(),
                crate::sound::SoundGroup::Sfx,
                crate::sound::VolumePreset::Minimum,
                crate::sound::SpatialMode::Spatial {
                    position: beam.end_xy.as_map_coord_f32(),
                },
            ),
        ));
        game_state
            .effect_events
            .push(GameEffectEvent::SpawnLaserBeam(
                (
                    beam.start_xy.x as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                    beam.start_xy.y as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                ),
                (
                    beam.end_xy.x as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                    beam.end_xy.y as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                ),
                presentation_instant,
            ));

        let Some(&target_idx) = monster_index_by_id.get(&beam.target_monster_id) else {
            continue;
        };
        let target_xy = game_state.monsters[target_idx].center_world_xy();
        let target_xy_presentation = target_xy.as_map_coord_f32();

        if !attack.damage.is_zero() {
            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::DamageText(
                        crate::game_state::field_particle::DamageTextParticle::new(
                            target_xy_presentation,
                            attack.damage.as_f32(),
                            presentation_instant.as_namui(),
                        ),
                    ),
                ));
        }

        hits.push(MonsterHit {
            target_idx,
            damage: attack.damage,
            at_xy: target_xy,
            source_tower: attack.source_tower,
            on_hit_splashes: attack.on_hit_splashes.clone(),
        });
    }

    apply_monster_damage_and_remove_dead(game_state, hits, presentation_instant);
}

fn move_spatial_attacks(game_state: &mut GameState, presentation_instant: PresentationInstant) {
    let mut hits: Vec<MonsterHit> = Vec::new();

    {
        let GameState {
            in_flight_attacks,
            monsters,
            ..
        } = game_state;

        let monster_index_by_indicator: HashMap<_, _> = monsters
            .iter()
            .enumerate()
            .map(|(i, m)| (m.projectile_target_indicator, i))
            .collect();

        in_flight_attacks.retain_mut(|attack| {
            let InFlightAttackKind::Spatial(spatial) = &mut attack.kind else {
                return true; // Timed/Laser は別処理
            };

            let start_xy = spatial.xy;

            let Some(&monster_index) = monster_index_by_indicator.get(&spatial.target_indicator)
            else {
                return false;
            };

            let monster_xy = monsters[monster_index].center_world_xy();

            match spatial.behavior {
                ProjectileBehavior::Direct => spatial.move_by(monster_xy),
                ProjectileBehavior::Homing { .. } => spatial.move_homing(monster_xy),
            }
            let end_xy = spatial.xy;
            if !segment_hits_point(start_xy, end_xy, monster_xy, PROJECTILE_COLLISION_RADIUS)
                && end_xy != monster_xy
            {
                let moved_distance = (end_xy - start_xy).length().raw() as f32
                    / crate::world::WORLD_UNITS_PER_TILE as f32;
                game_state
                    .effect_events
                    .push(GameEffectEvent::SyncProjectileTrailState {
                        projectile_id: attack.id,
                        trail: spatial.trail,
                        start_xy: start_xy.as_map_coord_f32(),
                        end_xy: end_xy.as_map_coord_f32(),
                        moved_distance,
                        dt_secs: 1.0 / crate::world::SIM_TICKS_PER_SECOND as f32,
                        presentation_instant,
                    });
                return true;
            }

            // 도달 → 피격 처리
            let damage = attack.damage;

            if let Some(sound_fn) = spatial.trail.hit_sound() {
                game_state.effect_events.push(GameEffectEvent::PlaySound(
                    sound::EmitSoundParams::one_shot(
                        sound_fn(),
                        sound::SoundGroup::Sfx,
                        sound::VolumePreset::Minimum,
                        sound::SpatialMode::Spatial {
                            position: monster_xy.as_map_coord_f32(),
                        },
                    ),
                ));
            }

            if !damage.is_zero() {
                game_state
                    .effect_events
                    .push(GameEffectEvent::SpawnParticle(
                        ParticleSpawnRequest::DamageText(field_particle::DamageTextParticle::new(
                            monster_xy.as_map_coord_f32(),
                            damage.as_f32(),
                            presentation_instant.as_namui(),
                        )),
                    ));
            }

            match spatial.hit_effect {
                ProjectileHitEffect::TrashBounce => {
                    for p in field_particle::emitter::create_bounce_particles(
                        spatial.projectile_kind,
                        (
                            start_xy.x as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            start_xy.y as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                        ),
                        (
                            monster_xy.x as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                            monster_xy.y as f32 / crate::world::WORLD_UNITS_PER_TILE as f32,
                        ),
                        presentation_instant.as_namui(),
                    ) {
                        game_state
                            .effect_events
                            .push(GameEffectEvent::SpawnParticle(ParticleSpawnRequest::Trash(
                                p,
                            )));
                    }
                }
                hit_effect => {
                    game_state
                        .effect_events
                        .push(GameEffectEvent::SpawnProjectileHitEffect(
                            hit_effect,
                            monster_xy.as_map_coord_f32(),
                            presentation_instant,
                        ));
                }
            }

            hits.push(MonsterHit {
                target_idx: monster_index,
                damage,
                at_xy: monster_xy,
                source_tower: attack.source_tower,
                on_hit_splashes: attack.on_hit_splashes.clone(),
            });
            false
        });
    }

    apply_monster_damage_and_remove_dead(game_state, hits, presentation_instant);
}

/// 모든 공격 경로(Spatial/Timed/Laser)의 공통 종착점.
/// 데미지 적용 → 타워 데미지 기록 → 사망 판정 → monster_death 처리.
fn apply_monster_damage_and_remove_dead(
    game_state: &mut GameState,
    hits: Vec<MonsterHit>,
    presentation_instant: PresentationInstant,
) {
    let mut dead: Vec<(MonsterId, WorldCoord)> = Vec::new();

    let monster_centers: Vec<WorldCoord> = game_state
        .monsters
        .iter()
        .map(|monster| monster.center_world_xy())
        .collect();
    let mut hits = expand_on_hit_splashes(&monster_centers, hits);
    hits.sort_by_key(|hit| {
        (
            game_state
                .monsters
                .get(hit.target_idx)
                .map(|m| m.id())
                .unwrap_or(MonsterId::from_raw(u64::MAX)),
            hit.target_idx,
        )
    });

    for hit in hits {
        if hit.target_idx >= game_state.monsters.len() {
            continue;
        }

        let hp_before = game_state.monsters[hit.target_idx].hp;
        game_state.monsters[hit.target_idx].get_damage(hit.damage);
        let hp_dealt = hp_before.saturating_sub(game_state.monsters[hit.target_idx].hp);
        if !hp_dealt.is_zero()
            && let GameFlow::Defense(defense_flow) = &mut game_state.flow
        {
            defense_flow.stage_progress.processed_hp = defense_flow
                .stage_progress
                .processed_hp
                .saturating_add(hp_dealt);
        }

        if !hit.damage.is_zero() {
            game_state.effect_events.push(GameEffectEvent::PlaySound(
                crate::sound::EmitSoundParams::one_shot(
                    crate::sound::deterministic_whoop(),
                    crate::sound::SoundGroup::Sfx,
                    crate::sound::VolumePreset::Minimum,
                    crate::sound::SpatialMode::Spatial {
                        position: hit.at_xy.as_map_coord_f32(),
                    },
                ),
            ));

            if let Some(tower) = hit.source_tower {
                game_state.record_tower_damage(&tower, hit.damage);
            }
        }

        if game_state.monsters[hit.target_idx].dead() {
            dead.push((game_state.monsters[hit.target_idx].id(), hit.at_xy));
        }
    }

    dead.sort_by_key(|(id, _)| *id);
    dead.dedup_by_key(|(id, _)| *id);
    for (target_id, target_xy) in dead {
        let Some(target_idx) = game_state
            .monsters
            .iter()
            .position(|monster| monster.id() == target_id)
        else {
            continue;
        };
        super::monster_death::handle_monster_death(
            game_state,
            target_idx,
            target_xy,
            presentation_instant,
        );
    }
    game_state.monsters.sort_by_key(|monster| monster.id());
}

fn expand_on_hit_splashes(
    monster_centers: &[WorldCoord],
    hits: Vec<MonsterHit>,
) -> Vec<MonsterHit> {
    if hits.iter().all(|hit| hit.on_hit_splashes.is_empty()) {
        return hits;
    }

    let mut expanded = Vec::with_capacity(hits.len());
    for mut hit in hits {
        if !hit.on_hit_splashes.is_empty() {
            for (index, center) in monster_centers.iter().enumerate() {
                if index == hit.target_idx {
                    continue;
                }
                let damage_pct_raw: i64 = hit
                    .on_hit_splashes
                    .iter()
                    .filter(|splash| {
                        (*center - hit.at_xy).length_squared()
                            <= (splash.radius.raw() as u128)
                                .saturating_mul(splash.radius.raw() as u128)
                    })
                    .map(|splash| splash.damage_pct.raw())
                    .sum();
                if damage_pct_raw <= 0 {
                    continue;
                }
                expanded.push(MonsterHit {
                    target_idx: index,
                    damage: Damage::from_raw(
                        RatioProduct::one()
                            .with(FixedRatio::from_raw(damage_pct_raw))
                            .apply_raw(hit.damage.raw()),
                    ),
                    at_xy: *center,
                    source_tower: hit.source_tower,
                    on_hit_splashes: Vec::new(),
                });
            }
            hit.on_hit_splashes.clear();
        }
        expanded.push(hit);
    }
    expanded
}

pub(super) fn apply_area_damage_events(
    game_state: &mut GameState,
    events: Vec<super::AreaDamageEvent>,
    presentation_instant: PresentationInstant,
) {
    if events.is_empty() {
        return;
    }

    let monster_centers: Vec<WorldCoord> = game_state
        .monsters
        .iter()
        .map(|monster| monster.center_world_xy())
        .collect();
    let hits = expand_area_damage_events(&monster_centers, events);
    apply_monster_damage_and_remove_dead(game_state, hits, presentation_instant);
}

fn expand_area_damage_events(
    monster_centers: &[WorldCoord],
    events: Vec<super::AreaDamageEvent>,
) -> Vec<MonsterHit> {
    let mut hits = Vec::new();
    for event in events {
        for (index, center) in monster_centers.iter().enumerate() {
            let damage_pct_raw: i64 = event
                .splashes
                .iter()
                .filter(|splash| {
                    (*center - event.center).length_squared()
                        <= (splash.radius.raw() as u128).saturating_mul(splash.radius.raw() as u128)
                })
                .map(|splash| splash.damage_pct.raw())
                .sum();
            if damage_pct_raw <= 0 {
                continue;
            }
            hits.push(MonsterHit {
                target_idx: index,
                damage: Damage::from_raw(
                    RatioProduct::one()
                        .with(FixedRatio::from_raw(damage_pct_raw))
                        .apply_raw(event.damage.raw()),
                ),
                at_xy: *center,
                source_tower: Some(event.source_tower),
                on_hit_splashes: Vec::new(),
            });
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::EngravingSplash;

    fn hit(target_idx: usize, at_xy: WorldCoord, splash: Option<EngravingSplash>) -> MonsterHit {
        MonsterHit {
            target_idx,
            damage: Damage::from_integer(100),
            at_xy,
            source_tower: None,
            on_hit_splashes: splash.into_iter().collect(),
        }
    }

    #[test]
    fn hits_without_splash_pass_through_untouched() {
        let centers = vec![WorldCoord::from_tile(0, 0), WorldCoord::from_tile(1, 0)];

        let expanded = expand_on_hit_splashes(&centers, vec![hit(0, centers[0], None)]);

        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].damage, Damage::from_integer(100));
    }

    #[test]
    fn splash_adds_scaled_hits_for_monsters_inside_the_radius() {
        let centers = vec![
            WorldCoord::from_tile(0, 0),
            WorldCoord::from_tile(1, 0),
            WorldCoord::from_tile(9, 0),
        ];
        let splash = EngravingSplash {
            radius: crate::WorldDistance::from_tiles(2),
            damage_pct: FixedRatio::from_raw(500_000),
        };

        let expanded = expand_on_hit_splashes(&centers, vec![hit(0, centers[0], Some(splash))]);

        assert_eq!(expanded.len(), 2);
        let splashed = expanded
            .iter()
            .find(|hit| hit.target_idx == 1)
            .expect("반경 안 몬스터가 파생 타격을 받아야 한다");
        assert_eq!(splashed.damage, Damage::from_integer(50));
        assert!(splashed.on_hit_splashes.is_empty());
        assert!(expanded.iter().all(|hit| hit.target_idx != 2));
    }

    #[test]
    fn area_damage_hits_near_the_tower_instead_of_near_the_target() {
        let tower_xy = WorldCoord::from_tile(0, 0);
        let centers = vec![
            WorldCoord::from_tile(8, 0),
            WorldCoord::from_tile(1, 0),
            WorldCoord::from_tile(3, 0),
        ];
        let event = super::AreaDamageEvent {
            center: tower_xy,
            damage: Damage::from_integer(100),
            source_tower: crate::game_state::attack::TowerInfo {
                id: crate::TowerId::from_raw(1),
                kind: crate::game_state::tower::TowerKind::High,
                rank: None,
                suit: None,
            },
            splashes: vec![EngravingSplash {
                radius: crate::WorldDistance::from_tiles(2),
                damage_pct: FixedRatio::from_raw(300_000),
            }],
        };

        let expanded = expand_area_damage_events(&centers, vec![event]);

        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].target_idx, 1);
        assert_eq!(expanded[0].damage, Damage::from_integer(30));
    }

    #[test]
    fn duplicate_splashes_stack_damage_percentages() {
        let centers = vec![WorldCoord::from_tile(0, 0), WorldCoord::from_tile(1, 0)];
        let splashes = vec![
            EngravingSplash {
                radius: crate::WorldDistance::from_tiles(2),
                damage_pct: FixedRatio::from_raw(300_000),
            },
            EngravingSplash {
                radius: crate::WorldDistance::from_tiles(2),
                damage_pct: FixedRatio::from_raw(400_000),
            },
        ];

        let expanded = expand_on_hit_splashes(
            &centers,
            vec![MonsterHit {
                target_idx: 0,
                damage: Damage::from_integer(100),
                at_xy: centers[0],
                source_tower: None,
                on_hit_splashes: splashes,
            }],
        );

        let nearby = expanded
            .iter()
            .find(|hit| hit.target_idx == 1)
            .expect("중복 스플래시가 합산되어야 한다");
        assert_eq!(nearby.damage, Damage::from_integer(70));
    }

    #[test]
    fn splash_never_hits_its_own_primary_target_twice() {
        let centers = vec![WorldCoord::from_tile(0, 0)];
        let splash = EngravingSplash {
            radius: crate::WorldDistance::from_tiles(5),
            damage_pct: FixedRatio::ONE,
        };

        let expanded = expand_on_hit_splashes(&centers, vec![hit(0, centers[0], Some(splash))]);

        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].damage, Damage::from_integer(100));
    }
}
