use crate::{
    MapCoordF32,
    game_state::field_particle::{BlueDotSparkParticle, WindCurveTrailParticle},
};
use namui::*;
use rand::Rng;

const WIND_CURVE_SPAWN_DISTANCE: f32 = 0.12;
const PARTICLES_PER_EMIT: usize = 3;
const BLUE_DOT_SPAWN_CHANCE: f32 = 0.22;

#[derive(Clone, State)]
pub struct WindCurveTrailEmitter {
    from_xy: MapCoordF32,
    to_xy: MapCoordF32,
    created_at: Instant,
    total_particles: usize,
    emitted_particles: usize,
}

impl WindCurveTrailEmitter {
    pub fn new(from_xy: MapCoordF32, to_xy: MapCoordF32, created_at: Instant) -> Self {
        let distance = (to_xy - from_xy).length();
        let total_particles = (distance / WIND_CURVE_SPAWN_DISTANCE).ceil() as usize;

        Self {
            from_xy,
            to_xy,
            created_at,
            total_particles,
            emitted_particles: 0,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for WindCurveTrailEmitter
{
    fn emit(
        &mut self,
        now: Instant,
        dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted_particles >= self.total_particles {
            return vec![];
        }

        let dt_scale = (dt.as_secs_f32() / (1.0 / 60.0)).max(0.5);
        let mut max_emit = ((PARTICLES_PER_EMIT as f32) * dt_scale).round() as usize;
        if max_emit == 0 {
            max_emit = 1;
        }

        let remaining = self.total_particles - self.emitted_particles;
        let emit_count = remaining.min(max_emit);

        let movement_vec = self.to_xy - self.from_xy;
        let movement_len = movement_vec.length();
        let movement_direction = if movement_len > 0.0 {
            (movement_vec.x / movement_len, movement_vec.y / movement_len)
        } else {
            (0.0, -1.0)
        };

        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(emit_count * 2);

        for i in 0..emit_count {
            let index = self.emitted_particles + i + 1;
            let progress = (index as f32) / (self.total_particles as f32 + 1.0);
            let particle_xy = self.from_xy + (self.to_xy - self.from_xy) * progress;
            let px = (particle_xy.x, particle_xy.y);

            particles.push(
                crate::game_state::field_particle::FieldParticle::WindCurveTrail {
                    particle: WindCurveTrailParticle::new_with_random(
                        px,
                        movement_direction,
                        now,
                        &mut rng,
                    ),
                },
            );

            if rng.gen_range(0.0..1.0) < BLUE_DOT_SPAWN_CHANCE {
                particles.push(
                    crate::game_state::field_particle::FieldParticle::BlueDotSpark {
                        particle: BlueDotSparkParticle::new_with_random(
                            px,
                            movement_direction,
                            now,
                            &mut rng,
                        ),
                    },
                );
            }
        }

        self.emitted_particles += emit_count;
        particles
    }

    fn is_done(&self, now: Instant) -> bool {
        self.emitted_particles >= self.total_particles
            || (now - self.created_at) > Duration::from_secs(3)
    }
}
