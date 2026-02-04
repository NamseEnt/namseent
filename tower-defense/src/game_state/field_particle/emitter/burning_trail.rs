use crate::{MapCoordF32, game_state::field_particle::BurningTrailParticle};
use namui::*;

const BURNING_TRAIL_SPAWN_DISTANCE: f32 = 0.12; // 맵 좌표 단위
const PARTICLES_PER_EMIT: usize = 3; // 한 번의 emit당 최대 생성 개수

#[derive(Clone, State)]
pub struct BurningTrailEmitter {
    from_xy: MapCoordF32,
    to_xy: MapCoordF32,
    movement_dt: Duration,
    created_at: Instant,
    total_particles: usize,
    emitted_particles: usize,
}

impl BurningTrailEmitter {
    pub fn new(
        from_xy: MapCoordF32,
        to_xy: MapCoordF32,
        movement_dt: Duration,
        created_at: Instant,
    ) -> Self {
        let distance = (to_xy - from_xy).length();
        let total_particles = (distance / BURNING_TRAIL_SPAWN_DISTANCE).ceil() as usize;
        Self {
            from_xy,
            to_xy,
            movement_dt,
            created_at,
            total_particles,
            emitted_particles: 0,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for BurningTrailEmitter
{
    fn emit(
        &mut self,
        now: Instant,
        dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted_particles >= self.total_particles {
            return vec![];
        }

        // dt에 따라 생성량을 약간 보정: 프레임이 길면 더 많이 생성
        let dt_scale = (dt.as_secs_f32() / (1.0 / 60.0)).max(0.5);
        let mut max_emit = ((PARTICLES_PER_EMIT as f32) * dt_scale).round() as usize;
        if max_emit == 0 {
            max_emit = 1;
        }

        let remaining = self.total_particles - self.emitted_particles;
        let emit_count = remaining.min(max_emit);

        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(emit_count);

        for i in 0..emit_count {
            let index = self.emitted_particles + i + 1; // 1-based
            let progress = (index as f32) / (self.total_particles as f32 + 1.0);
            let particle_xy = self.from_xy + (self.to_xy - self.from_xy) * progress;

            let px = (particle_xy.x, particle_xy.y);
            particles.push(
                crate::game_state::field_particle::FieldParticle::BurningTrail {
                    particle: BurningTrailParticle::new_with_random(px, now, &mut rng),
                },
            );
        }

        self.emitted_particles += emit_count;
        particles
    }

    fn is_done(&self, now: Instant) -> bool {
        self.emitted_particles >= self.total_particles
            || (now - self.created_at) > Duration::from_secs(3)
    }
}
