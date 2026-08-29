use crate::MapCoordF32;
use crate::game_state::field_particle::HEARTS;
use crate::game_state::field_particle::particle::HeartParticle;
use namui::*;

pub const HEART_SPAWN_DISTANCE: f32 = 1.5;

pub fn spawn_heart_trail(
    from_xy: MapCoordF32,
    to_xy: MapCoordF32,
    total_particles: usize,
    now: Instant,
) {
    if total_particles == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let movement_vec = to_xy - from_xy;
    let movement_len = movement_vec.length();
    let movement_dir = if movement_len > 0.0 {
        (movement_vec.x / movement_len, movement_vec.y / movement_len)
    } else {
        (1.0, 0.0)
    };

    for i in 0..total_particles {
        let progress = (i as f32 + 1.0) / (total_particles as f32 + 1.0);
        let particle_xy = from_xy + (to_xy - from_xy) * progress;
        let px = (particle_xy.x, particle_xy.y);
        HEARTS.spawn(HeartParticle::new_trail(px, now, movement_dir, &mut rng));
    }
}
