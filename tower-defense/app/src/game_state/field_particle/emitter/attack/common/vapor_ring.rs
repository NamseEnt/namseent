use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common::VaporRing},
};
use namui::*;
use rand::Rng;

const SPACING_TILES: f32 = 1.5;
const OFFSET_JITTER_TILES: f32 = 0.25;

pub(in crate::game_state::field_particle::emitter::attack) fn spawn(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
) {
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let path_length_tiles = delta_x.hypot(delta_y);
    let angle = (delta_y * crate::game_state::TILE_PX_SIZE.height.as_f32())
        .atan2(delta_x * crate::game_state::TILE_PX_SIZE.width.as_f32());
    let rotation = (angle.cos(), angle.sin());
    let step_count = (path_length_tiles / SPACING_TILES).ceil() as usize;
    let mut rng = rand::thread_rng();

    for index in 0..=step_count {
        let progress = if step_count == 0 {
            0.0
        } else {
            index as f32 / step_count as f32
        };
        let expected_distance = path_length_tiles * progress;
        let offset = if index == 0 || index == step_count {
            0.0
        } else {
            rng.gen_range(-OFFSET_JITTER_TILES..=OFFSET_JITTER_TILES)
        };
        let distance = (expected_distance + offset).clamp(0.0, path_length_tiles);
        let path_progress = if path_length_tiles <= f32::EPSILON {
            0.0
        } else {
            distance / path_length_tiles
        };
        let position_xy = (
            start_xy.0 + delta_x * path_progress,
            start_xy.1 + delta_y * path_progress,
        );
        let delay = delay_offset + delay_step * index as i32;
        let opacity = 1.0 - path_progress;
        ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
            VaporRing::new(position_xy, created_at, delay, scale, rotation, opacity)
                .with_movement(rotation, 64.0),
        ));
    }
}
