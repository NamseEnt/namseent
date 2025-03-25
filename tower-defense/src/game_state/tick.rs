use super::{quest::QuestTriggerEvent, *};

const TICK_MAX_DURATION: Duration = Duration::from_millis(16);

pub struct Ticker {}

impl Component for Ticker {
    fn render(self, ctx: &RenderCtx) {
        ctx.interval("game state tick", TICK_MAX_DURATION, |mut dt| {
            crate::game_state::mutate_game_state(move |game_state| {
                let now = Instant::now();
                let mut tick_now = now - dt;
                while dt.is_positive() && dt.as_millis() > 0 {
                    let tick_dt = dt.min(TICK_MAX_DURATION);
                    dt -= tick_dt;

                    tick_now += tick_dt;

                    tick(game_state, tick_dt, tick_now);
                }
            });
        });
    }
}

fn tick(game_state: &mut GameState, dt: Duration, now: Instant) {
    crate::game_state::monster_spawn::tick(game_state, now);
    crate::game_state::tower::tower_cooldown_tick(game_state, dt);
    crate::game_state::tower::tower_animation_tick(game_state, now);
    crate::game_state::field_area_effect::field_area_effect_tick(game_state, now);

    crate::game_state::monster::remove_monster_finished_status_effects(game_state, now);
    crate::game_state::tower::remove_tower_finished_status_effects(game_state, now);
    crate::game_state::user_status_effect::remove_user_finished_status_effects(game_state, now);
    crate::game_state::field_area_effect::remove_finished_field_area_effects(game_state, now);

    crate::game_state::monster::activate_monster_skills(game_state, now);
    crate::game_state::tower::activate_tower_skills(game_state, now);

    crate::game_state::monster::move_monsters(game_state, dt);

    move_projectiles(game_state, dt);
    shoot_projectiles(game_state);
    check_defense_end(game_state);
}

fn move_projectiles(game_state: &mut GameState, dt: Duration) {
    let GameState {
        projectiles,
        monsters,
        gold,
        ..
    } = game_state;

    let mut total_earn_gold = 0;

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

        monster.get_damage(projectile.damage);

        if monster.dead() {
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            *gold += earn;
            total_earn_gold += earn;

            monsters.swap_remove(monster_index);
        }

        false
    });

    if total_earn_gold > 0 {
        on_quest_trigger_event(
            game_state,
            QuestTriggerEvent::EarnGold {
                gold: total_earn_gold,
            },
        );
    }
}

fn shoot_projectiles(game_state: &mut GameState) {
    let GameState {
        towers,
        upgrade_state,
        ..
    } = game_state;
    let projectiles = towers.iter_mut().filter_map(|tower| {
        if tower.in_cooltime() {
            return None;
        }

        let tower_upgrades = upgrade_state.tower_upgrades(&tower);

        let attack_range_radius = tower.attack_range_radius(&tower_upgrades);

        let Some(target) = game_state.monsters.iter().find(|monster| {
            (monster.move_on_route.xy() - tower.left_top.map(|t| t as f32)).length()
                < attack_range_radius
        }) else {
            return None;
        };

        Some(tower.shoot(target.projectile_target_indicator, &tower_upgrades))
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
        on_quest_trigger_event(game_state, quest::QuestTriggerEvent::ClearBossRound);
    } else {
        game_state.goto_selecting_tower();
    }
}
