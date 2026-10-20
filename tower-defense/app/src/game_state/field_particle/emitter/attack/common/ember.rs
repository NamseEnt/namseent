use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common::DirectionalEmber},
};
use namui::*;
use rand::Rng;
use rand::seq::SliceRandom;

const EMBER_COUNT: usize = 12;
const MIN_TRAVEL_DISTANCE_TILES: f32 = 0.5;
const MAX_TRAVEL_DISTANCE_TILES: f32 = 3.0;
const MIN_SCALE: f32 = 0.1;
const MAX_SCALE: f32 = 0.25;
const SPAWN_RANGE_MARGIN_TILES: f32 = 0.25;
const TRAVEL_ANGLE_VARIATION: f32 = std::f32::consts::PI / 6.0;
const HIT_EMBER_COUNT: usize = 6;

pub(in crate::game_state::field_particle::emitter::attack) fn spawn_cone(
    origin_xy: (f32, f32),
    aim_angle: f32,
    cone_half_angle: f32,
    attack_range_radius: f32,
    color: [f32; 3],
    created_at: Instant,
) {
    let mut rng = rand::thread_rng();
    let max_spawn_distance = (attack_range_radius - SPAWN_RANGE_MARGIN_TILES).max(0.0);
    let mut radial_bins = (0..EMBER_COUNT).collect::<Vec<_>>();
    radial_bins.shuffle(&mut rng);

    for (index, radial_bin) in radial_bins.into_iter().enumerate() {
        let angular_position =
            (index as f32 + 0.5 + rng.gen_range(-0.4_f32..=0.4_f32)) / EMBER_COUNT as f32;
        let spawn_angle = aim_angle + (angular_position * 2.0 - 1.0) * cone_half_angle;
        let area_position =
            (radial_bin as f32 + rng.gen_range(0.15_f32..=0.85_f32)) / EMBER_COUNT as f32;
        let spawn_distance = area_position.sqrt() * max_spawn_distance;
        let spawn_xy = (
            origin_xy.0 + spawn_angle.cos() * spawn_distance,
            origin_xy.1 + spawn_angle.sin() * spawn_distance,
        );
        let radial_angle = if spawn_distance > f32::EPSILON {
            (spawn_xy.1 - origin_xy.1).atan2(spawn_xy.0 - origin_xy.0)
        } else {
            aim_angle
        };
        let travel_angle =
            radial_angle + rng.gen_range(-TRAVEL_ANGLE_VARIATION..=TRAVEL_ANGLE_VARIATION);
        let travel_distance = rng.gen_range(MIN_TRAVEL_DISTANCE_TILES..=MAX_TRAVEL_DISTANCE_TILES);
        let scale = rng.gen_range(MIN_SCALE..=MAX_SCALE);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalEmber(DirectionalEmber::new(
            spawn_xy,
            travel_angle,
            travel_distance,
            scale,
            color,
            created_at,
        )));
    }
}

pub(in crate::game_state::field_particle::emitter::attack) fn spawn_from_point(
    impact_xy: (f32, f32),
    outward_angle: f32,
    color: [f32; 3],
    created_at: Instant,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..HIT_EMBER_COUNT {
        let direction_angle =
            outward_angle + rng.gen_range(-TRAVEL_ANGLE_VARIATION..=TRAVEL_ANGLE_VARIATION);
        let distance = rng.gen_range(MIN_TRAVEL_DISTANCE_TILES..=MAX_TRAVEL_DISTANCE_TILES);
        let scale = rng.gen_range(MIN_SCALE..=MAX_SCALE);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalEmber(DirectionalEmber::new(
            impact_xy,
            direction_angle,
            distance,
            scale,
            color,
            created_at,
        )));
    }
}
