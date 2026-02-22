use crate::MapCoordF32;
use crate::game_state::field_particle::particle::TrashParticleConfig;
use crate::game_state::field_particle::{EaseMode, TRASHES, TrashParticle};
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const TRASH_RAIN_COUNT: usize = 6;
const START_X_MIN: f32 = -0.4;
const START_X_MAX: f32 = 1.4;
const END_X_MIN: f32 = 0.1;
const END_X_MAX: f32 = 0.9;
const RAIN_HEIGHT_MIN: f32 = 2.5;
const RAIN_HEIGHT_MAX: f32 = 5.0;
const ROTATION_SPEED_MIN_DEG: f32 = -360.0;
const ROTATION_SPEED_MAX_DEG: f32 = 360.0;
const TRASH_RAIN_DURATION_MIN_MS: i64 = 120;
const TRASH_RAIN_DURATION_MAX_MS: i64 = 240;

pub fn spawn_trash_rain(target_xy: MapCoordF32, now: Instant) {
    let mut rng = rand::thread_rng();

    for _ in 0..TRASH_RAIN_COUNT {
        let kind = ProjectileKind::random_trash();
        let start_x = target_xy.x + rng.gen_range(START_X_MIN..START_X_MAX);
        let start_h = rng.gen_range(RAIN_HEIGHT_MIN..RAIN_HEIGHT_MAX);
        let target_y = target_xy.y + 1.0;
        let start = (start_x, target_y - start_h);

        let end_x = target_xy.x + rng.gen_range(END_X_MIN..END_X_MAX);
        let end = (end_x, target_y);

        let duration_ms = rng.gen_range(TRASH_RAIN_DURATION_MIN_MS..=TRASH_RAIN_DURATION_MAX_MS);
        let duration = Duration::from_millis(duration_ms);

        let cfg = TrashParticleConfig {
            kind,
            start_xy: start,
            end_xy: end,
            created_at: now,
            duration,
            ease_mode: EaseMode::Linear,
            should_bounce: true,
            gravity: 0.0,
            rotation_speed_deg_per_sec: (ROTATION_SPEED_MIN_DEG, ROTATION_SPEED_MAX_DEG),
        };
        let particle = TrashParticle::new_with_random_end(cfg);
        TRASHES.spawn(particle);
    }
}
