use super::MonsterSkillTemplate;
use crate::game_state::monster::MonsterKind;
use crate::game_state::route::Velocity;
use crate::{Damage, Health, WorldSpeed};
use namui::*;

#[derive(State, Clone)]
pub struct MonsterTemplate {
    pub kind: MonsterKind,
    pub max_hp: Health,
    pub skills: Vec<MonsterSkillTemplate>,
    pub velocity: Velocity,
    pub damage: Damage,
    pub reward: usize,
}

impl MonsterTemplate {
    fn velocity(mul: crate::FixedRatio) -> Velocity {
        WorldSpeed::from_raw(
            crate::RatioProduct::one()
                .with(mul)
                .apply_raw(5 * crate::world::WORLD_UNITS_PER_TILE),
        )
    }

    fn damage(damage: Damage) -> Damage {
        damage
    }

    fn reward(mul: usize) -> usize {
        mul
    }

    pub fn new(kind: MonsterKind, config: &crate::config::GameConfig) -> Self {
        let stats = config
            .monsters
            .stats
            .iter()
            .find(|stats| stats.kind == kind.to_core_raw())
            .expect("missing monster stats for kind");
        Self {
            kind,
            max_hp: Health::from_raw(stats.base_hp_raw),
            skills: vec![],
            velocity: Self::velocity(crate::FixedRatio::from_raw(stats.velocity_mul_raw)),
            damage: Self::damage(Damage::from_raw(stats.damage_raw)),
            reward: Self::reward(stats.reward),
        }
    }
}
