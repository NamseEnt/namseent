use super::super::ATTACK_FRAME_EASE_IN;
use namui::*;
use rand::Rng;

const DURATION: Duration = Duration::from_millis(120);
const MAX_DELAY: Duration = Duration::from_millis(200);
const MAX_ANGLE_OFFSET: f32 = std::f32::consts::PI / 36.0;

#[derive(Clone)]
pub struct ResidualSpark {
    center_xy: (f32, f32),
    angle_rad: f32,
    base_scale: f32,
    length_scale: f32,
    variant: usize,
    color: [f32; 3],
    created_at: Instant,
    delay: Duration,
    progress: f32,
    started: bool,
}

impl ResidualSpark {
    pub fn new(
        center_xy: (f32, f32),
        attack_angle_rad: f32,
        color: [f32; 3],
        created_at: Instant,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let angle_offset = rng.gen_range(-MAX_ANGLE_OFFSET..=MAX_ANGLE_OFFSET);
        Self {
            center_xy,
            angle_rad: attack_angle_rad + angle_offset,
            base_scale: rng.gen_range(0.25..=1.0),
            length_scale: rng.gen_range(1.0..=2.0),
            variant: rng.gen_range(0..3),
            color,
            created_at,
            delay: Duration::from_millis(rng.gen_range(0..=MAX_DELAY.as_millis())),
            progress: 0.0,
            started: false,
        }
    }

    pub fn with_base_scale_multiplier(mut self, multiplier: f32) -> Self {
        self.base_scale *= multiplier;
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    fn frame_index(progress: f32, frame_count: usize) -> usize {
        ((progress.clamp(0.0, 1.0) * frame_count as f32) as usize).min(frame_count - 1)
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

        let frames = frames_for_variant(self.variant);
        let frame = frames[Self::frame_index(self.progress, frames.len())];
        let base_scale = self.base_scale * crate::game_state::TILE_PX_SIZE.width.as_f32()
            / frame.height().as_f32();
        let scale = base_scale * self.length_scale;
        let alpha = 1.0 - ATTACK_FRAME_EASE_IN.sample(self.progress);
        let color = Color::from_f01(
            self.color[0].clamp(0.0, 1.0),
            self.color[1].clamp(0.0, 1.0),
            self.color[2].clamp(0.0, 1.0),
            alpha,
        );
        let center_x = px(self.center_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32());
        let center_y = px(self.center_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32());

        sprites.push(crate::atlas::centered_rotated_sprite(
            frame,
            center_x,
            center_y,
            scale,
            self.angle_rad,
            Some(color),
        ));
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + DURATION
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_RESIDUAL_SPARK_00,
        1 => &crate::atlas::generated::ATTACK_COMMON_RESIDUAL_SPARK_01,
        _ => &crate::atlas::generated::ATTACK_COMMON_RESIDUAL_SPARK_02,
    }
}

impl particle::Particle for ResidualSpark {
    fn tick(&mut self, now: Instant, dt: Duration) {
        ResidualSpark::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        ResidualSpark::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        ResidualSpark::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_selection_advances_linearly() {
        assert_eq!(ResidualSpark::frame_index(0.0, 5), 0);
        assert_eq!(ResidualSpark::frame_index(0.4, 5), 2);
        assert_eq!(ResidualSpark::frame_index(0.8, 5), 4);
        assert_eq!(ResidualSpark::frame_index(1.0, 5), 4);
    }

    #[test]
    fn particle_lives_for_delay_plus_one_hundred_twenty_ms() {
        let created_at = Instant::now();
        let spark = ResidualSpark::new((0.0, 0.0), 0.0, [0.6, 0.95, 1.0], created_at);

        assert!(spark.delay <= MAX_DELAY);
        assert!(!spark.is_done(created_at + spark.delay));
        assert!(spark.is_done(created_at + spark.delay + DURATION));
    }

    #[test]
    fn random_properties_stay_within_requested_ranges() {
        let spark = ResidualSpark::new((0.0, 0.0), 1.0, [0.6, 0.95, 1.0], Instant::now());

        assert!((spark.angle_rad - 1.0).abs() <= MAX_ANGLE_OFFSET);
        assert!((0.25..=1.0).contains(&spark.base_scale));
        assert!((1.0..=2.0).contains(&spark.length_scale));
        assert!(spark.delay <= MAX_DELAY);
    }

    #[test]
    fn base_scale_multiplier_scales_only_the_base_scale() {
        let spark = ResidualSpark::new((0.0, 0.0), 0.0, [0.6, 0.95, 1.0], Instant::now());
        let scaled = spark.clone().with_base_scale_multiplier(0.5);

        assert_eq!(scaled.base_scale, spark.base_scale * 0.5);
        assert_eq!(scaled.length_scale, spark.length_scale);
    }
}
