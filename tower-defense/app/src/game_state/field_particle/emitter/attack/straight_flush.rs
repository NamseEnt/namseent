use super::common::{ember, energy_ring};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{
        NewAttackParticle,
        common::{
            DirectionalFlash, DirectionalVolumeSmoke, DirectionalVolumeSmokeOptions, EnergyRing,
            ResidualSpark, RotationDirection, StarFlare, VaporRing,
        },
        straight_lightning::StraightLightning,
    },
};
use namui::*;
use rand::Rng;

const STRAIGHT_FLUSH_COLOR: [f32; 3] = [0.88, 0.68, 0.96];
const LIGHTNING_SEGMENT_LENGTH_TILES: f32 = 1.0;
const LIGHTNING_SEGMENT_SPACING_TILES: f32 = 0.95;
const LIGHTNING_STRANDS_PER_SEGMENT: usize = 2;
const MAX_LIGHTNING_STRAND_OFFSET_TILES: f32 = 0.06;
const MAX_CHAIN_LIGHTNING_SCALE: f32 = 1.0;
const MUZZLE_ENERGY_RING_COUNT: usize = 3;
const MUZZLE_ENERGY_RING_MAX_SCALE_TILES: f32 = 4.0;
const MUZZLE_ENERGY_RING_DELAY_STEP: Duration = Duration::from_millis(15);
const MUZZLE_ENERGY_RING_DURATION: Duration = Duration::from_millis(350);
const FLARE_DURATION: Duration = Duration::from_millis(200);
const FLARE_GROWTH: Duration = Duration::from_millis(50);
const EXPLOSION_FLASH_COUNT: usize = 6;
const EXPLOSION_SMOKE_COUNT: usize = 6;
const EXPLOSION_SMOKE_LAYER_RADII_TILES: [f32; 2] = [0.15, 0.35];
const EXPLOSION_RESIDUAL_SPARK_COUNT: usize = 5;
const EXPLOSION_RESIDUAL_SPARK_RADIUS_TILES: f32 = 2.5;
const MAX_RESIDUAL_SPARK_DELAY: u64 = 150;
const RESIDUAL_SPARKS_PER_TILE: f32 = 1.0;

pub(super) fn spawn_muzzle(center_xy: (f32, f32), created_at: Instant) {
    spawn_star_flares(center_xy, created_at, Duration::ZERO);
    spawn_muzzle_energy_rings(center_xy, created_at);
}

fn spawn_muzzle_energy_rings(center_xy: (f32, f32), created_at: Instant) {
    let mut rng = rand::thread_rng();
    for index in 0..MUZZLE_ENERGY_RING_COUNT {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let scale_tiles = rng.gen_range(0.85_f32..=1.15);
        let rotation = (angle.cos(), angle.sin());
        let delay = MUZZLE_ENERGY_RING_DELAY_STEP * index as i32;
        ATTACK_VISUAL.spawn(NewAttackParticle::EnergyRing(
            EnergyRing::new(center_xy, created_at, delay, scale_tiles, rotation, 1.0)
                .with_color(STRAIGHT_FLUSH_COLOR)
                .with_scale_growth(MUZZLE_ENERGY_RING_MAX_SCALE_TILES / scale_tiles - 1.0)
                .with_duration(MUZZLE_ENERGY_RING_DURATION)
                .with_ease_in_fade(),
        ));
    }
}

fn spawn_star_flares(center_xy: (f32, f32), created_at: Instant, delay: Duration) {
    for (scale, rotation) in [
        (2.0, RotationDirection::Clockwise),
        (1.5, RotationDirection::CounterClockwise),
        (1.0, RotationDirection::Clockwise),
        (0.5, RotationDirection::CounterClockwise),
    ] {
        ATTACK_VISUAL.spawn(NewAttackParticle::StarFlare(
            StarFlare::new(center_xy, scale, rotation, STRAIGHT_FLUSH_COLOR, created_at)
                .with_timing(FLARE_DURATION, FLARE_GROWTH)
                .with_delay(delay),
        ));
    }
}

pub(super) fn spawn_hit_effect(
    start_xy: (f32, f32),
    impact_xy: (f32, f32),
    created_at: Instant,
    chain_delay: Duration,
) {
    spawn_star_flares(impact_xy, created_at, chain_delay);
    spawn_chain_lightning(start_xy, impact_xy, created_at, chain_delay);
    energy_ring::spawn_with_color(
        start_xy,
        impact_xy,
        created_at,
        chain_delay,
        Duration::from_millis(10),
        1.5,
        STRAIGHT_FLUSH_COLOR,
    );
    ember::spawn_line_with_delay(
        start_xy,
        impact_xy,
        STRAIGHT_FLUSH_COLOR,
        created_at,
        chain_delay,
    );
    spawn_path_sparks(start_xy, impact_xy, created_at, chain_delay);
    spawn_reduced_flush_explosion(impact_xy, created_at, chain_delay);
}

fn spawn_chain_lightning(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    chain_delay: Duration,
) {
    let direction_x = end_xy.0 - start_xy.0;
    let direction_y = end_xy.1 - start_xy.1;
    let length = direction_x.hypot(direction_y);
    if length <= f32::EPSILON {
        return;
    }

    let angle = direction_y.atan2(direction_x);
    let direction = (direction_x / length, direction_y / length);
    let perpendicular = (-direction.1, direction.0);
    let segment_count = (length / LIGHTNING_SEGMENT_SPACING_TILES).ceil() as usize;
    let mut rng = rand::thread_rng();
    for index in 0..segment_count {
        let segment_start = index as f32 * LIGHTNING_SEGMENT_SPACING_TILES;
        let segment_length = LIGHTNING_SEGMENT_LENGTH_TILES.min(length - segment_start);
        let center_distance = segment_start + segment_length / 2.0;
        let center_xy = (
            start_xy.0 + direction.0 * center_distance,
            start_xy.1 + direction.1 * center_distance,
        );
        let variant = rng.gen_range(0..3);
        let flip_vertical = rng.gen_bool(0.5);
        let offset = rng.gen_range(0.02_f32..=MAX_LIGHTNING_STRAND_OFFSET_TILES);
        let offset_direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        for strand_index in 0..LIGHTNING_STRANDS_PER_SEGMENT {
            let side = if strand_index == 0 { -1.0 } else { 1.0 };
            let strand_center = (
                center_xy.0 + perpendicular.0 * offset * side * offset_direction,
                center_xy.1 + perpendicular.1 * offset * side * offset_direction,
            );
            ATTACK_VISUAL.spawn(NewAttackParticle::StraightLightning(
                StraightLightning::new(
                    strand_center,
                    angle,
                    segment_length,
                    variant,
                    flip_vertical,
                    created_at,
                    chain_delay,
                )
                .with_scale_multiplier(MAX_CHAIN_LIGHTNING_SCALE)
                .with_color(STRAIGHT_FLUSH_COLOR),
            ));
        }
    }
}

fn spawn_path_sparks(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    chain_delay: Duration,
) {
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let length = delta_x.hypot(delta_y);
    if length <= f32::EPSILON {
        return;
    }

    let direction = (delta_x / length, delta_y / length);
    let perpendicular = (-direction.1, direction.0);
    let angle = delta_y.atan2(delta_x);
    let spark_count = (length * RESIDUAL_SPARKS_PER_TILE).ceil().max(1.0) as usize;
    let spacing = length / spark_count as f32;
    let mut rng = rand::thread_rng();

    for index in 0..spark_count {
        let nominal_distance = (index as f32 + 0.5) * spacing;
        let distance = (nominal_distance + rng.gen_range(-0.25_f32..=0.25)).clamp(0.0, length);
        let lateral_offset = rng.gen_range(-0.1_f32..=0.1);
        let center_xy = (
            start_xy.0 + direction.0 * distance + perpendicular.0 * lateral_offset,
            start_xy.1 + direction.1 * distance + perpendicular.1 * lateral_offset,
        );
        let random_delay =
            Duration::from_millis(rng.gen_range(0..=MAX_RESIDUAL_SPARK_DELAY) as i64);
        ATTACK_VISUAL.spawn(NewAttackParticle::ResidualSpark(
            ResidualSpark::new(center_xy, angle, STRAIGHT_FLUSH_COLOR, created_at)
                .with_base_scale_multiplier(0.5)
                .with_delay(chain_delay + random_delay),
        ));
    }
}

fn spawn_reduced_flush_explosion(
    center_xy: (f32, f32),
    created_at: Instant,
    chain_delay: Duration,
) {
    let mut rng = rand::thread_rng();
    for index in 0..EXPLOSION_FLASH_COUNT {
        let angle = std::f32::consts::TAU * index as f32 / EXPLOSION_FLASH_COUNT as f32;
        let scale = rng.gen_range(1.5_f32..=2.75);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(
            DirectionalFlash::new(center_xy, angle, scale, STRAIGHT_FLUSH_COLOR, created_at)
                .with_delay(chain_delay),
        ));
    }

    for (layer_index, layer_radius) in EXPLOSION_SMOKE_LAYER_RADII_TILES.iter().enumerate() {
        for index in 0..EXPLOSION_SMOKE_COUNT {
            let angle = std::f32::consts::TAU * (index as f32 + layer_index as f32 * 0.5)
                / EXPLOSION_SMOKE_COUNT as f32
                + rng.gen_range(-std::f32::consts::PI / 36.0..=std::f32::consts::PI / 36.0);
            let direction = (angle.cos(), angle.sin());
            let radius = (layer_radius + rng.gen_range(-0.06_f32..=0.06)).max(0.0);
            let start_xy = (
                center_xy.0 + direction.0 * radius,
                center_xy.1 + direction.1 * radius,
            );
            ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalVolumeSmoke(
                DirectionalVolumeSmoke::new(
                    start_xy,
                    direction,
                    0.0,
                    angle,
                    DirectionalVolumeSmokeOptions {
                        delay: chain_delay,
                        start_color: Some(STRAIGHT_FLUSH_COLOR),
                        end_color: Some(STRAIGHT_FLUSH_COLOR),
                        start_scale: Some(0.75),
                        end_scale: Some(1.5),
                    },
                    created_at,
                ),
            ));
        }
    }

    ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
        VaporRing::new(center_xy, created_at, chain_delay, 1.0, (1.0, 0.0), 1.0)
            .with_color(STRAIGHT_FLUSH_COLOR)
            .with_scale_growth(3.0),
    ));
    ember::spawn_radial_with_count(center_xy, STRAIGHT_FLUSH_COLOR, created_at, chain_delay, 6);

    for _ in 0..EXPLOSION_RESIDUAL_SPARK_COUNT {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = EXPLOSION_RESIDUAL_SPARK_RADIUS_TILES * rng.gen_range(0.0_f32..=1.0).sqrt();
        let center_xy = (
            center_xy.0 + angle.cos() * radius,
            center_xy.1 + angle.sin() * radius,
        );
        let random_delay =
            Duration::from_millis(rng.gen_range(0..=MAX_RESIDUAL_SPARK_DELAY) as i64);
        ATTACK_VISUAL.spawn(NewAttackParticle::ResidualSpark(
            ResidualSpark::new(center_xy, angle, STRAIGHT_FLUSH_COLOR, created_at)
                .with_base_scale_multiplier(0.5)
                .with_delay(chain_delay + random_delay),
        ));
    }
}
