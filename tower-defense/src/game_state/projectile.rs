use crate::{
    FixedRatio, MonsterId, WorldAcceleration, WorldCoord, WorldDistance, WorldSpeed, WorldVec,
};
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileBehavior {
    Direct,
    Homing {
        velocity: WorldVec,
        acceleration: WorldAcceleration,
        turn_rate: FixedRatio,
        max_speed: WorldSpeed,
        acceleration_remainder: i64,
        turn_remainder: i64,
    },
}

pub(crate) const HOMING_INITIAL_SPEED_MIN: WorldSpeed =
    WorldSpeed::from_raw(24 * crate::world::WORLD_UNITS_PER_TILE);
pub(crate) const HOMING_INITIAL_SPEED_MAX: WorldSpeed =
    WorldSpeed::from_raw(32 * crate::world::WORLD_UNITS_PER_TILE);
pub(crate) const HOMING_MAX_SPEED: WorldSpeed =
    WorldSpeed::from_raw(36 * crate::world::WORLD_UNITS_PER_TILE);
pub(crate) const HOMING_ACCELERATION: WorldAcceleration =
    WorldAcceleration::from_raw(1024 * crate::world::WORLD_UNITS_PER_TILE);
pub(crate) const HOMING_TURN_RATE_MIN: FixedRatio = FixedRatio::from_raw(2_000_000);
pub(crate) const HOMING_TURN_RATE_MAX: FixedRatio = FixedRatio::from_raw(8_000_000);
pub(crate) const HOMING_SWITCH_TO_DIRECT_DISTANCE: WorldDistance = WorldDistance::from_tiles(4);
pub(crate) const HOMING_DIRECT_ACCELERATION_MULTIPLIER: FixedRatio = FixedRatio::from_raw(100_000);
pub(crate) const PROJECTILE_COLLISION_RADIUS: WorldDistance = WorldDistance::from_raw(100_000);

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
    pub fn deterministic_trash(key: u64) -> Self {
        match key % 4 {
            0 => Self::Trash01,
            1 => Self::Trash02,
            2 => Self::Trash03,
            _ => Self::Trash04,
        }
    }
    pub fn deterministic_girl(key: u64) -> Self {
        match key % 5 {
            0 => Self::Girl00,
            1 => Self::Girl01,
            2 => Self::Girl02,
            3 => Self::Girl03,
            _ => Self::Girl04,
        }
    }
    pub fn image(&self) -> Image {
        match self {
            Self::Trash01 => crate::asset::image::attack::projectile::TRASH_01,
            Self::Trash02 => crate::asset::image::attack::projectile::TRASH_02,
            Self::Trash03 => crate::asset::image::attack::projectile::TRASH_03,
            Self::Trash04 => crate::asset::image::attack::projectile::TRASH_04,
            Self::Girl00 => crate::asset::image::attack::projectile::GIRL_00,
            Self::Girl01 => crate::asset::image::attack::projectile::GIRL_01,
            Self::Girl02 => crate::asset::image::attack::projectile::GIRL_02,
            Self::Girl03 => crate::asset::image::attack::projectile::GIRL_03,
            Self::Girl04 => crate::asset::image::attack::projectile::GIRL_04,
            Self::Cards00 => crate::asset::image::attack::projectile::CARDS_00,
            Self::Heart00 => crate::asset::image::attack::projectile::HEART_00,
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
    pub fn hit_sound(self) -> Option<fn() -> namui::AudioAsset> {
        match self {
            Self::Burning => Some(crate::sound::deterministic_flamethrower),
            Self::LightningSparkle => Some(crate::sound::deterministic_smoke_bomb),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, State)]
pub struct ProjectileTargetIndicator {
    id: MonsterId,
}
impl ProjectileTargetIndicator {
    pub const fn from_id(id: MonsterId) -> Self {
        Self { id }
    }
    pub const fn id(self) -> MonsterId {
        self.id
    }
}

pub(crate) fn homing_speed_for_key(key: u64) -> WorldSpeed {
    let span = HOMING_INITIAL_SPEED_MAX.raw() - HOMING_INITIAL_SPEED_MIN.raw();
    WorldSpeed::from_raw(HOMING_INITIAL_SPEED_MIN.raw() + (key % (span as u64 + 1)) as i64)
}
pub(crate) fn homing_turn_rate_for_key(key: u64) -> FixedRatio {
    let span = HOMING_TURN_RATE_MAX.raw() - HOMING_TURN_RATE_MIN.raw();
    FixedRatio::from_raw(HOMING_TURN_RATE_MIN.raw() + (key % (span as u64 + 1)) as i64)
}

pub(crate) fn move_direct(
    position: &mut WorldCoord,
    velocity: &mut WorldVec,
    movement_remainder: &mut i64,
    target: WorldCoord,
    speed: WorldSpeed,
) {
    let direction = target - *position;
    let distance = direction.length();
    let numerator = speed.raw() as i128 + *movement_remainder as i128;
    let step = (numerator / crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
    *movement_remainder = (numerator % crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
    if distance.is_zero() || step >= distance.raw() {
        *position = target;
        *velocity = WorldVec::ZERO;
        return;
    }
    *velocity = direction.scaled_by_distance(WorldDistance::from_raw(speed.raw()), distance);
    *position += direction.scaled_by_distance(WorldDistance::from_raw(step), distance);
}
