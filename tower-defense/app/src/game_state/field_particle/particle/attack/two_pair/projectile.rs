use crate::game_state::TILE_PX_SIZE;
use namui::*;

#[derive(Clone)]
pub struct Projectile {
    start_x_px: f32,
    start_y_px: f32,
    rotation_cos: f32,
    rotation_sin: f32,
    length_px: f32,
    created_at: Instant,
    start_delay: Duration,
    duration: Duration,
    progress: f32,
    started: bool,
}

impl Projectile {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Self {
        let tile_width = TILE_PX_SIZE.width.as_f32();
        let tile_height = TILE_PX_SIZE.height.as_f32();
        let delta_x = end_xy.0 - start_xy.0;
        let delta_y = end_xy.1 - start_xy.1;
        let delta_x_px = delta_x * tile_width;
        let delta_y_px = delta_y * tile_height;
        let length_px = delta_x_px.hypot(delta_y_px);
        let angle = delta_y_px.atan2(delta_x_px);

        Self {
            start_x_px: start_xy.0 * tile_width,
            start_y_px: start_xy.1 * tile_height,
            rotation_cos: angle.cos(),
            rotation_sin: angle.sin(),
            length_px,
            created_at,
            start_delay: Duration::ZERO,
            duration: Duration::from_secs_f32(0.1),
            progress: 0.0,
            started: true,
        }
    }

    pub fn with_start_delay(mut self, start_delay: Duration) -> Self {
        self.start_delay = start_delay;
        self.started = start_delay == Duration::ZERO;
        self
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.start_delay;
        self.progress = if self.started {
            ((elapsed.as_secs_f32() - self.start_delay.as_secs_f32()) / self.duration.as_secs_f32())
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let src_rect = crate::atlas::generated::ATTACK_TWO_PAIR_PROJECTILE;
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let center_x = px(self.start_x_px + self.rotation_cos * self.length_px * self.progress);
        let center_y = px(self.start_y_px + self.rotation_sin * self.length_px * self.progress);
        let scos = self.rotation_cos;
        let ssin = self.rotation_sin;

        sprites.push(namui::ImageSprite {
            src_rect,
            xform: namui::RSXform {
                scos,
                ssin,
                tx: center_x - px(scos * sprite_width / 2.0 - ssin * sprite_height / 2.0),
                ty: center_y - px(ssin * sprite_width / 2.0 + scos * sprite_height / 2.0),
            },
            color: None,
        });
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.start_delay + self.duration
    }
}

impl particle::Particle for Projectile {
    fn tick(&mut self, now: Instant, dt: Duration) {
        Projectile::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        Projectile::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        Projectile::is_done(self, now)
    }
}
