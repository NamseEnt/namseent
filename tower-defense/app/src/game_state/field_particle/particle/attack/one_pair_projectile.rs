use crate::game_state::TILE_PX_SIZE;
use namui::*;

const PROJECTILE_SCALE: f32 = 2.0;

#[derive(Clone)]
pub struct OnePairProjectile {
    start_x_px: f32,
    start_y_px: f32,
    rotation_cos: f32,
    rotation_sin: f32,
    length_px: f32,
    created_at: Instant,
    duration: Duration,
    progress: f32,
}

impl OnePairProjectile {
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
            duration: Duration::from_secs_f32(0.15),
            progress: 0.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress =
            ((now - self.created_at).as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let src_rect = crate::atlas::generated::ATTACK_ONE_PAIR_PROJECTILE;
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let center_x = px(self.start_x_px + self.rotation_cos * self.length_px * self.progress);
        let center_y = px(self.start_y_px + self.rotation_sin * self.length_px * self.progress);
        let scos = self.rotation_cos * PROJECTILE_SCALE;
        let ssin = self.rotation_sin * PROJECTILE_SCALE;

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
        now - self.created_at >= self.duration
    }
}

impl particle::Particle for OnePairProjectile {
    fn tick(&mut self, now: Instant, dt: Duration) {
        OnePairProjectile::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        OnePairProjectile::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        OnePairProjectile::is_done(self, now)
    }
}
