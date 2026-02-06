use super::*;
use rand::{Rng, thread_rng};
use std::sync::atomic::{AtomicUsize, Ordering};

const PROJECTILE_ROTATION_SPEED_DEG_RANGE: std::ops::RangeInclusive<f32> = -720.0..=720.0;

#[derive(Clone, State)]
pub struct Projectile {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub velocity: Xy<f32>,
    pub target_indicator: ProjectileTargetIndicator,
    pub damage: f32,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub trail: ProjectileTrail,
    pub behavior: ProjectileBehavior,
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
        // Initialize with upward direction for Direct projectiles (tiles/second)
        let speed = velocity * Duration::from_secs(1);
        let initial_direction = Xy::new(0.0, -1.0);
        Self {
            xy,
            kind,
            velocity: initial_direction * speed,
            target_indicator,
            damage,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail,
            behavior: ProjectileBehavior::Direct,
        }
    }

    pub fn new_homing(
        xy: MapCoordF32,
        kind: ProjectileKind,
        target_indicator: ProjectileTargetIndicator,
        damage: f32,
        trail: ProjectileTrail,
    ) -> Self {
        // Randomize initial speed and turn rate within configured ranges
        let mut rng = thread_rng();
        let initial_speed =
            rng.gen_range(HOMING_INITIAL_SPEED_MIN_TILE..=HOMING_INITIAL_SPEED_MAX_TILE);
        let turn_rate = rng.gen_range(HOMING_TURN_RATE_MIN_TILE..=HOMING_TURN_RATE_MAX_TILE);
        // Initial velocity: straight up in map coordinate space (tiles/second)
        let initial_velocity = Xy::new(0.0, -initial_speed);
        Self {
            xy,
            kind,
            velocity: initial_velocity,
            target_indicator,
            damage,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail,
            behavior: ProjectileBehavior::Homing {
                velocity: initial_velocity,
                acceleration: HOMING_ACCELERATION_TILE,
                turn_rate,
                max_speed: HOMING_MAX_SPEED_TILE,
            },
        }
    }

    pub(crate) fn move_by(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        let direction = (dest_xy - self.xy).normalize();
        let speed = self.velocity.length();
        self.xy += direction * speed * dt.as_secs_f32();
        // Update velocity to reflect actual movement direction (tiles/second)
        self.velocity = direction * speed;
        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }

    pub(crate) fn move_homing(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        let dt_secs = dt.as_secs_f32();
        if let ProjectileBehavior::Homing {
            velocity,
            acceleration,
            turn_rate,
            max_speed,
        } = &mut self.behavior
        {
            let distance_to_target = (dest_xy - self.xy).length();
            let desired_dir = (dest_xy - self.xy).normalize();

            // Phase transition: far = homing, close = direct high-speed
            if distance_to_target > HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE {
                // Phase 1: Homing behavior - turn gradually toward target
                let mut speed = velocity.length();
                let acceleration = *acceleration;
                let turn_rate = *turn_rate;
                let max_speed = *max_speed;
                speed = (speed + acceleration * dt_secs).min(max_speed);
                let target_velocity = desired_dir * speed;
                let t = (turn_rate * dt_secs).clamp(0.0, 1.0);
                *velocity += (target_velocity - *velocity) * t;
                self.xy += *velocity * dt_secs;
            } else {
                // Phase 2: Direct high-speed - accelerate toward target in straight line
                let acceleration = *acceleration;
                let max_speed = *max_speed;
                let mut speed = velocity.length();
                // Accelerate more aggressively when close
                speed = (speed + acceleration * dt_secs * HOMING_DIRECT_ACCELERATION_MULTIPLIER)
                    .min(max_speed);
                // Move directly toward target
                *velocity = desired_dir * speed;
                self.xy += *velocity * dt_secs;
            }

            // Update self.velocity to reflect actual movement velocity
            self.velocity = *velocity;
        }

        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub enum ProjectileBehavior {
    Direct,
    Homing {
        /// Current velocity vector (tiles/second, map coordinate units)
        velocity: Xy<f32>,
        /// Acceleration magnitude (tiles/second², map coordinate units)
        acceleration: f32,
        /// Turn rate - blending factor for direction change per second
        turn_rate: f32,
        /// Maximum speed (tiles/second, map coordinate units)
        max_speed: f32,
    },
}

/// Homing projectile initial upward speed - minimum (tiles/second, map coordinate units)
const HOMING_INITIAL_SPEED_MIN_TILE: f32 = 24.0;
/// Homing projectile initial upward speed - maximum (tiles/second, map coordinate units)
const HOMING_INITIAL_SPEED_MAX_TILE: f32 = 32.0;
/// Homing projectile maximum speed (tiles/second, map coordinate units)
const HOMING_MAX_SPEED_TILE: f32 = 36.0;
/// Homing projectile acceleration (tiles/second², map coordinate units)
const HOMING_ACCELERATION_TILE: f32 = 1024.0;
/// Homing projectile turn rate - minimum blending factor for direction change (0.0 = no turn, 1.0 = instant turn)
const HOMING_TURN_RATE_MIN_TILE: f32 = 2.0;
/// Homing projectile turn rate - maximum blending factor for direction change (0.0 = no turn, 1.0 = instant turn)
const HOMING_TURN_RATE_MAX_TILE: f32 = 8.0;
/// Distance threshold to switch from homing to direct movement (tiles, map coordinate units)
const HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE: f32 = 4.0;
/// Acceleration multiplier when in direct phase (applied to base acceleration)
const HOMING_DIRECT_ACCELERATION_MULTIPLIER: f32 = 0.1;
impl Component for &Projectile {
    fn render(self, ctx: &RenderCtx) {
        let projectile_wh = TILE_PX_SIZE * Wh::new(0.4, 0.4);
        let image = self.kind.image();

        ctx.rotate(self.rotation).add(namui::image(ImageParam {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
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
