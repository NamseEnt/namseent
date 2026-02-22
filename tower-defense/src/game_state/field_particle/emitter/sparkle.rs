use crate::MapCoordF32;
use crate::game_state::field_particle::{
    EMBER_SPARKS, EmberSparkParticle, SPARKLES, SparkleParticle,
};
use namui::*;
use rand::Rng;

pub const SPARKLE_SPAWN_DISTANCE: f32 = 0.25;
const EMBER_SPARK_SPAWN_CHANCE: f32 = 0.15;

pub fn spawn_sparkle_trail(
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

        SPARKLES.spawn(SparkleParticle::new_with_random(px, now, &mut rng));

        if rng.gen_range(0.0..1.0) < EMBER_SPARK_SPAWN_CHANCE {
            EMBER_SPARKS.spawn(EmberSparkParticle::new_with_random(
                px,
                movement_dir,
                now,
                &mut rng,
            ));
        }
    }
}
