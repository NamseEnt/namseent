use crate::game_state::field_particle::{YellowExplosionParticle, spawn_yellow_explosion};
use namui::*;

const YELLOW_EXPLOSION_BURST_COUNT: usize = 5;

pub fn spawn_yellow_explosion_burst(target_xy: (f32, f32), now: Instant) {
    for i in 0..YELLOW_EXPLOSION_BURST_COUNT {
        let mut p = YellowExplosionParticle::new(target_xy, now);
        p.size_max_tile = 0.8 + 1.2 * (i as f32) / (YELLOW_EXPLOSION_BURST_COUNT - 1) as f32;
        spawn_yellow_explosion(p);
    }
}
