pub mod atlas;
pub mod emitter;
pub mod particle;

use namui::{Duration, Instant};

pub use particle::{
    BlueDotSparkParticle, BurningTrailParticle, CardParticle, DamageTextParticle, EaseMode,
    EmberSparkParticle, HeartParticle, IconParticle, InstantEmitParticle, InstantHitParticle,
    LaserBeamParticle, LaserLineParticle, LightningBoltParticle, MonsterCorpseParticle,
    MonsterSoulParticle, ProjectileParticle, SparkleParticle, TrashParticle,
    WindCurveTrailParticle,
};

pub static PROJECTILES: namui::particle::Emitter<ProjectileParticle> =
    namui::particle::Emitter::new();
pub static TRASHES: namui::particle::Emitter<TrashParticle> = namui::particle::Emitter::new();
pub static CARDS: namui::particle::Emitter<CardParticle> = namui::particle::Emitter::new();
pub static BURNING_TRAILS: namui::particle::Emitter<BurningTrailParticle> =
    namui::particle::Emitter::new();
pub static EMBER_SPARKS: namui::particle::Emitter<EmberSparkParticle> =
    namui::particle::Emitter::new();
pub static MONSTER_SOULS: namui::particle::Emitter<MonsterSoulParticle> =
    namui::particle::Emitter::new();
pub static MONSTER_CORPSES: namui::particle::Emitter<MonsterCorpseParticle> =
    namui::particle::Emitter::new();
pub static ICONS: namui::particle::Emitter<IconParticle> = namui::particle::Emitter::new();
pub static DAMAGE_TEXTS: namui::particle::Emitter<DamageTextParticle> =
    namui::particle::Emitter::new();
pub static BLUE_DOT_SPARKS: namui::particle::Emitter<BlueDotSparkParticle> =
    namui::particle::Emitter::new();
pub static LASER_LINES: namui::particle::Emitter<LaserLineParticle> =
    namui::particle::Emitter::new();
pub static INSTANT_EMITS: namui::particle::Emitter<InstantEmitParticle> =
    namui::particle::Emitter::new();
pub static INSTANT_HITS: namui::particle::Emitter<InstantHitParticle> =
    namui::particle::Emitter::new();
pub static LIGHTNING_BOLTS: namui::particle::Emitter<LightningBoltParticle> =
    namui::particle::Emitter::new();
pub static LASER_BEAMS: namui::particle::Emitter<LaserBeamParticle> =
    namui::particle::Emitter::new();
pub static SPARKLES: namui::particle::Emitter<SparkleParticle> =
    namui::particle::Emitter::new();
pub static WIND_CURVE_TRAILS: namui::particle::Emitter<WindCurveTrailParticle> =
    namui::particle::Emitter::new();
pub static HEARTS: namui::particle::Emitter<HeartParticle> = namui::particle::Emitter::new();

pub fn tick_all_emitters(now: Instant, dt: Duration) {
    BURNING_TRAILS.tick(now, dt);
    EMBER_SPARKS.tick(now, dt);
    PROJECTILES.tick(now, dt);
    TRASHES.tick(now, dt);
    CARDS.tick(now, dt);
    MONSTER_SOULS.tick(now, dt);
    MONSTER_CORPSES.tick(now, dt);
    ICONS.tick(now, dt);
    DAMAGE_TEXTS.tick(now, dt);
    BLUE_DOT_SPARKS.tick(now, dt);
    LASER_LINES.tick(now, dt);
    INSTANT_EMITS.tick(now, dt);
    INSTANT_HITS.tick(now, dt);
    LIGHTNING_BOLTS.tick(now, dt);
    LASER_BEAMS.tick(now, dt);
    SPARKLES.tick(now, dt);
    WIND_CURVE_TRAILS.tick(now, dt);
    HEARTS.tick(now, dt);
}
