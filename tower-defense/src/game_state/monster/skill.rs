use super::*;
use crate::game_state::GameState;
use namui::Instant;
use std::ops::Deref;

#[derive(Clone, Copy, State)]
pub struct MonsterSkillTemplate {
    pub kind: MonsterSkillKind,
    pub target: Target,
    pub cooldown: Duration,
    pub duration: Duration,
}

#[derive(State)]
pub struct MonsterSkill {
    pub last_used_at: Instant,
    pub template: MonsterSkillTemplate,
}

impl MonsterSkill {
    pub fn new(template: MonsterSkillTemplate, now: Instant) -> Self {
        Self {
            last_used_at: now,
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

#[derive(Clone, Copy, State)]
pub enum MonsterSkillKind {
    Invincible,
    SpeedMul { mul: f32 },
    ImmuneToSlow,
    HealByMaxHp { ratio: f32 },
}

#[derive(Clone, State)]
pub struct MonsterStatusEffect {
    pub kind: MonsterStatusEffectKind,
    pub end_at: Instant,
}

#[derive(Clone, Copy, State)]
pub enum Target {
    MySelf,
    MeAndNearby { radius: f32 },
}

#[derive(Clone, Copy, PartialEq, State)]
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

#[derive(State)]
pub enum PrebuiltSkill {
    AreaHeal01,
    AreaHeal02,
    AreaHeal03,
    AreaHeal04,
    AreaImmuneSlow01,
    AreaImmuneSlow02,
    AreaImmuneSlow03,
    AreaImmuneSlow04,
    AreaInvincible01,
    AreaInvincible02,
    AreaInvincible03,
    AreaInvincible04,
    AreaSpeedmul01,
    AreaSpeedmul02,
    AreaSpeedmul03,
    AreaSpeedmul04,
    Heal01,
    Heal02,
    Heal03,
    Heal04,
    ImmuneSlow01,
    ImmuneSlow02,
    ImmuneSlow03,
    ImmuneSlow04,
    Invincible01,
    Invincible02,
    Invincible03,
    Invincible04,
    Speedmul01,
    Speedmul02,
    Speedmul03,
    Speedmul04,
}
impl From<PrebuiltSkill> for MonsterSkillTemplate {
    fn from(val: PrebuiltSkill) -> Self {
        match val {
            PrebuiltSkill::AreaHeal01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.25 },
                target: Target::MeAndNearby { radius: 3.0 },
                cooldown: Duration::from_secs(5),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::AreaHeal02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.125 },
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(4),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::AreaHeal03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.125 },
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(3),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::AreaHeal04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.125 },
                target: Target::MeAndNearby { radius: 7.0 },
                cooldown: Duration::from_secs(2),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::AreaImmuneSlow01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(5),
                duration: Duration::from_secs(1),
            },
            PrebuiltSkill::AreaImmuneSlow02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(5),
                duration: Duration::from_secs(2),
            },
            PrebuiltSkill::AreaImmuneSlow03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MeAndNearby { radius: 7.0 },
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(4),
            },
            PrebuiltSkill::AreaImmuneSlow04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MeAndNearby { radius: 9.0 },
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(5),
            },
            PrebuiltSkill::AreaInvincible01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MeAndNearby { radius: 3.0 },
                cooldown: Duration::from_secs(4),
                duration: Duration::from_millis(500),
            },
            PrebuiltSkill::AreaInvincible02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MeAndNearby { radius: 3.0 },
                cooldown: Duration::from_secs(4),
                duration: Duration::from_secs(1),
            },
            PrebuiltSkill::AreaInvincible03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MeAndNearby { radius: 3.0 },
                cooldown: Duration::from_secs(4),
                duration: Duration::from_secs(1) + Duration::from_millis(500),
            },
            PrebuiltSkill::AreaInvincible04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(4),
                duration: Duration::from_secs(2),
            },
            PrebuiltSkill::AreaSpeedmul01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::MeAndNearby { radius: 3.0 },
                cooldown: Duration::from_secs(5),
                duration: Duration::from_secs(1),
            },
            PrebuiltSkill::AreaSpeedmul02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(3),
            },
            PrebuiltSkill::AreaSpeedmul03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.5 },
                target: Target::MeAndNearby { radius: 5.0 },
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(2),
            },
            PrebuiltSkill::AreaSpeedmul04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.75 },
                target: Target::MeAndNearby { radius: 7.0 },
                cooldown: Duration::from_secs(15),
                duration: Duration::from_secs(4),
            },
            PrebuiltSkill::Heal01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.25 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(5),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::Heal02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.125 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(4),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::Heal03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.125 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(3),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::Heal04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.125 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(2),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::ImmuneSlow01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_secs(5),
                duration: Duration::from_secs(1),
            },
            PrebuiltSkill::ImmuneSlow02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_secs(5),
                duration: Duration::from_secs(2),
            },
            PrebuiltSkill::ImmuneSlow03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(4),
            },
            PrebuiltSkill::ImmuneSlow04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(5),
            },
            PrebuiltSkill::Invincible01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_secs(4),
                duration: Duration::from_millis(500),
            },
            PrebuiltSkill::Invincible02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_secs(4),
                duration: Duration::from_secs(1),
            },
            PrebuiltSkill::Invincible03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_secs(4),
                duration: Duration::from_secs(1) + Duration::from_millis(500),
            },
            PrebuiltSkill::Invincible04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_secs(4),
                duration: Duration::from_secs(2),
            },
            PrebuiltSkill::Speedmul01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(5),
                duration: Duration::from_secs(1),
            },
            PrebuiltSkill::Speedmul02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(3),
            },
            PrebuiltSkill::Speedmul03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.5 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(10),
                duration: Duration::from_secs(2),
            },
            PrebuiltSkill::Speedmul04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.75 },
                target: Target::MySelf,
                cooldown: Duration::from_secs(15),
                duration: Duration::from_secs(4),
            },
        }
    }
}
