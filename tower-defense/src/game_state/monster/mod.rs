mod monster_hp_bar;
mod monster_template;
mod move_monsters;
mod render;
mod skill;

use crate::{
    MapCoordF32,
    game_state::projectile::ProjectileTargetIndicator,
    route::{MoveOnRoute, Route},
};
pub use monster_template::MonsterTemplate;
pub use move_monsters::move_monsters;
use namui::*;
pub use skill::{
    MonsterSkill, MonsterSkillTemplate, MonsterStatusEffect, MonsterStatusEffectKind,
    PrebuiltSkill, activate_monster_skills, remove_monster_finished_status_effects,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const MONSTER_HP_BAR_HEIGHT: Px = px(4.);

pub struct Monster {
    id: usize,
    pub move_on_route: MoveOnRoute,
    pub kind: MonsterKind,
    pub projectile_target_indicator: ProjectileTargetIndicator,
    pub hp: f32,
    pub max_hp: f32,
    pub skills: Vec<MonsterSkill>,
    pub status_effects: Vec<MonsterStatusEffect>,
    pub damage: f32,
    pub reward: usize,
}
impl Monster {
    pub fn new(template: &MonsterTemplate, route: Arc<Route>, now: Instant) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            move_on_route: MoveOnRoute::new(route, template.velocity),
            kind: template.kind,
            projectile_target_indicator: ProjectileTargetIndicator::new(),
            hp: template.max_hp,
            max_hp: template.max_hp,
            skills: template
                .skills
                .iter()
                .map(|&t| MonsterSkill::new(t, now))
                .collect(),
            status_effects: vec![],
            damage: template.damage,
            reward: template.reward,
        }
    }
    pub fn get_damage(&mut self, damage: f32) {
        if self.dead()
            || self.status_effects.iter().any(|status_effect| {
                matches!(status_effect.kind, MonsterStatusEffectKind::Invincible)
            })
        {
            return;
        }

        self.hp -= damage;
    }
    pub fn heal(&mut self, amount: f32) {
        if self.dead() {
            return;
        }

        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
    pub fn get_damage_to_user(&self) -> f32 {
        // weaken or strengthen the damage
        self.damage
    }

    pub fn dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn xy(&self) -> MapCoordF32 {
        self.move_on_route.xy()
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Clone, Copy)]
pub enum MonsterKind {
    Mob01,
    Mob02,
    Mob03,
    Mob04,
    Mob05,
    Mob06,
    Mob07,
    Mob08,
    Mob09,
    Mob10,
    Mob11,
    Mob12,
    Mob13,
    Mob14,
    Mob15,
    Named01,
    Named02,
    Named03,
    Named04,
    Named05,
    Named06,
    Named07,
    Named08,
    Named09,
    Named10,
    Named11,
    Named12,
    Named13,
    Named14,
    Named15,
    Named16,
    Boss01,
    Boss02,
    Boss03,
    Boss04,
    Boss05,
    Boss06,
    Boss07,
    Boss08,
    Boss09,
    Boss10,
    Boss11,
}
