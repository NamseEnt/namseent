use super::common::{ember, energy_ring};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{
        NewAttackParticle,
        common::{EnergyRing, VaporRing},
        royal_straight_flush::{
            Beam, ChargeLines, MagicStars, MuzzleFlash, ROYAL_GOLD, ROYAL_PURPLE, ROYAL_RED,
            ROYAL_VAPOR,
        },
    },
};
use namui::*;
use rand::Rng;

const EMBER_COUNT: usize = 12;
const ENERGY_RING_SCALE_MULTIPLIER: f32 = 8.0;
const RESIDUAL_SPARK_MAX_LATERAL_OFFSET_TILES: f32 = 0.8;
const EXTRA_MUZZLE_ENERGY_RING_COUNT: usize = 2;
const MUZZLE_ENERGY_RING_MAX_DELAY_MS: i64 = 100;
const VAPOR_RING_COUNT: usize = 3;
const VAPOR_RING_MIN_SIZE_TILES: f32 = 1.5;
const VAPOR_RING_MAX_SIZE_TILES: f32 = 3.0;
const VAPOR_RING_MIN_DISTANCE_TILES: f32 = 3.0;
const VAPOR_RING_MAX_DISTANCE_TILES: f32 = 4.0;
const VAPOR_RING_MIN_DURATION_MS: i64 = 250;
const VAPOR_RING_MAX_DURATION_MS: i64 = 600;
const VAPOR_RING_MAX_DELAY_MS: i64 = 120;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let delta = (end_xy.0 - start_xy.0, end_xy.1 - start_xy.1);
    let length = delta.0.hypot(delta.1);
    if length <= f32::EPSILON {
        return;
    }

    for beam in Beam::new(start_xy, end_xy, created_at) {
        ATTACK_VISUAL.spawn(NewAttackParticle::RoyalStraightFlushBeam(beam));
    }
    ATTACK_VISUAL.spawn(NewAttackParticle::RoyalStraightFlushChargeLines(
        ChargeLines::new(start_xy, created_at),
    ));
    ATTACK_VISUAL.spawn(NewAttackParticle::RoyalStraightFlushMuzzleFlash(
        MuzzleFlash::new(start_xy, created_at),
    ));
    for stars in MagicStars::new(start_xy, end_xy, created_at) {
        ATTACK_VISUAL.spawn(NewAttackParticle::RoyalStraightFlushMagicStars(stars));
    }

    let mut rng = rand::thread_rng();
    let muzzle_angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let muzzle_ring = EnergyRing::new(
        start_xy,
        created_at,
        Duration::from_millis(100),
        rng.gen_range(0.5_f32..=1.0) * ENERGY_RING_SCALE_MULTIPLIER,
        (muzzle_angle.cos(), muzzle_angle.sin()),
        1.0,
    )
    .with_color(ROYAL_PURPLE)
    .with_scale_growth(1.0)
    .with_linear_scale_growth()
    .with_duration(Duration::from_millis(100));
    ATTACK_VISUAL.spawn(NewAttackParticle::EnergyRing(muzzle_ring));

    for _ in 0..EXTRA_MUZZLE_ENERGY_RING_COUNT {
        let delay = Duration::from_millis(rng.gen_range(0_i64..=MUZZLE_ENERGY_RING_MAX_DELAY_MS));
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let ring = EnergyRing::new(
            start_xy,
            created_at,
            delay,
            rng.gen_range(0.5_f32..=1.0) * ENERGY_RING_SCALE_MULTIPLIER,
            (angle.cos(), angle.sin()),
            1.0,
        )
        .with_color(ROYAL_PURPLE)
        .with_scale_growth(1.0)
        .with_linear_scale_growth()
        .with_duration(Duration::from_millis(100));
        ATTACK_VISUAL.spawn(NewAttackParticle::EnergyRing(ring));
    }

    let tile_size = crate::game_state::TILE_PX_SIZE;
    let direction_px = (
        delta.0 * tile_size.width.as_f32(),
        delta.1 * tile_size.height.as_f32(),
    );
    let pixel_length = direction_px.0.hypot(direction_px.1);
    let reverse_direction = (
        -direction_px.0 / pixel_length,
        -direction_px.1 / pixel_length,
    );
    spawn_vapor_rings(start_xy, reverse_direction, created_at, &mut rng);

    energy_ring::spawn_with_color_and_ease_in_fade(
        start_xy,
        end_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(10),
        3.0,
        ROYAL_GOLD,
    );
    ember::spawn_line_with_variation(
        start_xy,
        end_xy,
        ROYAL_RED,
        created_at,
        EMBER_COUNT,
        std::f32::consts::PI / 2.0,
    );
    spawn_residual_sparks(start_xy, created_at, length, delta, &mut rng);
}

fn spawn_residual_sparks(
    start_xy: (f32, f32),
    created_at: Instant,
    length: f32,
    delta: (f32, f32),
    rng: &mut impl Rng,
) {
    let direction = (delta.0 / length, delta.1 / length);
    let perpendicular = (-direction.1, direction.0);
    let spark_count = length.ceil().max(1.0) as usize;

    for index in 0..spark_count {
        let nominal_distance = (index as f32 + 0.5) * length / spark_count as f32;
        let distance = (nominal_distance + rng.gen_range(-0.25_f32..=0.25)).clamp(0.0, length);
        let lateral_offset = rng.gen_range(
            -RESIDUAL_SPARK_MAX_LATERAL_OFFSET_TILES..=RESIDUAL_SPARK_MAX_LATERAL_OFFSET_TILES,
        );
        let center_xy = (
            start_xy.0 + direction.0 * distance + perpendicular.0 * lateral_offset,
            start_xy.1 + direction.1 * distance + perpendicular.1 * lateral_offset,
        );
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let delay = Duration::from_millis(rng.gen_range(0_i64..=100));
        let spark =
            crate::game_state::field_particle::particle::attack::common::ResidualSpark::new(
                center_xy, angle, ROYAL_GOLD, created_at,
            )
            .with_base_scale_multiplier(rng.gen_range(0.6_f32..=1.0))
            .with_delay(delay);
        ATTACK_VISUAL.spawn(NewAttackParticle::ResidualSpark(spark));
    }
}

fn spawn_vapor_rings(
    start_xy: (f32, f32),
    reverse_direction: (f32, f32),
    created_at: Instant,
    rng: &mut impl Rng,
) {
    let frame_height = crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_00[0]
        .height()
        .as_f32();
    let tile_size = crate::game_state::TILE_PX_SIZE;

    for _ in 0..VAPOR_RING_COUNT {
        let start_scale_tiles = rng.gen_range(0.5_f32..=1.0);
        let max_scale_tiles = rng.gen_range(VAPOR_RING_MIN_SIZE_TILES..=VAPOR_RING_MAX_SIZE_TILES);
        let start_scale = start_scale_tiles * tile_size.height.as_f32() / frame_height;
        let max_scale = max_scale_tiles * tile_size.height.as_f32() / frame_height;
        let scale_growth = max_scale / start_scale - 1.0;
        let movement_distance_px = rng
            .gen_range(VAPOR_RING_MIN_DISTANCE_TILES..=VAPOR_RING_MAX_DISTANCE_TILES)
            * tile_size.width.as_f32();
        let delay = Duration::from_millis(rng.gen_range(0_i64..=VAPOR_RING_MAX_DELAY_MS));
        let duration = Duration::from_millis(
            rng.gen_range(VAPOR_RING_MIN_DURATION_MS..=VAPOR_RING_MAX_DURATION_MS),
        );
        let ring = VaporRing::new(
            start_xy,
            created_at,
            delay,
            start_scale,
            reverse_direction,
            rng.gen_range(0.5_f32..=0.8),
        )
        .with_color(ROYAL_VAPOR)
        .with_movement(reverse_direction, movement_distance_px)
        .with_scale_growth(scale_growth)
        .with_duration(duration)
        .with_ease_in_fade();
        ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(ring));
    }
}
