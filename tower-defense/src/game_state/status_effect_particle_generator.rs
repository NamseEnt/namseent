use super::GameState;
use super::field_particle::emitter::{
    spawn_monster_status_effect_icons, spawn_tower_status_effect_icons,
};
use super::monster::MonsterStatusEffectKind;
use crate::PresentationInstant;
use namui::*;
use rand::random;

const STATUS_EFFECT_TICK_INTERVAL_MS: i64 = 100;
const MONSTER_STATUS_EFFECT_PARTICLE_CHANCE: f32 = 0.2;
const TOWER_STATUS_EFFECT_PARTICLE_CHANCE: f32 = 0.2;

#[derive(Clone, State)]
pub struct StatusEffectParticleGenerator {
    pub last_tick_time: PresentationInstant,
}

impl StatusEffectParticleGenerator {
    pub fn new(presentation_instant: PresentationInstant) -> Self {
        Self {
            last_tick_time: presentation_instant,
        }
    }
}

pub fn tick_status_effect_particle_generator(
    game_state: &mut GameState,
    presentation_instant: PresentationInstant,
) {
    if presentation_instant
        .delta_since(game_state.status_effect_particle_generator.last_tick_time)
        .as_secs_f32()
        < STATUS_EFFECT_TICK_INTERVAL_MS as f32 / 1_000.0
    {
        return;
    }

    for monster in &game_state.monsters {
        for status_effect in &monster.status_effects {
            if should_create_monster_particle(status_effect.kind)
                && random::<f32>() < MONSTER_STATUS_EFFECT_PARTICLE_CHANCE
            {
                spawn_monster_status_effect_icons(
                    presentation_instant.as_namui(),
                    monster.center_xy_tile(),
                    status_effect.kind,
                );
            }
        }
    }

    for tower in game_state.towers.iter() {
        for status_effect in &tower.status_effects {
            if random::<f32>() < TOWER_STATUS_EFFECT_PARTICLE_CHANCE {
                spawn_tower_status_effect_icons(
                    presentation_instant.as_namui(),
                    tower.center_xy_f32(),
                    status_effect.kind,
                );
            }
        }
    }

    game_state.status_effect_particle_generator.last_tick_time = presentation_instant;
}

fn should_create_monster_particle(effect_kind: MonsterStatusEffectKind) -> bool {
    match effect_kind {
        MonsterStatusEffectKind::SpeedMul { mul } => mul < 1.0,
        MonsterStatusEffectKind::Invincible => true,
        MonsterStatusEffectKind::ImmuneToSlow => false,
    }
}
