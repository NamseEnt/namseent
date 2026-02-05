use crate::game_state::field_particle::TrashParticle;
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const BOUNCE_COUNT: usize = 1;
const BOUNCE_DISTANCE_MIN: f32 = 3.5;
const BOUNCE_DISTANCE_MAX: f32 = 4.5;
const BOUNCE_DURATION_MIN_MS: i64 = 360;
const BOUNCE_DURATION_MAX_MS: i64 = 720;

#[derive(Clone, State)]
pub struct TrashBounceEmitter {
    pub kind: ProjectileKind,
    pub orig_start: (f32, f32),
    pub orig_end: (f32, f32),
    pub created_at: Instant,
    pub emitted: bool,
}

impl TrashBounceEmitter {
    pub fn new(
        kind: ProjectileKind,
        orig_start: (f32, f32),
        orig_end: (f32, f32),
        created_at: Instant,
    ) -> Self {
        Self {
            kind,
            orig_start,
            orig_end,
            created_at,
            emitted: false,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for TrashBounceEmitter
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
        let mut out = Vec::with_capacity(BOUNCE_COUNT);

        // original direction = orig_end - orig_start
        let dir_x = self.orig_end.0 - self.orig_start.0;
        let dir_y = self.orig_end.1 - self.orig_start.1;
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.0001);
        let nx = dir_x / len;
        let ny = dir_y / len;

        for _ in 0..BOUNCE_COUNT {
            // bounce direction roughly opposite original with small random offset
            let rand_off_x = rng.gen_range(-0.5..0.5);
            let rand_off_y = rng.gen_range(-0.2..0.2);
            let bounce_dir_x = -nx + rand_off_x;
            let bounce_dir_y = -ny + rand_off_y;
            let b_len = (bounce_dir_x * bounce_dir_x + bounce_dir_y * bounce_dir_y)
                .sqrt()
                .max(0.0001);
            let bx = bounce_dir_x / b_len;
            let by = bounce_dir_y / b_len;

            let dist = rng.gen_range(BOUNCE_DISTANCE_MIN..BOUNCE_DISTANCE_MAX);
            let start = (self.orig_end.0, self.orig_end.1);
            let end = (
                start.0 + bx * dist + rng.gen_range(-0.15..0.15),
                start.1 + by * dist + rng.gen_range(-0.08..0.08),
            );

            let dur_ms = rng.gen_range(BOUNCE_DURATION_MIN_MS..=BOUNCE_DURATION_MAX_MS);
            let duration = Duration::from_millis(dur_ms);

            let gravity = rng.gen_range(8.0..12.0);
            let cfg = crate::game_state::field_particle::particle::TrashParticleConfig {
                kind: self.kind,
                start_xy: start,
                end_xy: end,
                created_at: now,
                duration,
                ease_mode: crate::game_state::field_particle::EaseMode::EaseOutCubic,
                should_bounce: false,
                gravity,
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
