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
        Self::new_with_config(kind, &crate::config::GameConfig::default_config())
    }

    pub fn new_with_config(kind: MonsterKind, config: &crate::config::GameConfig) -> Self {
        let stats = config
            .monsters
            .stats
            .get(&kind)
            .expect("missing monster stats for kind");
        Self {
            kind,
            max_hp: stats.base_hp,
            skills: vec![],
            velocity: Self::velocity(stats.velocity_mul),
            damage: Self::damage(stats.damage),
            reward: Self::reward(stats.reward),
        }
    }

    pub fn get_base_max_hp(kind: MonsterKind) -> f32 {
        crate::config::GameConfig::default_config()
            .monsters
            .stats
            .get(&kind)
            .expect("missing monster stats for kind")
            .base_hp
    }

    pub fn skill_descriptions(&self) -> Vec<crate::l10n::monster_skill::MonsterSkillText> {
        self.skills
            .iter()
            .map(|skill| skill.kind.description())
            .collect()
    }
}
