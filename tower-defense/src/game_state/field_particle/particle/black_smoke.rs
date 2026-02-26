use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;
use std::f32::consts::PI;

const BLACK_SMOKE_LIFETIME_MIN_MS: i64 = 220;
const BLACK_SMOKE_LIFETIME_MAX_MS: i64 = 420;
const BLACK_SMOKE_SIZE_MIN_TILE: f32 = 0.0;
const BLACK_SMOKE_SIZE_MAX_TILE: f32 = 0.8;
const BLACK_SMOKE_ALPHA_MIN: f32 = 0.0;
const BLACK_SMOKE_ALPHA_MAX: f32 = 0.6;
const BLACK_SMOKE_FLOOR_Y_OFFSET_TILE: f32 = 1.0;
const BLACK_SMOKE_ASCENT_MAX_TILE: f32 = 3.0;
const BLACK_SMOKE_X_COS_AMPLITUDE_TILE: f32 = 1.0;
const BLACK_SMOKE_X_COS_NARROW_POW: f32 = 0.85;
const BLACK_SMOKE_SCALE_GROW_END_PROGRESS: f32 = 0.6;
const BLACK_SMOKE_SCALE_START_RATIO: f32 = 0.45;
const BLACK_SMOKE_SCALE_END_RATIO: f32 = 0.72;
const BLACK_SMOKE_DASH_TRAIL_LIFETIME_MIN_MS: i64 = 160;
const BLACK_SMOKE_DASH_TRAIL_LIFETIME_MAX_MS: i64 = 500;
const BLACK_SMOKE_DASH_TRAIL_SIZE_MIN_TILE: f32 = 0.1;
const BLACK_SMOKE_DASH_TRAIL_SIZE_MAX_TILE: f32 = 0.8;
const BLACK_SMOKE_DASH_TRAIL_ALPHA_MAX: f32 = 0.125;
const BLACK_SMOKE_DASH_TRAIL_SCALE_START_RATIO: f32 = 0.1;
const BLACK_SMOKE_DASH_TRAIL_SCALE_END_RATIO: f32 = 0.8;

#[inline]
fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

#[inline]
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

#[derive(Clone)]
pub struct BlackSmokeParticle {
    pub base_xy: (f32, f32),
    pub prev_xy: (f32, f32),
    pub xy: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub alpha: f32,
    pub initial_alpha: f32,
    pub radius_px: Px,
    pub scale_ratio: f32,
    pub rotation_rad: f32,
    pub cos_phase_offset_rad: f32,
    pub reverse_progress: bool,
    pub velocity_xy: (f32, f32),
    pub sprite_kind: BlackSmokeSpriteKind,
}

#[derive(Clone, Copy)]
pub enum BlackSmokeSpriteKind {
    Smoke00,
    Smoke01,
}

impl BlackSmokeParticle {
    pub fn new_with_phase_offset<R: Rng + ?Sized>(
        xy: (f32, f32),
        now: Instant,
        phase_offset_rad: f32,
        reverse_progress: bool,
        rng: &mut R,
    ) -> Self {
        let base_xy = xy;
        let final_xy = (base_xy.0, base_xy.1 + BLACK_SMOKE_FLOOR_Y_OFFSET_TILE);

        let lifetime_ms = rng.gen_range(BLACK_SMOKE_LIFETIME_MIN_MS..=BLACK_SMOKE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let radius_tile = rng.gen_range(BLACK_SMOKE_SIZE_MIN_TILE..=BLACK_SMOKE_SIZE_MAX_TILE);
        let radius_px = TILE_PX_SIZE.width * radius_tile;

        let initial_alpha = rng.gen_range(BLACK_SMOKE_ALPHA_MIN..=BLACK_SMOKE_ALPHA_MAX);

        Self {
            base_xy,
            prev_xy: final_xy,
            xy: final_xy,
            created_at: now,
            lifetime,
            alpha: initial_alpha,
            initial_alpha,
            radius_px,
            scale_ratio: BLACK_SMOKE_SCALE_START_RATIO,
            rotation_rad: 0.0,
            cos_phase_offset_rad: phase_offset_rad,
            reverse_progress,
            velocity_xy: (0.0, 0.0),
            sprite_kind: BlackSmokeSpriteKind::Smoke00,
        }
    }

    pub fn new_dash_trail<R: Rng + ?Sized>(
        xy: (f32, f32),
        velocity_xy: (f32, f32),
        now: Instant,
        rng: &mut R,
    ) -> Self {
        let lifetime_ms = rng.gen_range(
            BLACK_SMOKE_DASH_TRAIL_LIFETIME_MIN_MS..=BLACK_SMOKE_DASH_TRAIL_LIFETIME_MAX_MS,
        );
        let lifetime = Duration::from_millis(lifetime_ms);

        let radius_tile = rng
            .gen_range(BLACK_SMOKE_DASH_TRAIL_SIZE_MIN_TILE..=BLACK_SMOKE_DASH_TRAIL_SIZE_MAX_TILE);
        let radius_px = TILE_PX_SIZE.width * radius_tile;

        let dir_len_sq = velocity_xy.0 * velocity_xy.0 + velocity_xy.1 * velocity_xy.1;
        let rotation_rad = if dir_len_sq > 1e-8 {
            velocity_xy.1.atan2(velocity_xy.0)
        } else {
            0.0
        };

        Self {
            base_xy: xy,
            prev_xy: xy,
            xy,
            created_at: now,
            lifetime,
            alpha: 0.0,
            initial_alpha: BLACK_SMOKE_DASH_TRAIL_ALPHA_MAX,
            radius_px,
            scale_ratio: BLACK_SMOKE_DASH_TRAIL_SCALE_START_RATIO,
            rotation_rad,
            cos_phase_offset_rad: 0.0,
            reverse_progress: false,
            velocity_xy,
            sprite_kind: BlackSmokeSpriteKind::Smoke01,
        }
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        if matches!(self.sprite_kind, BlackSmokeSpriteKind::Smoke01) {
            self.tick_dash_trail_impl(now);
            return;
        }

        let progress = self.progress(now);
        let motion_progress = if self.reverse_progress {
            1.0 - progress
        } else {
            progress
        };

        self.alpha = (self.initial_alpha * (1.0 - progress)).max(0.0);

        if motion_progress <= BLACK_SMOKE_SCALE_GROW_END_PROGRESS {
            let t = (motion_progress / BLACK_SMOKE_SCALE_GROW_END_PROGRESS).clamp(0.0, 1.0);
            let eased = ease_out_cubic(t);
            self.scale_ratio =
                BLACK_SMOKE_SCALE_START_RATIO + (1.0 - BLACK_SMOKE_SCALE_START_RATIO) * eased;
        } else {
            let t = ((motion_progress - BLACK_SMOKE_SCALE_GROW_END_PROGRESS)
                / (1.0 - BLACK_SMOKE_SCALE_GROW_END_PROGRESS))
                .clamp(0.0, 1.0);
            let eased = ease_in_cubic(t);
            self.scale_ratio = 1.0 + (BLACK_SMOKE_SCALE_END_RATIO - 1.0) * eased;
        }

        let theta = self.cos_phase_offset_rad + 2.0 * PI * motion_progress;
        let narrow_mul = (1.0 - motion_progress).powf(BLACK_SMOKE_X_COS_NARROW_POW);
        self.prev_xy = self.xy;
        self.xy.0 = self.base_xy.0 + theta.cos() * BLACK_SMOKE_X_COS_AMPLITUDE_TILE * narrow_mul;

        let ascent_tile = BLACK_SMOKE_ASCENT_MAX_TILE * motion_progress * motion_progress;
        self.xy.1 = self.base_xy.1 + BLACK_SMOKE_FLOOR_Y_OFFSET_TILE - ascent_tile;

        let dx = self.xy.0 - self.prev_xy.0;
        let dy = self.xy.1 - self.prev_xy.1;
        if dx * dx + dy * dy > 1e-8 {
            self.rotation_rad = dy.atan2(dx);
        }
    }

    fn tick_dash_trail_impl(&mut self, now: Instant) {
        let progress = self.progress(now);
        let elapsed_secs = (now - self.created_at).as_secs_f32();

        self.prev_xy = self.xy;
        self.xy.0 = self.base_xy.0 + self.velocity_xy.0 * elapsed_secs;
        self.xy.1 = self.base_xy.1 + self.velocity_xy.1 * elapsed_secs;

        let triangle = if progress <= 0.5 {
            progress / 0.5
        } else {
            (1.0 - progress) / 0.5
        }
        .clamp(0.0, 1.0);
        self.alpha = BLACK_SMOKE_DASH_TRAIL_ALPHA_MAX * triangle;

        self.scale_ratio = BLACK_SMOKE_DASH_TRAIL_SCALE_START_RATIO
            + (BLACK_SMOKE_DASH_TRAIL_SCALE_END_RATIO - BLACK_SMOKE_DASH_TRAIL_SCALE_START_RATIO)
                * ease_out_cubic(progress);
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 {
            return sprites;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let scale = (self.radius_px.as_f32() * 2.0 * self.scale_ratio) / 128.0;
        let color = Color::WHITE.with_alpha((self.alpha * 255.0) as u8);
        let src_rect = match self.sprite_kind {
            BlackSmokeSpriteKind::Smoke00 => atlas::black_smoke_00(),
            BlackSmokeSpriteKind::Smoke01 => atlas::black_smoke_01(),
        };

        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            px_xy.x,
            px_xy.y,
            scale,
            self.rotation_rad,
            Some(color),
        ));

        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.lifetime.as_secs_f32()).clamp(0.0, 1.0)
    }
}

impl namui::particle::Particle for BlackSmokeParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        BlackSmokeParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        BlackSmokeParticle::is_done(self, now)
    }
}
