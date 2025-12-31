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
            MonsterKind::Mob01 => (25.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob02 => (25.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob03 => (27.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob04 => (38.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob05 => (43.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob06 => (57.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob07 => (84.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob08 => (116.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob09 => (146.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob10 => (193.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob11 => (267.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob12 => (356.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob13 => (496.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob14 => (699.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob15 => (851.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob16 => (1001.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob17 => (1135.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob18 => (1300.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob19 => (1661.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob20 => (2080.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob21 => (2459.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob22 => (2820.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob23 => (3308.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob24 => (4098.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob25 => (5221.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob26 => (6911.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob27 => (9063.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob28 => (10134.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob29 => (11566.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob30 => (13492.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob31 => (14463.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob32 => (15224.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob33 => (15979.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob34 => (16754.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob35 => (17770.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob36 => (18898.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob37 => (21179.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob38 => (22346.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob39 => (23773.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob40 => (25364.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob41 => (27940.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob42 => (30671.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob43 => (33685.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob44 => (36472.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob45 => (38745.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob46 => (41386.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob47 => (44090.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob48 => (49515.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob49 => (53252.0, 1.0, 1.0, 5, vec![]),
            MonsterKind::Mob50 => (61322.0, 1.0, 1.0, 5, vec![]),
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
