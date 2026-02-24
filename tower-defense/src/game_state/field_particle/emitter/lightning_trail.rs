use crate::MapCoordF32;
use crate::game_state::field_particle::{LIGHTNING_BOLTS, LightningBoltParticle};
use namui::*;
use rand::Rng;

pub const LIGHTNING_TRAIL_SPAWN_DISTANCE: f32 = 0.3;
const LIGHTNING_SPAWN_CHANCE: f32 = 0.6;

pub fn spawn_lightning_trail(
    from_xy: MapCoordF32,
    to_xy: MapCoordF32,
    total_particles: usize,
    now: Instant,
) {
    if total_particles == 0 {
        return;
    }

    let movement_vec = to_xy - from_xy;
    let movement_len = movement_vec.length();
    if movement_len < 0.001 {
        return;
    }

    let mut rng = rand::thread_rng();
    let movement_dir_x = movement_vec.x / movement_len;
    let movement_dir_y = movement_vec.y / movement_len;
    let perp_x = -movement_dir_y;
    let perp_y = movement_dir_x;

    for _ in 0..total_particles {
        let t = rng.gen_range(0.0..1.0);
        let start_pos = MapCoordF32 {
            x: from_xy.x + movement_vec.x * t,
            y: from_xy.y + movement_vec.y * t,
        };

        let end_t = rng.gen_range(0.6..1.0);
        let mut end_pos = MapCoordF32 {
            x: from_xy.x + movement_vec.x * end_t,
            y: from_xy.y + movement_vec.y * end_t,
        };

        let angle_offset = rng.gen_range(-0.3..0.3);
        end_pos.x += perp_x * angle_offset;
        end_pos.y += perp_y * angle_offset;

        let lifetime = Duration::from_millis(rng.gen_range(50..80));

        LIGHTNING_BOLTS.spawn(LightningBoltParticle::new(
            (start_pos.x, start_pos.y),
            (end_pos.x, end_pos.y),
            now,
            lifetime,
            LIGHTNING_SPAWN_CHANCE,
        ));
    }
}
