use super::*;

pub fn shoot_attacks(game_state: &mut GameState) {
    use crate::game_state::attack::AttackType;
    use crate::game_state::attack::instant_effect::InstantEffectKind;
    use crate::game_state::field_particle;

    let now = game_state.now();

    let mut projectiles = Vec::new();
    let mut attack_effect_particles = Vec::new();
    let mut field_emitters = Vec::new();
    let mut damage_emitters = Vec::new();
    let mut monster_death_emitters = Vec::new();
    let mut monster_kills = Vec::new(); // (target_idx, damage, target_xy) 튜플

    // towers.iter_mut()의 scope을 최소화
    {
        let towers = &mut game_state.towers;
        let upgrade_state = &game_state.upgrade_state;
        let stage_modifiers = &game_state.stage_modifiers;
        let monsters = &game_state.monsters;

        for tower in towers.iter_mut() {
            if tower.in_cooltime() {
                continue;
            }

            // Check if tower rank is disabled by contract
            if stage_modifiers.get_disabled_ranks().contains(&tower.rank()) {
                continue;
            }

            // Check if tower suit is disabled by contract
            if stage_modifiers.get_disabled_suits().contains(&tower.suit()) {
                continue;
            }

            let tower_upgrades = upgrade_state.tower_upgrades(tower);

            let attack_range_radius =
                tower.attack_range_radius(&tower_upgrades, stage_modifiers.get_range_multiplier());

            let target_idx = monsters.iter().position(|monster| {
                (monster.move_on_route.xy() - tower.left_top.map(|t| t as f32)).length()
                    < attack_range_radius
            });

            let Some(target_idx) = target_idx else {
                continue;
            };

            let contract_multiplier = stage_modifiers.get_damage_multiplier();
            let target_xy = monsters[target_idx].move_on_route.xy();
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

                    attack_effect_particles.push(field_particle::FieldParticle::LaserBeam {
                        particle: field_particle::LaserBeamParticle::new(
                            laser.start_xy,
                            laser.end_xy,
                            laser.created_at,
                        ),
                    });

                    if damage > 0.0 {
                        damage_emitters.push(field_particle::emitter::DamageTextEmitter::new(
                            target_xy, damage,
                        ));
                    }

                    monster_kills.push((target_idx, damage, target_xy));
                }
                AttackType::InstantEffect {
                    emit_effect,
                    hit_effect,
                } => {
                    // FullHouse 이펙트인 경우 특별한 particle 생성
                    match emit_effect.kind {
                        InstantEffectKind::FullHouseRain => {
                            // emit via TrashRain and TrashBurst emitters
                            field_emitters.push(field_particle::FieldParticleEmitter::TrashRain {
                                emitter: field_particle::emitter::TrashRainEmitter::new(
                                    crate::MapCoordF32::new(hit_effect.xy.0, hit_effect.xy.1),
                                    emit_effect.created_at,
                                ),
                            });
                            field_emitters.push(field_particle::FieldParticleEmitter::TrashBurst {
                                emitter: field_particle::emitter::TrashBurstEmitter::new(
                                    crate::MapCoordF32::new(emit_effect.tower_xy.0, emit_effect.tower_xy.1),
                                    emit_effect.created_at,
                                ),
                            });
                        }
                        _ => {
                            attack_effect_particles.push(
                                field_particle::FieldParticle::InstantEmit {
                                    particle: field_particle::InstantEmitParticle::new(
                                        emit_effect.tower_xy,
                                        emit_effect.target_xy,
                                        emit_effect.created_at,
                                        emit_effect.kind,
                                    ),
                                },
                            );
                            attack_effect_particles.push(
                                field_particle::FieldParticle::InstantHit {
                                    particle: field_particle::InstantHitParticle::new(
                                        hit_effect.xy,
                                        hit_effect.created_at,
                                        hit_effect.kind,
                                        hit_effect.scale,
                                    ),
                                },
                            );
                        }
                    }

                    if instant_damage > 0.0 {
                        damage_emitters.push(field_particle::emitter::DamageTextEmitter::new(
                            target_xy,
                            instant_damage,
                        ));
                    }

                    monster_kills.push((target_idx, instant_damage, target_xy));
                }
            }
        }
    } // towers 빌려주기 종료

    // 괴물에게 데미지 적용 및 사망 처리
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

    // 사망한 괴물 처리 (역순으로 처리해서 인덱스 문제 회피)
    let now = game_state.now();
    for (target_idx, target_xy) in indices_to_remove.into_iter().rev() {
        super::monster_death::handle_monster_death(
            game_state,
            target_idx,
            target_xy,
            now,
            &mut monster_death_emitters,
        );
    }

    game_state.projectiles.extend(projectiles);

    if !field_emitters.is_empty() {
        game_state
            .field_particle_system_manager
            .add_emitters(field_emitters);
    }

    super::particle_emit::emit_attack_effect_particles(game_state, attack_effect_particles);
    super::particle_emit::emit_damage_text_particles(game_state, damage_emitters);
    super::particle_emit::emit_monster_death_particles(game_state, monster_death_emitters);
}
