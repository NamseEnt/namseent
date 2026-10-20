use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common::BulletTrail},
};
use namui::*;

const SPACING_TILES: f32 = 0.8;

pub(super) fn spawn(
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
    for index in 0..=step_count {
        let progress = if step_count == 0 {
            0.0
        } else {
            index as f32 / step_count as f32
        };
        let position_xy = (
            start_xy.0 + delta_x * progress,
            start_xy.1 + delta_y * progress,
        );
        let delay = delay_offset + delay_step * index as i32;
        ATTACK_VISUAL.spawn(NewAttackParticle::BulletTrail(BulletTrail::new(
            position_xy,
            rotation.0,
            rotation.1,
            created_at,
            delay,
            scale,
            1.0 - progress,
        )));
    }
}
