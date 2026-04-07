use super::*;
use crate::game_state::camera::ShakeIntensity;
use namui::*;

const BASE_SIZE_TILE: f32 = 1.5;
const BASE_TRANSIT_FORCE_DURATION: Duration = Duration::from_millis(33);
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
        Self::new(Instant::now())
    }
}

impl BaseAnimationState {
    pub fn new(now: Instant) -> Self {
        Self {
            enemy_base_animation: BaseSpringAnimation::new(now),
            player_base_animation: BaseSpringAnimation::new(now),
        }
    }

    fn trigger_enemy_spawn(&mut self, now: Instant) {
        self.enemy_base_animation
            .trigger(ENEMY_BASE_SPAWN_FORCE, now);
    }

    fn trigger_player_damage(&mut self, now: Instant, intensity: ShakeIntensity) {
        self.player_base_animation
            .trigger(intensity.value() * PLAYER_DAMAGE_FORCE_MULTIPLIER, now);
    }

    fn update(&mut self, now: Instant) {
        self.enemy_base_animation.update(now);
        self.player_base_animation.update(now);
    }
}

#[derive(Clone, PartialEq, State)]
struct BaseSpringAnimation {
    tick_at: Instant,
    y_ratio_offset: f32,
    y_ratio_velocity: f32,
    transit_force: Option<TransitForce>,
}

impl BaseSpringAnimation {
    fn new(now: Instant) -> Self {
        Self {
            tick_at: now,
            y_ratio_offset: 0.0,
            y_ratio_velocity: 0.0,
            transit_force: None,
        }
    }

    fn trigger(&mut self, force: f32, now: Instant) {
        self.transit_force = Some(TransitForce {
            force,
            end_at: now + BASE_TRANSIT_FORCE_DURATION,
        });
        self.tick_at = now;
    }

    fn update(&mut self, now: Instant) {
        let delta_time = (now - self.tick_at).as_secs_f32();
        self.tick_at = now;
        if delta_time <= 0.0 {
            return;
        }

        let transit_force_expired = self
            .transit_force
            .is_some_and(|transit_force| transit_force.end_at < now);
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
    end_at: Instant,
}

impl GameState {
    pub fn on_enemy_spawned(&mut self) {
        self.base_animation_state.trigger_enemy_spawn(self.now());
    }

    pub fn on_player_damaged(&mut self, intensity: ShakeIntensity) {
        self.base_animation_state
            .trigger_player_damage(self.now(), intensity);
    }

    pub fn update_base_animations(&mut self, now: Instant) {
        self.base_animation_state.update(now);
    }
}

pub fn render_bases(ctx: &RenderCtx, game_state: &GameState) {
    render_enemy_base(ctx, game_state);
    render_player_base(ctx, game_state);
}

fn render_enemy_base(ctx: &RenderCtx, game_state: &GameState) {
    let center = coord_center_px(TRAVEL_POINTS[0]) + Xy::new(0.px(), TILE_PX_SIZE.height * -0.5);
    let animated_scale = game_state
        .base_animation_state
        .enemy_base_animation
        .scale_xy();

    draw_base_image(
        ctx,
        crate::asset::image::environment::ENEMY_BASE,
        center,
        animated_scale,
    );
}

fn render_player_base(ctx: &RenderCtx, game_state: &GameState) {
    let center = coord_center_px(TRAVEL_POINTS[TRAVEL_POINTS.len() - 1])
        + Xy::new(TILE_PX_SIZE.width * 0.5, 0.px());
    let animated_scale = game_state
        .base_animation_state
        .player_base_animation
        .scale_xy();

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
