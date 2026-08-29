use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;

const YELLOW_EXPLOSION_LIFETIME_MS: i64 = 150;

#[derive(Clone)]
pub struct YellowExplosionParticle {
    pub xy: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub alpha: f32,
    pub scale_ratio: f32,
    size_max_tile: f32,
}

impl YellowExplosionParticle {
    pub fn new(xy: (f32, f32), now: Instant, size_max_tile: f32) -> Self {
        Self {
            xy,
            created_at: now,
            lifetime: Duration::from_millis(YELLOW_EXPLOSION_LIFETIME_MS),
            alpha: 1.0,
            scale_ratio: 0.0,
            size_max_tile,
        }
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = (now - self.created_at).as_secs_f32();
        (elapsed / self.lifetime.as_secs_f32()).clamp(0.0, 1.0)
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);
        let eased = 1.0 - (1.0 - progress).powi(3);
        self.scale_ratio = eased * self.size_max_tile;
        self.alpha = (1.0 - progress).max(0.0);
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
        let scale = (TILE_PX_SIZE.width.as_f32() * self.scale_ratio) / 128.0;
        let color = Color::WHITE.with_alpha((self.alpha * 255.0) as u8);

        sprites.push(atlas::centered_sprite(
            atlas::yellow_explosion(),
            px_xy.x,
            px_xy.y,
            scale,
            Some(color),
        ));
        sprites
    }
}

impl namui::particle::Particle for YellowExplosionParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        YellowExplosionParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        YellowExplosionParticle::is_done(self, now)
    }
}
