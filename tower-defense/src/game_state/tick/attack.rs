use super::*;
use crate::game_state::effect_event::{GameEffectEvent, ParticleSpawnRequest};
use rand::Rng;
use std::collections::HashMap;

pub fn shoot_attacks(game_state: &mut GameState) {
    use crate::game_state::attack::AttackType;
    use crate::game_state::field_particle;
    use crate::game_state::tower::{
        AttackTypeParams, ShootLaserParams, ShootProjectileParams, royal_straight_flush_hit_delay,
    };

    process_delayed_hits(game_state);

    let now = game_state.now();

    let mut projectiles = Vec::new();
    let mut new_delayed_hits = Vec::new();
    let mut monster_kills = Vec::new();

    let mut tower_damage_updates = Vec::new();

    {
        let global_multiplier = game_state
            .upgrade_state
            .global_tower_damage_multiplier(game_state);
        let towers = &mut game_state.towers;
        let upgrade_state = &game_state.upgrade_state;
        let stage_modifiers = &game_state.stage_modifiers;
        let monsters = &game_state.monsters;
        let black_smoke_sources = &mut game_state.black_smoke_sources;

        for tower in towers.iter_mut() {
            if tower.in_cooltime() {
                continue;
            }

            if stage_modifiers.get_disabled_ranks().contains(&tower.rank()) {
                continue;
            }

            if stage_modifiers.get_disabled_suits().contains(&tower.suit()) {
                continue;
            }

            let tower_upgrades = upgrade_state.tower_upgrades(tower);

            let attack_range_radius = tower.attack_range_radius(&tower_upgrades, 1.0);

            let tower_center = tower.center_xy_f32();
            let target_idx = monsters.iter().position(|monster| {
                (monster.center_xy_tile() - tower_center).length() < attack_range_radius
            });

            let Some(target_idx) = target_idx else {
                continue;
            };

            let stage_damage_multiplier = stage_modifiers.get_damage_multiplier();
            let target_xy = monsters[target_idx].center_xy_tile();
            let (attack_type, instant_damage) = tower.attack_type(AttackTypeParams {
                target_xy: (target_xy.x, target_xy.y),
                tower_upgrade_states: &tower_upgrades,
                stage_damage_multiplier,
                global_damage_multiplier: global_multiplier,
                now,
            });

            match attack_type {
                AttackType::Projectile {
                    speed,
                    trail,
                    projectile_group,
                    hit_effect,
                } => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    let projectile = tower.shoot_projectile(ShootProjectileParams {
                        target_indicator,
                        speed,
                        trail,
                        projectile_group,
                        hit_effect,
                        tower_upgrade_states: &tower_upgrades,
                        stage_damage_multiplier,
                        global_damage_multiplier: global_multiplier,
                        now,
                        source_tower_id: Some(tower.id()),
                        source_tower_info: Some((tower.kind, tower.rank(), tower.suit())),
                    });
                    projectiles.push(projectile);
                }
                AttackType::Laser => {
                    let (laser, damage) = tower.shoot_laser(ShootLaserParams {
                        target_xy: (target_xy.x, target_xy.y),
                        tower_upgrade_states: &tower_upgrades,
                        stage_damage_multiplier,
                        global_damage_multiplier: global_multiplier,
                        now,
                    });

                    game_state.effect_events.push(GameEffectEvent::PlaySound(
                        crate::sound::EmitSoundParams::one_shot(
                            crate::sound::random_red_laser_shot(),
                            crate::sound::SoundGroup::Sfx,
                            crate::sound::VolumePreset::Minimum,
                            crate::sound::SpatialMode::Spatial {
                                position: crate::MapCoordF32::new(
                                    laser.start_xy.0,
                                    laser.start_xy.1,
                                ),
                            },
                        ),
                    ));
                    game_state.effect_events.push(GameEffectEvent::PlaySound(
                        crate::sound::EmitSoundParams::one_shot(
                            crate::sound::random_red_laser_shot(),
                            crate::sound::SoundGroup::Sfx,
                            crate::sound::VolumePreset::Minimum,
                            crate::sound::SpatialMode::Spatial {
                                position: crate::MapCoordF32::new(laser.end_xy.0, laser.end_xy.1),
                            },
                        ),
                    ));

                    game_state
                        .effect_events
                        .push(GameEffectEvent::SpawnLaserBeam(
                            laser.start_xy,
                            laser.end_xy,
                            laser.created_at,
                        ));

                    if damage > 0.0 {
                        tower_damage_updates.push((
                            tower.id(),
                            tower.kind,
                            tower.rank(),
                            tower.suit(),
                            damage,
                        ));
                        game_state
                            .effect_events
                            .push(GameEffectEvent::SpawnParticle(
                                ParticleSpawnRequest::DamageText(
                                    field_particle::DamageTextParticle::new(target_xy, damage, now),
                                ),
                            ));
                    }

                    monster_kills.push((target_idx, damage, target_xy));
                }
                AttackType::FullHouseRain {
                    tower_xy,
                    target_xy: _,
                } => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    let damage_per_projectile = instant_damage / 4.0;

                    for _ in 0..4 {
                        let projectile = Projectile::new_homing(
                            crate::MapCoordF32::new(tower_xy.0, tower_xy.1),
                            ProjectileKind::random_trash(),
                            target_indicator,
                            ProjectileParams {
                                damage: damage_per_projectile,
                                trail: ProjectileTrail::Burning,
                                hit_effect:
                                    crate::game_state::attack::ProjectileHitEffect::TrashBounce,
                                source_tower_id: Some(tower.id()),
                                source_tower_info: Some((tower.kind, tower.rank(), tower.suit())),
                            },
                        );
                        projectiles.push(projectile);
                    }
                }
                AttackType::RoyalStraightFlush { target_xy } => {
                    let target_monster_id = monsters[target_idx].id();
                    tower.spawn_royal_straight_flush_visual(
                        &mut game_state.effect_events,
                        target_xy,
                        target_monster_id,
                        now,
                        black_smoke_sources,
                    );
                    new_delayed_hits.push(crate::game_state::attack::DelayedHit {
                        target_monster_id,
                        damage: instant_damage,
                        execute_at: now + royal_straight_flush_hit_delay(),
                    });
                }
            }
        }
    }

    for (tower_id, tower_kind, rank, suit, damage) in tower_damage_updates {
        game_state.record_tower_damage(tower_id, tower_kind, rank, suit, damage);
    }

    apply_monster_kills(game_state, monster_kills);

    game_state.projectiles.extend(projectiles);
}

fn process_delayed_hits(game_state: &mut GameState) {
    let now = game_state.now();
    let mut rng = rand::thread_rng();
    let mut due_hits = Vec::new();

    let monster_index_by_id: HashMap<_, _> = game_state
        .monsters
        .iter()
        .enumerate()
        .map(|(index, monster)| (monster.id(), index))
        .collect();

    game_state.delayed_hits.retain(|hit| {
        if hit.execute_at <= now {
            due_hits.push(*hit);
            false
        } else {
            true
        }
    });

    let mut monster_kills = Vec::new();
    for hit in due_hits {
        let Some(&target_idx) = monster_index_by_id.get(&hit.target_monster_id) else {
            continue;
        };

        let target_xy = game_state.monsters[target_idx].center_xy_tile();

        game_state.effect_events.push(GameEffectEvent::PlaySound(
            crate::sound::EmitSoundParams::one_shot(
                crate::sound::random_knife_slash(),
                crate::sound::SoundGroup::Sfx,
                crate::sound::VolumePreset::Low,
                crate::sound::SpatialMode::Spatial {
                    position: target_xy,
                },
            ),
        ));
        let second_slash_delay_ms = rng.gen_range(30_i64..=60_i64);
        game_state
            .effect_events
            .push(GameEffectEvent::PlaySoundDelayed(
                crate::sound::EmitSoundParams::one_shot(
                    crate::sound::random_knife_slash(),
                    crate::sound::SoundGroup::Sfx,
                    crate::sound::VolumePreset::Low,
                    crate::sound::SpatialMode::Spatial {
                        position: target_xy,
                    },
                ),
                Duration::from_millis(second_slash_delay_ms),
            ));

        if hit.damage > 0.0 {
            game_state
                .effect_events
                .push(GameEffectEvent::SpawnParticle(
                    ParticleSpawnRequest::DamageText(
                        crate::game_state::field_particle::DamageTextParticle::new(
                            target_xy, hit.damage, now,
                        ),
                    ),
                ));
        }

        monster_kills.push((target_idx, hit.damage, target_xy));
    }

    apply_monster_kills(game_state, monster_kills);
}

fn apply_monster_kills(game_state: &mut GameState, monster_kills: Vec<(usize, f32, MapCoordF32)>) {
    let mut indices_to_remove: Vec<_> = monster_kills
        .into_iter()
        .filter_map(|(target_idx, damage, target_xy)| {
            if target_idx >= game_state.monsters.len() {
                return None;
            }

            game_state.monsters[target_idx].get_damage(damage);

            if damage > 0.0 {
                game_state.effect_events.push(GameEffectEvent::PlaySound(
                    crate::sound::EmitSoundParams::one_shot(
                        crate::sound::random_whoop(),
                        crate::sound::SoundGroup::Sfx,
                        crate::sound::VolumePreset::Minimum,
                        crate::sound::SpatialMode::Spatial {
                            position: target_xy,
                        },
                    ),
                ));
            }

            if game_state.monsters[target_idx].dead() {
                Some((target_idx, target_xy))
            } else {
                None
            }
        })
        .collect();

    let now = game_state.now();
    indices_to_remove.sort_by_key(|(target_idx, _)| *target_idx);
    indices_to_remove.dedup_by_key(|(target_idx, _)| *target_idx);
    for (target_idx, target_xy) in indices_to_remove.into_iter().rev() {
        super::monster_death::handle_monster_death(game_state, target_idx, target_xy, now);
    }
}
