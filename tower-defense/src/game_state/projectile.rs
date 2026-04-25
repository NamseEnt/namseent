use super::*;
use rand::{Rng, thread_rng};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

const PROJECTILE_ROTATION_SPEED_DEG_RANGE: std::ops::RangeInclusive<f32> = -720.0..=720.0;
static NEXT_PROJECTILE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, State)]
pub struct Projectile {
    pub id: u64,
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub velocity: Xy<f32>,
    pub target_indicator: ProjectileTargetIndicator,
    pub damage: f32,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub trail: ProjectileTrail,
    pub behavior: ProjectileBehavior,
    pub hit_effect: attack::ProjectileHitEffect,
    pub source_tower_id: Option<usize>,
    pub source_tower_info: Option<(TowerKind, Rank, Suit)>,
}

#[derive(Clone, Debug)]
pub struct ProjectileParams {
    pub damage: f32,
    pub trail: ProjectileTrail,
    pub hit_effect: attack::ProjectileHitEffect,
    pub source_tower_id: Option<usize>,
    pub source_tower_info: Option<(TowerKind, Rank, Suit)>,
}

impl Projectile {
    pub fn new(
        xy: MapCoordF32,
        kind: ProjectileKind,
        velocity: Velocity,
        target_indicator: ProjectileTargetIndicator,
        params: ProjectileParams,
    ) -> Self {
        let speed = velocity * Duration::from_secs(1);
        let initial_direction = Xy::new(0.0, -1.0);
        Self {
            id: NEXT_PROJECTILE_ID.fetch_add(1, Ordering::Relaxed),
            xy,
            kind,
            velocity: initial_direction * speed,
            target_indicator,
            damage: params.damage,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail: params.trail,
            behavior: ProjectileBehavior::Direct,
            hit_effect: params.hit_effect,
            source_tower_id: params.source_tower_id,
            source_tower_info: params.source_tower_info,
        }
    }

    pub fn new_homing(
        xy: MapCoordF32,
        kind: ProjectileKind,
        target_indicator: ProjectileTargetIndicator,
        params: ProjectileParams,
    ) -> Self {
        let mut rng = thread_rng();
        let initial_speed =
            rng.gen_range(HOMING_INITIAL_SPEED_MIN_TILE..=HOMING_INITIAL_SPEED_MAX_TILE);
        let turn_rate = rng.gen_range(HOMING_TURN_RATE_MIN_TILE..=HOMING_TURN_RATE_MAX_TILE);
        let initial_velocity = Xy::new(0.0, -initial_speed);
        Self {
            id: NEXT_PROJECTILE_ID.fetch_add(1, Ordering::Relaxed),
            xy,
            kind,
            velocity: initial_velocity,
            target_indicator,
            damage: params.damage,
            rotation: 0.0.deg(),
            rotation_speed: random_rotation_speed(),
            trail: params.trail,
            behavior: ProjectileBehavior::Homing {
                velocity: initial_velocity,
                acceleration: HOMING_ACCELERATION_TILE,
                turn_rate,
                max_speed: HOMING_MAX_SPEED_TILE,
            },
            hit_effect: params.hit_effect,
            source_tower_id: params.source_tower_id,
            source_tower_info: params.source_tower_info,
        }
    }

    pub(crate) fn move_by(&mut self, dt: Duration, dest_xy: MapCoordF32) {
        let direction = (dest_xy - self.xy).normalize();
        let speed = self.velocity.length();
        self.xy += direction * speed * dt.as_secs_f32();
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

            if distance_to_target > HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE {
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
                let acceleration = *acceleration;
                let max_speed = *max_speed;
                let mut speed = velocity.length();
                speed = (speed + acceleration * dt_secs * HOMING_DIRECT_ACCELERATION_MULTIPLIER)
                    .min(max_speed);
                *velocity = desired_dir * speed;
                self.xy += *velocity * dt_secs;
            }

            self.velocity = *velocity;
        }

        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub enum ProjectileBehavior {
    Direct,
    Homing {
        velocity: Xy<f32>,
        acceleration: f32,
        turn_rate: f32,
        max_speed: f32,
    },
}

const HOMING_INITIAL_SPEED_MIN_TILE: f32 = 24.0;
const HOMING_INITIAL_SPEED_MAX_TILE: f32 = 32.0;
const HOMING_MAX_SPEED_TILE: f32 = 36.0;
const HOMING_ACCELERATION_TILE: f32 = 1024.0;
const HOMING_TURN_RATE_MIN_TILE: f32 = 2.0;
const HOMING_TURN_RATE_MAX_TILE: f32 = 8.0;
const HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE: f32 = 4.0;
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
    Girl00,
    Girl01,
    Girl02,
    Girl03,
    Girl04,
    Cards00,
    Heart00,
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

    pub fn random_girl() -> Self {
        match thread_rng().gen_range(0..5) {
            0 => ProjectileKind::Girl00,
            1 => ProjectileKind::Girl01,
            2 => ProjectileKind::Girl02,
            3 => ProjectileKind::Girl03,
            4 => ProjectileKind::Girl04,
            _ => unreachable!(),
        }
    }

    pub fn random_cards() -> Self {
        ProjectileKind::Cards00
    }

    pub fn random_heart() -> Self {
        ProjectileKind::Heart00
    }

    pub fn image(&self) -> Image {
        match self {
            ProjectileKind::Trash01 => crate::asset::image::attack::projectile::TRASH_01,
            ProjectileKind::Trash02 => crate::asset::image::attack::projectile::TRASH_02,
            ProjectileKind::Trash03 => crate::asset::image::attack::projectile::TRASH_03,
            ProjectileKind::Trash04 => crate::asset::image::attack::projectile::TRASH_04,
            ProjectileKind::Girl00 => crate::asset::image::attack::projectile::GIRL_00,
            ProjectileKind::Girl01 => crate::asset::image::attack::projectile::GIRL_01,
            ProjectileKind::Girl02 => crate::asset::image::attack::projectile::GIRL_02,
            ProjectileKind::Girl03 => crate::asset::image::attack::projectile::GIRL_03,
            ProjectileKind::Girl04 => crate::asset::image::attack::projectile::GIRL_04,
            ProjectileKind::Cards00 => crate::asset::image::attack::projectile::CARDS_00,
            ProjectileKind::Heart00 => crate::asset::image::attack::projectile::HEART_00,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileTrail {
    None,
    Burning,
    Sparkle,
    WindCurve,
    Heart,
    LightningSparkle,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, State)]
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
