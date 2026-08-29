use super::*;
use crate::game_state::camera::ShakeIntensity;
use crate::{SimTick, SimTickSpan};
use namui::*;

const BASE_SIZE_TILE: f32 = 3.0;
const BASE_TRANSIT_FORCE_DURATION: SimTickSpan = SimTickSpan::from_millis_ceil(33);
const ENEMY_BASE_SPAWN_FORCE: f32 = -320.0;
const PLAYER_DAMAGE_FORCE_MULTIPLIER: f32 = 14.0;
const BASE_SPRING_STIFFNESS: f32 = -1500.0;
const BASE_SPRING_DAMPING: f32 = -10.0;

#[derive(State, Clone)]
pub struct BaseAnimationState {
    enemy_base_animation: BaseSpringAnimation,
    player_base_animation: BaseSpringAnimation,
}

impl Default for BaseAnimationState {
    fn default() -> Self {
        Self::new(SimTick::ZERO)
    }
}

impl BaseAnimationState {
    pub fn new(sim_tick: SimTick) -> Self {
        Self {
            enemy_base_animation: BaseSpringAnimation::new(sim_tick),
            player_base_animation: BaseSpringAnimation::new(sim_tick),
        }
    }

    fn trigger_enemy_spawn(&mut self, sim_tick: SimTick) {
        self.enemy_base_animation
            .trigger(ENEMY_BASE_SPAWN_FORCE, sim_tick);
    }

    fn trigger_player_damage(&mut self, sim_tick: SimTick, intensity: f32) {
        self.player_base_animation
            .trigger(intensity * PLAYER_DAMAGE_FORCE_MULTIPLIER, sim_tick);
    }

    fn update(&mut self, sim_tick: SimTick) {
        self.enemy_base_animation.update(sim_tick);
        self.player_base_animation.update(sim_tick);
    }
}

#[derive(Clone, PartialEq, State)]
struct BaseSpringAnimation {
    tick_at: SimTick,
    y_ratio_offset: f32,
    y_ratio_velocity: f32,
    transit_force: Option<TransitForce>,
}

impl BaseSpringAnimation {
    fn new(sim_tick: SimTick) -> Self {
        Self {
            tick_at: sim_tick,
            y_ratio_offset: 0.0,
            y_ratio_velocity: 0.0,
            transit_force: None,
        }
    }

    fn trigger(&mut self, force: f32, sim_tick: SimTick) {
        self.transit_force = Some(TransitForce {
            force,
            end_at: sim_tick + BASE_TRANSIT_FORCE_DURATION,
        });
        self.tick_at = sim_tick;
    }

    fn update(&mut self, sim_tick: SimTick) {
        const DELTA_TIME_SECONDS: f32 = 1.0 / 60.0;
        let delta_time = DELTA_TIME_SECONDS;
        self.tick_at = sim_tick;

        let transit_force_expired = self
            .transit_force
            .is_some_and(|transit_force| transit_force.end_at < sim_tick);
        let transit_force = self
            .transit_force
            .map(|transit_force| transit_force.force)
            .unwrap_or(0.0);

        let spring_force = BASE_SPRING_STIFFNESS * self.y_ratio_offset;
        let damping_force = BASE_SPRING_DAMPING * self.y_ratio_velocity;
        let acceleration = spring_force + damping_force + transit_force;

        self.y_ratio_velocity += acceleration * delta_time;
        self.y_ratio_offset += self.y_ratio_velocity * delta_time;

        if transit_force_expired {
            self.transit_force = None;
        }
    }

    fn scale_xy(&self) -> Xy<f32> {
        let y_scale = (1.0 + self.y_ratio_offset).clamp(0.75, 1.2);
        let x_scale = (1.0 - self.y_ratio_offset * 0.35).clamp(0.85, 1.15);
        Xy::new(x_scale, y_scale)
    }
}

#[derive(Clone, Copy, PartialEq, State)]
struct TransitForce {
    force: f32,
    end_at: SimTick,
}

impl GameState {
    pub fn on_enemy_spawned(&mut self) {
        self.pending_presentation_events
            .push(PresentationEvent::AnimateBase(
                BaseAnimationEvent::EnemySpawn,
            ));
    }

    pub fn on_player_damaged(&mut self, intensity: ShakeIntensity) {
        self.pending_presentation_events
            .push(PresentationEvent::AnimateBase(
                BaseAnimationEvent::PlayerDamage {
                    intensity: intensity.value(),
                },
            ));
        self.pending_presentation_events
            .push(PresentationEvent::ShakeCamera {
                intensity: intensity.value(),
            });
    }

    pub(crate) fn apply_presentation_triggers(
        &mut self,
        presentation_instant: crate::PresentationInstant,
        base_animation_state: &mut BaseAnimationState,
        black_smoke_sources: &mut Vec<field_particle::emitter::BlackSmokeSource>,
    ) {
        let events = std::mem::take(&mut self.pending_presentation_events.events);
        let mut remaining = Vec::with_capacity(events.len());
        for event in events {
            match event {
                PresentationEvent::AnimateBase(BaseAnimationEvent::EnemySpawn) => {
                    base_animation_state.trigger_enemy_spawn(self.sim_tick());
                }
                PresentationEvent::AnimateBase(BaseAnimationEvent::PlayerDamage { intensity }) => {
                    base_animation_state.trigger_player_damage(self.sim_tick(), intensity);
                }
                PresentationEvent::SpawnRoyalStraightFlushVisual {
                    tower_id,
                    target_xy,
                    target_monster_id,
                    sim_tick,
                } => {
                    let tower_center_xy = self
                        .raw_render_snapshot()
                        .towers
                        .iter()
                        .find(|tower| tower.id == tower_id.raw())
                        .map(|tower| {
                            (
                                tower.left_top[0] as f32 + 1.5,
                                tower.left_top[1] as f32 + 1.5,
                            )
                        });
                    if let Some(tower_center_xy) = tower_center_xy {
                        crate::game_state::tower::spawn_royal_straight_flush_visual(
                            &mut self.presentation_metadata,
                            &mut self.pending_presentation_events,
                            crate::game_state::tower::RoyalStraightFlushVisualParams {
                                tower_id,
                                tower_center_xy,
                                target_xy: (target_xy[0], target_xy[1]),
                                target_monster_id,
                                sim_tick,
                                presentation_instant,
                            },
                            black_smoke_sources,
                        );
                    }
                }

                event => remaining.push(event),
            }
        }
        self.pending_presentation_events.events = remaining;
    }

    pub(crate) fn update_base_animations(
        &mut self,
        sim_tick: SimTick,
        base_animation_state: &mut BaseAnimationState,
    ) {
        base_animation_state.update(sim_tick);
    }

    pub(crate) fn render_base_scales(
        &self,
        base_animation_state: &BaseAnimationState,
    ) -> (Xy<f32>, Xy<f32>) {
        (
            base_animation_state.enemy_base_animation.scale_xy(),
            base_animation_state.player_base_animation.scale_xy(),
        )
    }
}

pub(crate) fn render_bases(ctx: &RenderCtx, game_state: &crate::headed_game::HeadedGame) {
    let scales = game_state
        .render_frame()
        .and_then(|frame| frame.base_scales(true))
        .unwrap_or_else(|| game_state.render_base_scales());
    render_enemy_base(ctx, scales.0);
    render_player_base(ctx, scales.1);
}

fn render_enemy_base(ctx: &RenderCtx, animated_scale: Xy<f32>) {
    let center = coord_center_px(TRAVEL_POINTS[0]) + Xy::new(0.px(), TILE_PX_SIZE.height * -1.0);

    draw_base_image(
        ctx,
        crate::asset::image::environment::ENEMY_BASE,
        center,
        animated_scale,
    );
}

fn render_player_base(ctx: &RenderCtx, animated_scale: Xy<f32>) {
    let center = coord_center_px(TRAVEL_POINTS[TRAVEL_POINTS.len() - 1])
        + Xy::new(TILE_PX_SIZE.width * 1.0, TILE_PX_SIZE.height * -1.0);

    draw_base_image(
        ctx,
        crate::asset::image::environment::PLAYER_BASE,
        center,
        animated_scale,
    );
}

fn draw_base_image(ctx: &RenderCtx, image: Image, center: Xy<Px>, scale: Xy<f32>) {
    let base_wh = TILE_PX_SIZE * BASE_SIZE_TILE;
    let bottom_center = Xy::new(0.px(), base_wh.height * 0.5);

    ctx.translate(center + bottom_center)
        .scale(scale)
        .add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::new(-base_wh.width * 0.5, -base_wh.height), base_wh),
            image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));
}

fn coord_center_px(coord: MapCoord) -> Xy<Px> {
    Xy::new(
        (coord.x.as_f32() + 0.5) * TILE_PX_SIZE.width.as_f32(),
        (coord.y.as_f32() + 0.5) * TILE_PX_SIZE.height.as_f32(),
    )
    .map(px)
}
