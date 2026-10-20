pub mod common;
pub mod high;
pub mod one_pair;
pub mod two_pair;

use crate::animation::CubicBezier;
use namui::*;

pub(crate) const ATTACK_FRAME_EASE_OUT: CubicBezier = CubicBezier::new(0.0, 0.5, 0.5, 1.0);

#[derive(Clone)]
/// migrating
pub enum NewAttackParticle {
    HighProjectile(high::Projectile),
    OnePairProjectile(one_pair::Projectile),
    TwoPairProjectile(two_pair::Projectile),
    BulletTrail(common::BulletTrail),
    VaporRing(common::VaporRing),
}

impl namui::particle::Particle for NewAttackParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        match self {
            NewAttackParticle::HighProjectile(p) => p.tick(now, dt),
            NewAttackParticle::OnePairProjectile(p) => p.tick(now, dt),
            NewAttackParticle::TwoPairProjectile(p) => p.tick(now, dt),
            NewAttackParticle::BulletTrail(p) => p.tick(now, dt),
            NewAttackParticle::VaporRing(p) => p.tick(now, dt),
        }
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        match self {
            NewAttackParticle::HighProjectile(p) => p.render(),
            NewAttackParticle::OnePairProjectile(p) => p.render(),
            NewAttackParticle::TwoPairProjectile(p) => p.render(),
            NewAttackParticle::BulletTrail(p) => p.render(),
            NewAttackParticle::VaporRing(p) => p.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            NewAttackParticle::HighProjectile(p) => p.is_done(now),
            NewAttackParticle::OnePairProjectile(p) => p.is_done(now),
            NewAttackParticle::TwoPairProjectile(p) => p.is_done(now),
            NewAttackParticle::BulletTrail(p) => p.is_done(now),
            NewAttackParticle::VaporRing(p) => p.is_done(now),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn common_attack_particle_atlases_have_expected_frame_counts() {
        let projectile = crate::atlas::generated::ATTACK_ONE_PAIR_PROJECTILE;

        assert_eq!(projectile.width().as_f32(), 32.0);
        assert_eq!(projectile.height().as_f32(), 32.0);
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_BULLET_TRAIL_00.len(),
            6
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_BULLET_TRAIL_01.len(),
            7
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_BULLET_TRAIL_02.len(),
            7
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_00.len(),
            7
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_01.len(),
            6
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_VAPOR_RING_02.len(),
            6
        );
    }
}
