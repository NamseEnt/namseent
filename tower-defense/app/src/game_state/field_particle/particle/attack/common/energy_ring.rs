use super::ring_animation::RingAnimation;
use namui::*;

#[derive(Clone)]
pub struct EnergyRing(RingAnimation);

impl EnergyRing {
    pub fn new(
        center_xy: (f32, f32),
        created_at: Instant,
        delay: Duration,
        scale: f32,
        rotation: (f32, f32),
        opacity: f32,
    ) -> Self {
        Self(RingAnimation::new(
            center_xy,
            created_at,
            delay,
            scale,
            rotation,
            opacity,
            frames_for_variant,
        ))
    }

    pub fn with_movement(mut self, direction: (f32, f32), movement_distance_px: f32) -> Self {
        self.0 = self.0.with_movement(direction, movement_distance_px);
        self
    }

    pub fn with_color(mut self, color: [f32; 3]) -> Self {
        self.0 = self.0.with_color(color);
        self
    }

    pub fn with_scale_growth(mut self, scale_growth: f32) -> Self {
        self.0 = self.0.with_scale_growth(scale_growth);
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.0 = self.0.with_duration(duration);
        self
    }

    pub fn with_ease_out_fade(mut self) -> Self {
        self.0 = self.0.with_ease_out_fade();
        self
    }

    pub fn with_ease_in_fade(mut self) -> Self {
        self.0 = self.0.with_ease_in_fade();
        self
    }

    pub fn with_muzzle_animation(
        mut self,
        direction: (f32, f32),
        movement_distance_px: f32,
    ) -> Self {
        self.0 = self
            .0
            .with_muzzle_animation(direction, movement_distance_px);
        self
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        self.0.tick(now, dt);
    }

    pub fn render(&self) -> particle::ParticleSprites {
        self.0.render()
    }

    pub fn is_done(&self, now: Instant) -> bool {
        self.0.is_done(now)
    }
}

fn frames_for_variant(variant: usize) -> &'static [Rect<Px>] {
    match variant {
        0 => &crate::atlas::generated::ATTACK_COMMON_ENERGY_RING_00,
        1 => &crate::atlas::generated::ATTACK_COMMON_ENERGY_RING_01,
        _ => &crate::atlas::generated::ATTACK_COMMON_ENERGY_RING_02,
    }
}

impl particle::Particle for EnergyRing {
    fn tick(&mut self, now: Instant, dt: Duration) {
        EnergyRing::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        EnergyRing::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        EnergyRing::is_done(self, now)
    }
}
