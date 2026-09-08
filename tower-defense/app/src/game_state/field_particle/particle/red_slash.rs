use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;

const RED_SLASH_SIZE_TILE: f32 = 1.0;
const RED_SLASH_FADE_IN_RATIO: f32 = 0.15;
const RED_SLASH_ALPHA_MAX: f32 = 0.75;

#[derive(Clone)]
pub struct RedSlashParticle {
    pub xy: (f32, f32),
    pub angle_rad: f32,
    pub created_at: Instant,
    pub lifetime: Duration,
    pub alpha: f32,
}

impl RedSlashParticle {
    pub fn new(xy: (f32, f32), angle_rad: f32, created_at: Instant, lifetime: Duration) -> Self {
        Self {
            xy,
            angle_rad,
            created_at,
            lifetime,
            alpha: 0.0,
        }
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = (now - self.created_at).as_secs_f32();
        (elapsed / self.lifetime.as_secs_f32()).clamp(0.0, 1.0)
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);
        self.alpha = if progress < RED_SLASH_FADE_IN_RATIO {
            RED_SLASH_ALPHA_MAX * (progress / RED_SLASH_FADE_IN_RATIO)
        } else {
            RED_SLASH_ALPHA_MAX * (1.0 - progress) / (1.0 - RED_SLASH_FADE_IN_RATIO)
        };
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 {
            return sprites;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let size_px = TILE_PX_SIZE.width.as_f32() * RED_SLASH_SIZE_TILE;
        let scale = size_px / 128.0;
        let color = Color::WHITE.with_alpha((self.alpha * 255.0) as u8);

        sprites.push(atlas::centered_rotated_sprite(
            atlas::red_slash(),
            px_xy.x,
            px_xy.y,
            scale,
            self.angle_rad,
            Some(color),
        ));
        sprites
    }
}

impl namui::particle::Particle for RedSlashParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        RedSlashParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        RedSlashParticle::is_done(self, now)
    }
}
