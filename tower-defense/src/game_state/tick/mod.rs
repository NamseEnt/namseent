pub(crate) mod defense_end;
pub(crate) mod monster_death;
mod resolve;
pub(crate) mod scheduler;
mod shoot;

use super::*;
use crate::Damage;
use crate::WorldCoord;
use crate::game_state::attack::TowerInfo;

/// 공격이 몬스터에 명중했을 때의 정보. 모든 공격 경로(Spatial/Timed/Laser)가 동일한 struct를 사용.
pub(super) struct MonsterHit {
    pub target_idx: usize,
    pub damage: Damage,
    pub at_xy: WorldCoord,
    pub source_tower: Option<TowerInfo>,
    pub on_hit_splashes: Vec<crate::card::EngravingSplash>,
}

pub(super) struct AreaDamageEvent {
    pub center: WorldCoord,
    pub damage: Damage,
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
    tick_logic(game_state, sim_tick, presentation_instant);
    tick_world_visuals(game_state, sim_tick, presentation_instant);
}

fn tick_logic(
    game_state: &mut GameState,
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

    monster::move_monsters(game_state);

    resolve::update_in_flight_attacks(game_state, presentation_instant);
    shoot::shoot_attacks(game_state, presentation_instant);
    monster::resolve_base_damage(game_state);
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

    #[test]
    fn repeated_initial_states_have_identical_authoritative_snapshots() {
        let mut left = crate::game_state::create_game_state_with_seed(0x0D3F_3ACE);
        let mut right = crate::game_state::create_game_state_with_seed(0x0D3F_3ACE);
        left.action(crate::game_state::GameStateAction::StartDefense);
        right.action(crate::game_state::GameStateAction::StartDefense);
        step_simulation(&mut left, PresentationInstant::zero());
        step_simulation(&mut right, PresentationInstant::zero());
        assert!(!left.monsters.is_empty());
        assert!(!right.monsters.is_empty());

        let add_homing_attack = |game_state: &mut GameState| {
            let target = &game_state.monsters[0];
            let target_xy = target.center_world_xy();
            let target_indicator = target.projectile_target_indicator;
            let attack_id = crate::AttackId::from_entity_id(game_state.allocate_entity_id());
            game_state.in_flight_attacks.push(
                crate::game_state::attack::InFlightAttack::new_spatial(
                    attack_id,
                    crate::game_state::attack::SpatialAttack::new_homing(
                        target_xy - crate::WorldVec::new(8 * crate::world::WORLD_UNITS_PER_TILE, 0),
                        target_indicator,
                        0x00A7_7ACE,
                        crate::game_state::projectile::ProjectileKind::Cards00,
                        crate::game_state::projectile::ProjectileTrail::None,
                        crate::game_state::attack::ProjectileHitEffect::CardBurst,
                    ),
                    Damage::from_integer(1_000_000),
                    None,
                ),
            );
        };
        add_homing_attack(&mut left);
        add_homing_attack(&mut right);
        let initial_gold = left.gold;

        for _ in 0..240 {
            step_simulation(&mut left, PresentationInstant::zero());
            step_simulation(&mut right, PresentationInstant::zero());
            assert_eq!(left.sim_tick(), right.sim_tick());
            assert_eq!(left.hp, right.hp);
            assert_eq!(left.shield, right.shield);
            assert_eq!(left.gold, right.gold);
            assert_eq!(left.calculate_clear_rate(), right.calculate_clear_rate());
            assert_eq!(left.monsters.len(), right.monsters.len());
            for (a, b) in left.monsters.iter().zip(right.monsters.iter()) {
                assert_eq!(a.id(), b.id());
                assert_eq!(a.world_xy(), b.world_xy());
                assert_eq!(a.move_on_route.route_index(), b.move_on_route.route_index());
                assert_eq!(
                    a.move_on_route.route_progress(),
                    b.move_on_route.route_progress()
                );
                assert_eq!(a.hp, b.hp);
                assert_eq!(a.max_hp, b.max_hp);
                assert_eq!(a.damage, b.damage);
                assert_eq!(a.stage_progress_counted, b.stage_progress_counted);
            }
            assert_eq!(left.in_flight_attacks.len(), right.in_flight_attacks.len());
            for (a, b) in left
                .in_flight_attacks
                .iter()
                .zip(right.in_flight_attacks.iter())
            {
                assert_eq!(a.id, b.id);
                assert_eq!(a.damage, b.damage);
                assert_eq!(a.source_tower, b.source_tower);
                assert_eq!(a.on_hit_splashes, b.on_hit_splashes);
                match (&a.kind, &b.kind) {
                    (
                        crate::game_state::attack::InFlightAttackKind::Spatial(a),
                        crate::game_state::attack::InFlightAttackKind::Spatial(b),
                    ) => {
                        assert_eq!(a.xy, b.xy);
                        assert_eq!(a.target_indicator.id(), b.target_indicator.id());
                        assert_eq!(a.velocity, b.velocity);
                        assert_eq!(a.projectile_kind, b.projectile_kind);
                        assert_eq!(a.trail, b.trail);
                        assert_eq!(a.behavior, b.behavior);
                        assert_eq!(a.hit_effect, b.hit_effect);
                        assert_eq!(a.movement_remainder, b.movement_remainder);
                        assert_eq!(a.stable_key, b.stable_key);
                    }
                    (
                        crate::game_state::attack::InFlightAttackKind::Timed(a),
                        crate::game_state::attack::InFlightAttackKind::Timed(b),
                    ) => {
                        assert_eq!(a.target_monster_id, b.target_monster_id);
                        assert_eq!(a.execute_at, b.execute_at);
                    }
                    (
                        crate::game_state::attack::InFlightAttackKind::Laser(a),
                        crate::game_state::attack::InFlightAttackKind::Laser(b),
                    ) => {
                        assert_eq!(a.start_xy, b.start_xy);
                        assert_eq!(a.end_xy, b.end_xy);
                        assert_eq!(a.created_at, b.created_at);
                        assert_eq!(a.target_monster_id, b.target_monster_id);
                    }
                    _ => panic!("attack kinds diverged"),
                }
            }
            left.effect_events.events.clear();
            right.effect_events.events.clear();
        }
        assert!(
            left.gold > initial_gold,
            "injected attack must resolve a death"
        );
        assert!(left.in_flight_attacks.is_empty());
    }
}
