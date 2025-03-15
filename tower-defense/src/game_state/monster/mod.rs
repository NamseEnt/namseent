mod skill;

use super::{user_status_effect::UserStatusEffectKind, *};
use namui::*;
pub use skill::*;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    pub fn new(template: &MonsterTemplate, route: Arc<Route>) -> Self {
        const ID: AtomicUsize = AtomicUsize::new(0);
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
                .map(|&t| MonsterSkill::new(t))
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
    pub fn get_damage_to_user(&self) -> f32 {
        let damage = self.damage;
        // weaken or strengthen the damage
        damage
    }

    pub fn dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn xy(&self) -> MapCoordF32 {
        self.move_on_route.xy()
    }
}
impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {
        let monster_wh = TILE_PX_SIZE
            * match self.kind {
                MonsterKind::Ball => 0.6,
                MonsterKind::BigBall => 0.8,
            };
        let path = Path::new().add_oval(Rect::from_xy_wh(monster_wh.as_xy() * -0.5, monster_wh));
        let paint = Paint::new(Color::RED);
        ctx.translate(TILE_PX_SIZE.as_xy() * 0.5)
            .add(namui::path(path, paint));

        let hp_bar_wh = Wh::new(monster_wh.width, MONSTER_HP_BAR_HEIGHT);
        ctx.translate(Xy::new(
            TILE_PX_SIZE.width * 0.5,
            TILE_PX_SIZE.width * 0.5 + monster_wh.height * 0.6,
        ))
        .add(MonsterHpBar {
            wh: hp_bar_wh,
            progress: self.hp / self.max_hp,
        });
    }
}

struct MonsterHpBar {
    wh: Wh<Px>,
    progress: f32,
}
impl Component for MonsterHpBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, progress } = self;

        let container_rect = Rect::from_xy_wh(wh.as_xy() * -0.5, wh);

        ctx.add(rect(RectParam {
            rect: Rect::from_xy_wh(container_rect.xy(), Wh::new(wh.width * progress, wh.height)),
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill { color: Color::RED }),
                round: None,
            },
        }));

        ctx.add(rect(RectParam {
            rect: container_rect,
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Outside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: None,
            },
        }));
    }
}

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
        Per::new(10.0 * mul, Duration::from_secs(1))
    }
    fn damage(mul: f32) -> f32 {
        mul
    }
    fn reward(mul: usize) -> usize {
        mul
    }
    pub fn new_mob_01() -> Self {
        Self {
            kind: MonsterKind::Ball,
            max_hp: 10.0,
            skills: vec![],
            velocity: Self::velocity(0.5),
            damage: Self::damage(1.0),
            reward: Self::reward(1),
        }
    }
    pub fn new_mob_02() -> Self {
        Self {
            kind: MonsterKind::Ball,
            max_hp: 15.0,
            skills: vec![],
            velocity: Self::velocity(0.5),
            damage: Self::damage(1.0),
            reward: Self::reward(1),
        }
    }

    pub fn new_named_01() -> Self {
        Self {
            kind: MonsterKind::BigBall,
            max_hp: 100.0,
            skills: vec![],
            velocity: Self::velocity(1.0),
            damage: Self::damage(10.0),
            reward: Self::reward(10),
        }
    }

    pub fn new_boss_01() -> Self {
        Self {
            kind: MonsterKind::BigBall,
            max_hp: 1000.0,
            skills: vec![],
            velocity: Self::velocity(1.0),
            damage: Self::damage(25.0),
            reward: Self::reward(100),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MonsterKind {
    Ball,
    BigBall,
}

pub fn move_monsters(game_state: &mut GameState, dt: Duration) {
    for monster in &mut game_state.monsters {
        let is_immune_to_slow = monster.status_effects.iter().any(|status_effect| {
            matches!(status_effect.kind, MonsterStatusEffectKind::ImmuneToSlow)
        });
        let mut dt = dt;

        for status_effect in &monster.status_effects {
            match status_effect.kind {
                MonsterStatusEffectKind::SpeedMul { mul } => {
                    if is_immune_to_slow && mul < 1.0 {
                        continue;
                    }
                    dt *= mul;
                }
                MonsterStatusEffectKind::Invincible | MonsterStatusEffectKind::ImmuneToSlow => {}
            }
        }

        monster.move_on_route.move_by(dt);
    }

    let mut damage = 0.0;
    game_state.monsters.retain(|monster| {
        if monster.move_on_route.is_finished() {
            damage += monster.get_damage_to_user();
            return false;
        }
        true
    });

    for user_status_effect in &game_state.user_status_effects {
        match user_status_effect.kind {
            UserStatusEffectKind::DamageReduction { damage_multiply } => {
                damage *= damage_multiply;
            }
        }
    }

    if game_state.shield > 0.0 {
        let min = damage.min(game_state.shield);
        damage -= min;
        game_state.shield -= min;
    }
    game_state.hp -= damage;
    if game_state.hp <= 0.0 {
        game_state.goto_result();
    }
}
