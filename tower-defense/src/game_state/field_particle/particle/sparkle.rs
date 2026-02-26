use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const SPARKLE_LIFETIME_MIN_MS: i64 = 150;
const SPARKLE_LIFETIME_MAX_MS: i64 = 300;
const SPARKLE_SIZE_TILE: f32 = 0.25;
const OFFSET_RANGE: f32 = 0.25;
const VELOCITY_RANGE: f32 = 0.8;
const SIZE_SCALE_MIN: f32 = 0.5;
const SIZE_SCALE_MAX: f32 = 1.0;

const SPARKLE_ALPHA: f32 = 0.8;

const BLINK_CYCLES: f32 = 3.0;
const BLINK_RISE_RATIO: f32 = 0.4;

const RESPAWN_CHANCE: f32 = 0.25;
const MAX_RESPAWN_COUNT: u8 = 1;
const SPARKLE_ANGLE_MIN_RAD: f32 = 0.0;
const SPARKLE_ANGLE_MAX_RAD: f32 = std::f32::consts::TAU;

#[derive(Clone)]
pub struct SparkleParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub alpha: f32,
    pub respawn_count: u8,
    pub size_scale: f32,
    pub angle_rad: f32,
}

impl SparkleParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        Self::new_with_random_velocity(xy, created_at, rng, VELOCITY_RANGE)
    }

    pub fn new_with_random_velocity<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
        velocity_range: f32,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let velocity_x = rng.gen_range(-velocity_range..=velocity_range);
        let velocity_y = rng.gen_range(-velocity_range..=velocity_range);

        let lifetime_ms = rng.gen_range(SPARKLE_LIFETIME_MIN_MS..=SPARKLE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let size_scale = rng.gen_range(SIZE_SCALE_MIN..=SIZE_SCALE_MAX);
        let angle_rad = rng.gen_range(SPARKLE_ANGLE_MIN_RAD..SPARKLE_ANGLE_MAX_RAD);

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at,
            lifetime,
            alpha: SPARKLE_ALPHA,
            respawn_count: 0,
            size_scale,
            angle_rad,
        }
    }

    fn respawn_from<R: Rng + ?Sized>(&self, now: Instant, rng: &mut R) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (self.xy.0 + offset_x, self.xy.1 + offset_y);

        let velocity_x = rng.gen_range(-VELOCITY_RANGE..=VELOCITY_RANGE);
        let velocity_y = rng.gen_range(-VELOCITY_RANGE..=VELOCITY_RANGE);

        let lifetime_ms = rng.gen_range(SPARKLE_LIFETIME_MIN_MS..=SPARKLE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let size_scale = rng.gen_range(SIZE_SCALE_MIN..=SIZE_SCALE_MAX);
        let angle_rad = rng.gen_range(SPARKLE_ANGLE_MIN_RAD..SPARKLE_ANGLE_MAX_RAD);

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at: now,
            lifetime,
            alpha: SPARKLE_ALPHA,
            respawn_count: self.respawn_count + 1,
            size_scale,
            angle_rad,
        }
    }

    pub fn tick_impl(&mut self, now: Instant, dt: Duration) {
        let progress = self.progress(now);

        let dt_sec = dt.as_secs_f32();
        self.xy.0 += self.velocity.0 * dt_sec;
        self.xy.1 += self.velocity.1 * dt_sec;

        let cycle_progress = (progress * BLINK_CYCLES) % 1.0;

        if cycle_progress <= BLINK_RISE_RATIO {
            let t = cycle_progress / BLINK_RISE_RATIO;
            self.alpha = SPARKLE_ALPHA * t;
        } else {
            let t = (cycle_progress - BLINK_RISE_RATIO) / (1.0 - BLINK_RISE_RATIO);
            self.alpha = SPARKLE_ALPHA * (1.0 - t);
        }

        if self.is_done(now) && self.respawn_count < MAX_RESPAWN_COUNT {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) < RESPAWN_CHANCE {
                let new = self.respawn_from(now, &mut rng);
                crate::game_state::field_particle::spawn_sparkle(new);
            }
        }
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.01 {
            return sprites;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);

        let scale = (TILE_PX_SIZE.width.as_f32() * SPARKLE_SIZE_TILE * self.size_scale) / 128.0;
        let color = Color::WHITE.with_alpha((self.alpha * 255.0) as u8);

        sprites.push(atlas::centered_rotated_sprite(
            atlas::sparkle(),
            xy_px.x,
            xy_px.y,
            scale,
            self.angle_rad,
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

impl namui::particle::Particle for SparkleParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        SparkleParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        SparkleParticle::is_done(self, now)
    }
}
