use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const EMBER_SPARK_LIFETIME_MIN_MS: i64 = 120;
const EMBER_SPARK_LIFETIME_MAX_MS: i64 = 480;
const EMBER_SPARK_RADIUS_MIN_TILE: f32 = 0.05;
const EMBER_SPARK_RADIUS_MAX_TILE: f32 = 0.1;
const EMBER_SPARK_SPEED_MIN: f32 = 2.0; // 맵 좌표 단위/초
const EMBER_SPARK_SPEED_MAX: f32 = 8.0;
const EMBER_SPARK_GRAVITY: f32 = 1.0;
const EMBER_SPARK_FADE_START: f32 = 0.5;
const EMBER_SPARK_INNER_RADIUS_RATIO: f32 = 0.4;
const MIN_DIRECTION_SPEED_SQ: f32 = 0.000001;

#[derive(Clone)]
pub struct EmberSparkParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32), // 맵 좌표 단위/초
    pub angle_rad: f32,
    pub created_at: Instant,
    pub lifetime: Duration,
    pub radius: Px,
    pub alpha: f32,
}

impl EmberSparkParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        movement_direction: (f32, f32), // 진행 방향 벡터 (정규화되어 있어야 함)
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let lifetime_ms = rng.gen_range(EMBER_SPARK_LIFETIME_MIN_MS..=EMBER_SPARK_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let radius = TILE_PX_SIZE.width
            * rng.gen_range(EMBER_SPARK_RADIUS_MIN_TILE..=EMBER_SPARK_RADIUS_MAX_TILE);

        // 진행 방향의 반대로 튀어나감 (약간의 각도 변화 추가)
        let opposite_dir = (-movement_direction.0, -movement_direction.1);
        let angle_variation: f32 = rng.gen_range(-0.3..=0.3); // 라디안
        let cos_a = angle_variation.cos();
        let sin_a = angle_variation.sin();
        let rotated_dir = (
            opposite_dir.0 * cos_a - opposite_dir.1 * sin_a,
            opposite_dir.0 * sin_a + opposite_dir.1 * cos_a,
        );

        let speed = rng.gen_range(EMBER_SPARK_SPEED_MIN..=EMBER_SPARK_SPEED_MAX);
        let velocity = (rotated_dir.0 * speed, rotated_dir.1 * speed);
        let angle_rad = velocity.1.atan2(velocity.0);

        Self {
            xy,
            velocity,
            angle_rad,
            created_at,
            lifetime,
            radius,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        let dt_sec = dt.as_secs_f32();

        self.velocity.1 += EMBER_SPARK_GRAVITY * dt_sec;

        self.xy.0 += self.velocity.0 * dt_sec;
        self.xy.1 += self.velocity.1 * dt_sec;

        let speed_sq = self.velocity.0 * self.velocity.0 + self.velocity.1 * self.velocity.1;
        if speed_sq > MIN_DIRECTION_SPEED_SQ {
            self.angle_rad = self.velocity.1.atan2(self.velocity.0);
        }

        let progress = self.progress(now);
        if progress >= EMBER_SPARK_FADE_START {
            let fade_progress =
                (progress - EMBER_SPARK_FADE_START) / (1.0 - EMBER_SPARK_FADE_START);
            self.alpha = 1.0 - fade_progress;
        } else {
            self.alpha = 1.0;
        }
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 {
            return sprites;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let angle_rad = self.angle_rad;

        let outer_radius = self.radius;
        let outer_scale = (outer_radius.as_f32() * 2.0) / 128.0;
        let outer_color = Color::from_f01(1.0, 0.5, 0.1, self.alpha * 0.8);
        sprites.push(atlas::centered_rotated_sprite(
            atlas::ember_spark(),
            xy_px.x,
            xy_px.y,
            outer_scale,
            angle_rad,
            Some(outer_color),
        ));

        let inner_radius = px(self.radius.as_f32() * EMBER_SPARK_INNER_RADIUS_RATIO);
        let inner_scale = (inner_radius.as_f32() * 2.0) / 128.0;
        let inner_color = Color::from_f01(1.0, 0.9, 0.4, self.alpha);
        sprites.push(atlas::centered_rotated_sprite(
            atlas::ember_spark(),
            xy_px.x,
            xy_px.y,
            inner_scale,
            angle_rad,
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

impl namui::particle::Particle for EmberSparkParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        EmberSparkParticle::tick(self, now, dt);
    }
    fn render(&self) -> namui::particle::ParticleSprites {
        EmberSparkParticle::render(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        EmberSparkParticle::is_done(self, now)
    }
}
