use crate::{
    MapCoordF32,
    game_state::field_particle::{EmberSparkParticle, SparkleParticle},
};
use namui::*;
use rand::Rng;

pub(crate) const SPARKLE_SPAWN_DISTANCE: f32 = 0.25; // 맵 좌표 단위
const PARTICLES_PER_EMIT: usize = 2; // 한 번의 emit당 최대 생성 개수
const EMBER_SPARK_SPAWN_CHANCE: f32 = 0.15; // 15% 확률로 ember spark 생성

#[derive(Clone, State)]
pub struct SparkleEmitter {
    from_xy: MapCoordF32,
    to_xy: MapCoordF32,
    created_at: Instant,
    total_particles: usize,
    emitted_particles: usize,
}

impl SparkleEmitter {
    pub fn new(
        from_xy: MapCoordF32,
        to_xy: MapCoordF32,
        _movement_dt: Duration,
        created_at: Instant,
    ) -> Self {
        let distance = (to_xy - from_xy).length();
        let total_particles = (distance / SPARKLE_SPAWN_DISTANCE).ceil() as usize;
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

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle> for SparkleEmitter {
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
        let mut particles = Vec::with_capacity(emit_count * 2); // 여유있게 할당

        // 이동 방향 벡터 계산 (정규화)
        let movement_vec = self.to_xy - self.from_xy;
        let movement_len = movement_vec.length();
        let movement_dir = if movement_len > 0.0 {
            (movement_vec.x / movement_len, movement_vec.y / movement_len)
        } else {
            (1.0, 0.0) // 기본값
        };

        for i in 0..emit_count {
            let index = self.emitted_particles + i + 1; // 1-based
            let progress = (index as f32) / (self.total_particles as f32 + 1.0);
            let particle_xy = self.from_xy + (self.to_xy - self.from_xy) * progress;

            let px = (particle_xy.x, particle_xy.y);

            // 항상 sparkle 파티클 생성
            particles.push(crate::game_state::field_particle::FieldParticle::Sparkle {
                particle: SparkleParticle::new_with_random(px, now, &mut rng),
            });

            // 낮은 확률로 ember spark 파티클 추가 생성
            if rng.gen_range(0.0..1.0) < EMBER_SPARK_SPAWN_CHANCE {
                particles.push(
                    crate::game_state::field_particle::FieldParticle::EmberSpark {
                        particle: EmberSparkParticle::new_with_random(
                            px,
                            movement_dir,
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
