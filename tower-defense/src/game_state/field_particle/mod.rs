pub mod atlas;
pub mod emitter;
pub mod particle;

pub use particle::{
    BlueDotSparkParticle, BurningTrailParticle, DamageTextParticle, EaseMode, EmberSparkParticle,
    IconParticle, InstantEmitParticle, InstantHitParticle, LaserBeamParticle, LaserLineParticle,
    LightningBoltParticle, MonsterCorpseParticle, MonsterSoulParticle, ProjectileParticle,
    TrashParticle,
};

pub static PROJECTILES: namui::particle::Emitter<ProjectileParticle> =
    namui::particle::Emitter::new();
pub static TRASHES: namui::particle::Emitter<TrashParticle> = namui::particle::Emitter::new();
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
