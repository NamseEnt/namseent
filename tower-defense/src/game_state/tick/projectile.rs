use super::*;
use crate::sound;
use rand::Rng;

const WHOOSH_INTERVAL_MIN_SECS: f32 = 0.5;
const WHOOSH_INTERVAL_MAX_SECS: f32 = 0.75;

pub fn move_projectiles(game_state: &mut GameState, dt: Duration, now: Instant) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    let mut total_earn_gold = 0;
    let mut rng = rand::thread_rng();

    projectiles.retain_mut(|projectile| {
        let start_xy = projectile.xy;

        let Some(monster_index) = monsters
            .iter()
            .position(|monster| monster.projectile_target_indicator == projectile.target_indicator)
        else {
            if projectile.current_whoosh_sound_id != 0 {
                sound::stop_sound(projectile.current_whoosh_sound_id);
                projectile.current_whoosh_sound_id = 0;
            }
            if projectile.current_crackling_sound_id != 0 {
                sound::stop_sound(projectile.current_crackling_sound_id);
                projectile.current_crackling_sound_id = 0;
            }
            if projectile.current_shining_sound_id != 0 {
                sound::stop_sound(projectile.current_shining_sound_id);
                projectile.current_shining_sound_id = 0;
            }
            if projectile.current_wind_sound_id != 0 {
                sound::stop_sound(projectile.current_wind_sound_id);
                projectile.current_wind_sound_id = 0;
            }
            field_particle::PROJECTILES.spawn(field_particle::ProjectileParticle::new(
                projectile.xy,
                projectile.kind,
                projectile.rotation,
                projectile.rotation_speed,
                projectile.velocity,
                now,
                Duration::from_millis(300),
            ));
            return false;
        };

        let monster = &mut monsters[monster_index];
        let monster_xy = monster.center_xy_tile();

        let step_distance = match projectile.behavior {
            ProjectileBehavior::Direct => projectile.velocity.length() * dt.as_secs_f32(),
            ProjectileBehavior::Homing { velocity, .. } => velocity.length() * dt.as_secs_f32(),
        };

        if (monster_xy - start_xy).length() > step_distance {
            match projectile.behavior {
                ProjectileBehavior::Direct => projectile.move_by(dt, monster_xy),
                ProjectileBehavior::Homing { .. } => projectile.move_homing(dt, monster_xy),
            }

            let moved_distance = (projectile.xy - start_xy).length();

            let spawn_distance = match projectile.trail {
                ProjectileTrail::None => None,
                ProjectileTrail::Burning => {
                    Some(field_particle::emitter::BURNING_TRAIL_SPAWN_DISTANCE)
                }
                ProjectileTrail::Sparkle => Some(field_particle::emitter::SPARKLE_SPAWN_DISTANCE),
                ProjectileTrail::WindCurve => {
                    Some(field_particle::emitter::WIND_CURVE_SPAWN_DISTANCE)
                }
                ProjectileTrail::Heart => Some(field_particle::emitter::HEART_SPAWN_DISTANCE),
                ProjectileTrail::LightningSparkle => {
                    Some(field_particle::emitter::LIGHTNING_TRAIL_SPAWN_DISTANCE)
                }
            };

            if let Some(spawn_distance) = spawn_distance {
                projectile.trail_distance_remainder += moved_distance;
                let spawn_count =
                    (projectile.trail_distance_remainder / spawn_distance).floor() as usize;
                if spawn_count > 0 {
                    projectile.trail_distance_remainder -= spawn_count as f32 * spawn_distance;
                    match projectile.trail {
                        ProjectileTrail::Burning => {
                            field_particle::emitter::spawn_burning_trail(
                                start_xy,
                                projectile.xy,
                                spawn_count,
                                now,
                            );
                        }
                        ProjectileTrail::Sparkle => {
                            field_particle::emitter::spawn_sparkle_trail(
                                start_xy,
                                projectile.xy,
                                spawn_count,
                                now,
                            );
                        }
                        ProjectileTrail::WindCurve => {
                            field_particle::emitter::spawn_wind_curve_trail(
                                start_xy,
                                projectile.xy,
                                spawn_count,
                                now,
                            );
                        }
                        ProjectileTrail::Heart => {
                            field_particle::emitter::spawn_heart_trail(
                                start_xy,
                                projectile.xy,
                                spawn_count,
                                now,
                            );
                        }
                        ProjectileTrail::LightningSparkle => {
                            field_particle::emitter::spawn_lightning_trail(
                                start_xy,
                                projectile.xy,
                                spawn_count,
                                now,
                            );
                            field_particle::emitter::spawn_sparkle_trail(
                                start_xy,
                                projectile.xy,
                                spawn_count,
                                now,
                            );
                        }
                        ProjectileTrail::None => {}
                    }
                }
            }

            projectile.whoosh_cooldown_secs -= dt.as_secs_f32();

            if projectile.whoosh_cooldown_secs <= 0.0 {
                if projectile.current_whoosh_sound_id != 0 {
                    sound::stop_sound(projectile.current_whoosh_sound_id);
                }

                projectile.current_whoosh_sound_id =
                    sound::emit_sound(sound::EmitSoundParams::one_shot(
                        sound::random_whoosh(),
                        sound::SoundGroup::Sfx,
                        sound::VolumePreset::Minimum,
                        sound::SpatialMode::Spatial {
                            position: projectile.xy,
                        },
                    ));
                projectile.whoosh_cooldown_secs =
                    rng.gen_range(WHOOSH_INTERVAL_MIN_SECS..=WHOOSH_INTERVAL_MAX_SECS);
            }

            if projectile.current_whoosh_sound_id != 0 {
                sound::update_sound_position(projectile.current_whoosh_sound_id, projectile.xy);
            }

            if matches!(projectile.trail, ProjectileTrail::Burning) {
                if projectile.current_crackling_sound_id == 0 {
                    projectile.current_crackling_sound_id = sound::emit_sound(
                        sound::EmitSoundParams::looping(
                            sound::random_crackling_fire(),
                            sound::SoundGroup::Sfx,
                            sound::VolumePreset::Minimum,
                            sound::SpatialMode::Spatial {
                                position: projectile.xy,
                            },
                        )
                        .with_max_duration(Duration::from_secs(32)),
                    );
                } else {
                    sound::update_sound_position(
                        projectile.current_crackling_sound_id,
                        projectile.xy,
                    );
                }
            } else if projectile.current_crackling_sound_id != 0 {
                sound::stop_sound(projectile.current_crackling_sound_id);
                projectile.current_crackling_sound_id = 0;
            }

            if matches!(projectile.trail, ProjectileTrail::Sparkle) {
                if projectile.current_shining_sound_id == 0 {
                    projectile.current_shining_sound_id =
                        sound::emit_sound(sound::EmitSoundParams::looping(
                            sound::random_shining_ringing(),
                            sound::SoundGroup::Sfx,
                            sound::VolumePreset::Minimum,
                            sound::SpatialMode::Spatial {
                                position: projectile.xy,
                            },
                        ));
                } else {
                    sound::update_sound_position(
                        projectile.current_shining_sound_id,
                        projectile.xy,
                    );
                }
            } else if projectile.current_shining_sound_id != 0 {
                sound::stop_sound(projectile.current_shining_sound_id);
                projectile.current_shining_sound_id = 0;
            }

            if matches!(projectile.trail, ProjectileTrail::WindCurve) {
                if projectile.current_wind_sound_id == 0 {
                    projectile.current_wind_sound_id =
                        sound::emit_sound(sound::EmitSoundParams::looping(
                            sound::random_wind(),
                            sound::SoundGroup::Sfx,
                            sound::VolumePreset::Minimum,
                            sound::SpatialMode::Spatial {
                                position: projectile.xy,
                            },
                        ));
                } else {
                    sound::update_sound_position(projectile.current_wind_sound_id, projectile.xy);
                }
            } else if projectile.current_wind_sound_id != 0 {
                sound::stop_sound(projectile.current_wind_sound_id);
                projectile.current_wind_sound_id = 0;
            }

            return true;
        }

        if projectile.current_whoosh_sound_id != 0 {
            sound::stop_sound(projectile.current_whoosh_sound_id);
            projectile.current_whoosh_sound_id = 0;
        }
        if projectile.current_crackling_sound_id != 0 {
            sound::stop_sound(projectile.current_crackling_sound_id);
            projectile.current_crackling_sound_id = 0;
        }
        if projectile.current_shining_sound_id != 0 {
            sound::stop_sound(projectile.current_shining_sound_id);
            projectile.current_shining_sound_id = 0;
        }
        if projectile.current_wind_sound_id != 0 {
            sound::stop_sound(projectile.current_wind_sound_id);
            projectile.current_wind_sound_id = 0;
        }

        let damage = projectile.damage;
        monster.get_damage(damage);
        if matches!(projectile.trail, ProjectileTrail::Burning) {
            sound::emit_sound(sound::EmitSoundParams::one_shot(
                sound::random_flamethrower(),
                sound::SoundGroup::Sfx,
                sound::VolumePreset::Minimum,
                sound::SpatialMode::Spatial {
                    position: monster_xy,
                },
            ));
        }
        if damage > 0.0 {
            field_particle::DAMAGE_TEXTS.spawn(field_particle::DamageTextParticle::new(
                monster_xy, damage, now,
            ));
        }

        use crate::game_state::attack::ProjectileHitEffect;
        match projectile.hit_effect {
            ProjectileHitEffect::TrashBounce => {
                let bounce_particles = field_particle::emitter::create_bounce_particles(
                    projectile.kind,
                    (start_xy.x, start_xy.y),
                    (monster_xy.x, monster_xy.y),
                    now,
                );
                for p in bounce_particles {
                    field_particle::TRASHES.spawn(p);
                }
            }
            ProjectileHitEffect::CardBurst => {
                field_particle::emitter::spawn_card_burst(monster_xy, now);
            }
            ProjectileHitEffect::SparkleBurst => {
                field_particle::emitter::spawn_sparkle_burst(monster_xy, now);
            }
            ProjectileHitEffect::HeartBurst => {
                field_particle::emitter::spawn_heart_burst(monster_xy, now);
            }
        }

        if monster.dead() {
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp += monster.max_hp;
            }
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            let earn =
                (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;
            total_earn_gold += earn;

            let monster_kind = monster.kind;
            let rotation = monster.animation.rotation;
            let y_offset = monster.animation.y_offset;
            let wh = monster::monster_wh(monster_kind);

            let tile_base_xy = TILE_PX_SIZE.to_xy() * monster_xy;
            let monster_center_offset = Xy::new(
                TILE_PX_SIZE.width * 0.5,
                TILE_PX_SIZE.height - wh.height * 0.5 + TILE_PX_SIZE.height * y_offset,
            );
            let pixel_xy = tile_base_xy + monster_center_offset;

            field_particle::MONSTER_CORPSES.spawn(field_particle::MonsterCorpseParticle::new(
                pixel_xy,
                now,
                rotation,
                monster_kind,
                wh,
            ));

            field_particle::MONSTER_SOULS.spawn(field_particle::MonsterSoulParticle::new(
                pixel_xy, now, rotation,
            ));

            monsters.swap_remove(monster_index);
        }

        false
    });

    if total_earn_gold > 0 {
        game_state.earn_gold(total_earn_gold);
    }
}
