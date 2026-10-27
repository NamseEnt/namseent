use super::super::ATTACK_FRAME_EASE_OUT;
use super::{MAX_LIFETIME, WHITE, frame_index, progress_after_delay};
use namui::*;
use rand::Rng;

const DELAY: Duration = Duration::from_millis(100);
const DURATION: Duration = Duration::from_millis(150);
const SCALE_MULTIPLIER: f32 = 12.0;

#[derive(Clone)]
pub struct MuzzleFlash {
    start_xy: (f32, f32),
    created_at: Instant,
    elapsed: Duration,
    angle: f32,
}

impl MuzzleFlash {
    pub fn new(start_xy: (f32, f32), created_at: Instant) -> Self {
        Self {
            start_xy,
            created_at,
            elapsed: Duration::ZERO,
            angle: rand::thread_rng().gen_range(0.0..std::f32::consts::TAU),
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.elapsed = now - self.created_at;
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        let Some(progress) = progress_after_delay(self.elapsed, DELAY, DURATION) else {
            return sprites;
        };
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_MUZZLE_FLASH;
        let frame = frames[frame_index(ATTACK_FRAME_EASE_OUT.sample(progress), frames.len())];
        let tile_size = crate::game_state::TILE_PX_SIZE;
        let scale = tile_size.width.as_f32() * SCALE_MULTIPLIER / frame.width().as_f32();
        sprites.push(crate::atlas::centered_rotated_sprite(
            frame,
            px(self.start_xy.0 * tile_size.width.as_f32()),
            px(self.start_xy.1 * tile_size.height.as_f32()),
            scale,
            self.angle,
            Some(Color::from_f01(WHITE[0], WHITE[1], WHITE[2], 1.0)),
        ));
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= MAX_LIFETIME
    }
}

impl particle::Particle for MuzzleFlash {
    fn tick(&mut self, now: Instant, dt: Duration) {
        MuzzleFlash::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        MuzzleFlash::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        MuzzleFlash::is_done(self, now)
    }
}
