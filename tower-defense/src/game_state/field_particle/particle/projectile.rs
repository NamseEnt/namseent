use crate::MapCoordF32;
use crate::game_state::projectile::ProjectileKind;
use namui::*;

#[derive(Clone, State)]
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
}
