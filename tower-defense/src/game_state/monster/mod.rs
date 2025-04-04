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
    pub fn new(template: &MonsterTemplate, route: Arc<Route>, now: Instant) -> Self {
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
        // TODO: add monster image
        let monster_wh = TILE_PX_SIZE * 0.6;
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
        Per::new(5.0 * mul, Duration::from_secs(1))
    }
    fn damage(mul: f32) -> f32 {
        mul
    }
    fn reward(mul: usize) -> usize {
        mul
    }
    pub fn new(kind: MonsterKind) -> Self {
        let (max_hp, velocity, damage, reward, skills) = match kind {
            MonsterKind::Mob01 => (10.0, 0.5, 1.0, 2, vec![]),
            MonsterKind::Mob02 => (25.0, 0.5, 1.0, 2, vec![]),
            MonsterKind::Mob03 => (90.0, 0.3, 1.0, 2, vec![]),
            MonsterKind::Mob04 => (75.0, 1.1, 1.0, 2, vec![]),
            MonsterKind::Mob05 => (250.0, 1.0, 1.0, 2, vec![]),
            MonsterKind::Mob06 => (850.0, 0.75, 1.0, 3, vec![]),
            MonsterKind::Mob07 => (1750.0, 0.75, 1.0, 3, vec![]),
            MonsterKind::Mob08 => (6000.0, 0.4, 1.0, 3, vec![]),
            MonsterKind::Mob09 => (3500.0, 1.25, 1.0, 3, vec![]),
            MonsterKind::Mob10 => (7500.0, 0.75, 1.0, 3, vec![]),
            MonsterKind::Mob11 => (10000.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob12 => (15000.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Mob13 => (45000.0, 0.5, 1.0, 3, vec![]),
            MonsterKind::Mob14 => (20000.0, 1.5, 1.0, 3, vec![]),
            MonsterKind::Mob15 => (45000.0, 1.0, 1.0, 3, vec![]),
            MonsterKind::Named01 => (100.0, 0.5, 3.0, 10, vec![]),
            MonsterKind::Named02 => (500.0, 0.75, 3.0, 12, vec![]),
            MonsterKind::Named03 => (1500.0, 1.0, 3.0, 14, vec![]),
            MonsterKind::Named04 => (3500.0, 1.25, 3.0, 16, vec![]),
            MonsterKind::Named05 => (15000.0, 0.5, 5.0, 18, vec![]),
            MonsterKind::Named06 => (30000.0, 0.75, 5.0, 20, vec![]),
            MonsterKind::Named07 => (45000.0, 1.0, 5.0, 22, vec![]),
            MonsterKind::Named08 => (65000.0, 1.25, 5.0, 24, vec![]),
            MonsterKind::Named09 => (200000.0, 0.5, 7.0, 26, vec![]),
            MonsterKind::Named10 => (250000.0, 0.75, 7.0, 28, vec![]),
            MonsterKind::Named11 => (300000.0, 1.0, 7.0, 30, vec![]),
            MonsterKind::Named12 => (300000.0, 1.25, 7.0, 32, vec![]),
            MonsterKind::Named13 => (650000.0, 0.5, 10.0, 34, vec![]),
            MonsterKind::Named14 => (550000.0, 0.75, 10.0, 36, vec![]),
            MonsterKind::Named15 => (550000.0, 1.0, 10.0, 38, vec![]),
            MonsterKind::Named16 => (750000.0, 1.25, 10.0, 40, vec![]),
            MonsterKind::Boss01 => (3500.0, 1.0, 15.0, 50, vec![]),
            MonsterKind::Boss02 => (10000.0, 1.0, 20.0, 75, vec![]),
            MonsterKind::Boss03 => (35000.0, 1.0, 20.0, 100, vec![]),
            MonsterKind::Boss04 => (85000.0, 1.0, 25.0, 100, vec![]),
            MonsterKind::Boss05 => (200000.0, 1.0, 25.0, 125, vec![]),
            MonsterKind::Boss06 => (300000.0, 1.0, 25.0, 125, vec![]),
            MonsterKind::Boss07 => (375000.0, 1.0, 50.0, 125, vec![]),
            MonsterKind::Boss08 => (500000.0, 1.0, 50.0, 125, vec![]),
            MonsterKind::Boss09 => (700000.0, 1.0, 50.0, 125, vec![]),
            MonsterKind::Boss10 => (850000.0, 1.0, 50.0, 125, vec![]),
            MonsterKind::Boss11 => (1125000.0, 1.0, 50.0, 125, vec![]),
        };
        Self {
            kind,
            max_hp,
            skills,
            velocity: Self::velocity(velocity),
            damage: Self::damage(damage),
            reward: Self::reward(reward),
        }
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
