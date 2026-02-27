use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const WIND_CURVE_LIFETIME_MIN_MS: i64 = 150;
const WIND_CURVE_LIFETIME_MAX_MS: i64 = 300;
const WIND_CURVE_LENGTH_MIN_TILE: f32 = 0.6;
const WIND_CURVE_LENGTH_MAX_TILE: f32 = 1.2;
const OFFSET_RANGE_TILE: f32 = 0.15;

const ALPHA: f32 = 0.8;
const FADE_START_PROGRESS: f32 = 0.15;

#[derive(Clone)]
pub struct WindCurveTrailParticle {
    pub center_xy: (f32, f32),
    pub movement_direction: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub length_tile: f32,
    pub alpha: f32,
}

impl WindCurveTrailParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        movement_direction: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE_TILE..=OFFSET_RANGE_TILE);
        let offset_y = rng.gen_range(-OFFSET_RANGE_TILE..=OFFSET_RANGE_TILE);
        let center_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let lifetime_ms = rng.gen_range(WIND_CURVE_LIFETIME_MIN_MS..=WIND_CURVE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        Self {
            center_xy,
            movement_direction: normalize_or_default(movement_direction),
            created_at,
            lifetime,
            length_tile: rng.gen_range(WIND_CURVE_LENGTH_MIN_TILE..=WIND_CURVE_LENGTH_MAX_TILE),
            alpha: 1.0,
        }
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);
        if progress <= FADE_START_PROGRESS {
            self.alpha = 1.0;
        } else {
            let t = (progress - FADE_START_PROGRESS) / (1.0 - FADE_START_PROGRESS);
            self.alpha = 1.0 - t;
        }
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 {
            return sprites;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.center_xy.0, self.center_xy.1);
        let src_rect = atlas::wind_curve_trail();
        let angle_rad = self.movement_direction.1.atan2(self.movement_direction.0);

        let color = Color::WHITE.with_alpha((self.alpha * ALPHA * 255.0) as u8);
        let scale = (TILE_PX_SIZE.width.as_f32() * self.length_tile) / src_rect.width().as_f32();
        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            xy_px.x,
            xy_px.y,
            scale,
            angle_rad,
            Some(color),
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

fn normalize_or_default(direction: (f32, f32)) -> (f32, f32) {
    let len = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
    if len > 0.0001 {
        (direction.0 / len, direction.1 / len)
    } else {
        (0.0, -1.0)
    }
}

impl namui::particle::Particle for WindCurveTrailParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        WindCurveTrailParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        WindCurveTrailParticle::is_done(self, now)
    }
}
