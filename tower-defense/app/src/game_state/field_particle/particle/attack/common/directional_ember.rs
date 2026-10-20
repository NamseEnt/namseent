use super::super::{ATTACK_FRAME_EASE_IN, ATTACK_FRAME_EASE_OUT};
use namui::*;
use rand::Rng;

const MIN_LIFETIME_MS: i64 = 250;
const MAX_LIFETIME_MS: i64 = 1_000;

#[derive(Clone)]
pub struct DirectionalEmber {
    start_xy: (f32, f32),
    direction: (f32, f32),
    distance_tiles: f32,
    scale: f32,
    color: [f32; 3],
    variant: usize,
    lifetime: Duration,
    created_at: Instant,
    progress: f32,
}

impl DirectionalEmber {
    pub fn new(
        start_xy: (f32, f32),
        direction_angle: f32,
        distance_tiles: f32,
        scale: f32,
        color: [f32; 3],
        created_at: Instant,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let lifetime = Duration::from_millis(rng.gen_range(MIN_LIFETIME_MS..=MAX_LIFETIME_MS));
        Self {
            start_xy,
            direction: (direction_angle.cos(), direction_angle.sin()),
            distance_tiles,
            scale,
            color,
            variant: rng.gen_range(0..3),
            lifetime,
            created_at,
            progress: 0.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress =
            ((now - self.created_at).as_secs_f32() / self.lifetime.as_secs_f32()).clamp(0.0, 1.0);
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let travel_progress = ATTACK_FRAME_EASE_OUT.sample(self.progress);
        let opacity = 1.0 - ATTACK_FRAME_EASE_IN.sample(self.progress);
        let xy = (
            self.start_xy.0 + self.direction.0 * self.distance_tiles * travel_progress,
            self.start_xy.1 + self.direction.1 * self.distance_tiles * travel_progress,
        );
        let frame = frame_for_variant(self.variant);
        let center_x = px(xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32());
        let center_y = px(xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32());
        let scale =
            crate::game_state::TILE_PX_SIZE.width.as_f32() * self.scale / frame.width().as_f32();
        let color = Color::from_f01(
            self.color[0].clamp(0.0, 1.0),
            self.color[1].clamp(0.0, 1.0),
            self.color[2].clamp(0.0, 1.0),
            opacity,
        );
        sprites.push(
            crate::game_state::field_particle::atlas::centered_rotated_sprite(
                frame,
                center_x,
                center_y,
                scale,
                self.direction.1.atan2(self.direction.0),
                Some(color),
            ),
        );
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }
}

fn frame_for_variant(variant: usize) -> Rect<Px> {
    match variant {
        0 => crate::atlas::generated::ATTACK_COMMON_EMBER_00,
        1 => crate::atlas::generated::ATTACK_COMMON_EMBER_01,
        _ => crate::atlas::generated::ATTACK_COMMON_EMBER_02,
    }
}

impl particle::Particle for DirectionalEmber {
    fn tick(&mut self, now: Instant, dt: Duration) {
        DirectionalEmber::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        DirectionalEmber::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        DirectionalEmber::is_done(self, now)
    }
}
