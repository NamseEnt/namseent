mod emit;
pub mod emitter;
mod particle;

use crate::game_state::{
    GameState,
    field_particle::{
        emitter::FieldAreaEffectEmitter,
        particle::{FieldDamageAreaParticle, IconParticle},
    },
};
pub use emit::*;
use namui::{
    particle::{Emitter, Particle},
    *,
};

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

    fn add_system(&mut self, system: FieldParticleSystem) {
        self.systems.push(system);
    }

    fn remove_finished_field_particle_systems(&mut self, now: Instant) {
        self.systems.retain(|system| !system.is_done(now));
    }
}

pub enum FieldParticleEmitter {
    FieldAreaEffect { emitter: FieldAreaEffectEmitter },
}
impl Emitter<FieldParticle> for FieldParticleEmitter {
    fn emit(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticle> {
        match self {
            FieldParticleEmitter::FieldAreaEffect { emitter } => emitter.emit(now, dt),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            FieldParticleEmitter::FieldAreaEffect { emitter } => emitter.is_done(now),
        }
    }
}

#[derive(Clone)]
pub enum FieldParticle {
    Icon { particle: IconParticle },
    FieldDamageArea { particle: FieldDamageAreaParticle },
}
impl Particle<FieldParticleEmitter> for FieldParticle {
    fn tick(&mut self, now: Instant, dt: Duration) -> Vec<FieldParticleEmitter> {
        match self {
            FieldParticle::Icon { particle } => {
                particle.tick(now, dt);
                vec![]
            }
            FieldParticle::FieldDamageArea { particle } => {
                particle.tick(now, dt);
                vec![]
            }
        }
    }

    fn render(&self) -> RenderingTree {
        match self {
            FieldParticle::Icon { particle } => particle.render(),
            FieldParticle::FieldDamageArea { particle } => particle.render(),
        }
    }

    fn is_done(&self, now: Instant) -> bool {
        match self {
            FieldParticle::Icon { particle } => particle.is_done(now),
            FieldParticle::FieldDamageArea { particle } => particle.is_done(now),
        }
    }
}

pub fn remove_finished_field_particle_systems(game_state: &mut GameState, now: Instant) {
    game_state
        .field_particle_system_manager
        .remove_finished_field_particle_systems(now);
}
