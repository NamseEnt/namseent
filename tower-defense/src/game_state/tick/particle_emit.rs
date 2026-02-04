use super::*;

pub fn emit_damage_text_particles(
    game_state: &mut GameState,
    emitters: Vec<field_particle::emitter::DamageTextEmitter>,
) {
    if !emitters.is_empty() {
        let field_emitters = emitters
            .into_iter()
            .map(|emitter| field_particle::FieldParticleEmitter::DamageText { emitter })
            .collect::<Vec<_>>();
        game_state
            .field_particle_system_manager
            .add_emitters(field_emitters);
    }
}

pub fn emit_attack_effect_particles(
    game_state: &mut GameState,
    particles: Vec<field_particle::FieldParticle>,
) {
    if particles.is_empty() {
        return;
    }

    game_state.field_particle_system_manager.add_emitters(vec![
        field_particle::FieldParticleEmitter::TempParticle {
            emitter: field_particle::TempParticleEmitter::new(particles),
        },
    ]);
}

pub fn emit_monster_death_particles(
    game_state: &mut GameState,
    emitters: Vec<field_particle::emitter::MonsterDeathEmitter>,
) {
    if emitters.is_empty() {
        return;
    }

    let field_emitters = emitters
        .into_iter()
        .map(|emitter| field_particle::FieldParticleEmitter::MonsterDeath { emitter })
        .collect::<Vec<_>>();
    game_state
        .field_particle_system_manager
        .add_emitters(field_emitters);
}

pub fn emit_burning_trail_emitters(
    game_state: &mut GameState,
    emitters: Vec<field_particle::emitter::BurningTrailEmitter>,
) {
    if emitters.is_empty() {
        return;
    }

    let field_emitters = emitters
        .into_iter()
        .map(|emitter| field_particle::FieldParticleEmitter::BurningTrail { emitter })
        .collect::<Vec<_>>();
    game_state
        .field_particle_system_manager
        .add_emitters(field_emitters);
}
