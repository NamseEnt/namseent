use super::*;

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

    {
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

            let attack_range_radius =
                tower.attack_range_radius(&tower_upgrades, stage_modifiers.get_range_multiplier());

            let tower_center = tower.center_xy_f32();
            let target_idx = monsters.iter().position(|monster| {
                (monster.center_xy_tile() - tower_center).length() < attack_range_radius
            });

            let Some(target_idx) = target_idx else {
                continue;
            };

            let contract_multiplier = stage_modifiers.get_damage_multiplier();
            let target_xy = monsters[target_idx].center_xy_tile();
            let (attack_type, instant_damage) = tower.attack_type(AttackTypeParams {
                target_xy: (target_xy.x, target_xy.y),
                tower_upgrade_states: &tower_upgrades,
                contract_multiplier,
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
                        contract_multiplier,
                        now,
                    });
                    projectiles.push(projectile);
                }
                AttackType::Laser => {
                    let (laser, damage) = tower.shoot_laser(ShootLaserParams {
                        target_xy: (target_xy.x, target_xy.y),
                        tower_upgrade_states: &tower_upgrades,
                        contract_multiplier,
                        now,
                    });

                    field_particle::emitter::spawn_laser_beam(
                        laser.start_xy,
                        laser.end_xy,
                        laser.created_at,
                    );

                    if damage > 0.0 {
                        field_particle::DAMAGE_TEXTS.spawn(
                            field_particle::DamageTextParticle::new(target_xy, damage, now),
                        );
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
                            damage_per_projectile,
                            ProjectileTrail::Burning,
                            crate::game_state::attack::ProjectileHitEffect::TrashBounce,
                        );
                        projectiles.push(projectile);
                    }
                }
                AttackType::RoyalStraightFlush { target_xy } => {
                    let target_monster_id = monsters[target_idx].id();
                    tower.spawn_royal_straight_flush_visual(
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

    game_state.delayed_hits.extend(new_delayed_hits);

    apply_monster_kills(game_state, monster_kills);

    game_state.projectiles.extend(projectiles);
}

fn process_delayed_hits(game_state: &mut GameState) {
    let now = game_state.now();
    let mut due_hits = Vec::new();

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
        let Some(target_idx) = game_state
            .monsters
            .iter()
            .position(|monster| monster.id() == hit.target_monster_id)
        else {
            continue;
        };

        let target_xy = game_state.monsters[target_idx].center_xy_tile();

        if hit.damage > 0.0 {
            crate::game_state::field_particle::DAMAGE_TEXTS.spawn(
                crate::game_state::field_particle::DamageTextParticle::new(
                    target_xy, hit.damage, now,
                ),
            );
        }

        monster_kills.push((target_idx, hit.damage, target_xy));
    }

    apply_monster_kills(game_state, monster_kills);
}

fn apply_monster_kills(game_state: &mut GameState, monster_kills: Vec<(usize, f32, MapCoordF32)>) {
    let indices_to_remove: Vec<_> = monster_kills
        .into_iter()
        .filter_map(|(target_idx, damage, target_xy)| {
            if target_idx >= game_state.monsters.len() {
                return None;
            }

            game_state.monsters[target_idx].get_damage(damage);

            if game_state.monsters[target_idx].dead() {
                Some((target_idx, target_xy))
            } else {
                None
            }
        })
        .collect();

    let now = game_state.now();
    for (target_idx, target_xy) in indices_to_remove.into_iter().rev() {
        super::monster_death::handle_monster_death(game_state, target_idx, target_xy, now);
    }
}
