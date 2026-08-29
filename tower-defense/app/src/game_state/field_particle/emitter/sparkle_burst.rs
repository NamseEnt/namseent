use crate::MapCoordF32;
use crate::game_state::field_particle::{SparkleParticle, spawn_sparkle};
use namui::*;

const SPARKLE_BURST_COUNT: usize = 64;
const BURST_VELOCITY_RANGE: f32 = 2.5;

pub fn spawn_sparkle_burst(xy: MapCoordF32, now: Instant) {
    let mut rng = rand::thread_rng();

    for _ in 0..SPARKLE_BURST_COUNT {
        spawn_sparkle(SparkleParticle::new_with_random_velocity(
            (xy.x, xy.y),
            now,
            &mut rng,
            BURST_VELOCITY_RANGE,
        ));
    }
}
