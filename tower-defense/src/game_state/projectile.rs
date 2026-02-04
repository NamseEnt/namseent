use super::*;
use rand::{Rng, thread_rng};
use std::sync::atomic::{AtomicUsize, Ordering};

const PROJECTILE_ROTATION_SPEED_DEG_RANGE: std::ops::RangeInclusive<f32> = -720.0..=720.0;

#[derive(Clone, State)]
pub struct Projectile {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub velocity: Velocity,
    pub target_indicator: ProjectileTargetIndicator,
    pub damage: f32,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub trail: ProjectileTrail,
}
impl Projectile {
    pub fn new(
        xy: MapCoordF32,
        kind: ProjectileKind,
        velocity: Velocity,
        target_indicator: ProjectileTargetIndicator,
        damage: f32,
        trail: ProjectileTrail,
    ) -> Self {
        Self {
            xy,
            kind,
            velocity,
            target_indicator,
            damage,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail,
        }
    }

    pub(crate) fn move_by(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        self.xy += (dest_xy - self.xy).normalize() * (self.velocity * dt);
        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }
}
impl Component for &Projectile {
    fn render(self, ctx: &RenderCtx) {
        let projectile_wh = TILE_PX_SIZE * Wh::new(0.4, 0.4);
        let image = self.kind.image();

        ctx.translate(TILE_PX_SIZE.to_xy() * 0.5)
            .rotate(self.rotation)
            .add(namui::image(ImageParam {
                rect: Rect::from_xy_wh(projectile_wh.to_xy() * -0.5, projectile_wh),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
    }
}

fn random_rotation_speed() -> Angle {
    let degrees_per_sec = thread_rng().gen_range(PROJECTILE_ROTATION_SPEED_DEG_RANGE);
    degrees_per_sec.deg()
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub enum ProjectileKind {
    Trash01,
    Trash02,
    Trash03,
    Trash04,
}
impl ProjectileKind {
    pub fn random_trash() -> Self {
        match thread_rng().gen_range(0..4) {
            0 => ProjectileKind::Trash01,
            1 => ProjectileKind::Trash02,
            2 => ProjectileKind::Trash03,
            3 => ProjectileKind::Trash04,
            _ => unreachable!(),
        }
    }

    pub fn image(&self) -> Image {
        match self {
            ProjectileKind::Trash01 => crate::asset::image::attack::projectile::TRASH_01,
            ProjectileKind::Trash02 => crate::asset::image::attack::projectile::TRASH_02,
            ProjectileKind::Trash03 => crate::asset::image::attack::projectile::TRASH_03,
            ProjectileKind::Trash04 => crate::asset::image::attack::projectile::TRASH_04,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileTrail {
    None,
    Burning,
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
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
