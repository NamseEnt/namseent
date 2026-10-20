use super::common::{bullet_trail, vapor_ring};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{self, NewAttackParticle},
};
use namui::*;
use rand::Rng;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let delta_x_px = delta_x * tile_size.width.as_f32();
    let delta_y_px = delta_y * tile_size.height.as_f32();
    let angle = delta_y_px.atan2(delta_x_px);
    let rotation_cos = angle.cos();
    let rotation_sin = angle.sin();
    ATTACK_VISUAL.spawn(NewAttackParticle::OnePairProjectile(
        attack::one_pair::Projectile::new(start_xy, end_xy, created_at),
    ));

    let mut rng = rand::thread_rng();
    bullet_trail::spawn(
        start_xy,
        end_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(30),
        0.125,
    );
    vapor_ring::spawn(
        start_xy,
        end_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(30),
        1.0,
    );

    for index in 0..3 {
        let delay = Duration::from_millis(index * 50);
        let scale = rng.gen_range(1.2..=2.0);
        ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
            attack::common::VaporRing::new(
                start_xy,
                created_at,
                delay,
                scale,
                (rotation_cos, rotation_sin),
                1.0,
            )
            .with_muzzle_animation((rotation_cos, rotation_sin), 128.0),
        ));
    }
}
