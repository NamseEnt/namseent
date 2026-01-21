mod monster_hp_bar;
mod monster_kind;
mod monster_template;
mod move_monsters;
mod render;
pub mod skill;

use crate::{
    MapCoordF32,
    game_state::{monster::render::MonsterAnimation, projectile::ProjectileTargetIndicator},
    route::{MoveOnRoute, Route},
};
pub use monster_kind::MonsterKind;
pub use monster_template::MonsterTemplate;
pub use move_monsters::move_monsters;
use namui::*;
pub use render::{monster_animation_tick, monster_wh};
pub use skill::{
    MonsterSkill, MonsterSkillTemplate, MonsterStatusEffect, MonsterStatusEffectKind,
    PrebuiltSkill, activate_monster_skills, remove_monster_finished_status_effects,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const MONSTER_HP_BAR_HEIGHT: Px = px(4.);

#[derive(State, Clone)]
pub struct Monster {
    id: usize,
    pub move_on_route: MoveOnRoute,
    pub kind: MonsterKind,
    pub projectile_target_indicator: ProjectileTargetIndicator,
    pub hp: f32,
    pub max_hp: f32,
    #[cfg(feature = "debug-tools")]
    pub base_max_hp: f32,
    pub skills: Vec<MonsterSkill>,
    pub status_effects: Vec<MonsterStatusEffect>,
    pub damage: f32,
    pub reward: usize,
    pub animation: MonsterAnimation,
}
impl Monster {
    pub fn new(
        template: &MonsterTemplate,
        route: Arc<Route>,
        now: Instant,
        health_multiplier: f32,
    ) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        let adjusted_max_hp = template.max_hp * health_multiplier;
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            move_on_route: MoveOnRoute::new(route, template.velocity),
            kind: template.kind,
            projectile_target_indicator: ProjectileTargetIndicator::new(),
            hp: adjusted_max_hp,
            max_hp: adjusted_max_hp,
            #[cfg(feature = "debug-tools")]
            base_max_hp: adjusted_max_hp,
            skills: template
                .skills
                .iter()
                .map(|&t| MonsterSkill::new(t, now))
                .collect(),
            status_effects: vec![],
            damage: template.damage,
            reward: template.reward,
            animation: MonsterAnimation::new(),
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
    pub fn get_speed_multiplier(&self) -> f32 {
        let is_immune_to_slow = self.status_effects.iter().any(|status_effect| {
            matches!(status_effect.kind, MonsterStatusEffectKind::ImmuneToSlow)
        });
        let mut speed_multiplier = 1.0f32;
        for status_effect in &self.status_effects {
            match status_effect.kind {
                MonsterStatusEffectKind::SpeedMul { mul } => {
                    if is_immune_to_slow && mul < 1.0 {
                        continue;
                    }
                    speed_multiplier *= mul;
                }
                MonsterStatusEffectKind::Invincible | MonsterStatusEffectKind::ImmuneToSlow => {}
            }
        }
        speed_multiplier
    }
}
