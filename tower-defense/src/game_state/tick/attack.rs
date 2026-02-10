use super::*;

pub fn shoot_attacks(game_state: &mut GameState) {
    use crate::game_state::attack::AttackType;
    use crate::game_state::field_particle;

    let now = game_state.now();

    let mut projectiles = Vec::new();
    let mut monster_kills = Vec::new();

    {
        let towers = &mut game_state.towers;
        let upgrade_state = &game_state.upgrade_state;
        let stage_modifiers = &game_state.stage_modifiers;
        let monsters = &game_state.monsters;

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
            let (attack_type, instant_damage) = tower.attack_type(
                (target_xy.x, target_xy.y),
                &tower_upgrades,
                contract_multiplier,
                now,
            );

            match attack_type {
                AttackType::Projectile { speed, trail } => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    let projectile = tower.shoot_projectile(
                        target_indicator,
                        speed,
                        trail,
                        &tower_upgrades,
                        contract_multiplier,
                        now,
                    );
                    projectiles.push(projectile);
                }
                AttackType::Laser => {
                    let (laser, damage) = tower.shoot_laser(
                        (target_xy.x, target_xy.y),
                        &tower_upgrades,
                        contract_multiplier,
                        now,
                    );

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
                AttackType::InstantEffect {
                    emit_effect,
                    hit_effect,
                } => {
                    field_particle::INSTANT_EMITS.spawn(
                        field_particle::InstantEmitParticle::new(
                            emit_effect.tower_xy,
                            emit_effect.target_xy,
                            emit_effect.created_at,
                            emit_effect.kind,
                        ),
                    );
                    field_particle::INSTANT_HITS.spawn(
                        field_particle::InstantHitParticle::new(
                            hit_effect.xy,
                            hit_effect.created_at,
                            hit_effect.kind,
                            hit_effect.scale,
                        ),
                    );

                    if instant_damage > 0.0 {
                        field_particle::DAMAGE_TEXTS.spawn(
                            field_particle::DamageTextParticle::new(target_xy, instant_damage, now),
                        );
                    }

                    monster_kills.push((target_idx, instant_damage, target_xy));
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
                        );
                        projectiles.push(projectile);
                    }
                }
            }
        }
    }

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
        super::monster_death::handle_monster_death(
            game_state,
            target_idx,
            target_xy,
            now,
        );
    }

    game_state.projectiles.extend(projectiles);
}
