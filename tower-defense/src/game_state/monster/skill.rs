use super::*;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct MonsterSkillTemplate {
    pub kind: MonsterSkillKind,
    pub cooldown: Duration,
    pub duration: Duration,
}

pub struct MonsterSkill {
    pub last_used_at: Instant,
    pub template: MonsterSkillTemplate,
}

impl MonsterSkill {
    pub fn new(template: MonsterSkillTemplate) -> Self {
        Self {
            last_used_at: Instant::now(),
            template,
        }
    }
}

impl Deref for MonsterSkill {
    type Target = MonsterSkillTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Clone, Copy)]
pub enum MonsterSkillKind {
    SelfInvincible,
    NearbyMonsterSpeedMul { mul: f32, range_radius: f32 },
    SelfImmuneToSlow,
}

#[derive(Clone)]
pub struct MonsterStatusEffect {
    pub kind: MonsterStatusEffectKind,
    pub end_at: Instant,
}

#[derive(Clone, Copy)]
pub enum MonsterStatusEffectKind {
    SpeedMul { mul: f32 },
    Invincible,
    ImmuneToSlow,
}

pub fn remove_monster_finished_status_effects(game_state: &mut GameState, now: Instant) {
    for monster in game_state.monsters.iter_mut() {
        monster.status_effects.retain(|e| now < e.end_at);
    }
}

pub fn activate_monster_skills(game_state: &mut GameState, now: Instant) {
    let mut activated_skills = vec![];

    for monster in game_state.monsters.iter_mut() {
        for skill in monster.skills.iter_mut() {
            if now < skill.last_used_at + skill.cooldown {
                continue;
            }

            skill.last_used_at = now;
            activated_skills.push((monster.id, skill.template));
        }
    }

    for (monster_id, skill) in activated_skills {
        let mut push_status_effect = |kind| {
            game_state
                .monsters
                .iter_mut()
                .find(|m| m.id == monster_id)
                .unwrap()
                .status_effects
                .push(MonsterStatusEffect {
                    kind,
                    end_at: now + skill.duration,
                });
        };

        match skill.kind {
            MonsterSkillKind::SelfInvincible => {
                push_status_effect(MonsterStatusEffectKind::Invincible);
            }
            MonsterSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => {
                let caster_xy = game_state
                    .monsters
                    .iter()
                    .find(|m| m.id == monster_id)
                    .unwrap()
                    .xy();

                for monster in game_state.monsters.iter_mut() {
                    if caster_xy.distance(monster.xy()) <= range_radius {
                        monster.status_effects.push(MonsterStatusEffect {
                            kind: MonsterStatusEffectKind::SpeedMul { mul },
                            end_at: now + skill.duration,
                        });
                    }
                }
            }
            MonsterSkillKind::SelfImmuneToSlow => {
                push_status_effect(MonsterStatusEffectKind::ImmuneToSlow);
            }
        }
    }
}
