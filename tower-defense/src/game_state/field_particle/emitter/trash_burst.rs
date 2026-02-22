use crate::MapCoordF32;
use crate::game_state::field_particle::particle::TrashParticleConfig;
use crate::game_state::field_particle::{EaseMode, TRASHES, TrashParticle};
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const TRASH_BURST_COUNT: usize = 8;
const TRASH_BURST_DURATION_MIN_MS: i64 = 200;
const TRASH_BURST_DURATION_MAX_MS: i64 = 800;
const BURST_HEIGHT_MIN: f32 = 1.2;
const BURST_HEIGHT_MAX: f32 = 3.0;
const ROTATION_SPEED_MIN_DEG: f32 = -360.0;
const ROTATION_SPEED_MAX_DEG: f32 = 360.0;

pub fn spawn_trash_burst(tower_xy: MapCoordF32, now: Instant) {
    let mut rng = rand::thread_rng();

    for _ in 0..TRASH_BURST_COUNT {
        let kind = ProjectileKind::random_trash();
        let start = (tower_xy.x + rng.gen_range(0.2..0.8), tower_xy.y);
        let height = rng.gen_range(BURST_HEIGHT_MIN..BURST_HEIGHT_MAX);
        let end = (start.0 + rng.gen_range(-0.2..0.2), start.1 - height);

        let dur_ms = rng.gen_range(TRASH_BURST_DURATION_MIN_MS..=TRASH_BURST_DURATION_MAX_MS);
        let duration = Duration::from_millis(dur_ms);

        let cfg = TrashParticleConfig {
            kind,
            start_xy: start,
            end_xy: end,
            created_at: now,
            duration,
            ease_mode: EaseMode::EaseOutCubic,
            should_bounce: false,
            gravity: 0.0,
            rotation_speed_deg_per_sec: (ROTATION_SPEED_MIN_DEG, ROTATION_SPEED_MAX_DEG),
        };
        let particle = TrashParticle::new_with_random_end(cfg);
        TRASHES.spawn(particle);
    }
}
