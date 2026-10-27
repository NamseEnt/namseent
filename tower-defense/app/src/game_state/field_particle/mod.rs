pub mod atlas;
pub mod emitter;
pub mod particle;

use namui::{Duration, Instant};

pub use particle::{
    DamageTextParticle, DustParticle, DustParticleConfig, IconParticle, MonsterCorpseParticle,
    MonsterSoulParticle,
};

pub fn spawn_dust_particle(particle: DustParticle) {
    DUSTS.spawn(particle);
}

pub struct EmitterWrapper<T: namui::particle::Particle>(pub namui::particle::Emitter<T>);
impl<T: namui::particle::Particle> std::ops::Deref for EmitterWrapper<T> {
    type Target = namui::particle::Emitter<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: namui::particle::Particle> EmitterWrapper<T> {
    pub const fn new() -> Self {
        Self(namui::particle::Emitter::new())
    }
    pub fn spawn(&self, particle: T) {
        self.0.spawn(particle);
    }
    pub fn tick(&self, now: Instant, dt: Duration) {
        self.0.tick(now, dt);
    }
}

impl<T: namui::particle::Particle> Default for EmitterWrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub static ATTACK_VISUAL: EmitterWrapper<particle::attack::NewAttackParticle> =
    EmitterWrapper::new();
pub static MONSTER_SOULS: EmitterWrapper<MonsterSoulParticle> = EmitterWrapper::new();
pub static MONSTER_CORPSES: EmitterWrapper<MonsterCorpseParticle> = EmitterWrapper::new();
pub static ICONS: EmitterWrapper<IconParticle> = EmitterWrapper::new();
pub static DAMAGE_TEXTS: EmitterWrapper<DamageTextParticle> = EmitterWrapper::new();
pub static DUSTS: EmitterWrapper<DustParticle> = EmitterWrapper::new();

pub fn tick_all_emitters(now: Instant, dt: Duration) {
    ATTACK_VISUAL.tick(now, dt);
    MONSTER_SOULS.tick(now, dt);
    MONSTER_CORPSES.tick(now, dt);
    ICONS.tick(now, dt);
    DAMAGE_TEXTS.tick(now, dt);
    DUSTS.tick(now, dt);
}
