#[cfg(test)]
use crate::{DamageHit, DamageHitResult};
use crate::{
    MonsterStatusEffect, MonsterStatusEffectKind, MoveOnRouteState, RATIO_SCALE,
    advance_move_on_route, move_on_route_is_finished, multiply_ratio_raw, reset_move_on_route,
};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MonsterSkillKind {
    Invincible,
    SpeedMul { mul_raw: i64 },
    ImmuneToSlow,
    HealByMaxHp { ratio_raw: i64 },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MonsterSkillTarget {
    MySelf,
    AllMonsters,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterSkillTemplate {
    pub kind: MonsterSkillKind,
    pub target: MonsterSkillTarget,
    pub cooldown: u64,
    pub duration: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterSkill {
    pub last_used_at: u64,
    pub template: MonsterSkillTemplate,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterState {
    pub id: u64,
    pub move_on_route: MoveOnRouteState,
    pub kind: u8,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
    pub stage_progress_counted: bool,
    pub skills: Vec<MonsterSkill>,
    pub status_effects: Vec<MonsterStatusEffect>,
    pub damage_raw: i64,
    pub reward: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActivatedMonsterSkill {
    pub monster_id: u64,
    pub skill: MonsterSkill,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MonsterDamageResult {
    pub applied_damage_raw: i64,
    pub dead: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MonsterDeathResult {
    pub remaining_hp_raw: i64,
    pub should_count_stage_progress: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MonsterEscapeResult {
    pub damage_raw: i64,
    pub escaped_hp_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemovedMonster {
    pub monster: MonsterState,
    pub death: MonsterDeathResult,
}

pub(crate) fn advance_monster_states(
    monsters: &mut [MonsterState],
    enemy_speed_multiplier_raw: i64,
) {
    for monster in monsters {
        let status_speed_multiplier_raw = monster
            .status_effects
            .iter()
            .filter_map(|effect| match effect.kind {
                MonsterStatusEffectKind::SpeedMul { mul_raw } => Some(mul_raw),
                MonsterStatusEffectKind::Invincible | MonsterStatusEffectKind::ImmuneToSlow => None,
            })
            .fold(RATIO_SCALE, multiply_ratio_raw);
        let speed_raw = multiply_ratio_raw(
            multiply_ratio_raw(
                monster.move_on_route.velocity_raw,
                status_speed_multiplier_raw,
            ),
            enemy_speed_multiplier_raw,
        );
        advance_move_on_route(&mut monster.move_on_route, speed_raw);
    }
}

pub(crate) fn apply_monster_damage(
    monster: &mut MonsterState,
    damage_raw: i64,
) -> MonsterDamageResult {
    if monster.hp_raw <= 0
        || monster
            .status_effects
            .iter()
            .any(|effect| matches!(effect.kind, MonsterStatusEffectKind::Invincible))
    {
        return MonsterDamageResult {
            applied_damage_raw: 0,
            dead: monster.hp_raw <= 0,
        };
    }

    let previous_hp_raw = monster.hp_raw;
    monster.hp_raw = monster.hp_raw.saturating_sub(damage_raw).max(0);
    MonsterDamageResult {
        applied_damage_raw: previous_hp_raw.saturating_sub(monster.hp_raw),
        dead: monster.hp_raw == 0,
    }
}

#[cfg(test)]
pub(crate) fn apply_damage_hits(
    monsters: &mut [MonsterState],
    hits: &[DamageHit],
) -> Vec<DamageHitResult> {
    hits.iter()
        .filter_map(|hit| {
            let monster = monsters.get_mut(hit.target_index)?;
            Some(DamageHitResult {
                target_index: hit.target_index,
                damage: apply_monster_damage(monster, hit.damage_raw),
            })
        })
        .collect()
}

pub(crate) fn prepare_monster_death(monster: &mut MonsterState) -> Option<MonsterDeathResult> {
    if monster.hp_raw != 0 {
        return None;
    }
    let should_count_stage_progress = !monster.stage_progress_counted;
    monster.stage_progress_counted = true;
    Some(MonsterDeathResult {
        remaining_hp_raw: monster.hp_raw,
        should_count_stage_progress,
    })
}

pub(crate) fn remove_dead_monster(
    monsters: &mut Vec<MonsterState>,
    target_index: usize,
) -> Option<RemovedMonster> {
    let monster = monsters.get_mut(target_index)?;
    let death = prepare_monster_death(monster)?;
    Some(RemovedMonster {
        monster: monsters.swap_remove(target_index),
        death,
    })
}

pub(crate) fn activate_monster_skills(
    monsters: &mut [MonsterState],
    sim_tick: u64,
) -> Vec<ActivatedMonsterSkill> {
    let mut activated = Vec::new();
    for monster in monsters {
        for skill in &mut monster.skills {
            if sim_tick < skill.last_used_at.saturating_add(skill.template.cooldown) {
                continue;
            }
            skill.last_used_at = sim_tick;
            activated.push(ActivatedMonsterSkill {
                monster_id: monster.id,
                skill: skill.clone(),
            });
        }
    }
    activated
}

pub(crate) fn apply_monster_skill_activations(
    monsters: &mut [MonsterState],
    activations: &[ActivatedMonsterSkill],
    sim_tick: u64,
) {
    for activation in activations {
        let target = activation.skill.template.target.clone();
        match target {
            MonsterSkillTarget::MySelf => {
                let Some(monster) = monsters
                    .iter_mut()
                    .find(|monster| monster.id == activation.monster_id)
                else {
                    continue;
                };
                apply_monster_skill(monster, &activation.skill.template, sim_tick);
            }
            MonsterSkillTarget::AllMonsters => {
                for monster in monsters.iter_mut() {
                    apply_monster_skill(monster, &activation.skill.template, sim_tick);
                }
            }
        }
    }
}

fn apply_monster_skill(monster: &mut MonsterState, skill: &MonsterSkillTemplate, sim_tick: u64) {
    let end_at = sim_tick.saturating_add(skill.duration);
    match skill.kind {
        MonsterSkillKind::Invincible => monster.status_effects.push(MonsterStatusEffect {
            kind: MonsterStatusEffectKind::Invincible,
            end_at,
        }),
        MonsterSkillKind::SpeedMul { mul_raw } => {
            monster.status_effects.push(MonsterStatusEffect {
                kind: MonsterStatusEffectKind::SpeedMul { mul_raw },
                end_at,
            })
        }
        MonsterSkillKind::ImmuneToSlow => monster.status_effects.push(MonsterStatusEffect {
            kind: MonsterStatusEffectKind::ImmuneToSlow,
            end_at,
        }),
        MonsterSkillKind::HealByMaxHp { ratio_raw } => {
            monster.hp_raw = monster
                .hp_raw
                .saturating_add(crate::multiply_ratio_raw(monster.max_hp_raw, ratio_raw))
                .min(monster.max_hp_raw);
        }
    }
}

pub(crate) fn resolve_monster_escapes(monsters: &mut Vec<MonsterState>) -> MonsterEscapeResult {
    let mut damage_raw: i64 = 0;
    let mut escaped_hp_raw: i64 = 0;
    for monster in monsters.iter_mut() {
        if move_on_route_is_finished(&monster.move_on_route) {
            if !monster.stage_progress_counted {
                escaped_hp_raw = escaped_hp_raw.saturating_add(monster.hp_raw);
                monster.stage_progress_counted = true;
            }
            damage_raw = damage_raw.saturating_add(monster.damage_raw);
            if monster.kind >= 50 {
                reset_move_on_route(&mut monster.move_on_route);
            }
        }
    }
    monsters.retain(|monster| {
        !(move_on_route_is_finished(&monster.move_on_route) && monster.kind < 50)
    });
    monsters.sort_by_key(|monster| monster.id);
    MonsterEscapeResult {
        damage_raw,
        escaped_hp_raw,
    }
}
