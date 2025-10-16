use super::MonsterSkillTemplate;
use super::PrebuiltSkill;
use crate::game_state::monster::MonsterKind;
use crate::game_state::route::Velocity;
use crate::*;
use namui::{Duration, State};

#[derive(State)]
pub struct MonsterTemplate {
    pub kind: MonsterKind,
    pub max_hp: f32,
    pub skills: Vec<MonsterSkillTemplate>,
    pub velocity: Velocity,
    pub damage: f32,
    pub reward: usize,
}

impl MonsterTemplate {
    fn velocity(mul: f32) -> Velocity {
        Velocity::new(5.0 * mul, Duration::from_secs(1))
    }
    fn damage(mul: f32) -> f32 {
        mul
    }
    fn reward(mul: usize) -> usize {
        mul
    }
    pub fn new(kind: MonsterKind) -> Self {
        let (max_hp, velocity, damage, reward, skills) = match kind {
            MonsterKind::Mob01 => (10.0, 0.5, 1.0, 3, vec![]),
            MonsterKind::Mob02 => (25.0, 0.5, 1.0, 3, vec![]),
            MonsterKind::Mob03 => (90.0, 0.3, 1.0, 3, vec![]),
            MonsterKind::Mob04 => (75.0, 1.1, 1.0, 3, vec![]),
            MonsterKind::Mob05 => (250.0, 1.0, 1.0, 3, vec![PrebuiltSkill::Invincible01]),
            MonsterKind::Mob06 => (850.0, 0.75, 1.0, 5, vec![]),
            MonsterKind::Mob07 => (1750.0, 0.75, 1.0, 5, vec![]),
            MonsterKind::Mob08 => (6000.0, 0.4, 1.0, 5, vec![]),
            MonsterKind::Mob09 => (3500.0, 1.25, 1.0, 5, vec![PrebuiltSkill::Speedmul01]),
            MonsterKind::Mob10 => (7500.0, 0.75, 1.0, 5, vec![PrebuiltSkill::Speedmul02]),
            MonsterKind::Mob11 => (10000.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob12 => (15000.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob13 => (45000.0, 0.5, 1.0, 5, vec![]),
            MonsterKind::Mob14 => (20000.0, 1.5, 1.0, 5, vec![PrebuiltSkill::ImmuneSlow01]),
            MonsterKind::Mob15 => (45000.0, 1.0, 1.0, 5, vec![PrebuiltSkill::Invincible02]),
            MonsterKind::Named01 => (100.0, 0.5, 3.0, 30, vec![PrebuiltSkill::ImmuneSlow02]),
            MonsterKind::Named02 => (500.0, 0.75, 3.0, 30, vec![PrebuiltSkill::Invincible03]),
            MonsterKind::Named03 => (1500.0, 1.0, 3.0, 30, vec![PrebuiltSkill::Speedmul03]),
            MonsterKind::Named04 => (3500.0, 1.25, 3.0, 30, vec![PrebuiltSkill::ImmuneSlow03]),
            MonsterKind::Named05 => (15000.0, 0.5, 5.0, 30, vec![PrebuiltSkill::Heal01]),
            MonsterKind::Named06 => (30000.0, 0.75, 5.0, 30, vec![PrebuiltSkill::ImmuneSlow04]),
            MonsterKind::Named07 => (45000.0, 1.0, 5.0, 30, vec![PrebuiltSkill::Heal02]),
            MonsterKind::Named08 => (65000.0, 1.25, 5.0, 30, vec![PrebuiltSkill::Speedmul04]),
            MonsterKind::Named09 => (200000.0, 0.5, 7.0, 30, vec![PrebuiltSkill::Invincible04]),
            MonsterKind::Named10 => (250000.0, 0.75, 7.0, 30, vec![PrebuiltSkill::Heal03]),
            MonsterKind::Named11 => (300000.0, 1.0, 7.0, 30, vec![PrebuiltSkill::Heal04]),
            MonsterKind::Named12 => (300000.0, 1.25, 7.0, 30, vec![PrebuiltSkill::AreaSpeedmul01]),
            MonsterKind::Named13 => (
                650000.0,
                0.5,
                10.0,
                30,
                vec![PrebuiltSkill::AreaImmuneSlow01],
            ),
            MonsterKind::Named14 => (
                550000.0,
                0.75,
                10.0,
                30,
                vec![PrebuiltSkill::AreaInvincible01],
            ),
            MonsterKind::Named15 => (
                550000.0,
                1.0,
                10.0,
                30,
                vec![PrebuiltSkill::AreaInvincible02],
            ),
            MonsterKind::Named16 => (
                750000.0,
                1.25,
                10.0,
                30,
                vec![PrebuiltSkill::AreaImmuneSlow02],
            ),
            MonsterKind::Boss01 => (3500.0, 1.0, 15.0, 50, vec![PrebuiltSkill::AreaInvincible03]),
            MonsterKind::Boss02 => (10000.0, 1.0, 20.0, 75, vec![PrebuiltSkill::AreaHeal01]),
            MonsterKind::Boss03 => (35000.0, 1.0, 20.0, 100, vec![PrebuiltSkill::AreaSpeedmul02]),
            MonsterKind::Boss04 => (85000.0, 1.0, 25.0, 100, vec![PrebuiltSkill::AreaSpeedmul03]),
            MonsterKind::Boss05 => (
                200000.0,
                1.0,
                25.0,
                125,
                vec![PrebuiltSkill::AreaImmuneSlow03],
            ),
            MonsterKind::Boss06 => (300000.0, 1.0, 25.0, 125, vec![PrebuiltSkill::AreaHeal02]),
            MonsterKind::Boss07 => (
                375000.0,
                1.0,
                50.0,
                125,
                vec![PrebuiltSkill::AreaImmuneSlow04],
            ),
            MonsterKind::Boss08 => (500000.0, 1.0, 50.0, 125, vec![PrebuiltSkill::AreaHeal03]),
            MonsterKind::Boss09 => (
                700000.0,
                1.0,
                50.0,
                125,
                vec![PrebuiltSkill::AreaSpeedmul04],
            ),
            MonsterKind::Boss10 => (
                850000.0,
                1.0,
                50.0,
                125,
                vec![PrebuiltSkill::AreaInvincible04],
            ),
            MonsterKind::Boss11 => (1125000.0, 1.0, 50.0, 125, vec![PrebuiltSkill::AreaHeal04]),
        };
        Self {
            kind,
            max_hp,
            skills: skills.into_iter().map(|prebuilt| prebuilt.into()).collect(),
            velocity: Self::velocity(velocity),
            damage: Self::damage(damage),
            reward: Self::reward(reward),
        }
    }
}
