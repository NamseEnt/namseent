use crate::MapCoordF32;
use crate::game_state::field_particle::{EaseMode, TrashParticle};
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const TRASH_RAIN_COUNT: usize = 6;

// Start X offset from target: start_x = target_x + random in [START_X_MIN, START_X_MAX]
const START_X_MIN: f32 = -0.4;
const START_X_MAX: f32 = 1.4;

// End X offset from target: end_x = target_x + random in [END_X_MIN, END_X_MAX]
const END_X_MIN: f32 = 0.1;
const END_X_MAX: f32 = 0.9;

// Rain start height above target (tiles): start_y = target_y - random in [RAIN_HEIGHT_MIN, RAIN_HEIGHT_MAX]
const RAIN_HEIGHT_MIN: f32 = 2.5;
const RAIN_HEIGHT_MAX: f32 = 5.0;

// Duration per particle in milliseconds (randomized)
const TRASH_RAIN_DURATION_MIN_MS: i64 = 120;
const TRASH_RAIN_DURATION_MAX_MS: i64 = 240;

#[derive(Clone, State)]
pub struct TrashRainEmitter {
    pub target_xy: MapCoordF32,
    pub created_at: Instant,
    pub emitted: bool,
}

impl TrashRainEmitter {
    pub fn new(target_xy: MapCoordF32, created_at: Instant) -> Self {
        Self {
            target_xy,
            created_at,
            emitted: false,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for TrashRainEmitter
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
        let mut out = Vec::with_capacity(TRASH_RAIN_COUNT);

        for _ in 0..TRASH_RAIN_COUNT {
            let kind = ProjectileKind::random_trash();
            let start_x = self.target_xy.x + rng.gen_range(START_X_MIN..START_X_MAX);
            let start_h = rng.gen_range(RAIN_HEIGHT_MIN..RAIN_HEIGHT_MAX);
            let target_y = self.target_xy.y + 1.0; // target at bottom of tile
            let start = (start_x, target_y - start_h);

            let end_x = self.target_xy.x + rng.gen_range(END_X_MIN..END_X_MAX);
            let end = (end_x, target_y);

            let duration_ms =
                rng.gen_range(TRASH_RAIN_DURATION_MIN_MS..=TRASH_RAIN_DURATION_MAX_MS);
            let duration = Duration::from_millis(duration_ms);

            let cfg = crate::game_state::field_particle::particle::TrashParticleConfig {
                kind,
                start_xy: start,
                end_xy: end,
                created_at: now,
                duration,
                ease_mode: EaseMode::Linear,
                should_bounce: true,
                gravity: 0.0,
            };
            let particle = TrashParticle::new_with_random_end(cfg);
            out.push(crate::game_state::field_particle::FieldParticle::Trash { particle });
        }

        out
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}
