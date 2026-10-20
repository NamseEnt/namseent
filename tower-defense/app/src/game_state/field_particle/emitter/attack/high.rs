use super::common::{spawn_bullet_trail, spawn_vapor_ring_trail};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{self, NewAttackParticle},
};
use namui::*;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    ATTACK_VISUAL.spawn(NewAttackParticle::HighProjectile(
        attack::high::Projectile::new(start_xy, end_xy, created_at),
    ));
    spawn_bullet_trail(
        start_xy,
        end_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(30),
        0.075,
    );
    spawn_vapor_ring_trail(
        start_xy,
        end_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(30),
        0.75,
    );
}
