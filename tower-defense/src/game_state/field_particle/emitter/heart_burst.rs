use crate::{MapCoordF32, game_state::field_particle::HeartParticle};
use namui::*;

const BURST_DURATION_MS: i64 = 100;
const PARTICLES_PER_EMIT: usize = 4; // 한 번의 emit당 생성 개수
const TOTAL_PARTICLES: usize = 8; // 총 생성할 파티클 개수

#[derive(Clone, State)]
pub struct HeartBurstEmitter {
    xy: MapCoordF32,
    created_at: Instant,
    emitted_particles: usize,
}

impl HeartBurstEmitter {
    pub fn new(xy: MapCoordF32, created_at: Instant) -> Self {
        Self {
            xy,
            created_at,
            emitted_particles: 0,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for HeartBurstEmitter
{
    fn emit(
        &mut self,
        now: Instant,
        dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted_particles >= TOTAL_PARTICLES {
            return vec![];
        }

        // dt에 따라 생성량을 보정
        let dt_scale = (dt.as_secs_f32() / (1.0 / 60.0)).max(0.5);
        let mut max_emit = ((PARTICLES_PER_EMIT as f32) * dt_scale).round() as usize;
        if max_emit == 0 {
            max_emit = 1;
        }

        let remaining = TOTAL_PARTICLES - self.emitted_particles;
        let emit_count = remaining.min(max_emit);

        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(emit_count);

        for _ in 0..emit_count {
            particles.push(crate::game_state::field_particle::FieldParticle::Heart {
                particle: HeartParticle::new_burst((self.xy.x, self.xy.y), now, &mut rng),
            });
        }

        self.emitted_particles += emit_count;
        particles
    }

    fn is_done(&self, now: Instant) -> bool {
        self.emitted_particles >= TOTAL_PARTICLES
            || (now - self.created_at) > Duration::from_millis(BURST_DURATION_MS)
    }
}
