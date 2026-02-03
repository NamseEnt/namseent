use super::*;

const TICK_MAX_DURATION: Duration = Duration::from_millis(16);

pub struct Ticker;

impl Component for Ticker {
    fn render(self, ctx: &RenderCtx) {
        ctx.interval("game state tick", TICK_MAX_DURATION, |real_dt| {
            mutate_game_state(move |game_state| {
                let mut scaled_dt =
                    real_dt * game_state.fast_forward_multiplier.time_scale().get() as i32;
                while scaled_dt.as_millis() > 0 {
                    let tick_dt = scaled_dt.min(TICK_MAX_DURATION);
                    scaled_dt -= tick_dt;

                    game_state.game_now += tick_dt;

                    tick(game_state, tick_dt, game_state.game_now);
                }
            });
        });
    }
}

fn tick(game_state: &mut GameState, dt: Duration, now: Instant) {
    game_state.flow.update();
    flow::contract::update_contract_flow(game_state);

    game_state.update_camera_shake(dt);

    monster_spawn::tick(game_state, now);
    tower::tower_cooldown_tick(game_state, dt);
    tower::tower_animation_tick(game_state, now);
    monster::monster_animation_tick(game_state, dt);

    // Update UI state (info popup animations, etc.)
    game_state.ui_state.tick(now);

    // Clean up unused popup states periodically (only when needed)
    if game_state.ui_state.should_cleanup(now) {
        game_state.cleanup_unused_tower_popup_states();
    }

    monster::remove_monster_finished_status_effects(game_state, now);
    tower::remove_tower_finished_status_effects(game_state, now);
    user_status_effect::remove_user_finished_status_effects(game_state, now);
    field_particle::remove_finished_field_particle_systems(game_state, now);

    monster::activate_monster_skills(game_state, now);
    tower::activate_tower_skills(game_state, now);

    monster::move_monsters(game_state, dt);

    move_projectiles(game_state, dt, now);
    shoot_attacks(game_state);
    remove_expired_lasers(game_state, now);
    remove_expired_effects(game_state, now);
    check_defense_end(game_state);
}

fn move_projectiles(game_state: &mut GameState, dt: Duration, now: Instant) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    let mut total_earn_gold = 0;
    let mut damage_emitters = Vec::new();
    let mut monster_death_emitters = Vec::new();

    projectiles.retain_mut(|projectile| {
        let start_xy = projectile.xy;

        let Some(monster_index) = monsters
            .iter()
            .position(|monster| monster.projectile_target_indicator == projectile.target_indicator)
        else {
            return false;
        };

        let monster = &mut monsters[monster_index];
        let monster_xy = monster.move_on_route.xy();

        if (monster_xy - start_xy).length() > projectile.velocity * dt {
            projectile.move_by(dt, monster_xy);
            return true;
        }

        let damage = projectile.damage;
        monster.get_damage(damage);
        if damage > 0.0 {
            damage_emitters.push(field_particle::emitter::DamageTextEmitter::new(
                monster_xy, damage,
            ));
        }

        if monster.dead() {
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp += monster.max_hp;
            }
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            let earn =
                (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;
            total_earn_gold += earn;

            // Emit monster death particle (soul)
            monster_death_emitters.push(field_particle::emitter::MonsterDeathEmitter::new(
                monster_xy,
            ));

            // Emit monster corpse particle
            let monster_kind = monster.kind;
            let rotation = monster.animation.rotation;
            let wh = monster::monster_wh(monster_kind);

            // Calculate the pixel position where the monster is rendered
            let tile_base_xy = TILE_PX_SIZE.to_xy() * monster_xy;
            let monster_center_offset = Xy::new(
                TILE_PX_SIZE.width * 0.5,
                TILE_PX_SIZE.height - wh.height * 0.5
                    + TILE_PX_SIZE.height * monster.animation.y_offset,
            );
            let pixel_xy = tile_base_xy + monster_center_offset;

            let corpse_particle = field_particle::MonsterCorpseParticle::new(
                pixel_xy,
                now,
                rotation,
                monster_kind,
                wh,
            );
            game_state.field_particle_system_manager.add_emitters(vec![
                field_particle::FieldParticleEmitter::MonsterCorpse {
                    emitter: field_particle::TempParticleEmitter::new(vec![
                        field_particle::FieldParticle::MonsterCorpse {
                            particle: corpse_particle,
                        },
                    ]),
                },
            ]);

            monsters.swap_remove(monster_index);
        }

        false
    });

    emit_damage_text_particles(game_state, damage_emitters);
    emit_monster_death_particles(game_state, monster_death_emitters);

    if total_earn_gold > 0 {
        game_state.earn_gold(total_earn_gold);
    }
}

fn emit_damage_text_particles(
    game_state: &mut GameState,
    emitters: Vec<field_particle::emitter::DamageTextEmitter>,
) {
    if !emitters.is_empty() {
        let field_emitters = emitters
            .into_iter()
            .map(|emitter| field_particle::FieldParticleEmitter::DamageText { emitter })
            .collect::<Vec<_>>();
        game_state
            .field_particle_system_manager
            .add_emitters(field_emitters);
    }
}

fn emit_monster_death_particles(
    game_state: &mut GameState,
    emitters: Vec<field_particle::emitter::MonsterDeathEmitter>,
) {
    if !emitters.is_empty() {
        let field_emitters = emitters
            .into_iter()
            .map(|emitter| field_particle::FieldParticleEmitter::MonsterDeath { emitter })
            .collect::<Vec<_>>();
        game_state
            .field_particle_system_manager
            .add_emitters(field_emitters);
    }
}

fn shoot_attacks(game_state: &mut GameState) {
    use crate::game_state::attack::AttackType;

    let now = game_state.now();

    let mut projectiles = Vec::new();
    let mut lasers = Vec::new();
    let mut emit_effects = Vec::new();
    let mut hit_effects = Vec::new();
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

            match tower.attack_type {
                AttackType::Projectile => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    let projectile =
                        tower.shoot(target_indicator, &tower_upgrades, contract_multiplier, now);
                    projectiles.push(projectile);
                }
                AttackType::Laser => {
                    let (laser, damage) = tower.shoot_laser(
                        (target_xy.x, target_xy.y),
                        &tower_upgrades,
                        contract_multiplier,
                        now,
                    );

                    lasers.push(laser);

                    if damage > 0.0 {
                        damage_emitters.push(field_particle::emitter::DamageTextEmitter::new(
                            target_xy, damage,
                        ));
                    }

                    monster_kills.push((target_idx, damage, target_xy));
                }
                AttackType::InstantEffect => {
                    let (emit_effect, hit_effect, damage) = tower.shoot_instant_effect(
                        (target_xy.x, target_xy.y),
                        &tower_upgrades,
                        contract_multiplier,
                        now,
                    );

                    emit_effects.push(emit_effect);
                    hit_effects.push(hit_effect);

                    if damage > 0.0 {
                        damage_emitters.push(field_particle::emitter::DamageTextEmitter::new(
                            target_xy, damage,
                        ));
                    }

                    monster_kills.push((target_idx, damage, target_xy));
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
    for (target_idx, target_xy) in indices_to_remove.into_iter().rev() {
        handle_monster_death(
            game_state,
            target_idx,
            target_xy,
            now,
            &mut monster_death_emitters,
        );
    }

    game_state.projectiles.extend(projectiles);
    game_state.laser_beams.extend(lasers);
    game_state.tower_emit_effects.extend(emit_effects);
    game_state.target_hit_effects.extend(hit_effects);

    emit_damage_text_particles(game_state, damage_emitters);
    emit_monster_death_particles(game_state, monster_death_emitters);
}

fn check_defense_end(game_state: &mut GameState) {
    let GameFlow::Defense(_) = game_state.flow else {
        return;
    };
    if !game_state.monster_spawn_state.is_idle() {
        return;
    }
    if !game_state.monsters.is_empty() {
        return;
    }

    #[cfg(feature = "debug-tools")]
    {
        if debug_tools::monster_hp_balance::get_balance_state().is_some() {
            debug_tools::monster_hp_balance::check_and_adjust_hp_balance(game_state);
            return;
        }
    }

    let is_boss_stage = is_boss_stage(game_state.stage);
    game_state.stage += 1;
    if game_state.stage > 50 {
        game_state.stage -= 1;
        game_state.goto_result();
        return;
    }

    // 보스 스테이지를 클리어했다면 플래그 설정
    game_state.just_cleared_boss_stage = is_boss_stage;

    game_state.goto_next_stage();
}
fn remove_expired_lasers(game_state: &mut GameState, now: Instant) {
    game_state
        .laser_beams
        .retain(|laser| !laser.is_expired(now));
}

fn remove_expired_effects(game_state: &mut GameState, now: Instant) {
    game_state
        .tower_emit_effects
        .retain(|effect| !effect.is_expired(now));
    game_state
        .target_hit_effects
        .retain(|effect| !effect.is_expired(now));
}

fn handle_monster_death(
    game_state: &mut GameState,
    target_idx: usize,
    target_xy: Xy<f32>,
    now: Instant,
    monster_death_emitters: &mut Vec<field_particle::emitter::MonsterDeathEmitter>,
) {
    // 몬스터 데이터 먼저 추출 (mutable borrow 충돌 회피)
    let monster_max_hp = game_state.monsters[target_idx].max_hp;
    let monster_reward = game_state.monsters[target_idx].reward;
    let monster_kind = game_state.monsters[target_idx].kind;
    let rotation = game_state.monsters[target_idx].animation.rotation;
    let y_offset = game_state.monsters[target_idx].animation.y_offset;

    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        defense_flow.stage_progress.processed_hp += monster_max_hp;
    }

    let earn = monster_reward + game_state.upgrade_state.gold_earn_plus;
    let earn = (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;

    monster_death_emitters.push(field_particle::emitter::MonsterDeathEmitter::new(target_xy));

    let wh = monster::monster_wh(monster_kind);

    let tile_base_xy = TILE_PX_SIZE.to_xy() * target_xy;
    let monster_center_offset = Xy::new(
        TILE_PX_SIZE.width * 0.5,
        TILE_PX_SIZE.height - wh.height * 0.5 + TILE_PX_SIZE.height * y_offset,
    );
    let pixel_xy = tile_base_xy + monster_center_offset;

    let corpse_particle =
        field_particle::MonsterCorpseParticle::new(pixel_xy, now, rotation, monster_kind, wh);
    game_state.field_particle_system_manager.add_emitters(vec![
        field_particle::FieldParticleEmitter::MonsterCorpse {
            emitter: field_particle::TempParticleEmitter::new(vec![
                field_particle::FieldParticle::MonsterCorpse {
                    particle: corpse_particle,
                },
            ]),
        },
    ]);

    game_state.earn_gold(earn);
    game_state.monsters.swap_remove(target_idx);
}
