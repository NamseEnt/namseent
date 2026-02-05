use crate::MapCoordF32;
use crate::game_state::field_particle::{EaseMode, TrashParticle};
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const TRASH_BURST_COUNT: usize = 8;
const TRASH_BURST_DURATION_MIN_MS: i64 = 200;
const TRASH_BURST_DURATION_MAX_MS: i64 = 800;
const BURST_HEIGHT_MIN: f32 = 1.2;
const BURST_HEIGHT_MAX: f32 = 3.0;

#[derive(Clone, State)]
pub struct TrashBurstEmitter {
    pub tower_xy: MapCoordF32,
    pub created_at: Instant,
    pub emitted: bool,
}

impl TrashBurstEmitter {
    pub fn new(tower_xy: MapCoordF32, created_at: Instant) -> Self {
        Self {
            tower_xy,
            created_at,
            emitted: false,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for TrashBurstEmitter
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
        let mut out = Vec::with_capacity(TRASH_BURST_COUNT);

        for _ in 0..TRASH_BURST_COUNT {
            let kind = ProjectileKind::random_trash();
            // start: tower base with small horizontal random offset within +/-0.4 tiles
            let start = (self.tower_xy.x + rng.gen_range(0.2..0.8), self.tower_xy.y);
            // end: up by randomized burst height with small horizontal offset
            let height = rng.gen_range(BURST_HEIGHT_MIN..BURST_HEIGHT_MAX);
            let end = (start.0 + rng.gen_range(-0.2..0.2), start.1 - height);

            let dur_ms = rng.gen_range(TRASH_BURST_DURATION_MIN_MS..=TRASH_BURST_DURATION_MAX_MS);
            let duration = Duration::from_millis(dur_ms);

            let cfg = crate::game_state::field_particle::particle::TrashParticleConfig {
                kind,
                start_xy: start,
                end_xy: end,
                created_at: now,
                duration,
                ease_mode: EaseMode::EaseOutCubic,
                should_bounce: false,
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
