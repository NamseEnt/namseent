use super::super::ATTACK_FRAME_EASE_OUT;
use namui::*;
use rand::Rng;

const DURATION: Duration = Duration::from_millis(350);

#[derive(Clone)]
pub struct DirectionalFlash {
    anchor_xy: (f32, f32),
    angle_rad: f32,
    scale: f32,
    color: [f32; 3],
    variant: usize,
    created_at: Instant,
    progress: f32,
}

impl DirectionalFlash {
    pub fn new(
        anchor_xy: (f32, f32),
        angle_rad: f32,
        scale: f32,
        color: [f32; 3],
        created_at: Instant,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            anchor_xy,
            angle_rad,
            scale,
            color,
            variant: rng.gen_range(0..4),
            created_at,
            progress: 0.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress =
            ((now - self.created_at).as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0);
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let frames = frames_for_variant(self.variant);
        let frame_index = ((ATTACK_FRAME_EASE_OUT.sample(self.progress) * frames.len() as f32)
            as usize)
            .min(frames.len() - 1);
        let frame = frames[frame_index];
        let scale =
            crate::game_state::TILE_PX_SIZE.width.as_f32() * self.scale / frame.width().as_f32();
        let image_half_length_px = frame.width().as_f32() * scale / 2.0;
        let center_x = px(
            self.anchor_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32()
                + self.angle_rad.cos() * image_half_length_px,
        );
        let center_y = px(
            self.anchor_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32()
                + self.angle_rad.sin() * image_half_length_px,
        );
        let color = Color::from_f01(
            self.color[0].clamp(0.0, 1.0),
            self.color[1].clamp(0.0, 1.0),
            self.color[2].clamp(0.0, 1.0),
            1.0 - self.progress,
        );
        sprites.push(
            crate::game_state::field_particle::atlas::centered_rotated_sprite(
                frame,
                center_x,
                center_y,
                scale,
                self.angle_rad,
                Some(color),
            ),
        );
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= DURATION
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_FLASH_00,
        1 => &crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_FLASH_01,
        2 => &crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_FLASH_02,
        _ => &crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_FLASH_03,
    }
}

impl particle::Particle for DirectionalFlash {
    fn tick(&mut self, now: Instant, dt: Duration) {
        DirectionalFlash::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        DirectionalFlash::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        DirectionalFlash::is_done(self, now)
    }
}
