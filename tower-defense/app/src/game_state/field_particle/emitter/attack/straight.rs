use super::common::{ember, energy_ring, muzzle_flash, residual_spark};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, straight_lightning},
};
use namui::*;
use rand::Rng;

const SEGMENT_LENGTH_TILES: f32 = 1.0;
const SEGMENT_SPACING_TILES: f32 = 0.95;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let direction_x = end_xy.0 - start_xy.0;
    let direction_y = end_xy.1 - start_xy.1;
    let length = direction_x.hypot(direction_y);
    if length <= f32::EPSILON {
        return;
    }

    let direction = (direction_x / length, direction_y / length);
    let angle = direction_y.atan2(direction_x);
    muzzle_flash::spawn_fan(
        start_xy,
        angle,
        straight_lightning::LIGHTNING_COLOR,
        created_at,
    );
    energy_ring::spawn(
        start_xy,
        end_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(10),
        1.5,
    );
    residual_spark::spawn_along_line(
        start_xy,
        end_xy,
        straight_lightning::LIGHTNING_COLOR,
        created_at,
        0.5,
        0.5,
        0.5,
    );
    let mut rng = rand::thread_rng();
    let segment_count = (length / SEGMENT_SPACING_TILES).ceil() as usize;
    for index in 0..segment_count {
        let segment_start = index as f32 * SEGMENT_SPACING_TILES;
        let segment_length = SEGMENT_LENGTH_TILES.min(length - segment_start);
        let center_distance = segment_start + segment_length / 2.0;
        let center_xy = (
            start_xy.0 + direction.0 * center_distance,
            start_xy.1 + direction.1 * center_distance,
        );
        let variant = rng.gen_range(0..3);
        let flip_vertical = rng.gen_bool(0.5);
        ATTACK_VISUAL.spawn(NewAttackParticle::StraightLightning(
            straight_lightning::StraightLightning::new(
                center_xy,
                angle,
                segment_length,
                variant,
                flip_vertical,
                created_at,
                Duration::from_millis((index * 5) as i64),
            ),
        ));
    }

    ember::spawn_line(
        start_xy,
        end_xy,
        straight_lightning::LIGHTNING_COLOR,
        created_at,
    );
}

pub(super) fn spawn_hit_effect(start_xy: (f32, f32), impact_xy: (f32, f32), created_at: Instant) {
    let direction_x = impact_xy.0 - start_xy.0;
    let direction_y = impact_xy.1 - start_xy.1;
    let angle = if direction_x.hypot(direction_y) > f32::EPSILON {
        direction_y.atan2(direction_x)
    } else {
        -std::f32::consts::FRAC_PI_2
    };
    muzzle_flash::spawn_directional(
        impact_xy,
        angle,
        2.0,
        straight_lightning::LIGHTNING_COLOR,
        created_at,
    );
}
