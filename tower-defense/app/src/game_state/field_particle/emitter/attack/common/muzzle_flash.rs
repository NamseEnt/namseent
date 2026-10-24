use crate::game_state::field_particle::{
    particle::attack::{common::DirectionalFlash, NewAttackParticle},
    ATTACK_VISUAL,
};
use namui::*;
use rand::Rng;

const FLASH_COUNT: usize = 8;
const HALF_ANGLE: f32 = std::f32::consts::PI / 6.0;

pub(in crate::game_state::field_particle::emitter::attack) fn spawn_fan(
    start_xy: (f32, f32),
    aim_angle: f32,
    color: [f32; 3],
    created_at: Instant,
) {
    let mut rng = rand::thread_rng();
    for index in 0..FLASH_COUNT {
        let angle_offset = -HALF_ANGLE + 2.0 * HALF_ANGLE * index as f32 / (FLASH_COUNT - 1) as f32;
        let angle = aim_angle + angle_offset;
        let center_bias = 1.0 - angle_offset.abs() / HALF_ANGLE;
        let scale = (0.5 + center_bias * 1.0) * rng.gen_range(0.8..=1.2) * 2.0;
        spawn_directional(start_xy, angle, scale, color, created_at);
    }
}

pub(in crate::game_state::field_particle::emitter::attack) fn spawn_directional(
    anchor_xy: (f32, f32),
    angle: f32,
    scale: f32,
    color: [f32; 3],
    created_at: Instant,
) {
    ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(DirectionalFlash::new(
        anchor_xy, angle, scale, color, created_at,
    )));
}
