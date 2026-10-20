use super::super::{ATTACK_FRAME_EASE_IN, ATTACK_FRAME_EASE_OUT};
use namui::*;
use rand::Rng;

const DURATION: Duration = Duration::from_millis(300);

#[derive(Clone, Copy, Default)]
pub struct DirectionalVolumeSmokeOptions {
    pub delay: Duration,
    pub start_color: Option<[f32; 3]>,
    pub end_color: Option<[f32; 3]>,
    pub start_scale: Option<f32>,
    pub end_scale: Option<f32>,
}

#[derive(Clone)]
pub struct DirectionalVolumeSmoke {
    start_xy: (f32, f32),
    direction: (f32, f32),
    distance_tiles: f32,
    angle_rad: f32,
    delay: Duration,
    start_color: [f32; 3],
    end_color: [f32; 3],
    start_scale: f32,
    end_scale: f32,
    variant: usize,
    created_at: Instant,
    progress: f32,
    started: bool,
}

impl DirectionalVolumeSmoke {
    pub fn new(
        start_xy: (f32, f32),
        direction: (f32, f32),
        distance_tiles: f32,
        angle_rad: f32,
        options: DirectionalVolumeSmokeOptions,
        created_at: Instant,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let start_scale = options
            .start_scale
            .unwrap_or_else(|| rng.gen_range(0.65..=1.15));
        Self {
            start_xy,
            direction,
            distance_tiles,
            angle_rad,
            delay: options.delay,
            start_color: options.start_color.unwrap_or([1.0, 0.75, 0.25]),
            end_color: options.end_color.unwrap_or([0.24, 0.09, 0.025]),
            start_scale,
            end_scale: options.end_scale.unwrap_or(start_scale),
            variant: rng.gen_range(0..2),
            created_at,
            progress: 0.0,
            started: false,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed - self.delay).as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let eased_progress = ATTACK_FRAME_EASE_OUT.sample(self.progress);
        let frames = frames_for_variant(self.variant);
        let frame_index = ((eased_progress * frames.len() as f32) as usize).min(frames.len() - 1);
        let frame = frames[frame_index];
        let color_progress = ATTACK_FRAME_EASE_IN.sample(self.progress);
        let xy = (
            self.start_xy.0 + self.direction.0 * self.distance_tiles * eased_progress,
            self.start_xy.1 + self.direction.1 * self.distance_tiles * eased_progress,
        );
        let scale_value = self.start_scale + (self.end_scale - self.start_scale) * eased_progress;
        let scale =
            crate::game_state::TILE_PX_SIZE.width.as_f32() * scale_value / frame.width().as_f32();
        let image_half_length_px = frame.width().as_f32() * scale / 2.0;
        let center_x = px(xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32()
            + self.direction.0 * image_half_length_px);
        let center_y = px(xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32()
            + self.direction.1 * image_half_length_px);
        let red = self.start_color[0] + (self.end_color[0] - self.start_color[0]) * color_progress;
        let green =
            self.start_color[1] + (self.end_color[1] - self.start_color[1]) * color_progress;
        let blue = self.start_color[2] + (self.end_color[2] - self.start_color[2]) * color_progress;
        let alpha = (1.0 - self.progress) * 0.9;
        let color = Color::from_f01(
            red.clamp(0.0, 1.0),
            green.clamp(0.0, 1.0),
            blue.clamp(0.0, 1.0),
            alpha,
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
        now - self.created_at >= self.delay + DURATION
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_VOLUME_SMOKE_00,
        _ => &crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_VOLUME_SMOKE_01,
    }
}

impl particle::Particle for DirectionalVolumeSmoke {
    fn tick(&mut self, now: Instant, dt: Duration) {
        DirectionalVolumeSmoke::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        DirectionalVolumeSmoke::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        DirectionalVolumeSmoke::is_done(self, now)
    }
}
