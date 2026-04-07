pub mod atlas;
pub mod emitter;
pub mod particle;

use namui::{Duration, Instant};

pub use particle::{
    BlackSmokeParticle, BlueSparkParticle, BurningTrailParticle, CardParticle, DamageTextParticle,
    DustParticle, DustParticleConfig, EaseMode, EmberSparkParticle, HeartParticle, IconParticle,
    LaserLineParticle,
    LightningBoltParticle, MonsterCorpseParticle, MonsterSoulParticle, ProjectileParticle,
    RedSlashParticle, SparkleParticle, TrashParticle, WindCurveTrailParticle,
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

pub static PROJECTILES: namui::particle::Emitter<ProjectileParticle> =
    namui::particle::Emitter::new();
pub static TRASHES: namui::particle::Emitter<TrashParticle> = namui::particle::Emitter::new();
pub static CARDS: namui::particle::Emitter<CardParticle> = namui::particle::Emitter::new();
pub static MONSTER_SOULS: namui::particle::Emitter<MonsterSoulParticle> =
    namui::particle::Emitter::new();
pub static MONSTER_CORPSES: namui::particle::Emitter<MonsterCorpseParticle> =
    namui::particle::Emitter::new();
pub static ICONS: namui::particle::Emitter<IconParticle> = namui::particle::Emitter::new();
pub static DAMAGE_TEXTS: namui::particle::Emitter<DamageTextParticle> =
    namui::particle::Emitter::new();
pub static ATTACK_PARTICLES: namui::particle::Emitter<AttackParticle> =
    namui::particle::Emitter::new();
pub static HEARTS: namui::particle::Emitter<HeartParticle> = namui::particle::Emitter::new();
pub static BLACK_SMOKES: namui::particle::Emitter<BlackSmokeParticle> =
    namui::particle::Emitter::new();
pub static DUSTS: namui::particle::Emitter<DustParticle> = namui::particle::Emitter::new();

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
