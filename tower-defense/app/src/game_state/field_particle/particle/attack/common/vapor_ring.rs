use super::super::ATTACK_FRAME_EASE_OUT;
use namui::*;
use rand::Rng;

const DURATION: Duration = Duration::from_millis(250);

#[derive(Clone)]
pub struct VaporRing {
    center_x_px: f32,
    center_y_px: f32,
    created_at: Instant,
    delay: Duration,
    scale: f32,
    variant: usize,
    rotation: (f32, f32),
    opacity: f32,
    movement_direction: (f32, f32),
    movement_distance_px: f32,
    muzzle_animation: bool,
    progress: f32,
    started: bool,
}

impl VaporRing {
    pub fn new(
        center_xy: (f32, f32),
        created_at: Instant,
        delay: Duration,
        scale: f32,
        rotation: (f32, f32),
        opacity: f32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            center_x_px: center_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32(),
            center_y_px: center_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32(),
            created_at,
            delay,
            scale,
            variant: rng.gen_range(0..3),
            rotation,
            opacity: opacity.clamp(0.0, 1.0),
            movement_direction: (0.0, 0.0),
            movement_distance_px: 0.0,
            muzzle_animation: false,
            progress: 0.0,
            started: false,
        }
    }

    pub fn with_movement(mut self, direction: (f32, f32), movement_distance_px: f32) -> Self {
        self.movement_direction = direction;
        self.movement_distance_px = movement_distance_px;
        self
    }

    pub fn with_muzzle_animation(
        mut self,
        direction: (f32, f32),
        movement_distance_px: f32,
    ) -> Self {
        self = self.with_movement(direction, movement_distance_px);
        self.muzzle_animation = true;
        self
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

        let eased_progress = ATTACK_FRAME_EASE_OUT.sample(self.progress);
        let frames = frames_for_variant(self.variant);
        let frame_index = ((eased_progress * frames.len() as f32) as usize).min(frames.len() - 1);
        let src_rect = frames[frame_index];
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let center_x_px = self.center_x_px
            + self.movement_direction.0 * self.movement_distance_px * eased_progress;
        let center_y_px = self.center_y_px
            + self.movement_direction.1 * self.movement_distance_px * eased_progress;
        let scale = if self.muzzle_animation {
            self.scale * (1.0 + eased_progress * 0.5)
        } else {
            self.scale
        };
        let alpha = if self.muzzle_animation {
            eased_progress * 255.0
        } else {
            (1.0 - self.progress) * self.opacity * 255.0
        };
        let scos = self.rotation.0 * scale;
        let ssin = self.rotation.1 * scale;
        let center_x = px(center_x_px);
        let center_y = px(center_y_px);

        sprites.push(namui::ImageSprite {
            src_rect,
            xform: namui::RSXform {
                scos,
                ssin,
                tx: center_x - px(scos * sprite_width / 2.0 - ssin * sprite_height / 2.0),
                ty: center_y - px(ssin * sprite_width / 2.0 + scos * sprite_height / 2.0),
            },
            color: Some(Color::WHITE.with_alpha(alpha.round() as u8)),
        });
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + DURATION
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_00,
        1 => &crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_01,
        _ => &crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_02,
    }
}

impl particle::Particle for VaporRing {
    fn tick(&mut self, now: Instant, dt: Duration) {
        VaporRing::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        VaporRing::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        VaporRing::is_done(self, now)
    }
}
