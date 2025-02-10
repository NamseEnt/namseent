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
    crate::game_state::spawn_tick(game_state, now);
    move_monsters(game_state, dt);
    move_projectiles(game_state, dt);
    shoot_projectiles(game_state, now);
}

fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        monster.move_on_route.move_by(dt);
    }

    // todo: deal damage to user
    game_state
        .monsters
        .retain(|monster| !monster.move_on_route.is_finished());
}

fn move_projectiles(game_state: &mut GameState, dt: Duration) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    projectiles.retain_mut(|projectile| {
        let start_xy = projectile.xy;

        let Some(monster_index) = monsters.iter().enumerate().find_map(|(i, monster)| {
            if monster.projectile_target_indicator == projectile.target_indicator {
                Some(i)
            } else {
                None
            }
        }) else {
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
            monsters.swap_remove(monster_index);
        }

        return false;
    });
}

fn shoot_projectiles(game_state: &mut GameState, now: Instant) {
    let projectiles = game_state
        .towers
        .iter_mut()
        .map(|(tower_xy, tower)| {
            if tower.in_cooltime(now) {
                return None;
            }

            let Some(target) = game_state.monsters.iter().find(|monster| {
                (monster.move_on_route.xy() - tower_xy.map(|t| t as f32)).length()
                    < tower.attack_range_radius
            }) else {
                return None;
            };

            Some(tower.shoot(target.projectile_target_indicator, now))
        })
        .flatten();

    game_state.projectiles.extend(projectiles);
}
