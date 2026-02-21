use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const LIGHTNING_BOLT_MIN_POINTS: usize = 4;
const LIGHTNING_BOLT_MAX_POINTS: usize = 6;
const LIGHTNING_BOLT_OFFSET_RANGE: f32 = 0.2;

const LIGHTNING_BOLT_SPAWN_LIFETIME_MIN_MS: i64 = 50;
const LIGHTNING_BOLT_SPAWN_LIFETIME_MAX_MS: i64 = 150;
const LIGHTNING_BOLT_SPAWN_CHANCE_REDUCTION: f32 = 0.8;
const LIGHTNING_BOLT_END_OFFSET_RANGE: f32 = 0.2;

const LIGHTNING_BOLT_ALPHA_APPEAR_PHASE: f32 = 0.2;

#[derive(Clone)]
pub struct LightningBoltParticle {
    pub points: Vec<(f32, f32)>,
    pub created_at: Instant,
    pub duration: Duration,
    pub alpha: f32,
    pub spawn_chance: f32,
    pub has_spawned: bool,
}

impl LightningBoltParticle {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        spawn_chance: f32,
    ) -> Self {
        let points = Self::generate_points(start_xy, end_xy);

        Self {
            points,
            created_at,
            duration,
            alpha: 1.0,
            spawn_chance,
            has_spawned: false,
        }
    }

    fn generate_points(start_xy: (f32, f32), end_xy: (f32, f32)) -> Vec<(f32, f32)> {
        let mut rng = rand::thread_rng();
        let point_count = rng.gen_range(LIGHTNING_BOLT_MIN_POINTS..=LIGHTNING_BOLT_MAX_POINTS);

        let dx = end_xy.0 - start_xy.0;
        let dy = end_xy.1 - start_xy.1;
        let length = (dx * dx + dy * dy).sqrt();

        let perp_x = -dy / length.max(0.001);
        let perp_y = dx / length.max(0.001);

        let mut points = Vec::with_capacity(point_count);

        for i in 0..point_count {
            let t = i as f32 / (point_count - 1) as f32;

            let base_x = start_xy.0 + dx * t;
            let base_y = start_xy.1 + dy * t;

            let offset = if i == 0 || i == point_count - 1 {
                0.0
            } else {
                rng.gen_range(-LIGHTNING_BOLT_OFFSET_RANGE..LIGHTNING_BOLT_OFFSET_RANGE)
            };

            let x = base_x + perp_x * offset;
            let y = base_y + perp_y * offset;

            points.push((x, y));
        }

        points
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        self.alpha = self.current_alpha(now);

        if !self.has_spawned && self.is_done(now) && self.points.len() >= 2 {
            self.has_spawned = true;
            if let Some(child) = self.try_spawn_child(now) {
                crate::game_state::field_particle::LIGHTNING_BOLTS.spawn(child);
            }
        }
    }

    fn try_spawn_child(&self, now: Instant) -> Option<LightningBoltParticle> {
        let mut rng = rand::thread_rng();

        if rng.gen_range(0.0..1.0) >= self.spawn_chance {
            return None;
        }

        let mid_idx = self.points.len() / 2;
        let start_xy = self.points[mid_idx];

        let last_xy = self.points[self.points.len() - 1];
        let offset_x =
            rng.gen_range(-LIGHTNING_BOLT_END_OFFSET_RANGE..LIGHTNING_BOLT_END_OFFSET_RANGE);
        let offset_y =
            rng.gen_range(-LIGHTNING_BOLT_END_OFFSET_RANGE..LIGHTNING_BOLT_END_OFFSET_RANGE);
        let end_xy = (last_xy.0 + offset_x, last_xy.1 + offset_y);

        let duration = Duration::from_millis(
            rng.gen_range(LIGHTNING_BOLT_SPAWN_LIFETIME_MIN_MS..LIGHTNING_BOLT_SPAWN_LIFETIME_MAX_MS),
        );

        let new_spawn_chance = self.spawn_chance * LIGHTNING_BOLT_SPAWN_CHANCE_REDUCTION;

        Some(LightningBoltParticle::new(
            start_xy,
            end_xy,
            now,
            duration,
            new_spawn_chance,
        ))
    }

    pub fn render(&self) -> Option<ImageSprite> {
        if self.alpha <= 0.0 || self.points.len() < 2 {
            return None;
        }

        let first = self.points[0];
        let last = self.points[self.points.len() - 1];
        let start_px = TILE_PX_SIZE.to_xy() * Xy::new(first.0, first.1);
        let end_px = TILE_PX_SIZE.to_xy() * Xy::new(last.0, last.1);
        let color = Color::from_f01(0.2, 0.5, 1.0, self.alpha);
        let thickness = TILE_PX_SIZE.width.as_f32() * 0.03;
        atlas::line_sprite(start_px.x, start_px.y, end_px.x, end_px.y, thickness, Some(color))
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    fn current_alpha(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        if elapsed >= self.duration {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        if progress < LIGHTNING_BOLT_ALPHA_APPEAR_PHASE {
            let appear_progress = progress / LIGHTNING_BOLT_ALPHA_APPEAR_PHASE;
            let inv = 1.0 - appear_progress;
            1.0 - (inv * inv)
        } else {
            let fade_progress = (progress - LIGHTNING_BOLT_ALPHA_APPEAR_PHASE)
                / (1.0 - LIGHTNING_BOLT_ALPHA_APPEAR_PHASE);
            1.0 - fade_progress
        }
    }
}

impl namui::particle::Particle for LightningBoltParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> Option<ImageSprite> {
        LightningBoltParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        LightningBoltParticle::is_done(self, now)
    }
}
