mod attack;
pub(crate) mod defense_end;
pub(crate) mod monster_death;
mod projectile;

use super::*;

pub(crate) const TICK_MAX_DURATION: Duration = Duration::from_millis(16);

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

                    tick_logic(game_state, tick_dt, game_state.game_now);
                    tick_visuals(game_state, tick_dt, game_state.game_now);
                    game_state.flush_effect_events();
                }
            });
        });
    }
}

fn tick_logic(game_state: &mut GameState, dt: Duration, now: Instant) {
    game_state.flow.update();
    game_state.hand.update();

    monster_spawn::tick(game_state, now);
    tower::tower_cooldown_tick(game_state, dt);

    monster::remove_monster_finished_status_effects(game_state, now);
    tower::remove_tower_finished_status_effects(game_state, now);
    user_status_effect::remove_user_finished_status_effects(game_state, now);

    monster::activate_monster_skills(game_state, now);
    tower::activate_tower_skills(game_state, now);

    monster::move_monsters(game_state, dt);

    projectile::move_projectiles(game_state, dt, now);
    attack::shoot_attacks(game_state);
    defense_end::check_defense_end(game_state);
}

fn tick_visuals(game_state: &mut GameState, dt: Duration, now: Instant) {
    game_state.update_camera_shake(dt);
    game_state.update_base_animations(now);

    tower::tower_animation_tick(game_state, now);
    tower::tick_royal_straight_flush_visuals(game_state, now);
    monster::monster_animation_tick(game_state, dt);

    game_state.ui_state.tick(now);

    if game_state.ui_state.should_cleanup(now) {
        game_state.cleanup_unused_tower_popup_states();
    }

    status_effect_particle_generator::tick_status_effect_particle_generator(game_state, now);

    field_particle::emitter::tick_black_smoke_emitters(
        &mut game_state.black_smoke_sources,
        now,
        dt,
    );
    field_particle::tick_all_emitters(now, dt);
}

/// Headless tick for simulation - skips rendering/animation/particle side effects.
/// Game logic is identical to the normal tick.
#[cfg(feature = "simulator")]
pub(crate) fn tick_headless(game_state: &mut GameState, dt: Duration) {
    let now = game_state.now();

    tick_logic(game_state, dt, now);

    game_state.black_smoke_sources.clear();
    game_state.effect_events.events.clear();
}
