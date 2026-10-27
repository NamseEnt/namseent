use super::super::ATTACK_FRAME_EASE_OUT;
use super::{MAX_LIFETIME, WHITE, frame_index, progress_after_delay};
use namui::*;
use rand::Rng;

const DELAY: Duration = Duration::from_millis(100);
const DURATION: Duration = Duration::from_millis(150);
const SCALE_MULTIPLIER: f32 = 12.0;

#[derive(Clone)]
pub struct MuzzleFlash {
    center_x_px: f32,
    center_y_px: f32,
    created_at: Instant,
    progress: Option<f32>,
    eased_progress: f32,
    rotation: (f32, f32),
}

impl MuzzleFlash {
    pub fn new(start_xy: (f32, f32), created_at: Instant) -> Self {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        let angle = rand::thread_rng().gen_range(0.0..std::f32::consts::TAU);
        let (rotation_sin, rotation_cos) = angle.sin_cos();
        Self {
            center_x_px: start_xy.0 * tile_size.width.as_f32(),
            center_y_px: start_xy.1 * tile_size.height.as_f32(),
            created_at,
            progress: None,
            eased_progress: 0.0,
            rotation: (rotation_cos, rotation_sin),
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress = progress_after_delay(now - self.created_at, DELAY, DURATION);
        if let Some(progress) = self.progress {
            self.eased_progress = ATTACK_FRAME_EASE_OUT.sample(progress);
        }
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if self.progress.is_none() {
            return sprites;
        }
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_MUZZLE_FLASH;
        let frame = frames[frame_index(self.eased_progress, frames.len())];
        let tile_size = crate::game_state::TILE_PX_SIZE;
        let scale = tile_size.width.as_f32() * SCALE_MULTIPLIER / frame.width().as_f32();
        sprites.push(crate::atlas::centered_rotated_sprite_with_sin_cos(
            frame,
            px(self.center_x_px),
            px(self.center_y_px),
            scale,
            self.rotation.0,
            self.rotation.1,
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
