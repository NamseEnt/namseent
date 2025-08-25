use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct Projectile {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub velocity: Velocity,
    pub target_indicator: ProjectileTargetIndicator,
    pub damage: f32,
}
impl Projectile {
    pub(crate) fn move_by(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        self.xy += (dest_xy - self.xy).normalize() * (self.velocity * dt);
    }
}
impl Component for &Projectile {
    fn render(self, ctx: &RenderCtx) {
        let projectile_wh = TILE_PX_SIZE
            * match self.kind {
                ProjectileKind::Ball => Wh::new(0.1, 0.1),
            };
        let path = Path::new().add_oval(Rect::from_xy_wh(
            projectile_wh.to_xy() * -0.5,
            projectile_wh,
        ));
        let paint = Paint::new(Color::GREEN);
        ctx.translate(TILE_PX_SIZE.to_xy() * 0.5)
            .add(namui::path(path, paint));
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ProjectileKind {
    Ball,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProjectileTargetIndicator {
    id: usize,
}

impl Default for ProjectileTargetIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectileTargetIndicator {
    pub fn new() -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}
