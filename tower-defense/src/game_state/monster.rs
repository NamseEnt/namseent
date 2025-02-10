use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::*;
use namui::*;

pub struct Monster {
    id: usize,
    pub move_on_route: MoveOnRoute,
    pub kind: MonsterKind,
    pub projectile_target_indicator: ProjectileTargetIndicator,
    pub hp: usize,
    pub skills: Vec<MonsterSkill>,
    pub status_effects: Vec<MonsterStatusEffect>,
}
impl Monster {
    pub fn new(template: &MonsterTemplate, route: Arc<Route>) -> Self {
        const ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            move_on_route: MoveOnRoute::new(route, template.velocity),
            kind: template.kind,
            projectile_target_indicator: ProjectileTargetIndicator::new(),
            hp: template.max_hp,
            skills: template
                .skills
                .iter()
                .map(|&t| MonsterSkill::new(t))
                .collect(),
            status_effects: vec![],
        }
    }
    pub fn get_damage(&mut self, damage: usize) {
        self.hp = self.hp.saturating_sub(damage);
    }

    pub fn dead(&self) -> bool {
        self.hp == 0
    }

    fn xy(&self) -> MapCoordF32 {
        self.move_on_route.xy()
    }
}
impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {}
}

pub struct MonsterTemplate {
    pub kind: MonsterKind,
    pub max_hp: usize,
    pub skills: Vec<MonsterSkillTemplate>,
    pub velocity: Velocity,
}

#[derive(Clone, Copy)]
pub enum MonsterKind {}

pub struct MonsterStatusEffect {
    pub kind: MonsterStatusEffectKind,
    pub end_at: Instant,
}

pub enum MonsterStatusEffectKind {
    SpeedMul { mul: f32 },
    SpeedAdd { add: f32 },
    Invincible,
    ImmuneToSlow,
}

#[derive(Clone, Copy)]
pub struct MonsterSkillTemplate {
    pub kind: MonsterSkillKind,
    pub cooldown: Duration,
    pub duration: Duration,
}

pub struct MonsterSkill {
    pub last_used_at: Instant,
    template: MonsterSkillTemplate,
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
    SelfInvincible,
    NearbyMonsterSpeedMul { mul: f32, range_radius: f32 },
    SelfImmuneToSlow,
}

pub fn remove_finished_status_effects(game_state: &mut GameState, now: Instant) {
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
        let mut push_status_effect = |kind| {
            game_state
                .monsters
                .iter_mut()
                .find(|m| m.id == monster_id)
                .unwrap()
                .status_effects
                .push(MonsterStatusEffect {
                    kind,
                    end_at: now + skill.duration,
                });
        };

        match skill.kind {
            MonsterSkillKind::SelfInvincible => {
                push_status_effect(MonsterStatusEffectKind::Invincible);
            }
            MonsterSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => {
                let caster_xy = game_state
                    .monsters
                    .iter()
                    .find(|m| m.id == monster_id)
                    .unwrap()
                    .xy();

                for monster in game_state.monsters.iter_mut() {
                    if caster_xy.distance(monster.xy()) <= range_radius {
                        monster.status_effects.push(MonsterStatusEffect {
                            kind: MonsterStatusEffectKind::SpeedMul { mul },
                            end_at: now + skill.duration,
                        });
                    }
                }
            }
            MonsterSkillKind::SelfImmuneToSlow => {
                push_status_effect(MonsterStatusEffectKind::ImmuneToSlow);
            }
        }
    }
}

pub fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        monster.move_on_route.move_by(dt);
    }

    // todo: deal damage to user
    game_state
        .monsters
        .retain(|monster| !monster.move_on_route.is_finished());
}
