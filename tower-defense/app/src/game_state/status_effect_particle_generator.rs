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
    generator: &mut StatusEffectParticleGenerator,
    presentation_instant: PresentationInstant,
) {
    if presentation_instant
        .delta_since(generator.last_tick_time)
        .as_secs_f32()
        < STATUS_EFFECT_TICK_INTERVAL_MS as f32 / 1_000.0
    {
        return;
    }

    let raw_state = game_state.raw_core_state();
    for monster in raw_state.monsters() {
        for status_effect in &monster.status_effects {
            let Some(kind) = monster_status_effect_kind(status_effect) else {
                continue;
            };
            if should_create_monster_particle(kind)
                && random::<f32>() < MONSTER_STATUS_EFFECT_PARTICLE_CHANCE
            {
                spawn_monster_status_effect_icons(
                    presentation_instant.as_namui(),
                    crate::MapCoordF32::new(
                        (monster.move_on_route.map_coord[0] + td_core::WORLD_UNITS_PER_TILE / 2)
                            as f32
                            / td_core::WORLD_UNITS_PER_TILE as f32,
                        (monster.move_on_route.map_coord[1] + td_core::WORLD_UNITS_PER_TILE / 2)
                            as f32
                            / td_core::WORLD_UNITS_PER_TILE as f32,
                    ),
                    kind,
                );
            }
        }
    }

    for tower in raw_state.towers() {
        for status_effect in &tower.status_effects {
            let Some(kind) = tower_status_effect_kind(status_effect) else {
                continue;
            };
            if random::<f32>() < TOWER_STATUS_EFFECT_PARTICLE_CHANCE {
                spawn_tower_status_effect_icons(
                    presentation_instant.as_namui(),
                    crate::MapCoordF32::new(
                        tower.left_top[0] as f32 + 1.0,
                        tower.left_top[1] as f32 + 1.0,
                    ),
                    kind,
                );
            }
        }
    }

    fn monster_status_effect_kind(
        effect: &td_core::MonsterStatusEffect,
    ) -> Option<MonsterStatusEffectKind> {
        Some(match effect.kind {
            td_core::MonsterStatusEffectKind::SpeedMul { mul_raw } => {
                MonsterStatusEffectKind::SpeedMul {
                    mul: crate::FixedRatio::from_raw(mul_raw),
                }
            }
            td_core::MonsterStatusEffectKind::Invincible => MonsterStatusEffectKind::Invincible,
            td_core::MonsterStatusEffectKind::ImmuneToSlow => MonsterStatusEffectKind::ImmuneToSlow,
        })
    }

    fn tower_status_effect_kind(
        effect: &td_core::TowerStatusEffect,
    ) -> Option<crate::game_state::tower::TowerStatusEffectKind> {
        Some(match effect.kind {
            td_core::TowerStatusEffectKind::DamageMul { mul_raw } => {
                crate::game_state::tower::TowerStatusEffectKind::DamageMul {
                    mul: crate::FixedRatio::from_raw(mul_raw),
                }
            }
            td_core::TowerStatusEffectKind::DamageAdd { add_raw } => {
                crate::game_state::tower::TowerStatusEffectKind::DamageAdd {
                    add: crate::DamageDelta::from_raw(add_raw),
                }
            }
        })
    }

    generator.last_tick_time = presentation_instant;
}

fn should_create_monster_particle(effect_kind: MonsterStatusEffectKind) -> bool {
    match effect_kind {
        MonsterStatusEffectKind::SpeedMul { mul } => mul < crate::FixedRatio::ONE,
        MonsterStatusEffectKind::Invincible => true,
        MonsterStatusEffectKind::ImmuneToSlow => false,
    }
}
