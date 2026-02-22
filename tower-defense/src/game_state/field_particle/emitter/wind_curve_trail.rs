use crate::MapCoordF32;
use crate::game_state::field_particle::{
    BLUE_DOT_SPARKS, BlueDotSparkParticle, WIND_CURVE_TRAILS, WindCurveTrailParticle,
};
use namui::*;
use rand::Rng;

pub const WIND_CURVE_SPAWN_DISTANCE: f32 = 0.12;
const BLUE_DOT_SPAWN_CHANCE: f32 = 0.22;

pub fn spawn_wind_curve_trail(
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
    let movement_direction = if movement_len > 0.0 {
        (movement_vec.x / movement_len, movement_vec.y / movement_len)
    } else {
        (0.0, -1.0)
    };

    let mut rng = rand::thread_rng();

    for i in 0..total_particles {
        let progress = (i as f32 + 1.0) / (total_particles as f32 + 1.0);
        let particle_xy = from_xy + (to_xy - from_xy) * progress;
        let px = (particle_xy.x, particle_xy.y);

        WIND_CURVE_TRAILS.spawn(WindCurveTrailParticle::new_with_random(
            px,
            movement_direction,
            now,
            &mut rng,
        ));

        if rng.gen_range(0.0..1.0) < BLUE_DOT_SPAWN_CHANCE {
            BLUE_DOT_SPARKS.spawn(BlueDotSparkParticle::new_with_random(
                px,
                movement_direction,
                now,
                &mut rng,
            ));
        }
    }
}
