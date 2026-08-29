use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;
use std::f32::consts::TAU;

const BURNING_TRAIL_LIFETIME_MIN_MS: i64 = 120;
const BURNING_TRAIL_LIFETIME_MAX_MS: i64 = 240;
const OUTER_RADIUS_MIN_TILE: f32 = 0.3;
const OUTER_RADIUS_MAX_TILE: f32 = 0.6;
const OFFSET_RANGE: f32 = 0.06;

const ALPHA_RISE_END_PROGRESS: f32 = 0.2;
const ALPHA_MIN: f32 = 0.2;
const ALPHA_PEAK: f32 = 0.9;
const RADIUS_START_RATIO: f32 = 0.6;
const RADIUS_PEAK_RATIO: f32 = 1.0;
const RADIUS_END_RATIO: f32 = 0.0;

const IMAGE_SIZE: f32 = 128.0;

#[derive(Clone)]
pub struct BurningTrailParticle {
    pub xy: (f32, f32),
    pub angle_rad: f32,
    pub created_at: Instant,
    pub lifetime: Duration,
    pub initial_radius: Px,
    pub alpha: f32,
    pub radius: Px,
}

impl BurningTrailParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let lifetime_ms =
            rng.gen_range(BURNING_TRAIL_LIFETIME_MIN_MS..=BURNING_TRAIL_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let initial_radius =
            TILE_PX_SIZE.width * rng.gen_range(OUTER_RADIUS_MIN_TILE..=OUTER_RADIUS_MAX_TILE);
        let angle_rad = rng.gen_range(0.0..TAU);

        Self {
            xy: final_xy,
            angle_rad,
            created_at,
            lifetime,
            initial_radius,
            alpha: 1.0,
            radius: initial_radius,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);

        if progress <= ALPHA_RISE_END_PROGRESS {
            let t = progress / ALPHA_RISE_END_PROGRESS;
            self.alpha = ALPHA_MIN + (ALPHA_PEAK - ALPHA_MIN) * t;
            let radius_ratio = RADIUS_START_RATIO + (RADIUS_PEAK_RATIO - RADIUS_START_RATIO) * t;
            let r = self.initial_radius.as_f32() * radius_ratio;
            self.radius = px(r);
        } else {
            let t = (progress - ALPHA_RISE_END_PROGRESS) / (1.0 - ALPHA_RISE_END_PROGRESS);
            self.alpha = ALPHA_PEAK * (1.0 - t);
            let radius_ratio = RADIUS_PEAK_RATIO + (RADIUS_END_RATIO - RADIUS_PEAK_RATIO) * t;
            let r = self.initial_radius.as_f32() * radius_ratio;
            self.radius = px(r);
        }
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 {
            return sprites;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let src_rect = atlas::burning_tail();

        let outer_scale = (self.radius.as_f32() * 2.0) / IMAGE_SIZE;
        let outer_color = Color::from_f01(1.0, 0.35, 0.05, self.alpha * 0.7);
        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            xy_px.x,
            xy_px.y,
            outer_scale,
            self.angle_rad,
            Some(outer_color),
        ));

        let inner_scale = (self.radius.as_f32() * 2.0 * 0.5) / IMAGE_SIZE;
        let inner_color = Color::from_f01(1.0, 0.85, 0.2, self.alpha);
        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            xy_px.x,
            xy_px.y,
            inner_scale,
            self.angle_rad,
            Some(inner_color),
        ));

        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.lifetime.as_secs_f32()).min(1.0)
    }
}

impl namui::particle::Particle for BurningTrailParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        BurningTrailParticle::tick(self, now, dt);
    }
    fn render(&self) -> namui::particle::ParticleSprites {
        BurningTrailParticle::render(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        BurningTrailParticle::is_done(self, now)
    }
}
