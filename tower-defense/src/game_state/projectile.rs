use super::*;
use rand::{Rng, thread_rng};
use std::sync::atomic::{AtomicUsize, Ordering};

const PROJECTILE_ROTATION_SPEED_DEG_RANGE: std::ops::RangeInclusive<f32> = -720.0..=720.0;

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

pub(crate) const HOMING_INITIAL_SPEED_MIN_TILE: f32 = 24.0;
pub(crate) const HOMING_INITIAL_SPEED_MAX_TILE: f32 = 32.0;
pub(crate) const HOMING_MAX_SPEED_TILE: f32 = 36.0;
pub(crate) const HOMING_ACCELERATION_TILE: f32 = 1024.0;
pub(crate) const HOMING_TURN_RATE_MIN_TILE: f32 = 2.0;
pub(crate) const HOMING_TURN_RATE_MAX_TILE: f32 = 8.0;
pub(crate) const HOMING_SWITCH_TO_DIRECT_DISTANCE_TILE: f32 = 4.0;
pub(crate) const HOMING_DIRECT_ACCELERATION_MULTIPLIER: f32 = 0.1;

pub(crate) fn random_rotation_speed() -> Angle {
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

impl ProjectileTrail {
    /// 투사체가 명중할 때 재생할 사운드 함수. 없으면 None.
    /// Trail 타입이 자신의 사운드 책임을 소유함으로써 처리 루프의 하드코딩을 제거.
    pub fn hit_sound(self) -> Option<fn() -> namui::AudioAsset> {
        match self {
            Self::Burning => Some(crate::sound::random_flamethrower),
            Self::LightningSparkle => Some(crate::sound::random_smoke_bomb),
            _ => None,
        }
    }
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
