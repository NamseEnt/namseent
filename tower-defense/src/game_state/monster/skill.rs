use super::*;
use crate::game_state::GameState;
use crate::l10n::LocalizedText;
use namui::Instant;
use std::ops::Deref;

#[derive(Clone, Copy, State)]
pub struct MonsterSkillTemplate {
    pub kind: MonsterSkillKind,
    pub target: Target,
    pub cooldown: Duration,
    pub duration: Duration,
}

#[derive(State, Clone)]
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

impl MonsterSkillKind {
    pub fn description(&self, locale: &crate::l10n::Locale) -> String {
        crate::l10n::monster_skill::MonsterSkillText::Description(*self).localized_text(locale)
    }
}

#[derive(Clone, State)]
pub struct MonsterStatusEffect {
    pub kind: MonsterStatusEffectKind,
    pub end_at: Instant,
}

#[derive(Clone, Copy, State)]
pub enum Target {
    MySelf,
    AllMonsters,
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
            Target::AllMonsters => game_state.monsters.iter_mut().collect(),
        };

        target_monsters.into_iter().for_each(|monster| {
            println!("Activating skill for monster {}", monster.max_hp);
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

#[derive(State, Clone, Copy)]
pub enum PrebuiltSkill {
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
    SelfHeal01,
    SelfHeal02,
    SelfHeal03,
    SelfHeal04,
    SelfImmuneSlow01,
    SelfImmuneSlow02,
    SelfImmuneSlow03,
    SelfImmuneSlow04,
    SelfInvincible01,
    SelfInvincible02,
    SelfInvincible03,
    SelfInvincible04,
    SelfSpeedmul01,
    SelfSpeedmul02,
    SelfSpeedmul03,
    SelfSpeedmul04,
}
impl From<PrebuiltSkill> for MonsterSkillTemplate {
    fn from(val: PrebuiltSkill) -> Self {
        match val {
            PrebuiltSkill::Heal01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.075 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::Heal02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.1 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::Heal03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.15 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(4000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::Heal04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.15 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::ImmuneSlow01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::ImmuneSlow02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(2500),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::ImmuneSlow03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(2000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::ImmuneSlow04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(1000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::Invincible01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3000),
                duration: Duration::from_millis(500),
            },
            PrebuiltSkill::Invincible02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(4000),
                duration: Duration::from_millis(750),
            },
            PrebuiltSkill::Invincible03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3500),
                duration: Duration::from_millis(750),
            },
            PrebuiltSkill::Invincible04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(3000),
                duration: Duration::from_millis(750),
            },
            PrebuiltSkill::Speedmul01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 2.0 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(5000),
                duration: Duration::from_millis(3000),
            },
            PrebuiltSkill::Speedmul02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(5000),
                duration: Duration::from_millis(3000),
            },
            PrebuiltSkill::Speedmul03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(2000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::Speedmul04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.5 },
                target: Target::AllMonsters,
                cooldown: Duration::from_millis(1000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfHeal01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.05 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(3000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::SelfHeal02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.05 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(2500),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::SelfHeal03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.075 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(2000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::SelfHeal04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::HealByMaxHp { ratio: 0.1 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(1000),
                duration: Duration::ZERO,
            },
            PrebuiltSkill::SelfImmuneSlow01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_millis(3000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfImmuneSlow02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_millis(2500),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfImmuneSlow03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_millis(2000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfImmuneSlow04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::ImmuneToSlow,
                target: Target::MySelf,
                cooldown: Duration::from_millis(1000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfInvincible01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_millis(3000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfInvincible02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_millis(2500),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfInvincible03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_millis(2000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfInvincible04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::Invincible,
                target: Target::MySelf,
                cooldown: Duration::from_millis(1000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfSpeedmul01 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.1 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(3000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfSpeedmul02 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.25 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(2500),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfSpeedmul03 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.5 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(2000),
                duration: Duration::from_millis(1000),
            },
            PrebuiltSkill::SelfSpeedmul04 => MonsterSkillTemplate {
                kind: MonsterSkillKind::SpeedMul { mul: 1.5 },
                target: Target::MySelf,
                cooldown: Duration::from_millis(1000),
                duration: Duration::from_millis(1000),
            },
        }
    }
}
