mod skill;

use super::*;
use namui::*;
pub use skill::*;
use std::sync::atomic::{AtomicUsize, Ordering};

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

    pub fn xy(&self) -> MapCoordF32 {
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

pub fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        monster.move_on_route.move_by(dt);
    }

    // todo: deal damage to user
    game_state
        .monsters
        .retain(|monster| !monster.move_on_route.is_finished());
}
