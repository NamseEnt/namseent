mod skill;

use super::*;
use namui::*;
pub use skill::*;
use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct Tower {
    id: usize,
    pub left_top: MapCoord,
    pub last_shoot_time: Instant,
    template: TowerTemplate,
    pub status_effects: Vec<TowerStatusEffect>,
    pub skills: Vec<TowerSkill>,
}
impl Tower {
    pub fn new(template: &TowerTemplate, left_top: MapCoord) -> Self {
        const ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            left_top,
            last_shoot_time: Instant::now(),
            template: template.clone(),
            status_effects: vec![],
            skills: vec![],
        }
    }
    pub fn in_cooltime(&self, now: Instant) -> bool {
        now < self.last_shoot_time + self.shoot_interval
    }

    pub fn shoot(
        &mut self,
        target_indicator: ProjectileTargetIndicator,
        now: Instant,
    ) -> Projectile {
        self.last_shoot_time = now;

        Projectile {
            kind: self.projectile_kind,
            xy: self.left_top.map(|t| t as f32 + 0.5),
            velocity: self.projectile_speed,
            target_indicator,
            damage: self.damage,
        }
    }

    fn center_xy(&self) -> MapCoord {
        self.left_top + MapCoord::new(1, 1)
    }
    fn center_xy_f32(&self) -> MapCoordF32 {
        self.center_xy().map(|t| t as f32)
    }
}
impl Component for &Tower {
    fn render(self, ctx: &RenderCtx) {}
}
impl Deref for Tower {
    type Target = TowerTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Clone)]
pub struct TowerTemplate {
    pub kind: TowerKind,
    pub shoot_interval: Duration,
    pub attack_range_radius: f32,
    pub projectile_kind: ProjectileKind,
    pub projectile_speed: Velocity,
    pub damage: usize,
}

#[derive(Clone, Copy)]
pub enum TowerKind {}
