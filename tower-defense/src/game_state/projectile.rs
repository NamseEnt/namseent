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

#[derive(Clone, Copy)]
pub enum ProjectileKind {
    Ball,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProjectileTargetIndicator {
    id: usize,
}

impl ProjectileTargetIndicator {
    pub fn new() -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}
