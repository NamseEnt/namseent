use super::trash::EaseMode;
use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const CARD_SIZE_TILE: f32 = 0.3;

#[derive(Clone, Copy)]
pub enum CardKind {
    Card00,
    Card01,
    Card02,
    Card03,
}

impl CardKind {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => CardKind::Card00,
            1 => CardKind::Card01,
            2 => CardKind::Card02,
            3 => CardKind::Card03,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct CardParticle {
    pub kind: CardKind,
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
    pub duration: Duration,
    pub progress: f32,
    pub ease_mode: EaseMode,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub gravity: f32,
}

#[derive(Clone)]
pub struct CardParticleConfig {
    pub kind: CardKind,
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
    pub duration: Duration,
    pub ease_mode: EaseMode,
    pub gravity: f32,
    pub rotation_speed_deg_per_sec: (f32, f32),
}

impl CardParticle {
    pub fn tick_impl(&mut self, now: Instant, dt: Duration) {
        self.progress = self.progress(now);
        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let eased = match self.ease_mode {
            EaseMode::Linear => self.progress,
            EaseMode::EaseOutCubic => {
                let inv = 1.0 - self.progress;
                1.0 - (inv * inv * inv)
            }
        };

        let x = self.start_xy.0 + (self.end_xy.0 - self.start_xy.0) * eased;
        let mut y = self.start_xy.1 + (self.end_xy.1 - self.start_xy.1) * eased;

        let elapsed_secs = self.progress * self.duration.as_secs_f32();
        let y_offset = 0.5 * self.gravity * elapsed_secs * elapsed_secs;
        y += y_offset;

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(x, y);

        let scale = (TILE_PX_SIZE.width.as_f32() * CARD_SIZE_TILE) / 128.0;

        let alpha = (1.0 - self.progress).max(0.0);
        let color = Color::WHITE.with_alpha((alpha * 255.0) as u8);

        let angle_rad = self.rotation.as_radians();
        let src_rect = atlas::card_particle_rect(self.kind);

        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            px_xy.x,
            px_xy.y,
            scale,
            angle_rad,
            Some(color),
        ));
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
    }

    pub fn new_with_random_burst(config: CardParticleConfig) -> Self {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-0.3..0.3);
        let offset_y = rng.gen_range(-0.3..0.3);
        let end = (config.end_xy.0 + offset_x, config.end_xy.1 + offset_y);

        let rotation = rng.gen_range(0.0..360.0).deg();
        let rotation_speed = rng
            .gen_range(config.rotation_speed_deg_per_sec.0..config.rotation_speed_deg_per_sec.1)
            .deg();

        Self {
            kind: config.kind,
            start_xy: config.start_xy,
            end_xy: end,
            created_at: config.created_at,
            duration: config.duration,
            progress: 0.0,
            ease_mode: config.ease_mode,
            rotation,
            rotation_speed,
            gravity: config.gravity,
        }
    }
}

impl namui::particle::Particle for CardParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        CardParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        CardParticle::is_done(self, now)
    }
}
