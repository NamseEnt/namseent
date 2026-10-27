use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{self, NewAttackParticle},
};
use namui::*;
use rand::Rng;

use super::common::{bullet_trail, vapor_ring};

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let mut rng = rand::thread_rng();
    for bundle_index in 0..2 {
        let bundle_delay = Duration::from_millis(bundle_index * 75);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = rng.gen_range(0.25_f32..0.5).sqrt();
        let projectile_start_xy = (
            start_xy.0 + radius * angle.cos(),
            start_xy.1 + radius * angle.sin(),
        );
        ATTACK_VISUAL.spawn(NewAttackParticle::TwoPairProjectile(
            attack::two_pair::Projectile::new(projectile_start_xy, end_xy, created_at)
                .with_start_delay(bundle_delay),
        ));

        bullet_trail::spawn(
            projectile_start_xy,
            end_xy,
            created_at,
            bundle_delay,
            Duration::from_millis(10),
            0.1,
        );
        vapor_ring::spawn(
            projectile_start_xy,
            end_xy,
            created_at,
            bundle_delay,
            Duration::from_millis(15),
            0.75,
        );
    }
}
