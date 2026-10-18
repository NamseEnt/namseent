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
    HighTrail(high::Trail),
    OnePairProjectile(one_pair::Projectile),
    OnePairTrail(one_pair::Trail),
    OnePairMuzzleRing(one_pair::MuzzleRing),
    TwoPairProjectile(two_pair::Projectile),
    TwoPairTrail(two_pair::Trail),
    TwoPairTrailRing(two_pair::TrailRing),
    TwoPairMuzzleRing(two_pair::MuzzleRing),
}

impl namui::particle::Particle for NewAttackParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        match self {
            NewAttackParticle::HighProjectile(p) => p.tick(now, dt),
            NewAttackParticle::HighTrail(p) => p.tick(now, dt),
            NewAttackParticle::OnePairProjectile(p) => p.tick(now, dt),
            NewAttackParticle::OnePairTrail(p) => p.tick(now, dt),
            NewAttackParticle::OnePairMuzzleRing(p) => p.tick(now, dt),
            NewAttackParticle::TwoPairProjectile(p) => p.tick(now, dt),
            NewAttackParticle::TwoPairTrail(p) => p.tick(now, dt),
            NewAttackParticle::TwoPairTrailRing(p) => p.tick(now, dt),
            NewAttackParticle::TwoPairMuzzleRing(p) => p.tick(now, dt),
        }
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        match self {
            NewAttackParticle::HighProjectile(p) => p.render(),
            NewAttackParticle::HighTrail(p) => p.render(),
            NewAttackParticle::OnePairProjectile(p) => p.render(),
            NewAttackParticle::OnePairTrail(p) => p.render(),
            NewAttackParticle::OnePairMuzzleRing(p) => p.render(),
            NewAttackParticle::TwoPairProjectile(p) => p.render(),
            NewAttackParticle::TwoPairTrail(p) => p.render(),
            NewAttackParticle::TwoPairTrailRing(p) => p.render(),
            NewAttackParticle::TwoPairMuzzleRing(p) => p.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            NewAttackParticle::HighProjectile(p) => p.is_done(now),
            NewAttackParticle::HighTrail(p) => p.is_done(now),
            NewAttackParticle::OnePairProjectile(p) => p.is_done(now),
            NewAttackParticle::OnePairTrail(p) => p.is_done(now),
            NewAttackParticle::OnePairMuzzleRing(p) => p.is_done(now),
            NewAttackParticle::TwoPairProjectile(p) => p.is_done(now),
            NewAttackParticle::TwoPairTrail(p) => p.is_done(now),
            NewAttackParticle::TwoPairTrailRing(p) => p.is_done(now),
            NewAttackParticle::TwoPairMuzzleRing(p) => p.is_done(now),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_pair_particle_atlas_rects_have_expected_dimensions() {
        let projectile = crate::atlas::generated::ATTACK_ONE_PAIR_PROJECTILE;
        let trail = crate::atlas::generated::ATTACK_ONE_PAIR_TRAIL[0];
        let muzzle_ring = crate::atlas::generated::ATTACK_ONE_PAIR_MUZZLE_RING[0];

        assert_eq!(projectile.width().as_f32(), 32.0);
        assert_eq!(projectile.height().as_f32(), 32.0);
        assert_eq!(trail.width().as_f32(), 128.0);
        assert_eq!(trail.height().as_f32(), 64.0);
        assert_eq!(muzzle_ring.width().as_f32(), 64.0);
        assert_eq!(muzzle_ring.height().as_f32(), 64.0);
        assert_eq!(
            crate::atlas::generated::ATTACK_ONE_PAIR_MUZZLE_RING.len(),
            7
        );
    }
}
