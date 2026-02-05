use super::{
    GameState,
    field_particle::{
        FieldParticleEmitter,
        emitter::MonsterStatusEffectEmitter,
    },
    monster::MonsterStatusEffectKind,
    tower::TowerStatusEffectKind,
};
use crate::MapCoordF32;
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

    let mut all_emitters = Vec::new();

    generate_monster_status_effect_emitters(game_state, now, &mut all_emitters);
    generate_tower_status_effect_emitters(game_state, now, &mut all_emitters);

    if !all_emitters.is_empty() {
        game_state
            .field_particle_system_manager
            .add_emitters(all_emitters);
    }

    game_state.status_effect_particle_generator.last_tick_time = now;
}

fn generate_monster_status_effect_emitters(
    game_state: &GameState,
    now: Instant,
    emitters: &mut Vec<FieldParticleEmitter>,
) {
    for monster in &game_state.monsters {
        for status_effect in &monster.status_effects {
            if should_create_monster_particle(status_effect.kind)
                && random::<f32>() < MONSTER_STATUS_EFFECT_PARTICLE_CHANCE
            {
                let monster_emitter = MonsterStatusEffectEmitter::new_with_default_duration(
                    now,
                    monster.center_xy_tile(),
                    status_effect.kind,
                );
                let field_particle_emitter = FieldParticleEmitter::MonsterStatusEffect {
                    emitter: monster_emitter,
                };
                emitters.push(field_particle_emitter);
            }
        }
    }
}

fn generate_tower_status_effect_emitters(
    game_state: &GameState,
    now: Instant,
    emitters: &mut Vec<FieldParticleEmitter>,
) {
    for tower in game_state.towers.iter() {
        for status_effect in &tower.status_effects {
            if random::<f32>() < TOWER_STATUS_EFFECT_PARTICLE_CHANCE {
                let buff_kind = convert_tower_effect_to_area_effect_kind(status_effect.kind);
                let tower_emitter = TowerStatusEffectEmitter::new_with_default_duration(
                    now,
                    tower.center_xy_f32(),
                    buff_kind,
                );
                let field_particle_emitter = FieldParticleEmitter::TowerStatusEffect {
                    emitter: tower_emitter,
                };
                emitters.push(field_particle_emitter);
            }
        }
    }
}

fn should_create_monster_particle(effect_kind: MonsterStatusEffectKind) -> bool {
    match effect_kind {
        MonsterStatusEffectKind::SpeedMul { mul } => mul < 1.0,
        MonsterStatusEffectKind::Invincible => true,
        MonsterStatusEffectKind::ImmuneToSlow => false,
    }
}

fn convert_tower_effect_to_area_effect_kind(
    effect_kind: TowerStatusEffectKind,
) -> FieldAreaEffectKind {
    match effect_kind {
        TowerStatusEffectKind::DamageAdd { add } => {
            FieldAreaEffectKind::TowerAttackPowerPlusBuffOverTime {
                amount: add,
                xy: MapCoordF32::new(0.0, 0.0),
                radius: 1.0,
            }
        }
        TowerStatusEffectKind::DamageMul { mul } => {
            FieldAreaEffectKind::TowerAttackPowerMultiplyBuffOverTime {
                amount: mul,
                xy: MapCoordF32::new(0.0, 0.0),
                radius: 1.0,
            }
        }
        TowerStatusEffectKind::AttackSpeedAdd { add } => {
            FieldAreaEffectKind::TowerAttackSpeedPlusBuffOverTime {
                amount: add,
                xy: MapCoordF32::new(0.0, 0.0),
                radius: 1.0,
            }
        }
        TowerStatusEffectKind::AttackSpeedMul { mul } => {
            FieldAreaEffectKind::TowerAttackSpeedMultiplyBuffOverTime {
                amount: mul,
                xy: MapCoordF32::new(0.0, 0.0),
                radius: 1.0,
            }
        }
        TowerStatusEffectKind::AttackRangeAdd { add } => {
            FieldAreaEffectKind::TowerAttackRangePlusBuffOverTime {
                amount: add,
                xy: MapCoordF32::new(0.0, 0.0),
                radius: 1.0,
            }
        }
    }
}
