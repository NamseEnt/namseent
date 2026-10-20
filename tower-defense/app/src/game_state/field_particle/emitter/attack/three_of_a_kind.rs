use super::common::{FLAME_START_COLOR, ember, muzzle_flash};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common},
};
use namui::*;
use rand::Rng;

const ATTACK_CONE_HALF_ANGLE: f32 = std::f32::consts::PI / 12.0;
const SMOKE_COUNT_AT_REFERENCE_RADIUS: f32 = 24.0;
const REFERENCE_RADIUS_TILES: f32 = 7.0;
const MAX_SMOKE_DELAY_MS: f32 = 15.0;
const MAX_SMOKE_END_SCALE: f32 = 4.0;
const SMOKE_RANGE_OVERDRAW_TILES: f32 = 2.0;
const MUZZLE_COLOR: [f32; 3] = [1.0, 0.75, 0.25];

pub(super) fn spawn(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    attack_range_radius: f32,
    created_at: Instant,
) {
    let mut rng = rand::thread_rng();
    let direction_x = end_xy.0 - start_xy.0;
    let direction_y = end_xy.1 - start_xy.1;
    let direction_length = direction_x.hypot(direction_y);
    let aim_angle = if direction_length > 0.0001 {
        direction_y.atan2(direction_x)
    } else {
        -std::f32::consts::FRAC_PI_2
    };

    muzzle_flash::spawn_fan(start_xy, aim_angle, MUZZLE_COLOR, created_at);
    ember::spawn_cone(
        start_xy,
        aim_angle,
        ATTACK_CONE_HALF_ANGLE,
        attack_range_radius,
        MUZZLE_COLOR,
        created_at,
    );

    let available_radius = (attack_range_radius - 0.25).max(0.0);
    let max_end_scale = MAX_SMOKE_END_SCALE.min(available_radius);
    let max_distance = (available_radius - max_end_scale + SMOKE_RANGE_OVERDRAW_TILES).max(0.0);
    let smoke_count = (attack_range_radius.max(0.0) * SMOKE_COUNT_AT_REFERENCE_RADIUS
        / REFERENCE_RADIUS_TILES)
        .ceil() as usize;
    for _ in 0..smoke_count {
        let angle = aim_angle + rng.gen_range(-ATTACK_CONE_HALF_ANGLE..=ATTACK_CONE_HALF_ANGLE);
        let direction = (angle.cos(), angle.sin());
        let offset_distance = if max_distance > 0.0 {
            rng.gen_range(0.0..=max_distance)
        } else {
            0.0
        };
        let final_distance = if max_distance > offset_distance {
            rng.gen_range(offset_distance..=max_distance)
        } else {
            offset_distance
        };
        let distance_ratio = final_distance / max_distance.max(0.001);
        let delay = Duration::from_millis(
            (MAX_SMOKE_DELAY_MS * offset_distance / max_distance.max(0.001)).round() as i64,
        );
        let smoke_start_xy = (
            start_xy.0 + direction.0 * offset_distance,
            start_xy.1 + direction.1 * offset_distance,
        );
        let end_scale =
            (max_end_scale * distance_ratio * rng.gen_range(0.7..=1.3)).min(max_end_scale);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalVolumeSmoke(
            common::DirectionalVolumeSmoke::new(
                smoke_start_xy,
                direction,
                final_distance - offset_distance,
                angle,
                common::DirectionalVolumeSmokeOptions {
                    delay,
                    start_color: Some(super::common::FLAME_START_COLOR),
                    end_color: Some([0.22, 0.07, 0.025]),
                    start_scale: Some(1.0_f32.min(available_radius)),
                    end_scale: Some(end_scale),
                },
                created_at,
            ),
        ));
    }
}

pub(super) fn spawn_hit_effect(start_xy: (f32, f32), impact_xy: (f32, f32), created_at: Instant) {
    let direction = (impact_xy.0 - start_xy.0, impact_xy.1 - start_xy.1);
    let angle = if direction.0.hypot(direction.1) > 0.0001 {
        direction.1.atan2(direction.0)
    } else {
        -std::f32::consts::FRAC_PI_2
    };
    muzzle_flash::spawn_directional(impact_xy, angle, 2.0, FLAME_START_COLOR, created_at);
    ember::spawn_from_point(impact_xy, angle, FLAME_START_COLOR, created_at);
}
