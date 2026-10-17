mod high_projectile;
mod high_trail;

pub use high_projectile::*;
pub use high_trail::*;
use namui::*;

#[derive(Clone)]
/// migrating
pub enum NewAttackParticle {
    HighProjectile(HighProjectile),
    HighTrail(HighTrail),
}

impl namui::particle::Particle for NewAttackParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        match self {
            NewAttackParticle::HighProjectile(p) => p.tick(now, dt),
            NewAttackParticle::HighTrail(p) => p.tick(now, dt),
        }
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        match self {
            NewAttackParticle::HighProjectile(p) => p.render(),
            NewAttackParticle::HighTrail(p) => p.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            NewAttackParticle::HighProjectile(p) => p.is_done(now),
            NewAttackParticle::HighTrail(p) => p.is_done(now),
        }
    }
}
