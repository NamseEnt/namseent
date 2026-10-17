use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{HighProjectile, HighTrail, NewAttackParticle},
};
use namui::*;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let delta_x_px = delta_x * tile_size.width.as_f32();
    let delta_y_px = delta_y * tile_size.height.as_f32();
    let length_px = delta_x_px.hypot(delta_y_px);
    let angle = delta_y_px.atan2(delta_x_px);
    let rotation_cos = angle.cos();
    let rotation_sin = angle.sin();
    let trail_width_px = crate::atlas::generated::ATTACK_HIGH_TRAIL[0]
        .width()
        .as_f32();
    let spacing_px = trail_width_px * HighTrail::INITIAL_SCALE * HighTrail::SPACING_RATIO;
    let trail_count = if length_px <= f32::EPSILON {
        1
    } else {
        (length_px / spacing_px).ceil() as usize + 1
    };

    ATTACK_VISUAL.spawn(NewAttackParticle::HighProjectile(HighProjectile::new(
        start_xy, end_xy, created_at,
    )));

    for index in 0..trail_count {
        let progress = if trail_count == 1 {
            0.0
        } else {
            index as f32 / (trail_count - 1) as f32
        };
        let delay = Duration::from_millis(index as i64 * 10);
        ATTACK_VISUAL.spawn(NewAttackParticle::HighTrail(HighTrail::new(
            start_xy,
            length_px,
            progress,
            rotation_cos,
            rotation_sin,
            created_at,
            delay,
        )));
    }
}
