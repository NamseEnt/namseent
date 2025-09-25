use super::*;

const TICK_MAX_DURATION: Duration = Duration::from_millis(16);

pub struct Ticker {}

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
    check_game_initialized(game_state);

    game_state.flow.update();
    flow::contract::update_contract_flow(game_state);

    monster_spawn::tick(game_state, now);
    tower::tower_cooldown_tick(game_state, dt);
    tower::tower_animation_tick(game_state, now);
    monster::monster_animation_tick(game_state, dt);

    monster::remove_monster_finished_status_effects(game_state, now);
    tower::remove_tower_finished_status_effects(game_state, now);
    user_status_effect::remove_user_finished_status_effects(game_state, now);
    field_particle::remove_finished_field_particle_systems(game_state, now);

    monster::activate_monster_skills(game_state, now);
    tower::activate_tower_skills(game_state, now);

    monster::move_monsters(game_state, dt);

    move_projectiles(game_state, dt);
    shoot_projectiles(game_state);
    check_defense_end(game_state);
}

fn move_projectiles(game_state: &mut GameState, dt: Duration) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    let mut total_earn_gold = 0;
    let mut damage_emitters = Vec::new();

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
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            let earn =
                (earn as f32 * game_state.contract_state.get_gold_gain_multiplier()) as usize;
            total_earn_gold += earn;
            monsters.swap_remove(monster_index);
        }

        false
    });

    emit_damage_text_particles(game_state, damage_emitters);

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

fn shoot_projectiles(game_state: &mut GameState) {
    let now = game_state.now();
    let GameState {
        towers,
        upgrade_state,
        ..
    } = game_state;
    let projectiles = towers.iter_mut().filter_map(|tower| {
        if tower.in_cooltime() {
            return None;
        }

        // Check if tower rank is disabled by contract
        if game_state
            .contract_state
            .get_disabled_ranks()
            .contains(&tower.rank())
        {
            return None;
        }

        // Check if tower suit is disabled by contract
        if game_state
            .contract_state
            .get_disabled_suits()
            .contains(&tower.suit())
        {
            return None;
        }

        let tower_upgrades = upgrade_state.tower_upgrades(tower);

        let attack_range_radius = tower.attack_range_radius(
            &tower_upgrades,
            game_state.contract_state.get_range_multiplier(),
        );

        let target = game_state.monsters.iter().find(|monster| {
            (monster.move_on_route.xy() - tower.left_top.map(|t| t as f32)).length()
                < attack_range_radius
        })?;

        let contract_multiplier = game_state.contract_state.get_damage_multiplier();

        Some(tower.shoot(
            target.projectile_target_indicator,
            &tower_upgrades,
            contract_multiplier,
            now,
        ))
    });

    game_state.projectiles.extend(projectiles);
}

fn check_defense_end(game_state: &mut GameState) {
    let GameFlow::Defense = game_state.flow else {
        return;
    };
    let MonsterSpawnState::Idle = game_state.monster_spawn_state else {
        return;
    };
    if !game_state.monsters.is_empty() {
        return;
    }

    let is_boss_stage = is_boss_stage(game_state.stage);
    game_state.stage += 1;
    if game_state.stage > 50 {
        // Game clear
        return;
    }

    if is_boss_stage {
        game_state.goto_selecting_upgrade();
    } else {
        game_state.goto_next_stage();
    }
}

fn check_game_initialized(game_state: &mut GameState) {
    if !matches!(game_state.flow, GameFlow::Initializing) {
        return;
    }

    game_state.goto_next_stage();
}
