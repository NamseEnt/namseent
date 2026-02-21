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
        }
    }

    pub fn tick(&mut self, _now: Instant, dt: Duration) {
        // Update rotation
        self.rotation += self.rotation_speed * dt.as_secs_f32();

        // Update position based on velocity vector (tiles/second)
        self.xy += self.velocity * dt.as_secs_f32();
    }

    pub fn is_alive(&self, now: Instant) -> bool {
        now - self.created_at < self.duration
    }

    pub fn render_particle(&self) -> Option<ImageSprite> {
        let tile_px_size = crate::game_state::TILE_PX_SIZE;
        let scale = (tile_px_size.width.as_f32() * 0.4) / 128.0;
        let particle_px_xy = tile_px_size.to_xy() * Xy::new(self.xy.x, self.xy.y);
        let angle_rad = self.rotation.as_radians();
        let src_rect = atlas::projectile_rect(self.kind);

        Some(atlas::centered_rotated_sprite(src_rect, particle_px_xy.x, particle_px_xy.y, scale, angle_rad, None))
    }
}

impl namui::particle::Particle for ProjectileParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        ProjectileParticle::tick(self, now, dt);
    }
    fn render(&self) -> Option<ImageSprite> {
        ProjectileParticle::render_particle(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        !self.is_alive(now)
    }
}
