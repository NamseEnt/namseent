use crate::MapCoordF32;
use crate::game_state::field_particle::particle::{
    CardKind, CardParticle, CardParticleConfig, EaseMode,
};
use namui::*;
use rand::Rng;

const CARD_BURST_COUNT: usize = 8;
const CARD_BURST_DURATION_MIN_MS: i64 = 300;
const CARD_BURST_DURATION_MAX_MS: i64 = 800;
const BURST_DISTANCE_MIN: f32 = 0.8;
const BURST_DISTANCE_MAX: f32 = 2.0;
const ROTATION_SPEED_MIN_DEG: f32 = -540.0;
const ROTATION_SPEED_MAX_DEG: f32 = 540.0;
const GRAVITY: f32 = 2.0; // tiles per second^2

#[derive(Clone, State)]
pub struct CardBurstEmitter {
    pub impact_xy: MapCoordF32,
    pub created_at: Instant,
    pub emitted: bool,
}

impl CardBurstEmitter {
    pub fn new(impact_xy: MapCoordF32, created_at: Instant) -> Self {
        Self {
            impact_xy,
            created_at,
            emitted: false,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for CardBurstEmitter
{
    fn emit(
        &mut self,
        now: Instant,
        _dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted {
            return vec![];
        }

        self.emitted = true;
        let mut rng = rand::thread_rng();
        let mut out = Vec::with_capacity(CARD_BURST_COUNT);

        for _ in 0..CARD_BURST_COUNT {
            let kind = CardKind::random();

            // Start position: impact point with small random offset
            let start = (
                self.impact_xy.x + rng.gen_range(-0.1..0.1),
                self.impact_xy.y + rng.gen_range(-0.1..0.1),
            );

            // End position: random direction in all directions (360 degrees)
            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let distance = rng.gen_range(BURST_DISTANCE_MIN..BURST_DISTANCE_MAX);
            let end = (
                start.0 + angle.cos() * distance,
                start.1 + angle.sin() * distance,
            );

            let dur_ms = rng.gen_range(CARD_BURST_DURATION_MIN_MS..=CARD_BURST_DURATION_MAX_MS);
            let duration = Duration::from_millis(dur_ms);

            let cfg = CardParticleConfig {
                kind,
                start_xy: start,
                end_xy: end,
                created_at: now,
                duration,
                ease_mode: EaseMode::EaseOutCubic,
                gravity: GRAVITY,
                rotation_speed_deg_per_sec: (ROTATION_SPEED_MIN_DEG, ROTATION_SPEED_MAX_DEG),
            };

            let particle = CardParticle::new_with_random_burst(cfg);
            out.push(crate::game_state::field_particle::FieldParticle::Card { particle });
        }

        out
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
