pub mod atlas;
pub mod emitter;
pub mod particle;

use namui::{Duration, Instant};

pub use particle::{
    BlackSmokeParticle, BlueSparkParticle, BurningTrailParticle, CardParticle, DamageTextParticle,
    DustParticle, DustParticleConfig, EaseMode, EmberSparkParticle, HeartParticle, IconParticle,
    LaserLineParticle, LightningBoltParticle, MonsterCorpseParticle, MonsterSoulParticle,
    ProjectileParticle, RedSlashParticle, SparkleParticle, TrashParticle, WindCurveTrailParticle,
    YellowExplosionParticle,
};

#[derive(Clone)]
pub enum AttackParticle {
    BurningTrail(BurningTrailParticle),
    EmberSpark(EmberSparkParticle),
    LaserLine(LaserLineParticle),
    LightningBolt(LightningBoltParticle),
    BlueDotSpark(BlueSparkParticle),
    Sparkle(SparkleParticle),
    WindCurveTrail(WindCurveTrailParticle),
    RedSlash(RedSlashParticle),
    YellowExplosion(YellowExplosionParticle),
}

impl namui::particle::Particle for AttackParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        match self {
            AttackParticle::BurningTrail(p) => p.tick(now, dt),
            AttackParticle::EmberSpark(p) => p.tick(now, dt),
            AttackParticle::LaserLine(p) => p.tick(now, dt),
            AttackParticle::LightningBolt(p) => p.tick(now, dt),
            AttackParticle::BlueDotSpark(p) => p.tick(now, dt),
            AttackParticle::Sparkle(p) => p.tick(now, dt),
            AttackParticle::WindCurveTrail(p) => p.tick(now, dt),
            AttackParticle::RedSlash(p) => p.tick(now, dt),
            AttackParticle::YellowExplosion(p) => p.tick(now, dt),
        }
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        match self {
            AttackParticle::BurningTrail(p) => p.render(),
            AttackParticle::EmberSpark(p) => p.render(),
            AttackParticle::LaserLine(p) => p.render(),
            AttackParticle::LightningBolt(p) => p.render(),
            AttackParticle::BlueDotSpark(p) => p.render(),
            AttackParticle::Sparkle(p) => p.render(),
            AttackParticle::WindCurveTrail(p) => p.render(),
            AttackParticle::RedSlash(p) => p.render(),
            AttackParticle::YellowExplosion(p) => p.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            AttackParticle::BurningTrail(p) => p.is_done(now),
            AttackParticle::EmberSpark(p) => p.is_done(now),
            AttackParticle::LaserLine(p) => p.is_done(now),
            AttackParticle::LightningBolt(p) => p.is_done(now),
            AttackParticle::BlueDotSpark(p) => p.is_done(now),
            AttackParticle::Sparkle(p) => p.is_done(now),
            AttackParticle::WindCurveTrail(p) => p.is_done(now),
            AttackParticle::RedSlash(p) => p.is_done(now),
            AttackParticle::YellowExplosion(p) => p.is_done(now),
        }
    }
}

pub fn spawn_burning_trail(particle: BurningTrailParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::BurningTrail(particle));
}

pub fn spawn_ember_spark(particle: EmberSparkParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::EmberSpark(particle));
}

pub fn spawn_laser_line(particle: LaserLineParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::LaserLine(particle));
}

pub fn spawn_lightning_bolt(particle: LightningBoltParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::LightningBolt(particle));
}

pub fn spawn_blue_dot_spark(particle: BlueSparkParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::BlueDotSpark(particle));
}

pub fn spawn_sparkle(particle: SparkleParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::Sparkle(particle));
}

pub fn spawn_wind_curve_trail_particle(particle: WindCurveTrailParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::WindCurveTrail(particle));
}

pub fn spawn_red_slash(particle: RedSlashParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::RedSlash(particle));
}

pub fn spawn_yellow_explosion(particle: YellowExplosionParticle) {
    ATTACK_PARTICLES.spawn(AttackParticle::YellowExplosion(particle));
}

pub fn spawn_black_smoke_particle(particle: BlackSmokeParticle) {
    BLACK_SMOKES.spawn(particle);
}

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
        if !crate::is_headless() {
            self.0.spawn(particle);
        }
    }
    pub fn tick(&self, now: Instant, dt: Duration) {
        self.0.tick(now, dt);
    }
}

pub static PROJECTILES: EmitterWrapper<ProjectileParticle> = EmitterWrapper::new();
pub static TRASHES: EmitterWrapper<TrashParticle> = EmitterWrapper::new();
pub static CARDS: EmitterWrapper<CardParticle> = EmitterWrapper::new();
pub static MONSTER_SOULS: EmitterWrapper<MonsterSoulParticle> = EmitterWrapper::new();
pub static MONSTER_CORPSES: EmitterWrapper<MonsterCorpseParticle> = EmitterWrapper::new();
pub static ICONS: EmitterWrapper<IconParticle> = EmitterWrapper::new();
pub static DAMAGE_TEXTS: EmitterWrapper<DamageTextParticle> = EmitterWrapper::new();
pub static ATTACK_PARTICLES: EmitterWrapper<AttackParticle> = EmitterWrapper::new();
pub static HEARTS: EmitterWrapper<HeartParticle> = EmitterWrapper::new();
pub static BLACK_SMOKES: EmitterWrapper<BlackSmokeParticle> = EmitterWrapper::new();
pub static DUSTS: EmitterWrapper<DustParticle> = EmitterWrapper::new();

pub fn tick_all_emitters(now: Instant, dt: Duration) {
    ATTACK_PARTICLES.tick(now, dt);
    PROJECTILES.tick(now, dt);
    TRASHES.tick(now, dt);
    CARDS.tick(now, dt);
    MONSTER_SOULS.tick(now, dt);
    MONSTER_CORPSES.tick(now, dt);
    ICONS.tick(now, dt);
    DAMAGE_TEXTS.tick(now, dt);
    HEARTS.tick(now, dt);
    BLACK_SMOKES.tick(now, dt);
    DUSTS.tick(now, dt);
}
