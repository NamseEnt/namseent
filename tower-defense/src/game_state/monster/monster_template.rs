use super::MonsterSkillTemplate;
use crate::game_state::monster::MonsterKind;
use crate::game_state::route::Velocity;
use crate::*;
use namui::{Duration, State};

#[derive(State, Clone)]
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
        let (velocity, damage, reward, _skills): (f32, f32, usize, Vec<()>) = match kind {
            MonsterKind::Mob01 => (1.0, 1.0, 3, vec![]),
            MonsterKind::Mob02 => (1.0, 1.0, 3, vec![]),
            MonsterKind::Mob03 => (1.0, 1.0, 3, vec![]),
            MonsterKind::Mob04 => (1.0, 1.0, 3, vec![]),
            MonsterKind::Mob05 => (1.0, 1.0, 3, vec![]),
            MonsterKind::Mob06 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob07 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob08 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob09 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob10 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob11 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob12 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob13 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob14 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob15 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob16 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob17 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob18 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob19 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob20 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob21 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob22 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob23 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob24 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob25 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob26 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob27 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob28 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob29 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob30 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob31 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob32 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob33 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob34 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob35 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob36 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob37 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob38 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob39 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob40 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob41 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob42 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob43 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob44 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob45 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob46 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob47 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob48 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob49 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Mob50 => (1.0, 1.0, 5, vec![]),
            MonsterKind::Boss01 => (1.0, 15.0, 50, vec![]),
            MonsterKind::Boss02 => (1.0, 20.0, 75, vec![]),
            MonsterKind::Boss03 => (1.0, 20.0, 100, vec![]),
            MonsterKind::Boss04 => (1.0, 25.0, 100, vec![]),
            MonsterKind::Boss05 => (1.0, 25.0, 125, vec![]),
            MonsterKind::Boss06 => (1.0, 25.0, 125, vec![]),
            MonsterKind::Boss07 => (1.0, 50.0, 125, vec![]),
            MonsterKind::Boss08 => (1.0, 50.0, 125, vec![]),
            MonsterKind::Boss09 => (1.0, 50.0, 125, vec![]),
            MonsterKind::Boss10 => (1.0, 50.0, 125, vec![]),
            MonsterKind::Boss11 => (1.0, 50.0, 125, vec![]),
        };
        Self {
            kind,
            max_hp: Self::get_base_max_hp(kind),
            skills: vec![],
            velocity: Self::velocity(velocity),
            damage: Self::damage(damage),
            reward: Self::reward(reward),
        }
    }

    pub fn get_base_max_hp(kind: MonsterKind) -> f32 {
        match kind {
            MonsterKind::Mob01 => 15.0,
            MonsterKind::Mob02 => 21.0,
            MonsterKind::Mob03 => 32.0,
            MonsterKind::Mob04 => 60.0,
            MonsterKind::Mob05 => 82.0,
            MonsterKind::Mob06 => 101.0,
            MonsterKind::Mob07 => 121.0,
            MonsterKind::Mob08 => 143.0,
            MonsterKind::Mob09 => 216.0,
            MonsterKind::Mob10 => 297.0,
            MonsterKind::Mob11 => 356.0,
            MonsterKind::Mob12 => 421.0,
            MonsterKind::Mob13 => 454.0,
            MonsterKind::Mob14 => 513.0,
            MonsterKind::Mob15 => 640.0,
            MonsterKind::Mob16 => 762.0,
            MonsterKind::Mob17 => 793.0,
            MonsterKind::Mob18 => 860.0,
            MonsterKind::Mob19 => 952.0,
            MonsterKind::Mob20 => 1084.0,
            MonsterKind::Mob21 => 1773.0,
            MonsterKind::Mob22 => 2393.0,
            MonsterKind::Mob23 => 2469.0,
            MonsterKind::Mob24 => 2550.0,
            MonsterKind::Mob25 => 2680.0,
            MonsterKind::Mob26 => 2889.0,
            MonsterKind::Mob27 => 3246.0,
            MonsterKind::Mob28 => 3271.0,
            MonsterKind::Mob29 => 3564.0,
            MonsterKind::Mob30 => 4194.0,
            MonsterKind::Mob31 => 4622.0,
            MonsterKind::Mob32 => 6305.0,
            MonsterKind::Mob33 => 6636.0,
            MonsterKind::Mob34 => 7099.0,
            MonsterKind::Mob35 => 7619.0,
            MonsterKind::Mob36 => 8095.0,
            MonsterKind::Mob37 => 9067.0,
            MonsterKind::Mob38 => 10743.0,
            MonsterKind::Mob39 => 12533.0,
            MonsterKind::Mob40 => 13211.0,
            MonsterKind::Mob41 => 14106.0,
            MonsterKind::Mob42 => 15242.0,
            MonsterKind::Mob43 => 16245.0,
            MonsterKind::Mob44 => 17590.0,
            MonsterKind::Mob45 => 19461.0,
            MonsterKind::Mob46 => 21610.0,
            MonsterKind::Mob47 => 21890.0,
            MonsterKind::Mob48 => 22963.0,
            MonsterKind::Mob49 => 23462.0,
            MonsterKind::Mob50 => 24207.0,
            MonsterKind::Boss01 => 1280.0,
            MonsterKind::Boss02 => 5360.0,
            MonsterKind::Boss03 => 8388.0,
            MonsterKind::Boss04 => 15238.0,
            MonsterKind::Boss05 => 26422.0,
            MonsterKind::Boss06 => 38922.0,
            MonsterKind::Boss07 => 43220.0,
            MonsterKind::Boss08 => 43780.0,
            MonsterKind::Boss09 => 45926.0,
            MonsterKind::Boss10 => 46924.0,
            MonsterKind::Boss11 => 48414.0,
        }
    }

    pub fn skill_descriptions(&self) -> Vec<crate::l10n::monster_skill::MonsterSkillText> {
        self.skills
            .iter()
            .map(|skill| skill.kind.description())
            .collect()
    }
}
