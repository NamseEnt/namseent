use super::*;
use crate::game_state::effect_event::{GameEffectEvent, ParticleSpawnRequest};
use std::collections::HashMap;

pub fn move_projectiles(game_state: &mut GameState, dt: Duration, now: Instant) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    let mut monster_index_by_indicator: HashMap<_, _> = monsters
        .iter()
        .enumerate()
        .map(|(index, monster)| (monster.projectile_target_indicator, index))
        .collect();

    let mut total_earn_gold = 0;

    projectiles.retain_mut(|projectile| {
        let start_xy = projectile.xy;

        let Some(&monster_index) = monster_index_by_indicator.get(&projectile.target_indicator)
        else {
            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::Projectile(field_particle::ProjectileParticle::new(
                        projectile.xy,
                        projectile.kind,
                        projectile.rotation,
                        projectile.rotation_speed,
                        projectile.velocity,
                        now,
                        Duration::from_millis(300),
                    )),
                ));
            return false;
        };

        let target_indicator = projectile.target_indicator;
        let last_monster_indicator = monsters
            .last()
            .map(|monster| monster.projectile_target_indicator);
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

            game_state
                .effect_events
                .push(GameEffectEvent::SyncProjectileTrailState {
                    projectile_id: projectile.id,
                    trail: projectile.trail,
                    start_xy,
                    end_xy: projectile.xy,
                    moved_distance,
                    dt_secs: dt.as_secs_f32(),
                    now,
                });

            return true;
        }

        let damage = projectile.damage;
        monster.get_damage(damage);
        if damage > 0.0 {
            game_state.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_whoop(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::Minimum,
                    sound::SpatialMode::Spatial {
                        position: monster_xy,
                    },
                ),
            ));
        }
        if matches!(projectile.trail, ProjectileTrail::Burning) {
            game_state.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_flamethrower(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::Minimum,
                    sound::SpatialMode::Spatial {
                        position: monster_xy,
                    },
                ),
            ));
        }
        if matches!(projectile.trail, ProjectileTrail::LightningSparkle) {
            game_state.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_smoke_bomb(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::Minimum,
                    sound::SpatialMode::Spatial {
                        position: monster_xy,
                    },
                ),
            ));
        }
        if damage > 0.0 {
            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::DamageText(field_particle::DamageTextParticle::new(
                        monster_xy, damage, now,
                    )),
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
                    game_state
                        .effect_events
                        .push(GameEffectEvent::SpawnParticle(ParticleSpawnRequest::Trash(
                            p,
                        )));
                }
            }
            ProjectileHitEffect::CardBurst => {
                game_state
                    .effect_events
                    .push(GameEffectEvent::SpawnProjectileHitEffect(
                        ProjectileHitEffect::CardBurst,
                        monster_xy,
                        now,
                    ));
            }
            ProjectileHitEffect::SparkleBurst => {
                game_state
                    .effect_events
                    .push(GameEffectEvent::SpawnProjectileHitEffect(
                        ProjectileHitEffect::SparkleBurst,
                        monster_xy,
                        now,
                    ));
            }
            ProjectileHitEffect::HeartBurst => {
                game_state
                    .effect_events
                    .push(GameEffectEvent::SpawnProjectileHitEffect(
                        ProjectileHitEffect::HeartBurst,
                        monster_xy,
                        now,
                    ));
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

            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::MonsterCorpse(
                        field_particle::MonsterCorpseParticle::new(
                            pixel_xy,
                            now,
                            rotation,
                            monster_kind,
                            wh,
                        ),
                    ),
                ));

            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::MonsterSoul(field_particle::MonsterSoulParticle::new(
                        pixel_xy, now, rotation,
                    )),
                ));

            monster_index_by_indicator.remove(&target_indicator);
            if let Some(last_indicator) = last_monster_indicator
                && monster_index != monsters.len() - 1
            {
                monster_index_by_indicator.insert(last_indicator, monster_index);
            }

            monsters.swap_remove(monster_index);
        }

        false
    });

    if total_earn_gold > 0 {
        game_state.earn_gold(total_earn_gold);
    }
}
