use crate::{MapCoordF32, game_state::field_particle::HeartParticle};
use namui::*;

// === Position offset ===
const EXPLOSION_Y_OFFSET: f32 = 0.5; // 착탄점 바닥 위치 (y + 0.5 tile)
const COLUMN_TOP_Y_OFFSET: f32 = -0.125; // 기둥 상단 높이

// === Particle counts per phase ===
const EXPLOSION_PARTICLES_PER_EMIT: usize = 8; // 폭발: 한 번 emit당
const EXPLOSION_TOTAL: usize = 16; // 폭발: 총 개수

const COLUMN_PARTICLES_PER_EMIT: usize = 8; // 기둥: 한 번 emit당
const COLUMN_TOTAL: usize = 48; // 기둥: 총 개수

const TOP_HEART_PARTICLES_PER_EMIT: usize = 1; // 상승 하트: 한 번 emit당
const TOP_HEART_TOTAL: usize = 1; // 상승 하트: 총 개수 1개

#[derive(Clone, State)]
pub struct HeartBurstEmitter {
    xy: MapCoordF32,
    created_at: Instant,
    // Phase별 emitted count
    explosion_emitted: usize,
    column_emitted: usize,
    top_heart_emitted: usize,
}

impl HeartBurstEmitter {
    pub fn new(xy: MapCoordF32, created_at: Instant) -> Self {
        Self {
            xy,
            created_at,
            explosion_emitted: 0,
            column_emitted: 0,
            top_heart_emitted: 0,
        }
    }

    /// 모든 phase 완료 여부
    fn all_phases_done(&self) -> bool {
        self.explosion_emitted >= EXPLOSION_TOTAL
            && self.column_emitted >= COLUMN_TOTAL
            && self.top_heart_emitted >= TOP_HEART_TOTAL
    }

    fn scaled_emit_count(remaining: usize, particles_per_emit: usize, dt_scale: f32) -> usize {
        if remaining == 0 {
            return 0;
        }

        let mut max_emit = ((particles_per_emit as f32) * dt_scale).round() as usize;
        if max_emit == 0 {
            max_emit = 1;
        }

        remaining.min(max_emit)
    }

    fn emit_phase<F>(
        emitted: &mut usize,
        total: usize,
        particles_per_emit: usize,
        dt_scale: f32,
        particles: &mut Vec<crate::game_state::field_particle::FieldParticle>,
        mut spawn: F,
    ) where
        F: FnMut(usize) -> crate::game_state::field_particle::FieldParticle,
    {
        if *emitted >= total {
            return;
        }

        let remaining = total - *emitted;
        let emit_count = Self::scaled_emit_count(remaining, particles_per_emit, dt_scale);

        for i in 0..emit_count {
            particles.push(spawn(*emitted + i));
        }
        *emitted += emit_count;
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
        if self.all_phases_done() {
            return vec![];
        }

        let mut rng = rand::thread_rng();
        let dt_scale = (dt.as_secs_f32() / (1.0 / 60.0)).max(0.5);
        let capacity = Self::scaled_emit_count(
            TOP_HEART_TOTAL.saturating_sub(self.top_heart_emitted),
            TOP_HEART_PARTICLES_PER_EMIT,
            dt_scale,
        ) + Self::scaled_emit_count(
            EXPLOSION_TOTAL.saturating_sub(self.explosion_emitted),
            EXPLOSION_PARTICLES_PER_EMIT,
            dt_scale,
        ) + Self::scaled_emit_count(
            COLUMN_TOTAL.saturating_sub(self.column_emitted),
            COLUMN_PARTICLES_PER_EMIT,
            dt_scale,
        );
        let mut particles = Vec::with_capacity(capacity);

        // === TOP HEART ===
        Self::emit_phase(
            &mut self.top_heart_emitted,
            TOP_HEART_TOTAL,
            TOP_HEART_PARTICLES_PER_EMIT,
            dt_scale,
            &mut particles,
            |emitted_index| crate::game_state::field_particle::FieldParticle::Heart {
                particle: HeartParticle::new_rising_heart(
                    (self.xy.x, self.xy.y),
                    now,
                    emitted_index as f32,
                    &mut rng,
                ),
            },
        );

        // === EXPLOSION ===
        let explosion_xy = (self.xy.x, self.xy.y + EXPLOSION_Y_OFFSET);
        Self::emit_phase(
            &mut self.explosion_emitted,
            EXPLOSION_TOTAL,
            EXPLOSION_PARTICLES_PER_EMIT,
            dt_scale,
            &mut particles,
            |_| crate::game_state::field_particle::FieldParticle::Heart {
                particle: HeartParticle::new_mushroom_explosion(explosion_xy, now, &mut rng),
            },
        );

        // === COLUMN ===
        let column_start_xy = (self.xy.x, self.xy.y + EXPLOSION_Y_OFFSET);
        let column_end_xy = (self.xy.x, self.xy.y + COLUMN_TOP_Y_OFFSET); // 기존 대비 2배 높이
        Self::emit_phase(
            &mut self.column_emitted,
            COLUMN_TOTAL,
            COLUMN_PARTICLES_PER_EMIT,
            dt_scale,
            &mut particles,
            |_| crate::game_state::field_particle::FieldParticle::Heart {
                particle: HeartParticle::new_mushroom_column(
                    column_start_xy,
                    column_end_xy,
                    now,
                    &mut rng,
                ),
            },
        );

        particles
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.all_phases_done()
    }
}
