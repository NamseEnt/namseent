use super::super::ATTACK_FRAME_EASE_IN;
use namui::*;

const DEFAULT_DURATION: Duration = Duration::from_millis(150);

#[derive(Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl RotationDirection {
    fn sign(self) -> f32 {
        match self {
            Self::Clockwise => 1.0,
            Self::CounterClockwise => -1.0,
        }
    }
}

#[derive(Clone)]
pub struct StarFlare {
    center_xy: (f32, f32),
    max_scale_tiles: f32,
    rotation_direction: RotationDirection,
    color: [f32; 3],
    created_at: Instant,
    delay: Duration,
    duration: Duration,
    growth_duration: Duration,
    peak_progress: f32,
    fade_after_growth: bool,
    progress: f32,
    scale_progress: f32,
    rotation: (f32, f32),
    started: bool,
}

impl StarFlare {
    pub fn new(
        center_xy: (f32, f32),
        max_scale_tiles: f32,
        rotation_direction: RotationDirection,
        color: [f32; 3],
        created_at: Instant,
    ) -> Self {
        Self {
            center_xy,
            max_scale_tiles,
            rotation_direction,
            color,
            created_at,
            delay: Duration::ZERO,
            duration: DEFAULT_DURATION,
            growth_duration: DEFAULT_DURATION / 2,
            peak_progress: 0.5,
            fade_after_growth: false,
            progress: 0.0,
            scale_progress: 0.0,
            rotation: (1.0, 0.0),
            started: false,
        }
    }

    pub fn with_timing(mut self, duration: Duration, growth_duration: Duration) -> Self {
        self.duration = duration;
        self.growth_duration = growth_duration.min(duration);
        self.peak_progress = self.growth_duration.as_secs_f32() / self.duration.as_secs_f32();
        self.fade_after_growth = true;
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    fn scale_progress(progress: f32) -> f32 {
        Self::scale_progress_with_peak(progress, 0.5)
    }

    fn scale_progress_with_peak(progress: f32, peak_progress: f32) -> f32 {
        let progress = progress.clamp(0.0, 1.0);
        if progress <= peak_progress {
            ATTACK_FRAME_EASE_IN.sample(progress / peak_progress.max(f32::EPSILON))
        } else {
            1.0 - ATTACK_FRAME_EASE_IN
                .sample((progress - peak_progress) / (1.0 - peak_progress).max(f32::EPSILON))
        }
    }

    fn rotation_angle(&self) -> f32 {
        self.rotation_direction.sign() * std::f32::consts::TAU * self.progress
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed - self.delay).as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
        } else {
            0.0
        };
        if self.started && self.progress < 1.0 {
            self.scale_progress = Self::scale_progress_with_peak(self.progress, self.peak_progress);
            let (rotation_sin, rotation_cos) = self.rotation_angle().sin_cos();
            self.rotation = (rotation_cos, rotation_sin);
        } else {
            self.scale_progress = 0.0;
            self.rotation = (1.0, 0.0);
        }
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let frame = crate::atlas::generated::ATTACK_COMMON_STAR_FLARE;
        let scale_tiles = self.max_scale_tiles * self.scale_progress;
        let scale =
            crate::game_state::TILE_PX_SIZE.width.as_f32() * scale_tiles / frame.width().as_f32();
        let center_x = px(self.center_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32());
        let center_y = px(self.center_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32());
        let alpha = if self.fade_after_growth && self.progress > self.peak_progress {
            1.0 - (self.progress - self.peak_progress)
                / (1.0 - self.peak_progress).max(f32::EPSILON)
        } else {
            1.0
        };
        let color = Color::from_f01(
            self.color[0].clamp(0.0, 1.0),
            self.color[1].clamp(0.0, 1.0),
            self.color[2].clamp(0.0, 1.0),
            alpha.clamp(0.0, 1.0),
        );

        sprites.push(crate::atlas::centered_rotated_sprite_with_sin_cos(
            frame,
            center_x,
            center_y,
            scale,
            self.rotation.0,
            self.rotation.1,
            Some(color),
        ));
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + self.duration
    }
}

impl particle::Particle for StarFlare {
    fn tick(&mut self, now: Instant, dt: Duration) {
        StarFlare::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        StarFlare::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        StarFlare::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_starts_and_ends_at_zero_with_eased_growth_and_shrink() {
        assert_eq!(StarFlare::scale_progress(0.0), 0.0);
        assert_eq!(StarFlare::scale_progress(0.5), 1.0);
        assert_eq!(StarFlare::scale_progress(1.0), 0.0);
        assert!(StarFlare::scale_progress(0.25) < 0.5);
        assert!(StarFlare::scale_progress(0.75) > 0.5);
    }

    #[test]
    fn timed_flare_reaches_maximum_after_50ms_and_finishes_after_200ms() {
        let flare = StarFlare::new(
            (0.0, 0.0),
            1.0,
            RotationDirection::Clockwise,
            [0.8, 0.55, 1.0],
            Instant::now(),
        )
        .with_timing(Duration::from_millis(200), Duration::from_millis(50));

        assert_eq!(flare.duration, Duration::from_millis(200));
        assert_eq!(flare.growth_duration, Duration::from_millis(50));
        assert_eq!(StarFlare::scale_progress_with_peak(0.25, 0.25), 1.0);
        assert_eq!(StarFlare::scale_progress_with_peak(1.0, 0.25), 0.0);
    }

    #[test]
    fn rotation_directions_have_opposite_signs() {
        let created_at = Instant::now();
        let mut clockwise = StarFlare::new(
            (0.0, 0.0),
            3.0,
            RotationDirection::Clockwise,
            [0.6, 0.95, 1.0],
            created_at,
        );
        let mut counter_clockwise = StarFlare::new(
            (0.0, 0.0),
            2.0,
            RotationDirection::CounterClockwise,
            [0.6, 0.95, 1.0],
            created_at,
        );
        let halfway = created_at + Duration::from_millis(150);
        clockwise.tick(halfway, Duration::ZERO);
        counter_clockwise.tick(halfway, Duration::ZERO);

        assert!(clockwise.rotation_angle() > 0.0);
        assert!(counter_clockwise.rotation_angle() < 0.0);
    }
}
