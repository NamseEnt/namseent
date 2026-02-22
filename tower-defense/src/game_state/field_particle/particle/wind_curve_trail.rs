use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const WIND_CURVE_LIFETIME_MIN_MS: i64 = 90;
const WIND_CURVE_LIFETIME_MAX_MS: i64 = 170;
const WIND_CURVE_LENGTH_MIN_TILE: f32 = 0.4;
const WIND_CURVE_LENGTH_MAX_TILE: f32 = 0.8;
const WIND_CURVE_AMPLITUDE_MIN_TILE: f32 = 0.1;
const WIND_CURVE_AMPLITUDE_MAX_TILE: f32 = 0.25;
const WIND_CURVE_THICKNESS_MIN_TILE: f32 = 0.02;
const WIND_CURVE_THICKNESS_MAX_TILE: f32 = 0.05;
const OFFSET_RANGE_TILE: f32 = 0.1;

const OUTER_COLOR_RGB: (f32, f32, f32) = (0.60, 0.72, 0.78);
const INNER_COLOR_RGB: (f32, f32, f32) = (0.85, 0.92, 0.95);
const OUTER_ALPHA_MULT: f32 = 0.55;
const FADE_START_PROGRESS: f32 = 0.15;

const BEZIER_SEGMENTS: usize = 4;

#[derive(Clone)]
pub struct WindCurveTrailParticle {
    pub center_xy: (f32, f32),
    pub movement_direction: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub length_tile: f32,
    pub amplitude_tile: f32,
    pub thickness_tile: f32,
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
            amplitude_tile: rng
                .gen_range(WIND_CURVE_AMPLITUDE_MIN_TILE..=WIND_CURVE_AMPLITUDE_MAX_TILE),
            thickness_tile: rng
                .gen_range(WIND_CURVE_THICKNESS_MIN_TILE..=WIND_CURVE_THICKNESS_MAX_TILE),
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

        let movement = Xy::new(self.movement_direction.0, self.movement_direction.1);
        let perpendicular = Xy::new(-movement.y, movement.x);
        let center_tile = Xy::new(self.center_xy.0, self.center_xy.1);

        let half_len_tile = self.length_tile * 0.5;
        let amp_tile = self.amplitude_tile;

        let start_tile = center_tile - movement * half_len_tile;
        let end_tile = center_tile + movement * half_len_tile;

        let ctrl1_tile = start_tile + movement * (half_len_tile * 0.45) + perpendicular * amp_tile;
        let ctrl2_tile = end_tile - movement * (half_len_tile * 0.45) - perpendicular * amp_tile;

        let mut bezier_points: [Xy<Px>; BEZIER_SEGMENTS + 1] =
            [Xy::new(px(0.0), px(0.0)); BEZIER_SEGMENTS + 1];

        for i in 0..=BEZIER_SEGMENTS {
            let t = i as f32 / BEZIER_SEGMENTS as f32;
            let inv_t = 1.0 - t;
            let point_tile = start_tile * (inv_t * inv_t * inv_t)
                + ctrl1_tile * (3.0 * inv_t * inv_t * t)
                + ctrl2_tile * (3.0 * inv_t * t * t)
                + end_tile * (t * t * t);
            bezier_points[i] = TILE_PX_SIZE.to_xy() * point_tile;
        }

        let outer_color = Color::from_f01(
            OUTER_COLOR_RGB.0,
            OUTER_COLOR_RGB.1,
            OUTER_COLOR_RGB.2,
            self.alpha * OUTER_ALPHA_MULT,
        );
        let inner_color = Color::from_f01(
            INNER_COLOR_RGB.0,
            INNER_COLOR_RGB.1,
            INNER_COLOR_RGB.2,
            self.alpha,
        );

        let outer_thickness = TILE_PX_SIZE.width.as_f32() * self.thickness_tile;
        let inner_thickness = outer_thickness * 0.45;

        for i in 0..BEZIER_SEGMENTS {
            let p0 = bezier_points[i];
            let p1 = bezier_points[i + 1];

            if let Some(s) =
                atlas::line_sprite(p0.x, p0.y, p1.x, p1.y, outer_thickness, Some(outer_color))
            {
                sprites.push(s);
            }

            if sprites.remaining_capacity() < 1 {
                break;
            }

            if let Some(s) =
                atlas::line_sprite(p0.x, p0.y, p1.x, p1.y, inner_thickness, Some(inner_color))
            {
                sprites.push(s);
            }

            if sprites.remaining_capacity() < 2 {
                break;
            }
        }

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
