pub(crate) mod scheduler;

use super::*;

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
            crate::game_state::mutate_headed_game(move |game_state| {
                game_state.advance_presentation(presentation_instant);
                if !game_state.sim_scheduler.has_render_snapshot() {
                    let snapshot = game_state.capture_render_snapshot();
                    game_state.sim_scheduler.rebase_render_snapshot(snapshot);
                }
                let presentation_delta = PresentationDelta::from_namui(real_dt);
                let fast_forward_multiplier = game_state.fast_forward_multiplier;
                let mut report = if presentation_gate_is_active(
                    game_state.headless,
                    game_state.presentation_is_blocking(),
                ) {
                    game_state
                        .sim_scheduler
                        .discard_blocked_frame(presentation_delta, fast_forward_multiplier)
                } else {
                    game_state
                        .sim_scheduler
                        .advance_frame(presentation_delta, fast_forward_multiplier)
                };
                game_state.sim_scheduler_report = report;
                let scheduled_ticks = report.executed_ticks;
                let mut executed_ticks = 0;
                for _ in 0..scheduled_ticks {
                    if presentation_gate_is_active(
                        game_state.headless,
                        game_state.presentation_is_blocking(),
                    ) {
                        report = game_state
                            .sim_scheduler
                            .discard_scheduled_ticks(scheduled_ticks - executed_ticks);
                        break;
                    }
                    advance_simulation_tick_at(game_state, presentation_instant);
                    game_state.consume_core_events(presentation_instant);
                    game_state.apply_presentation_triggers(presentation_instant);
                    game_state.flush_pending_action_effects();
                    executed_ticks += 1;
                    let sim_tick = game_state.sim_tick();
                    game_state.update_base_animations(sim_tick);
                    tick_world_visuals(game_state, sim_tick, presentation_instant);
                    let render_snapshot = game_state.capture_render_snapshot();
                    game_state
                        .sim_scheduler
                        .commit_render_snapshot(render_snapshot);
                    if !game_state.headless {
                        game_state.flush_presentation_events(presentation_instant);
                    } else {
                        game_state.clear_presentation_events();
                    }
                }
                if executed_ticks != scheduled_ticks {
                    report.executed_ticks = executed_ticks;
                    game_state.sim_scheduler_report = report;
                }
                if !game_state.headless {
                    update_presentation_frame(game_state, presentation_instant, presentation_delta);
                }
            });
        });
        game_state.record_as_used();
    }
}

fn presentation_gate_is_active(headless: bool, presentation_is_blocking: bool) -> bool {
    !headless && presentation_is_blocking
}

pub(crate) fn advance_simulation_tick(game_state: &mut GameState) {
    advance_simulation_tick_at(game_state, PresentationInstant::capture());
}

pub(crate) fn advance_simulation_tick_at(
    game_state: &mut GameState,
    presentation_instant: PresentationInstant,
) {
    game_state.step_raw_simulation_at(presentation_instant);
}

fn tick_world_visuals(
    game_state: &mut crate::headed_game::HeadedGame,
    sim_tick: SimTick,
    presentation_instant: PresentationInstant,
) {
    let dt = sim_step_duration();
    tower::tower_animation_tick(&mut game_state.state, sim_tick);
    tower::tick_royal_straight_flush_visuals(
        &mut game_state.state,
        sim_tick,
        presentation_instant,
        &mut game_state.black_smoke_sources,
    );
    monster::monster_animation_tick(&mut game_state.state, dt);
}

fn update_presentation_frame(
    game_state: &mut crate::headed_game::HeadedGame,
    presentation_instant: PresentationInstant,
    presentation_delta: PresentationDelta,
) {
    let dt = Duration::from_secs_f32(presentation_delta.as_secs_f32());
    game_state
        .state
        .presentation_flow_mut()
        .update(presentation_instant);
    game_state
        .state
        .presentation_hand_mut()
        .update(presentation_instant);
    game_state
        .state
        .presentation_inventory
        .update(presentation_instant);
    game_state
        .state
        .presentation_upgrades
        .update(presentation_instant);
    game_state
        .state
        .presentation_deck
        .update(presentation_instant);
    game_state
        .camera
        .update_shake(dt, presentation_instant - PresentationInstant::zero());

    game_state.ui_state.tick(presentation_instant);

    if game_state.ui_state.should_cleanup(presentation_instant) {
        game_state.cleanup_unused_tower_popup_states();
    }

    let presentation_now = presentation_instant.as_namui();
    status_effect_particle_generator::tick_status_effect_particle_generator(
        &mut game_state.state,
        &mut game_state.status_effect_particle_generator,
        presentation_instant,
    );

    field_particle::emitter::tick_black_smoke_emitters(
        &mut game_state.black_smoke_sources,
        presentation_now,
        dt,
    );
    field_particle::tick_all_emitters(presentation_now, dt);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    fn authoritative_hash(game_state: &GameState) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        game_state.sim_tick.hash(&mut hasher);
        game_state.hp.raw().hash(&mut hasher);
        game_state.shield_amount().raw().hash(&mut hasher);
        game_state.gold.hash(&mut hasher);
        game_state.monsters.len().hash(&mut hasher);
        for monster in &game_state.monsters {
            monster.id().hash(&mut hasher);
            monster.world_xy().hash(&mut hasher);
            monster.move_on_route.route_index().hash(&mut hasher);
            monster.move_on_route.route_progress().hash(&mut hasher);
            monster.hp.raw().hash(&mut hasher);
            monster.max_hp.raw().hash(&mut hasher);
            monster.damage.raw().hash(&mut hasher);
            monster.stage_progress_counted.hash(&mut hasher);
        }
        game_state.in_flight_attacks.len().hash(&mut hasher);
        for attack in &game_state.in_flight_attacks {
            attack.id.hash(&mut hasher);
            attack.damage.raw().hash(&mut hasher);
            std::mem::discriminant(&attack.kind).hash(&mut hasher);
            match &attack.kind {
                crate::game_state::attack::InFlightAttackKind::Spatial(spatial) => {
                    spatial.xy.hash(&mut hasher);
                    spatial.velocity.hash(&mut hasher);
                    spatial.target_indicator.id().hash(&mut hasher);
                    spatial.stable_key.hash(&mut hasher);
                }
                crate::game_state::attack::InFlightAttackKind::Timed(timed) => {
                    timed.target_monster_id.hash(&mut hasher);
                    timed.execute_at.hash(&mut hasher);
                }
                crate::game_state::attack::InFlightAttackKind::Laser(laser) => {
                    laser.start_xy.hash(&mut hasher);
                    laser.end_xy.hash(&mut hasher);
                    laser.created_at.hash(&mut hasher);
                    laser.target_monster_id.hash(&mut hasher);
                }
            }
        }
        hasher.finish()
    }

    #[test]
    fn rendered_and_headless_modes_share_the_same_simulation_step() {
        let mut rendered = crate::game_state::create_game_state_with_seed(0x5eed);
        let mut headless = crate::game_state::create_game_state_with_seed(0x5eed);
        headless.headless = true;

        for _ in 0..120 {
            advance_simulation_tick(&mut rendered);
            advance_simulation_tick(&mut headless);
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
    fn simulation_step_does_not_mutate_tower_or_monster_animation() {
        let mut game_state = crate::game_state::create_game_state_with_seed(0x5eed);
        let visual_state = |game_state: &GameState| {
            (
                game_state
                    .towers
                    .iter()
                    .map(|tower| tower.render_animation_state())
                    .collect::<Vec<_>>(),
                game_state
                    .monsters
                    .iter()
                    .map(|monster| (monster.animation.rotation, monster.animation.y_offset))
                    .collect::<Vec<_>>(),
                (Xy::single(1.0), Xy::single(1.0)),
            )
        };
        let before = visual_state(&game_state);

        advance_simulation_tick(&mut game_state);

        let after = visual_state(&game_state);
        assert!(before == after);
    }

    #[test]
    fn damage_presentation_trigger_waits_for_headed_application() {
        let mut game_state = crate::headed_game::HeadedGame::new(
            crate::game_state::create_game_state_with_seed(0x5eed),
        );
        let before_scales = game_state.render_base_scales();

        game_state.on_player_damaged(crate::game_state::camera::ShakeIntensity::Heavy);

        assert!(game_state.camera.shake_intensity == 0.0);
        assert!(game_state.render_base_scales() == before_scales);

        game_state.apply_presentation_triggers(PresentationInstant::zero());

        assert!(game_state.camera.shake_intensity == 30.0);
    }

    #[test]
    fn repeated_initial_states_have_identical_authoritative_snapshots() {
        let mut left = crate::game_state::create_game_state_with_seed(0x0D3F_3ACE);
        let mut right = crate::game_state::create_game_state_with_seed(0x0D3F_3ACE);
        left.apply_compatibility_action(crate::game_state::CompatibilityAction::StartDefense);
        right.apply_compatibility_action(crate::game_state::CompatibilityAction::StartDefense);
        advance_simulation_tick(&mut left);
        advance_simulation_tick(&mut right);
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
        left.sync_raw_core_from_projection();
        right.sync_raw_core_from_projection();
        let initial_gold = left.gold;
        let mut saw_core_event = false;

        for _ in 0..240 {
            advance_simulation_tick(&mut left);
            advance_simulation_tick(&mut right);
            let left_events = left.drain_core_events();
            let right_events = right.drain_core_events();
            assert_eq!(left_events, right_events);
            saw_core_event |= !left_events.is_empty();
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
            left.clear_presentation_events();
            right.clear_presentation_events();
        }
        assert!(
            left.gold > initial_gold,
            "injected attack must resolve a death"
        );
        assert!(saw_core_event, "injected attack must emit a core event");
        assert!(left.in_flight_attacks.is_empty());
    }

    #[test]
    fn interpolation_on_and_off_do_not_mutate_authoritative_state() {
        let mut game_state = crate::game_state::create_game_state_with_seed(0x51A7);
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::StartDefense);
        advance_simulation_tick(&mut game_state);
        let mut scheduler = scheduler::FixedTickScheduler::default();
        scheduler.commit_render_snapshot(
            crate::game_state::render_snapshot::WorldRenderSnapshot::capture(&game_state),
        );
        let before = authoritative_hash(&game_state);
        {
            let frame = scheduler.render_frame().unwrap();
            if let Some(snapshot) = frame.current_snapshot() {
                for monster in snapshot.monsters() {
                    let _ = frame.sample_monster(monster.id, true);
                    let _ = frame.sample_monster(monster.id, false);
                }
                for projectile in snapshot.spatial_projectiles() {
                    let _ = frame.sample_projectile(projectile.id, true);
                    let _ = frame.sample_projectile(projectile.id, false);
                }
            }
        }
        assert_eq!(authoritative_hash(&game_state), before);
    }

    fn run_fixed_cadence(
        multiplier: crate::game_state::fast_forward::FastForwardMultiplier,
        frames: usize,
    ) -> GameState {
        let mut game_state = crate::game_state::create_game_state_with_seed(0xCADA_2026);
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::StartDefense);
        let mut scheduler = crate::game_state::tick::scheduler::FixedTickScheduler::default();
        for _ in 0..frames {
            let report =
                scheduler.advance_frame(PresentationDelta::from_nanos(16_666_667), multiplier);
            for _ in 0..report.executed_ticks {
                advance_simulation_tick(&mut game_state);
            }
        }
        game_state
    }

    #[test]
    fn x1_and_x8_have_the_same_authoritative_hash_at_the_same_tick() {
        let x1 = run_fixed_cadence(
            crate::game_state::fast_forward::FastForwardMultiplier::X1,
            120,
        );
        let x8 = run_fixed_cadence(
            crate::game_state::fast_forward::FastForwardMultiplier::X8,
            15,
        );
        assert_eq!(x1.sim_tick(), SimTick::from_ticks(120));
        assert_eq!(x8.sim_tick(), SimTick::from_ticks(120));
        assert_eq!(authoritative_hash(&x1), authoritative_hash(&x8));
    }

    #[test]
    fn headless_mode_bypasses_the_presentation_gate() {
        assert!(!presentation_gate_is_active(true, true));
        assert!(presentation_gate_is_active(false, true));
        assert!(!presentation_gate_is_active(false, false));
    }
}
