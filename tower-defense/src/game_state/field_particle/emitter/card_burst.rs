use crate::MapCoordF32;
use crate::game_state::field_particle::particle::{
    CardKind, CardParticle, CardParticleConfig, EaseMode,
};
use crate::game_state::field_particle::CARDS;
use namui::*;
use rand::Rng;

const CARD_BURST_COUNT: usize = 8;
const CARD_BURST_DURATION_MIN_MS: i64 = 300;
const CARD_BURST_DURATION_MAX_MS: i64 = 800;
const BURST_DISTANCE_MIN: f32 = 0.8;
const BURST_DISTANCE_MAX: f32 = 2.0;
const ROTATION_SPEED_MIN_DEG: f32 = -540.0;
const ROTATION_SPEED_MAX_DEG: f32 = 540.0;
const GRAVITY: f32 = 2.0;

pub fn spawn_card_burst(impact_xy: MapCoordF32, now: Instant) {
    let mut rng = rand::thread_rng();

    for _ in 0..CARD_BURST_COUNT {
        let kind = CardKind::random();
        let start = (
            impact_xy.x + rng.gen_range(-0.1..0.1),
            impact_xy.y + rng.gen_range(-0.1..0.1),
        );
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

        CARDS.spawn(CardParticle::new_with_random_burst(cfg));
    }
}
