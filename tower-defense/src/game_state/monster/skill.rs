use super::*;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct MonsterSkillTemplate {
    pub kind: MonsterSkillKind,
    pub target: Target,
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
    Invincible,
    SpeedMul { mul: f32 },
    ImmuneToSlow,
    HealByMaxHp { ratio: f32 },
}

#[derive(Clone)]
pub struct MonsterStatusEffect {
    pub kind: MonsterStatusEffectKind,
    pub end_at: Instant,
}

#[derive(Clone, Copy)]
pub enum Target {
    MySelf,
    MeAndNearby { radius: f32 },
}

#[derive(Clone, Copy)]
pub enum MonsterStatusEffectKind {
    SpeedMul { mul: f32 },
    Invincible,
    ImmuneToSlow,
    // 자신에게 n초 동안 무적 버프 부여
    // 자신에게 n초 동안 둔화효과 무시 버프 부여
    // 자신에게 n초 동안 이동속도 m배 버프 부여
    // 자신에게 최대체력의 m배 회복 버프 부여
    // 자신과 주변 r 타일의 적에게 n초 동안 무적 버프 부여
    // 자신과 주변 r 타일의 적에게 n초 동안 둔화효과 무시 버프 부여
    // 자신과 주변 r 타일의 적에게 n초 동안 이동속도 m배 버프 부여
    // 자신과 주변 r 타일의 적에게 최대체력의 m배 회복 버프 부여
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
        let target_monsters = match skill.target {
            Target::MySelf => {
                vec![
                    game_state
                        .monsters
                        .iter_mut()
                        .find(|m| m.id == monster_id)
                        .unwrap(),
                ]
            }
            Target::MeAndNearby { radius } => {
                let caster_xy = game_state
                    .monsters
                    .iter()
                    .find(|m| m.id == monster_id)
                    .unwrap()
                    .xy();
                game_state
                    .monsters
                    .iter_mut()
                    .filter(|monster| {
                        monster.id != monster_id && caster_xy.distance(monster.xy()) <= radius
                    })
                    .collect()
            }
        };

        target_monsters.into_iter().for_each(|monster| {
            let mut push_status_effect = |kind| {
                monster.status_effects.push(MonsterStatusEffect {
                    kind,
                    end_at: now + skill.duration,
                });
            };
            match skill.kind {
                MonsterSkillKind::Invincible => {
                    push_status_effect(MonsterStatusEffectKind::Invincible);
                }
                MonsterSkillKind::SpeedMul { mul } => {
                    push_status_effect(MonsterStatusEffectKind::SpeedMul { mul });
                }
                MonsterSkillKind::ImmuneToSlow => {
                    push_status_effect(MonsterStatusEffectKind::ImmuneToSlow);
                }
                MonsterSkillKind::HealByMaxHp { ratio } => {
                    monster.heal(monster.max_hp * ratio);
                }
            }
        });
    }
}
