pub mod common;
pub mod high;
pub mod one_pair;
pub mod straight_lightning;
pub mod two_pair;

use crate::animation::CubicBezier;
use namui::*;

pub(crate) const ATTACK_FRAME_EASE_OUT: CubicBezier = CubicBezier::new(0.0, 0.5, 0.5, 1.0);
pub(crate) const ATTACK_FRAME_EASE_IN: CubicBezier = CubicBezier::new(0.5, 0.0, 1.0, 0.5);

#[derive(Clone)]
/// migrating
pub enum NewAttackParticle {
    HighProjectile(high::Projectile),
    OnePairProjectile(one_pair::Projectile),
    TwoPairProjectile(two_pair::Projectile),
    BulletTrail(common::BulletTrail),
    VaporRing(common::VaporRing),
    EnergyRing(common::EnergyRing),
    DirectionalEmber(common::DirectionalEmber),
    DirectionalFlash(common::DirectionalFlash),
    DirectionalVolumeSmoke(common::DirectionalVolumeSmoke),
    StraightLightning(straight_lightning::StraightLightning),
}

impl namui::particle::Particle for NewAttackParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        match self {
            NewAttackParticle::HighProjectile(p) => p.tick(now, dt),
            NewAttackParticle::OnePairProjectile(p) => p.tick(now, dt),
            NewAttackParticle::TwoPairProjectile(p) => p.tick(now, dt),
            NewAttackParticle::BulletTrail(p) => p.tick(now, dt),
            NewAttackParticle::VaporRing(p) => p.tick(now, dt),
            NewAttackParticle::EnergyRing(p) => p.tick(now, dt),
            NewAttackParticle::DirectionalEmber(p) => p.tick(now, dt),
            NewAttackParticle::DirectionalFlash(p) => p.tick(now, dt),
            NewAttackParticle::DirectionalVolumeSmoke(p) => p.tick(now, dt),
            NewAttackParticle::StraightLightning(p) => p.tick(now, dt),
        }
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        match self {
            NewAttackParticle::HighProjectile(p) => p.render(),
            NewAttackParticle::OnePairProjectile(p) => p.render(),
            NewAttackParticle::TwoPairProjectile(p) => p.render(),
            NewAttackParticle::BulletTrail(p) => p.render(),
            NewAttackParticle::VaporRing(p) => p.render(),
            NewAttackParticle::EnergyRing(p) => p.render(),
            NewAttackParticle::DirectionalEmber(p) => p.render(),
            NewAttackParticle::DirectionalFlash(p) => p.render(),
            NewAttackParticle::DirectionalVolumeSmoke(p) => p.render(),
            NewAttackParticle::StraightLightning(p) => p.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            NewAttackParticle::HighProjectile(p) => p.is_done(now),
            NewAttackParticle::OnePairProjectile(p) => p.is_done(now),
            NewAttackParticle::TwoPairProjectile(p) => p.is_done(now),
            NewAttackParticle::BulletTrail(p) => p.is_done(now),
            NewAttackParticle::VaporRing(p) => p.is_done(now),
            NewAttackParticle::EnergyRing(p) => p.is_done(now),
            NewAttackParticle::DirectionalEmber(p) => p.is_done(now),
            NewAttackParticle::DirectionalFlash(p) => p.is_done(now),
            NewAttackParticle::DirectionalVolumeSmoke(p) => p.is_done(now),
            NewAttackParticle::StraightLightning(p) => p.is_done(now),
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
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_ENERGY_RING_00.len(),
            7
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_ENERGY_RING_01.len(),
            4
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_ENERGY_RING_02.len(),
            4
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_FLASH_00.len(),
            6
        );
        assert_eq!(
            crate::atlas::generated::ATTACK_COMMON_DIRECTIONAL_VOLUME_SMOKE_00.len(),
            11
        );
        for ember in [
            crate::atlas::generated::ATTACK_COMMON_EMBER_00,
            crate::atlas::generated::ATTACK_COMMON_EMBER_01,
            crate::atlas::generated::ATTACK_COMMON_EMBER_02,
        ] {
            assert_eq!(ember.width().as_f32(), 16.0);
            assert_eq!(ember.height().as_f32(), 16.0);
        }
    }
}
