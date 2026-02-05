use crate::MapCoordF32;
use crate::game_state::field_particle::ProjectileParticle;
use crate::game_state::projectile::ProjectileKind;
use namui::*;

#[derive(Clone, State)]
pub struct ProjectileParticleEmitter {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub velocity: Xy<f32>,
    pub created_at: Instant,
    pub duration: Duration,
    pub emitted: bool,
}

impl ProjectileParticleEmitter {
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
            emitted: false,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for ProjectileParticleEmitter
{
    fn emit(
        &mut self,
        _now: Instant,
        _dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted {
            return vec![];
        }

        self.emitted = true;

        vec![
            crate::game_state::field_particle::FieldParticle::Projectile {
                particle: ProjectileParticle::new(
                    self.xy,
                    self.kind,
                    self.rotation,
                    self.rotation_speed,
                    self.velocity,
                    self.created_at,
                    self.duration,
                ),
            },
        ]
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
