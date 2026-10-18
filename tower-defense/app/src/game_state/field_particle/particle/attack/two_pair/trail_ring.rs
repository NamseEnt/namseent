use super::ATTACK_FRAME_EASE_OUT;
use namui::*;

const BASE_DURATION: Duration = Duration::from_millis(300);

#[derive(Clone)]
pub struct TrailRing {
    center_x_px: f32,
    center_y_px: f32,
    rotation_cos: f32,
    rotation_sin: f32,
    created_at: Instant,
    delay: Duration,
    duration: Duration,
    scale_variation: f32,
    progress: f32,
    started: bool,
}

impl TrailRing {
    pub fn new(
        center_xy: (f32, f32),
        rotation_cos: f32,
        rotation_sin: f32,
        created_at: Instant,
        delay: Duration,
        scale_variation: f32,
        duration_variation: f32,
    ) -> Self {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        Self {
            center_x_px: center_xy.0 * tile_size.width.as_f32(),
            center_y_px: center_xy.1 * tile_size.height.as_f32(),
            rotation_cos,
            rotation_sin,
            created_at,
            delay,
            duration: Duration::from_secs_f32(BASE_DURATION.as_secs_f32() * duration_variation),
            scale_variation,
            progress: 0.0,
            started: false,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed.as_secs_f32() - self.delay.as_secs_f32()) / self.duration.as_secs_f32())
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

        let frames = crate::atlas::generated::ATTACK_TWO_PAIR_TRAIL_RING;
        let src_rect = frames[frame_index_for_progress(self.progress, frames.len())];
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let scale = self.scale_variation * (1.0 + self.progress * 0.25);
        let scos = self.rotation_cos * scale;
        let ssin = self.rotation_sin * scale;
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
            color: Some(Color::WHITE.with_alpha(((1.0 - self.progress) * 255.0).round() as u8)),
        });
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + self.duration
    }
}

fn frame_index_for_progress(progress: f32, frame_count: usize) -> usize {
    ((ATTACK_FRAME_EASE_OUT.sample(progress) * frame_count as f32) as usize).min(frame_count - 1)
}

impl particle::Particle for TrailRing {
    fn tick(&mut self, now: Instant, dt: Duration) {
        TrailRing::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        TrailRing::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        TrailRing::is_done(self, now)
    }
}
