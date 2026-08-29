//! Namui state and pixel-conversion adapters for `td_core::world`.

use namui::*;
use serde::{
    Deserialize as SerdeDeserialize, Deserializer as SerdeDeserializer,
    Serialize as SerdeSerialize, Serializer as SerdeSerializer,
};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const WORLD_UNITS_PER_TILE: i64 = td_core::WORLD_UNITS_PER_TILE;
pub const SIM_TICKS_PER_SECOND: i64 = td_core::SIM_TICKS_PER_SECOND as i64;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct WorldCoord {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct WorldVec {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct WorldDistance(i64);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct WorldSpeed(i64);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct WorldAcceleration(i64);

impl WorldCoord {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn from_tile(x: i64, y: i64) -> Self {
        Self::from_core(td_core::WorldCoord::from_tile(x, y))
    }

    pub fn from_tile_center(x: i64, y: i64) -> Self {
        Self::from_core(td_core::WorldCoord::from_tile_center(x, y))
    }

    pub fn as_map_coord_f32(self) -> Xy<f32> {
        Xy::new(
            self.x as f32 / WORLD_UNITS_PER_TILE as f32,
            self.y as f32 / WORLD_UNITS_PER_TILE as f32,
        )
    }

    fn to_core(self) -> td_core::WorldCoord {
        td_core::WorldCoord::new(self.x, self.y)
    }

    const fn from_core(value: td_core::WorldCoord) -> Self {
        Self::new(value.x, value.y)
    }
}

impl WorldVec {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn length_squared(self) -> u128 {
        self.to_core().length_squared()
    }

    pub fn length(self) -> WorldDistance {
        WorldDistance::from_core(self.to_core().length())
    }

    pub fn scaled_by_distance(self, distance: WorldDistance, denominator: WorldDistance) -> Self {
        Self::from_core(
            self.to_core()
                .scaled_by_distance(distance.to_core(), denominator.to_core()),
        )
    }

    pub fn scaled_by_ratio(self, ratio: crate::FixedRatio) -> Self {
        Self::from_core(
            self.to_core()
                .scaled_by_ratio(td_core::FixedRatio::from_raw(ratio.raw())),
        )
    }

    fn to_core(self) -> td_core::WorldVec {
        td_core::WorldVec::new(self.x, self.y)
    }

    const fn from_core(value: td_core::WorldVec) -> Self {
        Self::new(value.x, value.y)
    }
}

pub fn segment_hits_point(
    start: WorldCoord,
    end: WorldCoord,
    point: WorldCoord,
    radius: WorldDistance,
) -> bool {
    td_core::world::segment_hits_point(
        start.to_core(),
        end.to_core(),
        point.to_core(),
        radius.to_core(),
    )
}

impl WorldDistance {
    pub const ZERO: Self = Self(0);

    pub const fn from_raw(raw: i64) -> Self {
        Self::from_core(td_core::WorldDistance::from_raw(raw))
    }

    pub const fn from_tiles(tiles: i64) -> Self {
        Self::from_core(td_core::WorldDistance::from_tiles(tiles))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn scaled_by(self, ratio: crate::FixedRatio) -> Self {
        Self::from_core(
            self.to_core()
                .scaled_by(td_core::FixedRatio::from_raw(ratio.raw())),
        )
    }

    fn to_core(self) -> td_core::WorldDistance {
        td_core::WorldDistance::from_raw(self.0)
    }

    const fn from_core(value: td_core::WorldDistance) -> Self {
        Self(value.raw())
    }
}

impl fmt::Display for WorldDistance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.3}",
            self.raw() as f64 / WORLD_UNITS_PER_TILE as f64
        )
    }
}

impl WorldSpeed {
    pub const fn from_raw(raw: i64) -> Self {
        Self::from_core(td_core::WorldSpeed::from_raw(raw))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    const fn from_core(value: td_core::WorldSpeed) -> Self {
        Self(value.raw())
    }
}

impl WorldAcceleration {
    pub const fn from_raw(raw: i64) -> Self {
        Self::from_core(td_core::WorldAcceleration::from_raw(raw))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    const fn from_core(value: td_core::WorldAcceleration) -> Self {
        Self(value.raw())
    }
}

impl Add<WorldVec> for WorldCoord {
    type Output = Self;

    fn add(self, rhs: WorldVec) -> Self {
        Self::from_core(self.to_core() + rhs.to_core())
    }
}

impl AddAssign<WorldVec> for WorldCoord {
    fn add_assign(&mut self, rhs: WorldVec) {
        *self = *self + rhs;
    }
}

impl Sub<WorldVec> for WorldCoord {
    type Output = Self;

    fn sub(self, rhs: WorldVec) -> Self {
        Self::from_core(self.to_core() - rhs.to_core())
    }
}

impl Sub for WorldCoord {
    type Output = WorldVec;

    fn sub(self, rhs: Self) -> WorldVec {
        WorldVec::from_core(self.to_core() - rhs.to_core())
    }
}

impl Add for WorldVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_core(self.to_core() + rhs.to_core())
    }
}

impl AddAssign for WorldVec {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for WorldVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_core(self.to_core() - rhs.to_core())
    }
}

impl SubAssign for WorldVec {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SerdeSerialize for WorldDistance {
    fn serialize<S: SerdeSerializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_core().serialize(serializer)
    }
}

impl<'de> SerdeDeserialize<'de> for WorldDistance {
    fn deserialize<D: SerdeDeserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        td_core::WorldDistance::deserialize(deserializer).map(Self::from_core)
    }
}

pub use td_core::integer_sqrt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_units_use_one_million_per_tile() {
        assert_eq!(
            WorldCoord::from_tile(3, -2),
            WorldCoord::new(3_000_000, -2_000_000)
        );
        assert_eq!(WorldDistance::from_tiles(2).raw(), 2_000_000);
    }

    #[test]
    fn wrapper_geometry_matches_core() {
        let vector = WorldVec::new(3_000_000, 4_000_000);
        assert_eq!(
            vector.length().raw(),
            td_core::WorldVec::new(3_000_000, 4_000_000).length().raw()
        );
        assert_eq!(
            vector
                .scaled_by_distance(WorldDistance::from_tiles(1), WorldDistance::from_tiles(2))
                .x,
            td_core::WorldVec::new(3_000_000, 4_000_000)
                .scaled_by_distance(
                    td_core::WorldDistance::from_tiles(1),
                    td_core::WorldDistance::from_tiles(2)
                )
                .x
        );
    }

    #[test]
    fn integer_sqrt_has_floor_semantics() {
        assert_eq!(integer_sqrt(0), 0);
        assert_eq!(integer_sqrt(1), 1);
        assert_eq!(integer_sqrt(15), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(17), 4);
        assert!(integer_sqrt(u128::MAX) > 0);
    }

    #[test]
    fn segment_collision_handles_tunneling() {
        let start = WorldCoord::new(0, 0);
        let end = WorldCoord::new(10 * WORLD_UNITS_PER_TILE, 0);
        let point = WorldCoord::new(5 * WORLD_UNITS_PER_TILE, 0);
        assert!(segment_hits_point(start, end, point, WorldDistance::ZERO));
        assert_eq!(
            segment_hits_point(start, end, point, WorldDistance::ZERO),
            td_core::world::segment_hits_point(
                td_core::WorldCoord::new(start.x, start.y),
                td_core::WorldCoord::new(end.x, end.y),
                td_core::WorldCoord::new(point.x, point.y),
                td_core::WorldDistance::ZERO,
            )
        );
    }
}
