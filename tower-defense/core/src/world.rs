//! Integer world geometry for authoritative simulation.

use serde::{
    Deserialize as SerdeDeserialize, Deserializer as SerdeDeserializer,
    Serialize as SerdeSerialize, Serializer as SerdeSerializer, de::Error as DeError,
};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const WORLD_UNITS_PER_TILE: i64 = 1_000_000;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct WorldCoord {
    pub x: i64,
    pub y: i64,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct WorldVec {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldDistance(i64);

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct WorldSpeed(i64);

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct WorldAcceleration(i64);

impl WorldCoord {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub const fn from_tile(x: i64, y: i64) -> Self {
        Self::new(
            x.saturating_mul(WORLD_UNITS_PER_TILE),
            y.saturating_mul(WORLD_UNITS_PER_TILE),
        )
    }
    pub const fn from_tile_center(x: i64, y: i64) -> Self {
        Self::new(
            x.saturating_mul(WORLD_UNITS_PER_TILE)
                .saturating_add(WORLD_UNITS_PER_TILE / 2),
            y.saturating_mul(WORLD_UNITS_PER_TILE)
                .saturating_add(WORLD_UNITS_PER_TILE / 2),
        )
    }
}

impl WorldVec {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn length_squared(self) -> u128 {
        let x = i128::from(self.x);
        let y = i128::from(self.y);
        x.saturating_mul(x).saturating_add(y.saturating_mul(y)) as u128
    }
    pub fn length(self) -> WorldDistance {
        WorldDistance::from_raw(integer_sqrt(self.length_squared()).min(i64::MAX as u128) as i64)
    }
    pub fn scaled_by_distance(self, distance: WorldDistance, denominator: WorldDistance) -> Self {
        Self::new(
            scale_component(self.x, distance.raw(), denominator.raw()),
            scale_component(self.y, distance.raw(), denominator.raw()),
        )
    }
    pub fn scaled_by_ratio(self, ratio: crate::FixedRatio) -> Self {
        let value = |component: i64| {
            (i128::from(component).saturating_mul(i128::from(ratio.raw()))
                / i128::from(crate::RATIO_SCALE))
            .clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
        };
        Self::new(value(self.x), value(self.y))
    }
}

impl WorldDistance {
    pub const ZERO: Self = Self(0);
    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }

    pub const fn from_tiles(tiles: i64) -> Self {
        Self::from_raw(tiles.saturating_mul(WORLD_UNITS_PER_TILE))
    }
    pub const fn raw(self) -> i64 {
        self.0
    }
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
    pub fn scaled_by(self, ratio: crate::FixedRatio) -> Self {
        Self::from_raw(crate::RatioProduct::one().with(ratio).apply_raw(self.0))
    }
}

impl SerdeSerialize for WorldDistance {
    fn serialize<S: SerdeSerializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(self.0 as f64 / WORLD_UNITS_PER_TILE as f64)
    }
}

impl<'de> SerdeDeserialize<'de> for WorldDistance {
    fn deserialize<D: SerdeDeserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <f64 as SerdeDeserialize>::deserialize(deserializer)?;
        if !value.is_finite() || value < 0.0 {
            return Err(D::Error::custom(
                "world distance must be finite and non-negative",
            ));
        }
        let scaled = value * WORLD_UNITS_PER_TILE as f64;
        let rounded = if scaled.fract() >= 0.5 {
            scaled.floor() + 1.0
        } else {
            scaled.floor()
        };
        if rounded > i64::MAX as f64 {
            return Err(D::Error::custom("world distance is out of range"));
        }
        Ok(WorldDistance::from_raw(rounded as i64))
    }
}

impl WorldSpeed {
    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }
    pub const fn raw(self) -> i64 {
        self.0
    }
}

impl WorldAcceleration {
    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }
    pub const fn raw(self) -> i64 {
        self.0
    }
}

impl Add<WorldVec> for WorldCoord {
    type Output = Self;
    fn add(self, rhs: WorldVec) -> Self {
        Self::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
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
        Self::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}
impl Sub for WorldCoord {
    type Output = WorldVec;
    fn sub(self, rhs: Self) -> WorldVec {
        WorldVec::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}
impl Add for WorldVec {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y))
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
        Self::new(self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y))
    }
}
impl SubAssign for WorldVec {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

pub fn segment_hits_point(
    start: WorldCoord,
    end: WorldCoord,
    point: WorldCoord,
    radius: WorldDistance,
) -> bool {
    let ab = end - start;
    let ap = point - start;
    let ab_len_sq = ab.length_squared();
    let radius_sq = u128::from(radius.raw().unsigned_abs()).saturating_pow(2);
    if ab_len_sq == 0 {
        return ap.length_squared() <= radius_sq;
    }
    let dot = i128::from(ap.x)
        .saturating_mul(i128::from(ab.x))
        .saturating_add(i128::from(ap.y).saturating_mul(i128::from(ab.y)));
    if dot <= 0 {
        return ap.length_squared() <= radius_sq;
    }
    if dot >= ab_len_sq as i128 {
        return (point - end).length_squared() <= radius_sq;
    }
    let cross = i128::from(ab.x)
        .saturating_mul(i128::from(ap.y))
        .saturating_sub(i128::from(ab.y).saturating_mul(i128::from(ap.x)));
    cross.unsigned_abs().saturating_pow(2) <= radius_sq.saturating_mul(ab_len_sq)
}

pub fn integer_sqrt(value: u128) -> u128 {
    if value < 2 {
        return value;
    }
    let shift = (127 - value.leading_zeros()) & !1;
    let mut bit = 1u128 << shift;
    let mut result = 0u128;
    let mut remainder = value;
    while bit != 0 {
        let candidate = result + bit;
        if remainder >= candidate {
            remainder -= candidate;
            result = (result >> 1) + bit;
        } else {
            result >>= 1;
        }
        bit >>= 2;
    }
    result
}

fn scale_component(component: i64, distance: i64, denominator: i64) -> i64 {
    if denominator <= 0 {
        return 0;
    }
    let value = i128::from(component).saturating_mul(i128::from(distance));
    let denominator = i128::from(denominator);
    let value = if value >= 0 {
        value.saturating_add(denominator / 2) / denominator
    } else {
        value.saturating_sub(denominator / 2) / denominator
    };
    value.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_matches_fixed_point_golden_values() {
        assert_eq!(
            WorldCoord::from_tile_center(3, -2),
            WorldCoord::new(3_500_000, -1_500_000)
        );
        assert_eq!(WorldVec::new(3, 4).length(), WorldDistance::from_raw(5));
        assert_eq!(
            serde_json::to_string(&WorldDistance::from_tiles(2)).unwrap(),
            "2.0"
        );
        assert!(segment_hits_point(
            WorldCoord::new(0, 0),
            WorldCoord::new(10, 0),
            WorldCoord::new(5, 0),
            WorldDistance::ZERO,
        ));
    }
}
