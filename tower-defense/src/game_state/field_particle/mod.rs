mod emit;
pub mod emitter;
mod particle;

use self::emitter::FieldAreaEffectEmitter;
use self::particle::{FieldAreaParticle, IconParticle};
use crate::game_state::GameState;
pub use emit::*;
use namui::{
    particle::{Emitter, Particle},
    *,
};

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

pub type FieldParticleSystem = namui::particle::System<FieldParticleEmitter, FieldParticle>;

#[derive(Default)]
pub struct FieldParticleSystemManager {
    systems: Vec<FieldParticleSystem>,
}

impl FieldParticleSystemManager {
    pub fn render(&self, ctx: &ComposeCtx, now: Instant) {
        for system in &self.systems {
            system.render(ctx, now);
        }
    }

    pub fn add_system(&mut self, system: FieldParticleSystem) {
        self.systems.push(system);
    }

    pub fn add_emitter(&mut self, emitter: FieldParticleEmitter) {
        let system = FieldParticleSystem::new(vec![emitter]);
        self.add_system(system);
    }

    pub fn add_particles(&mut self, particles: Vec<FieldParticle>) {
        if !particles.is_empty() {
            // Create a temporary emitter that emits all particles at once
            let temp_emitter = TempParticleEmitter::new(particles);
            let emitter = FieldParticleEmitter::TempParticle {
                emitter: temp_emitter,
            };
            self.add_emitter(emitter);
        }
    }

    fn remove_finished_field_particle_systems(&mut self, now: Instant) {
        self.systems.retain(|system| !system.is_done(now));
    }
}

pub enum FieldParticleEmitter {
    FieldAreaEffect { emitter: FieldAreaEffectEmitter },
    TempParticle { emitter: TempParticleEmitter },
}
impl Emitter<FieldParticle> for FieldParticleEmitter {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticle> {
        match self {
            FieldParticleEmitter::FieldAreaEffect { emitter } => emitter.emit(now, dt),
            FieldParticleEmitter::TempParticle { emitter } => emitter.emit(now, dt),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            FieldParticleEmitter::FieldAreaEffect { emitter } => emitter.is_done(now),
            FieldParticleEmitter::TempParticle { emitter } => emitter.is_done(now),
        }
    }
}

#[derive(Clone)]
pub enum FieldParticle {
    Icon { particle: IconParticle },
    FieldArea { particle: FieldAreaParticle },
}
impl Particle<FieldParticleEmitter> for FieldParticle {
    fn tick(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticleEmitter> {
        match self {
            FieldParticle::Icon { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::FieldArea { particle } => {
                particle.tick(now, dt);
                vec![]
            }
        }
    }

    fn render(&self) -> RenderingTree {
        match self {
            FieldParticle::Icon { particle } => particle.render(),
            FieldParticle::FieldArea { particle } => particle.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            FieldParticle::Icon { particle } => particle.is_done(now),
            FieldParticle::FieldArea { particle } => particle.is_done(now),
        }
    }
}

pub fn remove_finished_field_particle_systems(game_state: &mut GameState, now: Instant) {
    game_state
        .field_particle_system_manager
        .remove_finished_field_particle_systems(now);
}
