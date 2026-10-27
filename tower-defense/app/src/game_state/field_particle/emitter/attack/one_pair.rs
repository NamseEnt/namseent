use super::common::{bullet_trail, vapor_ring};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{self, NewAttackParticle},
};
use namui::*;
use rand::Rng;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let projectile = attack::one_pair::Projectile::new(start_xy, end_xy, created_at);
    let rotation = projectile.rotation();
    ATTACK_VISUAL.spawn(NewAttackParticle::OnePairProjectile(projectile));

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
            attack::common::VaporRing::new(start_xy, created_at, delay, scale, rotation, 1.0)
                .with_muzzle_animation(rotation, 128.0),
        ));
    }
}
