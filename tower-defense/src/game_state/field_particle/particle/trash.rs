use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const TRASH_SIZE_TILE: f32 = 0.5;

#[derive(Clone, Copy)]
pub enum EaseMode {
    EaseOutCubic,
}

#[derive(Clone)]
pub struct TrashParticle {
    pub kind: ProjectileKind,
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
    pub duration: Duration,
    pub progress: f32,
    pub ease_mode: EaseMode,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub should_bounce: bool,
    pub bounced: bool,
    pub gravity: f32,
}

#[derive(Clone)]
pub struct TrashParticleConfig {
    pub kind: ProjectileKind,
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
    pub duration: Duration,
    pub ease_mode: EaseMode,
    pub should_bounce: bool,
    pub gravity: f32,
    pub rotation_speed_deg_per_sec: (f32, f32),
}

impl TrashParticle {
    pub fn new(
        kind: ProjectileKind,
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        ease_mode: EaseMode,
    ) -> Self {
        Self {
            kind,
            start_xy,
            end_xy,
            created_at,
            duration,
            progress: 0.0,
            ease_mode,
            rotation: 0.deg(),
            rotation_speed: 0.deg(),
            should_bounce: false,
            bounced: false,
            gravity: 0.0,
        }
    }

    pub fn tick_impl(&mut self, now: Instant, dt: Duration) {
        self.progress = self.progress(now);
        self.rotation += self.rotation_speed * dt.as_secs_f32();

        if self.progress >= 1.0 && self.should_bounce && !self.bounced {
            self.bounced = true;
            for p in crate::game_state::field_particle::emitter::create_bounce_particles(
                self.kind,
                self.start_xy,
                self.end_xy,
                now,
            ) {
                crate::game_state::field_particle::TRASHES.spawn(p);
            }
        }
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let eased = match self.ease_mode {
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

        let scale = (TILE_PX_SIZE.width.as_f32() * TRASH_SIZE_TILE) / 128.0;

        let alpha = (1.0 - self.progress).max(0.0);
        let color = Color::WHITE.with_alpha((alpha * 255.0) as u8);

        let angle_rad = self.rotation.as_radians();
        let src_rect = atlas::projectile_rect(self.kind);

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
        let base_done = now - self.created_at >= self.duration;
        (self.bounced || !self.should_bounce) && base_done
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
    }

    pub fn new_with_random_end(config: TrashParticleConfig) -> Self {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-0.25..0.25);
        let offset_y = rng.gen_range(-0.1..0.1);
        let end = (config.end_xy.0 + offset_x, config.end_xy.1 + offset_y);
        let mut s = Self::new(
            config.kind,
            config.start_xy,
            end,
            config.created_at,
            config.duration,
            config.ease_mode,
        );
        s.rotation = rng.gen_range(0.0..360.0).deg();
        s.rotation_speed = rng
            .gen_range(config.rotation_speed_deg_per_sec.0..config.rotation_speed_deg_per_sec.1)
            .deg();
        s.should_bounce = config.should_bounce;
        s.gravity = config.gravity;
        s
    }
}

impl namui::particle::Particle for TrashParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        TrashParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        TrashParticle::is_done(self, now)
    }
}
