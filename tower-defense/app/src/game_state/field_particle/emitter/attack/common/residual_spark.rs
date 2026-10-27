use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common::ResidualSpark},
};
use namui::*;
use rand::Rng;

pub(in crate::game_state::field_particle::emitter::attack) fn spawn_along_line(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    color: [f32; 3],
    created_at: Instant,
    sparks_per_tile: f32,
    spread_width_tiles: f32,
    base_scale_multiplier: f32,
) {
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let length_tiles = delta_x.hypot(delta_y);
    if length_tiles <= f32::EPSILON {
        return;
    }

    let direction = (delta_x / length_tiles, delta_y / length_tiles);
    let perpendicular = (-direction.1, direction.0);
    let spark_count = (length_tiles * sparks_per_tile).ceil().max(1.0) as usize;
    let spacing = length_tiles / spark_count as f32;
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let lateral_offset_limit = spread_width_tiles.max(0.0) / 2.0;
    let attack_angle =
        (delta_y * tile_size.height.as_f32()).atan2(delta_x * tile_size.width.as_f32());
    let mut rng = rand::thread_rng();

    for spark_index in 0..spark_count {
        let nominal_distance = (spark_index as f32 + 0.5) * spacing;
        let distance = (nominal_distance + rng.gen_range(-1.0..=1.0)).clamp(0.0, length_tiles);
        let lateral_offset = rng.gen_range(-lateral_offset_limit..=lateral_offset_limit);
        let center_xy = (
            start_xy.0 + direction.0 * distance + perpendicular.0 * lateral_offset,
            start_xy.1 + direction.1 * distance + perpendicular.1 * lateral_offset,
        );
        ATTACK_VISUAL.spawn(NewAttackParticle::ResidualSpark(
            ResidualSpark::new(center_xy, attack_angle, color, created_at)
                .with_base_scale_multiplier(base_scale_multiplier),
        ));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn spark_count_tracks_requested_density() {
        let length_tiles = 10.0_f32;

        assert_eq!((length_tiles * 1.0_f32).ceil() as usize, 10);
        assert_eq!((length_tiles * 0.5_f32).ceil() as usize, 5);
    }

    #[test]
    fn lateral_offset_stays_within_half_of_the_requested_total_width() {
        let spread_width_tiles = 0.5_f32;
        let lateral_offset_limit = spread_width_tiles / 2.0;

        assert_eq!(lateral_offset_limit, 0.25);
        assert_eq!(lateral_offset_limit * 2.0, spread_width_tiles);
    }
}
