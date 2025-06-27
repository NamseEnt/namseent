mod emit_field_area_effect_particles;

use crate::game_state::{
    GameState, field_area_effect::FieldAreaEffect,
    field_particle::emit::emit_field_area_effect_particles::emit_field_area_effect_particles,
};

pub fn emit_field_particle(game_state: &mut GameState, kind: FieldParticleKind) {
    match kind {
        FieldParticleKind::FieldAreaEffect { field_area_effect } => {
            emit_field_area_effect_particles(
                game_state,
                &field_area_effect.kind,
                &field_area_effect.end_at,
            );
        }
    }
}

pub enum FieldParticleKind<'a> {
    FieldAreaEffect {
        field_area_effect: &'a FieldAreaEffect,
    },
}
