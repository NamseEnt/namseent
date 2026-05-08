use super::*;
use crate::game_state::attack::{HitSound, InFlightAttack, InFlightAttackKind, ProjectileHitEffect};
use crate::game_state::effect_event::{GameEffectEvent, ParticleSpawnRequest};
use crate::game_state::projectile::ProjectileBehavior;
use rand::Rng;
use std::collections::HashMap;

pub fn update_in_flight_attacks(game_state: &mut GameState, dt: Duration, now: Instant) {
    process_timed_attacks(game_state, now);
    process_laser_attacks(game_state, now);
    move_spatial_attacks(game_state, dt, now);
}

fn process_timed_attacks(game_state: &mut GameState, now: Instant) {
    let mut rng = rand::thread_rng();
    let mut due: Vec<InFlightAttack> = Vec::new();

    game_state.in_flight_attacks.retain(|attack| {
        if let InFlightAttackKind::Timed(timed) = &attack.kind
            && timed.execute_at <= now
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
        let target_xy = game_state.monsters[target_idx].center_xy_tile();

        match timed.hit_sound {
            HitSound::KnifeSlash => {
                game_state.effect_events.push(GameEffectEvent::PlaySound(
                    crate::sound::EmitSoundParams::one_shot(
                        crate::sound::random_knife_slash(),
                        crate::sound::SoundGroup::Sfx,
                        crate::sound::VolumePreset::Low,
                        crate::sound::SpatialMode::Spatial { position: target_xy },
                    ),
                ));
                let delay_ms = rng.gen_range(30_i64..=60_i64);
                game_state.effect_events.push(GameEffectEvent::PlaySoundDelayed(
                    crate::sound::EmitSoundParams::one_shot(
                        crate::sound::random_knife_slash(),
                        crate::sound::SoundGroup::Sfx,
                        crate::sound::VolumePreset::Low,
                        crate::sound::SpatialMode::Spatial { position: target_xy },
                    ),
                    Duration::from_millis(delay_ms),
                ));
            }
        }

        if attack.damage > 0.0 {
            game_state.effect_events.push(GameEffectEvent::SpawnParticle(
                ParticleSpawnRequest::DamageText(
                    crate::game_state::field_particle::DamageTextParticle::new(
                        target_xy,
                        attack.damage,
                        now,
                    ),
                ),
            ));
        }

        hits.push(MonsterHit {
            target_idx,
            damage: attack.damage,
            at_xy: target_xy,
            source_tower: attack.source_tower,
        });
    }

    apply_monster_damage_and_remove_dead(game_state, hits);
}

fn process_laser_attacks(game_state: &mut GameState, now: Instant) {
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
                crate::sound::random_red_laser_shot(),
                crate::sound::SoundGroup::Sfx,
                crate::sound::VolumePreset::Minimum,
                crate::sound::SpatialMode::Spatial {
                    position: crate::MapCoordF32::new(beam.start_xy.0, beam.start_xy.1),
                },
            ),
        ));
        game_state.effect_events.push(GameEffectEvent::PlaySound(
            crate::sound::EmitSoundParams::one_shot(
                crate::sound::random_red_laser_shot(),
                crate::sound::SoundGroup::Sfx,
                crate::sound::VolumePreset::Minimum,
                crate::sound::SpatialMode::Spatial {
                    position: crate::MapCoordF32::new(beam.end_xy.0, beam.end_xy.1),
                },
            ),
        ));
        game_state.effect_events.push(GameEffectEvent::SpawnLaserBeam(
            beam.start_xy,
            beam.end_xy,
            beam.created_at,
        ));

        let Some(&target_idx) = monster_index_by_id.get(&beam.target_monster_id) else {
            continue;
        };
        let target_xy = game_state.monsters[target_idx].center_xy_tile();

        if attack.damage > 0.0 {
            game_state.effect_events.push(GameEffectEvent::SpawnParticle(
                ParticleSpawnRequest::DamageText(
                    crate::game_state::field_particle::DamageTextParticle::new(
                        target_xy,
                        attack.damage,
                        now,
                    ),
                ),
            ));
        }

        hits.push(MonsterHit {
            target_idx,
            damage: attack.damage,
            at_xy: target_xy,
            source_tower: attack.source_tower,
        });
    }

    apply_monster_damage_and_remove_dead(game_state, hits);
}

fn move_spatial_attacks(game_state: &mut GameState, dt: Duration, now: Instant) {
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
                // 타겟 몬스터가 이미 사망 → 투사체를 파티클로 흩날림
                game_state.effect_events.push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::Projectile(field_particle::ProjectileParticle::new(
                        spatial.xy,
                        spatial.projectile_kind,
                        spatial.rotation,
                        spatial.rotation_speed,
                        spatial.velocity,
                        now,
                        Duration::from_millis(300),
                    )),
                ));
                return false;
            };

            let monster_xy = monsters[monster_index].center_xy_tile();

            let step_distance = match spatial.behavior {
                ProjectileBehavior::Direct => spatial.velocity.length() * dt.as_secs_f32(),
                ProjectileBehavior::Homing { velocity, .. } => velocity.length() * dt.as_secs_f32(),
            };

            if (monster_xy - start_xy).length() > step_distance {
                // 아직 도달 전 → 이동 처리
                match spatial.behavior {
                    ProjectileBehavior::Direct => spatial.move_by(dt, monster_xy),
                    ProjectileBehavior::Homing { .. } => spatial.move_homing(dt, monster_xy),
                }
                let moved_distance = (spatial.xy - start_xy).length();
                game_state.effect_events.push(GameEffectEvent::SyncProjectileTrailState {
                    projectile_id: attack.id,
                    trail: spatial.trail,
                    start_xy,
                    end_xy: spatial.xy,
                    moved_distance,
                    dt_secs: dt.as_secs_f32(),
                    now,
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
                        sound::SpatialMode::Spatial { position: monster_xy },
                    ),
                ));
            }

            if damage > 0.0 {
                game_state.effect_events.push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::DamageText(field_particle::DamageTextParticle::new(
                        monster_xy, damage, now,
                    )),
                ));
            }

            match spatial.hit_effect {
                ProjectileHitEffect::TrashBounce => {
                    for p in field_particle::emitter::create_bounce_particles(
                        spatial.projectile_kind,
                        (start_xy.x, start_xy.y),
                        (monster_xy.x, monster_xy.y),
                        now,
                    ) {
                        game_state.effect_events.push(GameEffectEvent::SpawnParticle(
                            ParticleSpawnRequest::Trash(p),
                        ));
                    }
                }
                hit_effect => {
                    game_state.effect_events.push(GameEffectEvent::SpawnProjectileHitEffect(
                        hit_effect,
                        monster_xy,
                        now,
                    ));
                }
            }

            hits.push(MonsterHit {
                target_idx: monster_index,
                damage,
                at_xy: monster_xy,
                source_tower: attack.source_tower,
            });
            false
        });
    }

    apply_monster_damage_and_remove_dead(game_state, hits);
}

/// 모든 공격 경로(Spatial/Timed/Laser)의 공통 종착점.
/// 데미지 적용 → 타워 데미지 기록 → 사망 판정 → monster_death 처리.
fn apply_monster_damage_and_remove_dead(game_state: &mut GameState, hits: Vec<MonsterHit>) {
    let now = game_state.now();
    let mut dead: Vec<(usize, MapCoordF32)> = Vec::new();

    for hit in hits {
        if hit.target_idx >= game_state.monsters.len() {
            continue;
        }

        game_state.monsters[hit.target_idx].get_damage(hit.damage);

        if hit.damage > 0.0 {
            game_state.effect_events.push(GameEffectEvent::PlaySound(
                crate::sound::EmitSoundParams::one_shot(
                    crate::sound::random_whoop(),
                    crate::sound::SoundGroup::Sfx,
                    crate::sound::VolumePreset::Minimum,
                    crate::sound::SpatialMode::Spatial { position: hit.at_xy },
                ),
            ));

            if let Some(tower) = hit.source_tower {
                game_state.record_tower_damage(&tower, hit.damage);
            }
        }

        if game_state.monsters[hit.target_idx].dead() {
            dead.push((hit.target_idx, hit.at_xy));
        }
    }

    dead.sort_by_key(|(idx, _)| *idx);
    dead.dedup_by_key(|(idx, _)| *idx);
    for (target_idx, target_xy) in dead.into_iter().rev() {
        super::monster_death::handle_monster_death(game_state, target_idx, target_xy, now);
    }
}
