use crate::MapCoordF32;
use crate::game_state::field_particle::atlas;
use crate::game_state::projectile::ProjectileKind;
use namui::*;

#[derive(Clone)]
pub struct ProjectileParticle {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub velocity: Xy<f32>,
    pub created_at: Instant,
    pub duration: Duration,
    pub progress: f32,
}

impl ProjectileParticle {
    pub fn new(
        xy: MapCoordF32,
        kind: ProjectileKind,
        rotation: Angle,
        rotation_speed: Angle,
        velocity: Xy<f32>,
        created_at: Instant,
        duration: Duration,
    ) -> Self {
        Self {
            xy,
            kind,
            rotation,
            rotation_speed,
            velocity,
            created_at,
            duration,
            progress: 0.0,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        // Update rotation
        self.rotation += self.rotation_speed * dt.as_secs_f32();

        // Update position based on velocity vector (tiles/second)
        self.xy += self.velocity * dt.as_secs_f32();

        let duration_secs = self.duration.as_secs_f32();
        self.progress = if duration_secs <= f32::EPSILON {
            1.0
        } else {
            ((now - self.created_at).as_secs_f32() / duration_secs).clamp(0.0, 1.0)
        };
    }

    pub fn is_alive(&self, now: Instant) -> bool {
        now - self.created_at < self.duration
    }

    pub fn render_particle(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let life_ratio = (1.0 - self.progress).max(0.0);
        if life_ratio <= 0.0 {
            return sprites;
        }

        let tile_px_size = crate::game_state::TILE_PX_SIZE;
        let scale = ((tile_px_size.width.as_f32() * 0.4) / 128.0) * life_ratio;
        let particle_px_xy = tile_px_size.to_xy() * Xy::new(self.xy.x, self.xy.y);
        let angle_rad = self.rotation.as_radians();
        let src_rect = atlas::projectile_rect(self.kind);
        let color = Color::WHITE.with_alpha((life_ratio * 255.0).round() as u8);
        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            particle_px_xy.x,
            particle_px_xy.y,
            scale,
            angle_rad,
            Some(color),
        ));
        sprites
    }
}

impl namui::particle::Particle for ProjectileParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        ProjectileParticle::tick(self, now, dt);
    }
    fn render(&self) -> namui::particle::ParticleSprites {
        ProjectileParticle::render_particle(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        !self.is_alive(now)
    }
}
