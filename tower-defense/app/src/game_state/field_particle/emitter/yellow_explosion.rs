use crate::game_state::field_particle::{YellowExplosionParticle, spawn_yellow_explosion};
use namui::*;

const YELLOW_EXPLOSION_BURST_COUNT: usize = 5;
const YELLOW_EXPLOSION_SIZE_MIN_TILE: f32 = 0.8;
const YELLOW_EXPLOSION_SIZE_MAX_TILE: f32 = 2.0;

pub fn spawn_yellow_explosion_burst(target_xy: (f32, f32), now: Instant) {
    for i in 0..YELLOW_EXPLOSION_BURST_COUNT {
        let size = YELLOW_EXPLOSION_SIZE_MIN_TILE
            + (YELLOW_EXPLOSION_SIZE_MAX_TILE - YELLOW_EXPLOSION_SIZE_MIN_TILE) * (i as f32)
                / (YELLOW_EXPLOSION_BURST_COUNT - 1) as f32;
        spawn_yellow_explosion(YellowExplosionParticle::new(target_xy, now, size));
    }
}
