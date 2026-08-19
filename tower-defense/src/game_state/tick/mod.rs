pub(crate) mod defense_end;
pub(crate) mod monster_death;
mod resolve;
pub(crate) mod scheduler;
mod shoot;

use super::*;
use crate::game_state::attack::TowerInfo;

/// 공격이 몬스터에 명중했을 때의 정보. 모든 공격 경로(Spatial/Timed/Laser)가 동일한 struct를 사용.
pub(super) struct MonsterHit {
    pub target_idx: usize,
    pub damage: f32,
    pub at_xy: MapCoordF32,
    pub source_tower: Option<TowerInfo>,
    pub on_hit_splashes: Vec<crate::card::EngravingSplash>,
}

pub(super) struct AreaDamageEvent {
    pub center: MapCoordF32,
    pub damage: f32,
    pub source_tower: TowerInfo,
    pub splashes: Vec<crate::card::EngravingSplash>,
}

pub(crate) const TICK_MAX_DURATION: Duration = Duration::from_millis(16);

fn sim_step_duration() -> Duration {
    Duration::from_secs_f32(1.0 / 60.0)
}

pub struct Ticker {
    pub presentation_instant: PresentationInstant,
}

impl Component for Ticker {
    fn render(self, ctx: &RenderCtx) {
        let game_state = crate::game_state::use_game_state(ctx);
        let presentation_instant = self.presentation_instant;
        ctx.interval("game state tick", TICK_MAX_DURATION, |real_dt| {
            mutate_game_state(move |game_state| {
                let presentation_delta = PresentationDelta::from_namui(real_dt);
                let report = game_state
                    .sim_scheduler
                    .advance(presentation_delta, game_state.fast_forward_multiplier);
                game_state.sim_scheduler_report = report;
                for _ in 0..report.executed_steps {
                    step_simulation(game_state, presentation_instant);
                    if !game_state.headless {
                        game_state.flush_effect_events();
                    } else {
                        game_state.effect_events.events.clear();
                    }
                }
                if !game_state.headless {
                    tick_presentation(game_state, presentation_instant, presentation_delta);
                }
            });
        });
        game_state.record_as_used();
    }
}

pub(crate) fn step_simulation(
    game_state: &mut GameState,
    presentation_instant: PresentationInstant,
) {
    game_state.sim_tick += SimTickSpan::ONE;
    let sim_tick = game_state.sim_tick();
    tick_logic(
        game_state,
        sim_step_duration(),
        sim_tick,
        presentation_instant,
    );
    tick_world_visuals(game_state, sim_tick, presentation_instant);
}

fn tick_logic(
    game_state: &mut GameState,
    dt: Duration,
    sim_tick: SimTick,
    presentation_instant: PresentationInstant,
) {
    monster_spawn::tick(game_state, sim_tick);
    tower::tower_cooldown_tick(game_state);

    monster::remove_monster_finished_status_effects(game_state, sim_tick);
    tower::remove_tower_finished_status_effects(game_state, sim_tick);
    user_status_effect::remove_user_finished_status_effects(game_state, sim_tick);

    monster::activate_monster_skills(game_state, sim_tick);
    tower::activate_tower_skills(game_state, sim_tick);

    monster::move_monsters(game_state, dt);

    resolve::update_in_flight_attacks(game_state, dt, presentation_instant);
    shoot::shoot_attacks(game_state, presentation_instant);
    defense_end::check_defense_end(game_state);
}

fn tick_world_visuals(
    game_state: &mut GameState,
    sim_tick: SimTick,
    presentation_instant: PresentationInstant,
) {
    let dt = sim_step_duration();
    game_state.update_base_animations(sim_tick);
    tower::tower_animation_tick(game_state, sim_tick);
    tower::tick_royal_straight_flush_visuals(game_state, sim_tick, presentation_instant);
    monster::monster_animation_tick(game_state, dt);
}

fn tick_presentation(
    game_state: &mut GameState,
    presentation_instant: PresentationInstant,
    presentation_delta: PresentationDelta,
) {
    let dt = Duration::from_secs_f32(presentation_delta.as_secs_f32());
    game_state.flow.update(presentation_instant);
    game_state.hand.update(presentation_instant);
    game_state.update_camera_shake(dt, presentation_instant);

    game_state.ui_state.tick(presentation_instant);

    if game_state.ui_state.should_cleanup(presentation_instant) {
        game_state.cleanup_unused_tower_popup_states();
    }

    let presentation_now = presentation_instant.as_namui();
    status_effect_particle_generator::tick_status_effect_particle_generator(
        game_state,
        presentation_instant,
    );

    field_particle::emitter::tick_black_smoke_emitters(
        &mut game_state.black_smoke_sources,
        presentation_now,
        dt,
    );
    field_particle::tick_all_emitters(presentation_now, dt);
}

/// Headless tick for simulation - skips rendering/animation/particle side effects.
/// Game logic is identical to the normal tick.
#[cfg(feature = "simulator")]
pub(crate) fn tick_headless(game_state: &mut GameState) {
    step_simulation(game_state, PresentationInstant::zero());
    game_state.black_smoke_sources.clear();
    game_state.effect_events.events.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rendered_and_headless_modes_share_the_same_simulation_step() {
        let mut rendered = crate::game_state::create_game_state_with_seed(0x5eed);
        let mut headless = crate::game_state::create_game_state_with_seed(0x5eed);
        headless.headless = true;

        for _ in 0..120 {
            step_simulation(&mut rendered, PresentationInstant::zero());
            step_simulation(&mut headless, PresentationInstant::zero());
        }

        assert_eq!(rendered.sim_tick(), headless.sim_tick());
        assert_eq!(rendered.gold, headless.gold);
        assert_eq!(rendered.hp, headless.hp);
        assert_eq!(rendered.shield, headless.shield);
        assert_eq!(rendered.monsters.len(), headless.monsters.len());
        for (rendered_monster, headless_monster) in
            rendered.monsters.iter().zip(headless.monsters.iter())
        {
            assert_eq!(rendered_monster.id(), headless_monster.id());
            assert_eq!(rendered_monster.hp, headless_monster.hp);
            assert_eq!(
                rendered_monster.center_xy_tile(),
                headless_monster.center_xy_tile()
            );
        }
        assert_eq!(
            rendered.in_flight_attacks.len(),
            headless.in_flight_attacks.len()
        );
    }
}
