use crate::MapCoordF32;
use crate::game_state::field_particle::HEARTS;
use crate::game_state::field_particle::particle::HeartParticle;
use namui::*;

const EXPLOSION_Y_OFFSET: f32 = 0.5;
const COLUMN_TOP_Y_OFFSET: f32 = -0.125;
const EXPLOSION_TOTAL: usize = 16;
const COLUMN_TOTAL: usize = 48;

pub fn spawn_heart_burst(xy: MapCoordF32, now: Instant) {
    let mut rng = rand::thread_rng();

    HEARTS.spawn(HeartParticle::new_rising_heart(
        (xy.x, xy.y),
        now,
        0.0,
        &mut rng,
    ));

    let explosion_xy = (xy.x, xy.y + EXPLOSION_Y_OFFSET);
    for _ in 0..EXPLOSION_TOTAL {
        HEARTS.spawn(HeartParticle::new_mushroom_explosion(
            explosion_xy,
            now,
            &mut rng,
        ));
    }

    let column_start_xy = (xy.x, xy.y + EXPLOSION_Y_OFFSET);
    let column_end_xy = (xy.x, xy.y + COLUMN_TOP_Y_OFFSET);
    for _ in 0..COLUMN_TOTAL {
        HEARTS.spawn(HeartParticle::new_mushroom_column(
            column_start_xy,
            column_end_xy,
            now,
            &mut rng,
        ));
    }
}
