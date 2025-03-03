use crate::upgrade::TowerUpgradeTarget;

use super::*;

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

    crate::game_state::monster::remove_monster_finished_status_effects(game_state, now);
    crate::game_state::tower::remove_tower_finished_status_effects(game_state, now);

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
        ..
    } = game_state;

    let mut killed_monster_count = 0;

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
            killed_monster_count += 1;
            monsters.swap_remove(monster_index);
        }

        false
    });

    if killed_monster_count > 0 {
        game_state.earn_gold_by_kill_monsters(killed_monster_count);
    }
}

fn shoot_projectiles(game_state: &mut GameState) {
    let projectiles = game_state.towers.iter_mut().filter_map(|tower| {
        if tower.in_cooltime() {
            return None;
        }

        let attack_range_radius = tower.attack_range_radius();

        let Some(target) = game_state.monsters.iter().find(|monster| {
            (monster.move_on_route.xy() - tower.left_top.map(|t| t as f32)).length()
                < attack_range_radius
        }) else {
            return None;
        };

        let tower_upgrades = [
            TowerUpgradeTarget::Rank { rank: tower.rank },
            TowerUpgradeTarget::Suit { suit: tower.suit },
        ]
        .iter()
        .map(|target| {
            game_state
                .upgrade_state
                .tower_upgrade_states
                .get(target)
                .map(|x| *x)
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

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

    game_state.stage += 1;
    if game_state.stage > 50 {
        // Game clear
        return;
    }

    match game_state.stage {
        15 | 25 | 30 | 35 | 40 | 45 | 46 | 47 | 48 | 49 | 50 => {
            game_state.goto_selecting_upgrade();
        }
        _ => {
            game_state.goto_selecting_tower();
        }
    }
}
