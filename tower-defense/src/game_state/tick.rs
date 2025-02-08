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
