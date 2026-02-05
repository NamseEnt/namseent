pub mod emitter;
pub mod particle;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::TILE_PX_SIZE;
use crate::game_state::{
    GameState,
    field_particle::emitter::{DamageTextEmitter, MonsterDeathEmitter, MonsterStatusEffectEmitter},
};
use namui::{
    particle::{Emitter, Particle},
    *,
};
pub use particle::{
    BurningTrailParticle, DamageTextParticle, EaseMode, EmberSparkParticle, IconParticle,
    InstantEmitParticle, InstantHitParticle, LaserBeamParticle, LaserLineParticle,
    MonsterCorpseParticle, MonsterSoulParticle, ProjectileParticle, TrashParticle,
};

#[derive(State)]
pub struct TempParticleEmitter {
    particles: Vec<FieldParticle>,
    emitted: bool,
}

impl TempParticleEmitter {
    pub fn new(particles: Vec<FieldParticle>) -> Self {
        Self {
            particles,
            emitted: false,
        }
    }

    pub fn emit(&mut self, _now: Instant, _dt: Duration) -> Vec<FieldParticle> {
        if self.emitted {
            return vec![];
        }
        self.emitted = true;
        std::mem::take(&mut self.particles)
    }

    pub fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}

#[derive(State)]
pub struct FieldParticleSystem {
    id: usize,
    system: namui::particle::System<FieldParticleEmitter, FieldParticle>,
}
impl FieldParticleSystem {
    pub fn new(emitters: Vec<FieldParticleEmitter>) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let system = namui::particle::System::new(emitters);
        Self { id, system }
    }
}
impl AsMut<namui::particle::System<FieldParticleEmitter, FieldParticle>> for FieldParticleSystem {
    fn as_mut(&mut self) -> &mut namui::particle::System<FieldParticleEmitter, FieldParticle> {
        &mut self.system
    }
}
impl AsRef<namui::particle::System<FieldParticleEmitter, FieldParticle>> for FieldParticleSystem {
    fn as_ref(&self) -> &namui::particle::System<FieldParticleEmitter, FieldParticle> {
        &self.system
    }
}

#[derive(Default, State)]
pub struct FieldParticleSystemManager {
    systems: Vec<FieldParticleSystem>,
}

impl FieldParticleSystemManager {
    pub fn render(&self, ctx: &ComposeCtx, now: Instant) {
        for system in &self.systems {
            ctx.compose_with_key(system.id, |ctx| {
                system.as_ref().render(&ctx, now);
            });
        }
    }

    pub fn add_system(&mut self, system: FieldParticleSystem) {
        self.systems.push(system);
    }

    pub fn add_emitters(&mut self, emitters: Vec<FieldParticleEmitter>) {
        if !emitters.is_empty() {
            let system = FieldParticleSystem::new(emitters);
            self.add_system(system);
        }
    }

    fn remove_finished_field_particle_systems(&mut self, now: Instant) {
        self.systems.retain(|system| !system.as_ref().is_done(now));
    }
}

#[derive(State)]
pub enum FieldParticleEmitter {
    TempParticle {
        emitter: TempParticleEmitter,
    },
    MonsterStatusEffect {
        emitter: MonsterStatusEffectEmitter,
    },
    DamageText {
        emitter: DamageTextEmitter,
    },
    MonsterDeath {
        emitter: MonsterDeathEmitter,
    },
    MonsterCorpse {
        emitter: TempParticleEmitter,
    },
    BurningTrail {
        emitter: emitter::BurningTrailEmitter,
    },
    TrashBurst {
        emitter: emitter::TrashBurstEmitter,
    },
    TrashBounce {
        emitter: emitter::TrashBounceEmitter,
    },
    TrashRain {
        emitter: emitter::TrashRainEmitter,
    },
    ProjectileParticle {
        emitter: emitter::ProjectileParticleEmitter,
    },
    LaserBeam {
        emitter: emitter::LaserBeamEmitter,
    },
}
impl Emitter<FieldParticle> for FieldParticleEmitter {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticle> {
        match self {
            FieldParticleEmitter::TempParticle { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::MonsterStatusEffect { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::DamageText { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::MonsterDeath { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::MonsterCorpse { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::BurningTrail { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::TrashBurst { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::TrashBounce { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::TrashRain { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::ProjectileParticle { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::LaserBeam { emitter } => emitter.emit(now, dt),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            FieldParticleEmitter::TempParticle { emitter } => emitter.is_done(now),
            FieldParticleEmitter::MonsterStatusEffect { emitter } => emitter.is_done(now),
            FieldParticleEmitter::DamageText { emitter } => emitter.is_done(now),
            FieldParticleEmitter::MonsterDeath { emitter } => emitter.is_done(now),
            FieldParticleEmitter::MonsterCorpse { emitter } => emitter.is_done(now),
            FieldParticleEmitter::BurningTrail { emitter } => emitter.is_done(now),
            FieldParticleEmitter::TrashBurst { emitter } => emitter.is_done(now),
            FieldParticleEmitter::TrashBounce { emitter } => emitter.is_done(now),
            FieldParticleEmitter::TrashRain { emitter } => emitter.is_done(now),
            FieldParticleEmitter::ProjectileParticle { emitter } => emitter.is_done(now),
            FieldParticleEmitter::LaserBeam { emitter } => emitter.is_done(now),
        }
    }
}

#[derive(Clone, State)]
pub enum FieldParticle {
    Icon { particle: IconParticle },
    DamageText { particle: DamageTextParticle },
    MonsterDeath { particle: MonsterSoulParticle },
    MonsterCorpse { particle: MonsterCorpseParticle },
    BurningTrail { particle: BurningTrailParticle },
    EmberSpark { particle: EmberSparkParticle },
    LaserBeam { particle: LaserBeamParticle },
    InstantEmit { particle: InstantEmitParticle },
    InstantHit { particle: InstantHitParticle },
    Trash { particle: TrashParticle },
    Projectile { particle: ProjectileParticle },
    LaserLine { particle: LaserLineParticle },
}
impl Particle<FieldParticleEmitter> for FieldParticle {
    fn tick(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticleEmitter> {
        match self {
            FieldParticle::Icon { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::DamageText { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::MonsterDeath { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::MonsterCorpse { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::BurningTrail { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::EmberSpark { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::LaserBeam { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::InstantEmit { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::InstantHit { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::Trash { particle } => particle.tick(now, dt),
            FieldParticle::Projectile { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::LaserLine { particle } => {
                particle.tick(now, dt);
                vec![]
            }
        }
    }

    fn render(&self) -> RenderingTree {
        match self {
            FieldParticle::Icon { particle } => particle.render(),
            FieldParticle::DamageText { particle } => particle.render(),
            FieldParticle::MonsterDeath { particle } => particle.render(),
            FieldParticle::MonsterCorpse { particle } => particle.render(),
            FieldParticle::BurningTrail { particle } => particle.render(),
            FieldParticle::EmberSpark { particle } => particle.render(),
            FieldParticle::LaserBeam { particle } => particle.render(),
            FieldParticle::InstantEmit { particle } => particle.render(),
            FieldParticle::InstantHit { particle } => particle.render(),
            FieldParticle::Trash { particle } => particle.render(),
            FieldParticle::Projectile { particle } => {
                // Render projectile using the same logic as the main projectile rendering
                render_projectile_particle(particle)
            }
            FieldParticle::LaserLine { particle } => particle.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            FieldParticle::Icon { particle } => particle.is_done(now),
            FieldParticle::DamageText { particle } => particle.is_done(now),
            FieldParticle::MonsterDeath { particle } => particle.is_done(now),
            FieldParticle::MonsterCorpse { particle } => particle.is_done(now),
            FieldParticle::BurningTrail { particle } => particle.is_done(now),
            FieldParticle::EmberSpark { particle } => particle.is_done(now),
            FieldParticle::LaserBeam { particle } => particle.is_done(now),
            FieldParticle::InstantEmit { particle } => particle.is_done(now),
            FieldParticle::InstantHit { particle } => particle.is_done(now),
            FieldParticle::Trash { particle } => particle.is_done(now),
            FieldParticle::Projectile { particle } => !particle.is_alive(now),            FieldParticle::LaserLine { particle } => particle.is_done(now),        }
    }
}

pub fn remove_finished_field_particle_systems(game_state: &mut GameState, now: Instant) {
    game_state
        .field_particle_system_manager
        .remove_finished_field_particle_systems(now);
}
fn render_projectile_particle(particle: &ProjectileParticle) -> RenderingTree {
    let projectile_wh = TILE_PX_SIZE * Wh::new(0.4, 0.4);
    let image = particle.kind.image();
    let half_wh = projectile_wh / 2.0;

    let tile_px = TILE_PX_SIZE.to_xy();
    let particle_px_xy = tile_px * Xy::new(particle.xy.x, particle.xy.y);

    namui::translate(
        particle_px_xy.x,
        particle_px_xy.y,
        namui::rotate(
            particle.rotation,
            namui::translate(
                -half_wh.width,
                -half_wh.height,
                namui::image(ImageParam {
                    rect: Rect::from_xy_wh(Xy::zero(), projectile_wh),
                    image,
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: None,
                    },
                }),
            ),
        ),
    )
}
