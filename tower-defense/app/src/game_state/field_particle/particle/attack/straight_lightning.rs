use super::ATTACK_FRAME_EASE_OUT;
use namui::*;

const DURATION: Duration = Duration::from_millis(250);
const THICKNESS_SCALE: f32 = 2.0;
pub const LIGHTNING_COLOR: [f32; 3] = [0.6, 0.95, 1.0];

#[derive(Clone)]
pub struct StraightLightning {
    center_xy: (f32, f32),
    angle_rad: f32,
    segment_length_tiles: f32,
    variant: usize,
    flip_vertical: bool,
    scale_multiplier: f32,
    color: [f32; 3],
    created_at: Instant,
    delay: Duration,
    progress: f32,
    started: bool,
}

impl StraightLightning {
    pub fn new(
        center_xy: (f32, f32),
        angle_rad: f32,
        segment_length_tiles: f32,
        variant: usize,
        flip_vertical: bool,
        created_at: Instant,
        delay: Duration,
    ) -> Self {
        Self {
            center_xy,
            angle_rad,
            segment_length_tiles,
            variant,
            flip_vertical,
            scale_multiplier: 1.0,
            color: LIGHTNING_COLOR,
            created_at,
            delay,
            progress: 0.0,
            started: false,
        }
    }

    pub fn with_scale_multiplier(mut self, scale_multiplier: f32) -> Self {
        self.scale_multiplier = scale_multiplier.max(0.0);
        self
    }

    pub fn with_color(mut self, color: [f32; 3]) -> Self {
        self.color = color;
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

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let frames = frames_for_variant(self.variant);
        let frame_index = ((ATTACK_FRAME_EASE_OUT.sample(self.progress) * frames.len() as f32)
            as usize)
            .min(frames.len() - 1);
        let frame = frames[frame_index];
        let scale = crate::game_state::TILE_PX_SIZE.width.as_f32() * self.segment_length_tiles
            / frame.width().as_f32();
        let thickness = scale * frame.height().as_f32() * THICKNESS_SCALE * self.scale_multiplier;
        let angle = self.angle_rad
            + if self.flip_vertical {
                std::f32::consts::PI
            } else {
                0.0
            };
        let opacity = 1.0 - self.progress;
        let color = Color::from_f01(self.color[0], self.color[1], self.color[2], opacity);
        let half_length_px =
            crate::game_state::TILE_PX_SIZE.width.as_f32() * self.segment_length_tiles / 2.0;
        let center_x = self.center_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32();
        let center_y = self.center_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32();
        let start_x = center_x - angle.cos() * half_length_px;
        let start_y = center_y - angle.sin() * half_length_px;
        let end_x = center_x + angle.cos() * half_length_px;
        let end_y = center_y + angle.sin() * half_length_px;
        if let Some(sprite) = crate::game_state::field_particle::atlas::line_sprite_from_rect(
            frame,
            px(start_x),
            px(start_y),
            px(end_x),
            px(end_y),
            thickness,
            Some(color),
        ) {
            sprites.push(sprite);
        }
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + DURATION
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_LIGHTNING_BOLT_00,
        1 => &crate::atlas::generated::ATTACK_COMMON_LIGHTNING_BOLT_01,
        _ => &crate::atlas::generated::ATTACK_COMMON_LIGHTNING_BOLT_02,
    }
}

impl particle::Particle for StraightLightning {
    fn tick(&mut self, now: Instant, dt: Duration) {
        StraightLightning::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        StraightLightning::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        StraightLightning::is_done(self, now)
    }
}
