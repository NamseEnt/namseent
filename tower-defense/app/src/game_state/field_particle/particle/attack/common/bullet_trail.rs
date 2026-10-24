use super::super::ATTACK_FRAME_EASE_OUT;
use namui::*;
use rand::Rng;

const DURATION: Duration = Duration::from_millis(250);

#[derive(Clone)]
pub struct BulletTrail {
    center_x_px: f32,
    center_y_px: f32,
    rotation_cos: f32,
    rotation_sin: f32,
    created_at: Instant,
    delay: Duration,
    variant: usize,
    scale: f32,
    opacity: f32,
    progress: f32,
    started: bool,
}

impl BulletTrail {
    pub fn new(
        center_xy: (f32, f32),
        rotation_cos: f32,
        rotation_sin: f32,
        created_at: Instant,
        delay: Duration,
        scale: f32,
        opacity: f32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let variant = rng.gen_range(0..3);
        Self {
            center_x_px: center_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32(),
            center_y_px: center_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32(),
            rotation_cos,
            rotation_sin,
            created_at,
            delay,
            variant,
            scale,
            opacity: opacity.clamp(0.0, 1.0),
            progress: 0.0,
            started: false,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed.as_secs_f32() - self.delay.as_secs_f32()) / DURATION.as_secs_f32())
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

        let frames = frames_for_variant(self.variant);
        let frame_index = ((ATTACK_FRAME_EASE_OUT.sample(self.progress) * frames.len() as f32)
            as usize)
            .min(frames.len() - 1);
        let frame_rect = frames[frame_index];
        let x = frame_rect.x();
        let width = frame_rect.width();
        let height = frame_rect.height() * self.scale;
        let y = frame_rect.y() + (frame_rect.height() - height) / 2.0;
        let src_rect = Rect::Xywh {
            x,
            y,
            width,
            height,
        };
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let sprite_scale = crate::game_state::TILE_PX_SIZE.width.as_f32() / sprite_width;
        let scos = self.rotation_cos * sprite_scale;
        let ssin = self.rotation_sin * sprite_scale;
        let center_x = px(self.center_x_px);
        let center_y = px(self.center_y_px);

        sprites.push(namui::ImageSprite {
            src_rect,
            xform: namui::RSXform {
                scos,
                ssin,
                tx: center_x - px(scos * sprite_width / 2.0 - ssin * sprite_height / 2.0),
                ty: center_y - px(ssin * sprite_width / 2.0 + scos * sprite_height / 2.0),
            },
            scale_xy: None,
            color: Some(
                Color::WHITE.with_alpha(
                    ((1.0 - self.progress) * self.opacity * 0.75 * 255.0).round() as u8,
                ),
            ),
        });
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + DURATION
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_BULLET_TRAIL_00,
        1 => &crate::atlas::generated::ATTACK_COMMON_BULLET_TRAIL_01,
        _ => &crate::atlas::generated::ATTACK_COMMON_BULLET_TRAIL_02,
    }
}

impl particle::Particle for BulletTrail {
    fn tick(&mut self, now: Instant, dt: Duration) {
        BulletTrail::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        BulletTrail::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        BulletTrail::is_done(self, now)
    }
}
