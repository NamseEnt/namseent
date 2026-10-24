use super::super::{ATTACK_FRAME_EASE_IN, ATTACK_FRAME_EASE_OUT};
use namui::*;
use rand::Rng;

const DEFAULT_DURATION: Duration = Duration::from_millis(250);

pub(super) type FramesForVariant = fn(usize) -> &'static [Rect<Px>];

#[derive(Clone, Copy)]
enum FadeMode {
    Linear,
    EaseIn,
    EaseOut,
}

#[derive(Clone)]
pub(super) struct RingAnimation {
    center_x_px: f32,
    center_y_px: f32,
    created_at: Instant,
    delay: Duration,
    duration: Duration,
    scale: f32,
    variant: usize,
    frames_for_variant: FramesForVariant,
    rotation: (f32, f32),
    opacity: f32,
    color: [f32; 3],
    movement_direction: (f32, f32),
    movement_distance_px: f32,
    scale_growth: f32,
    fade_mode: FadeMode,
    muzzle_animation: bool,
    progress: f32,
    started: bool,
}

impl RingAnimation {
    pub(super) fn new(
        center_xy: (f32, f32),
        created_at: Instant,
        delay: Duration,
        scale: f32,
        rotation: (f32, f32),
        opacity: f32,
        frames_for_variant: FramesForVariant,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            center_x_px: center_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32(),
            center_y_px: center_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32(),
            created_at,
            delay,
            duration: DEFAULT_DURATION,
            scale,
            variant: rng.gen_range(0..3),
            frames_for_variant,
            rotation,
            opacity: opacity.clamp(0.0, 1.0),
            color: [1.0; 3],
            movement_direction: (0.0, 0.0),
            movement_distance_px: 0.0,
            scale_growth: 0.0,
            fade_mode: FadeMode::Linear,
            muzzle_animation: false,
            progress: 0.0,
            started: false,
        }
    }

    pub(super) fn with_movement(
        mut self,
        direction: (f32, f32),
        movement_distance_px: f32,
    ) -> Self {
        self.movement_direction = direction;
        self.movement_distance_px = movement_distance_px;
        self
    }

    pub(super) fn with_color(mut self, color: [f32; 3]) -> Self {
        self.color = color;
        self
    }

    pub(super) fn with_scale_growth(mut self, scale_growth: f32) -> Self {
        self.scale_growth = scale_growth;
        self
    }

    pub(super) fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    pub(super) fn with_ease_out_fade(mut self) -> Self {
        self.fade_mode = FadeMode::EaseOut;
        self
    }

    pub(super) fn with_ease_in_fade(mut self) -> Self {
        self.fade_mode = FadeMode::EaseIn;
        self
    }

    pub(super) fn with_muzzle_animation(
        mut self,
        direction: (f32, f32),
        movement_distance_px: f32,
    ) -> Self {
        self = self.with_movement(direction, movement_distance_px);
        self.muzzle_animation = true;
        self
    }

    pub(super) fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed.as_secs_f32() - self.delay.as_secs_f32()) / self.duration.as_secs_f32())
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub(super) fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let eased_progress = ATTACK_FRAME_EASE_OUT.sample(self.progress);
        let frames = (self.frames_for_variant)(self.variant);
        let frame_index = ((eased_progress * frames.len() as f32) as usize).min(frames.len() - 1);
        let src_rect = frames[frame_index];
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let center_x_px = self.center_x_px
            + self.movement_direction.0 * self.movement_distance_px * eased_progress;
        let center_y_px = self.center_y_px
            + self.movement_direction.1 * self.movement_distance_px * eased_progress;
        let scale = self.scale
            * (1.0 + self.scale_growth * eased_progress)
            * if self.muzzle_animation {
                1.0 + eased_progress * 0.5
            } else {
                1.0
            };
        let alpha = self.alpha_at(eased_progress) * 255.0;
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
            scale_xy: None,
            color: Some(Color::from_f01(
                self.color[0].clamp(0.0, 1.0),
                self.color[1].clamp(0.0, 1.0),
                self.color[2].clamp(0.0, 1.0),
                (alpha / 255.0).clamp(0.0, 1.0),
            )),
        });
        sprites
    }

    pub(super) fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + self.duration
    }

    fn alpha_at(&self, eased_progress: f32) -> f32 {
        if self.muzzle_animation {
            eased_progress
        } else {
            let fade_progress = match self.fade_mode {
                FadeMode::Linear => self.progress,
                FadeMode::EaseIn => ATTACK_FRAME_EASE_IN.sample(self.progress),
                FadeMode::EaseOut => eased_progress,
            };
            (1.0 - fade_progress) * self.opacity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_frames(_: usize) -> &'static [Rect<Px>] {
        &[]
    }

    #[test]
    fn configured_duration_controls_progress_and_lifetime() {
        let created_at = Instant::now();
        let mut ring = RingAnimation::new(
            (0.0, 0.0),
            created_at,
            Duration::ZERO,
            1.0,
            (1.0, 0.0),
            1.0,
            no_frames,
        )
        .with_duration(Duration::from_millis(100));

        ring.tick(created_at + Duration::from_millis(50), Duration::ZERO);
        assert!((ring.progress - 0.5).abs() < f32::EPSILON);
        assert!(!ring.is_done(created_at + Duration::from_millis(99)));
        assert!(ring.is_done(created_at + Duration::from_millis(100)));
    }

    #[test]
    fn ease_out_fade_respects_the_configured_opacity_cap() {
        let created_at = Instant::now();
        let mut ring = RingAnimation::new(
            (0.0, 0.0),
            created_at,
            Duration::ZERO,
            1.0,
            (1.0, 0.0),
            0.5,
            no_frames,
        )
        .with_ease_out_fade();
        ring.tick(created_at + Duration::from_millis(50), Duration::ZERO);

        let eased_progress = ATTACK_FRAME_EASE_OUT.sample(ring.progress);
        let alpha = ring.alpha_at(eased_progress);
        assert!(alpha <= 0.5);
        assert_eq!(alpha, (1.0 - eased_progress) * 0.5);
    }

    #[test]
    fn ease_in_fade_stays_more_opaque_early_in_the_animation() {
        let created_at = Instant::now();
        let mut ring = RingAnimation::new(
            (0.0, 0.0),
            created_at,
            Duration::ZERO,
            1.0,
            (1.0, 0.0),
            1.0,
            no_frames,
        )
        .with_ease_in_fade();
        ring.tick(created_at + Duration::from_millis(50), Duration::ZERO);

        let eased_progress = ATTACK_FRAME_EASE_OUT.sample(ring.progress);
        let alpha = ring.alpha_at(eased_progress);
        assert_eq!(alpha, 1.0 - ATTACK_FRAME_EASE_IN.sample(ring.progress));
        assert!(alpha > 1.0 - ring.progress);
    }
}
