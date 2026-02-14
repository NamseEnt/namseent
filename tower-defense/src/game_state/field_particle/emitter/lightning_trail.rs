use crate::{
    MapCoordF32,
    game_state::field_particle::{FieldParticle, LightningBoltParticle},
};
use namui::*;
use rand::Rng;

pub(crate) const LIGHTNING_TRAIL_SPAWN_DISTANCE: f32 = 0.3; // 스파크보다 약간 더 많음
const PARTICLES_PER_EMIT: usize = 1;
const LIGHTNING_SPAWN_CHANCE: f32 = 0.6; // 번개줄기가 죽을 때 새로운 번개줄기를 생성할 확률

#[derive(Clone, State)]
pub struct LightningTrailEmitter {
    from_xy: MapCoordF32,
    to_xy: MapCoordF32,
    created_at: Instant,
    total_particles: usize,
    emitted_particles: usize,
}

impl LightningTrailEmitter {
    pub fn new(
        from_xy: MapCoordF32,
        to_xy: MapCoordF32,
        _movement_dt: Duration,
        created_at: Instant,
    ) -> Self {
        let distance = (to_xy - from_xy).length();
        let total_particles = (distance / LIGHTNING_TRAIL_SPAWN_DISTANCE).ceil() as usize;
        Self::new_with_particle_count(from_xy, to_xy, total_particles, created_at)
    }

    pub fn new_with_particle_count(
        from_xy: MapCoordF32,
        to_xy: MapCoordF32,
        total_particles: usize,
        created_at: Instant,
    ) -> Self {
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
    for LightningTrailEmitter
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

        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(emit_count);

        // 이동 방향 벡터 계산
        let movement_vec = self.to_xy - self.from_xy;
        let movement_len = movement_vec.length();

        if movement_len < 0.001 {
            self.emitted_particles = self.total_particles;
            return vec![];
        }

        for _ in 0..emit_count {
            let t = rng.gen_range(0.0..1.0);
            let start_pos = MapCoordF32 {
                x: self.from_xy.x + movement_vec.x * t,
                y: self.from_xy.y + movement_vec.y * t,
            };

            // 방향 벡터 (정규화)
            let movement_dir_x = movement_vec.x / movement_len;
            let movement_dir_y = movement_vec.y / movement_len;

            // 수직 벡터
            let perp_x = -movement_dir_y;
            let perp_y = movement_dir_x;

            // 끝 위치 - 약간 앞쪽으로
            let end_t = rng.gen_range(0.6..1.0);
            let mut end_pos = MapCoordF32 {
                x: self.from_xy.x + movement_vec.x * end_t,
                y: self.from_xy.y + movement_vec.y * end_t,
            };

            // 수직 방향으로 약간 오프셋
            let angle_offset = rng.gen_range(-0.3..0.3);
            end_pos.x += perp_x * angle_offset;
            end_pos.y += perp_y * angle_offset;

            let lifetime = Duration::from_millis(rng.gen_range(50..80));

            let lightning = LightningBoltParticle::new(
                (start_pos.x, start_pos.y),
                (end_pos.x, end_pos.y),
                now,
                lifetime,
                LIGHTNING_SPAWN_CHANCE,
            );

            particles.push(FieldParticle::LightningBolt {
                particle: lightning,
            });

            self.emitted_particles += 1;
        }

        particles
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted_particles >= self.total_particles
    }
}
