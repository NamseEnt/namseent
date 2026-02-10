use super::GameState;
use super::field_particle::emitter::{
    spawn_monster_status_effect_icons, spawn_tower_status_effect_icons,
};
use super::monster::MonsterStatusEffectKind;
use namui::{Duration, Instant};
use rand::random;

const STATUS_EFFECT_TICK_INTERVAL_MS: i64 = 100;
const MONSTER_STATUS_EFFECT_PARTICLE_CHANCE: f32 = 0.2;
const TOWER_STATUS_EFFECT_PARTICLE_CHANCE: f32 = 0.2;

pub struct StatusEffectParticleGenerator {
    pub last_tick_time: Instant,
}

impl StatusEffectParticleGenerator {
    pub fn new(now: Instant) -> Self {
        Self {
            last_tick_time: now,
        }
    }
}

pub fn tick_status_effect_particle_generator(game_state: &mut GameState, now: Instant) {
    if now - game_state.status_effect_particle_generator.last_tick_time
        < Duration::from_millis(STATUS_EFFECT_TICK_INTERVAL_MS)
    {
        return;
    }

    for monster in &game_state.monsters {
        for status_effect in &monster.status_effects {
            if should_create_monster_particle(status_effect.kind)
                && random::<f32>() < MONSTER_STATUS_EFFECT_PARTICLE_CHANCE
            {
                spawn_monster_status_effect_icons(
                    now,
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
                    now,
                    tower.center_xy_f32(),
                    status_effect.kind,
                );
            }
        }
    }

    game_state.status_effect_particle_generator.last_tick_time = now;
}

fn should_create_monster_particle(effect_kind: MonsterStatusEffectKind) -> bool {
    match effect_kind {
        MonsterStatusEffectKind::SpeedMul { mul } => mul < 1.0,
        MonsterStatusEffectKind::Invincible => true,
        MonsterStatusEffectKind::ImmuneToSlow => false,
    }
}
