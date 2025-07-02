use crate::game_state::{
    GameState,
    field_area_effect::FieldAreaEffectKind,
    field_particle::{FieldParticleEmitter, FieldParticleSystem, emitter::FieldAreaEffectEmitter},
    schedule::CountBasedSchedule,
};

pub fn emit_field_area_effect_particles(
    game_state: &mut GameState,
    kind: &FieldAreaEffectKind,
    schedule: &CountBasedSchedule,
) {
    let emitter = FieldParticleEmitter::FieldAreaEffect {
        emitter: FieldAreaEffectEmitter::new(game_state.now(), kind.clone(), schedule.clone()),
    };
    let system = FieldParticleSystem::new(vec![emitter]);

    game_state.field_particle_system_manager.add_system(system);
}
