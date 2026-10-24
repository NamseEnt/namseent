use super::common::{ember, muzzle_flash};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{
        NewAttackParticle,
        common::{
            BeamCore, DirectionalFlash, EnergyRing, ResidualSpark, RotationDirection, StarFlare,
            VaporRing,
        },
    },
};
use namui::*;
use rand::Rng;

const FOUR_OF_A_KIND_COLOR: [f32; 3] = [0.6, 0.95, 1.0];
const MUZZLE_FLASH_WAVE_COUNT: usize = 3;
const MUZZLE_FLASHES_PER_WAVE: usize = 4;
const MUZZLE_FLASH_HALF_ANGLE: f32 = std::f32::consts::PI / 2.0;
const MUZZLE_FLASH_INTERVAL: Duration = Duration::from_millis(15);
const MUZZLE_RING_COUNT: usize = 4;
const MUZZLE_RING_MIN_DURATION: i64 = 100;
const MUZZLE_RING_MAX_DURATION: i64 = 450;
const MUZZLE_RING_MOVEMENT_PX: f32 = 1024.0;
const MUZZLE_VAPOR_RING_MOVEMENT_PX: f32 = 256.0;
const TRAIL_EMBER_COUNT: usize = 12;
const HIT_FLASH_COUNT: usize = 3;
const HIT_FLASH_HALF_ANGLE: f32 = std::f32::consts::PI / 6.0;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let direction_x = end_xy.0 - start_xy.0;
    let direction_y = end_xy.1 - start_xy.1;
    let length = direction_x.hypot(direction_y);
    if length <= f32::EPSILON {
        return;
    }

    let angle = direction_y.atan2(direction_x);
    spawn_muzzle_flashes(start_xy, angle, created_at);
    spawn_muzzle_rings(start_xy, (angle.cos(), angle.sin()), created_at);
    spawn_star_flares(start_xy, end_xy, created_at);
    spawn_residual_sparks(start_xy, end_xy, created_at);
    ATTACK_VISUAL.spawn(NewAttackParticle::BeamCore(BeamCore::new(
        start_xy, end_xy, created_at,
    )));

    ember::spawn_line_with_variation(
        start_xy,
        end_xy,
        FOUR_OF_A_KIND_COLOR,
        created_at,
        TRAIL_EMBER_COUNT,
        MUZZLE_FLASH_HALF_ANGLE,
    );
}

fn spawn_residual_sparks(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let length_tiles = delta_x.hypot(delta_y);
    if length_tiles <= f32::EPSILON {
        return;
    }

    let direction = (delta_x / length_tiles, delta_y / length_tiles);
    let perpendicular = (-direction.1, direction.0);
    let tile_count = length_tiles.ceil().max(1.0) as usize;
    let spacing = length_tiles / tile_count as f32;
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let attack_angle =
        (delta_y * tile_size.height.as_f32()).atan2(delta_x * tile_size.width.as_f32());
    let mut rng = rand::thread_rng();

    for tile_index in 0..tile_count {
        let nominal_distance = (tile_index as f32 + 0.5) * spacing;
        let distance = (nominal_distance + rng.gen_range(-1.0..=1.0)).clamp(0.0, length_tiles);
        let lateral_offset = rng.gen_range(-1.0..=1.0);
        let center_xy = (
            start_xy.0 + direction.0 * distance + perpendicular.0 * lateral_offset,
            start_xy.1 + direction.1 * distance + perpendicular.1 * lateral_offset,
        );
        ATTACK_VISUAL.spawn(NewAttackParticle::ResidualSpark(ResidualSpark::new(
            center_xy,
            attack_angle,
            FOUR_OF_A_KIND_COLOR,
            created_at,
        )));
    }
}

fn spawn_star_flares(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    for center_xy in [start_xy, end_xy] {
        ATTACK_VISUAL.spawn(NewAttackParticle::StarFlare(StarFlare::new(
            center_xy,
            3.0,
            RotationDirection::Clockwise,
            FOUR_OF_A_KIND_COLOR,
            created_at,
        )));
        ATTACK_VISUAL.spawn(NewAttackParticle::StarFlare(StarFlare::new(
            center_xy,
            2.0,
            RotationDirection::CounterClockwise,
            FOUR_OF_A_KIND_COLOR,
            created_at,
        )));
    }
}

pub(super) fn spawn_hit_effect(start_xy: (f32, f32), impact_xy: (f32, f32), created_at: Instant) {
    let direction_x = impact_xy.0 - start_xy.0;
    let direction_y = impact_xy.1 - start_xy.1;
    let aim_angle = if direction_x.hypot(direction_y) > f32::EPSILON {
        direction_y.atan2(direction_x)
    } else {
        -std::f32::consts::FRAC_PI_2
    };
    let mut rng = rand::thread_rng();
    for _ in 0..HIT_FLASH_COUNT {
        let angle = aim_angle + rng.gen_range(-HIT_FLASH_HALF_ANGLE..=HIT_FLASH_HALF_ANGLE);
        muzzle_flash::spawn_directional(impact_xy, angle, 2.0, FOUR_OF_A_KIND_COLOR, created_at);
    }
    ember::spawn_from_point_with_options(
        impact_xy,
        aim_angle,
        FOUR_OF_A_KIND_COLOR,
        created_at,
        Duration::ZERO,
        TRAIL_EMBER_COUNT * 2,
        HIT_FLASH_HALF_ANGLE,
    );
}

fn spawn_muzzle_flashes(start_xy: (f32, f32), aim_angle: f32, created_at: Instant) {
    let mut rng = rand::thread_rng();
    for wave in 0..MUZZLE_FLASH_WAVE_COUNT {
        let delay = MUZZLE_FLASH_INTERVAL * wave as i32;
        for _ in 0..MUZZLE_FLASHES_PER_WAVE {
            let angle =
                aim_angle + rng.gen_range(-MUZZLE_FLASH_HALF_ANGLE..=MUZZLE_FLASH_HALF_ANGLE);
            let scale = rng.gen_range(0.8..=1.2) * 2.0;
            ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(
                DirectionalFlash::new(start_xy, angle, scale, FOUR_OF_A_KIND_COLOR, created_at)
                    .with_delay(delay),
            ));
        }
    }
}

fn spawn_muzzle_rings(start_xy: (f32, f32), direction: (f32, f32), created_at: Instant) {
    let mut rng = rand::thread_rng();
    let reverse_direction = (-direction.0, -direction.1);
    let rotation = (direction.0, direction.1);

    for _ in 0..MUZZLE_RING_COUNT {
        let duration = Duration::from_millis(
            rng.gen_range(MUZZLE_RING_MIN_DURATION..=MUZZLE_RING_MAX_DURATION),
        );
        let scale = rng.gen_range(1.0..=2.0);
        ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
            VaporRing::new(start_xy, created_at, Duration::ZERO, scale, rotation, 0.5)
                .with_color([0.82, 0.84, 0.86])
                .with_movement(reverse_direction, MUZZLE_VAPOR_RING_MOVEMENT_PX)
                .with_scale_growth(2.0)
                .with_duration(duration)
                .with_ease_in_fade(),
        ));
        ATTACK_VISUAL.spawn(NewAttackParticle::EnergyRing(
            EnergyRing::new(start_xy, created_at, Duration::ZERO, scale, rotation, 1.0)
                .with_color(FOUR_OF_A_KIND_COLOR)
                .with_movement(direction, MUZZLE_RING_MOVEMENT_PX)
                .with_scale_growth(2.0)
                .with_duration(duration)
                .with_ease_in_fade(),
        ));
    }
}
