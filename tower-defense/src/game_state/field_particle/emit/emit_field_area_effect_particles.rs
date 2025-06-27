use crate::game_state::{
    GameState,
    field_area_effect::{FieldAreaEffectEnd, FieldAreaEffectKind},
    field_particle::{FieldParticleEmitter, FieldParticleSystem, emitter::FieldAreaEffectEmitter},
};

pub(super) fn emit_field_area_effect_particles(
    game_state: &mut GameState,
    kind: &FieldAreaEffectKind,
    end_at: &FieldAreaEffectEnd,
) {
    let emitter = FieldParticleEmitter::FieldAreaEffect {
        emitter: FieldAreaEffectEmitter::new(game_state.now(), kind.clone(), end_at.clone()),
    };
    let system = FieldParticleSystem::new(vec![emitter]);

    game_state.field_particle_system_manager.add_system(system);
}
