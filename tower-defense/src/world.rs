use namui::*;
use serde::{
    Deserialize as SerdeDeserialize, Deserializer as SerdeDeserializer,
    Serialize as SerdeSerialize, Serializer as SerdeSerializer, de::Error as DeError,
};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const WORLD_UNITS_PER_TILE: i64 = 1_000_000;
pub const SIM_TICKS_PER_SECOND: i64 = 60;

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
        Self {
            x: x.saturating_mul(WORLD_UNITS_PER_TILE),
            y: y.saturating_mul(WORLD_UNITS_PER_TILE),
        }
    }
    pub fn from_tile_center(x: i64, y: i64) -> Self {
        Self::from_tile(x, y) + WorldVec::new(WORLD_UNITS_PER_TILE / 2, WORLD_UNITS_PER_TILE / 2)
    }
    pub fn as_map_coord_f32(self) -> Xy<f32> {
        Xy::new(
            self.x as f32 / WORLD_UNITS_PER_TILE as f32,
            self.y as f32 / WORLD_UNITS_PER_TILE as f32,
        )
    }
}

impl WorldVec {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn length_squared(self) -> u128 {
        let x = self.x as i128;
        let y = self.y as i128;
        x.saturating_mul(x).saturating_add(y.saturating_mul(y)) as u128
    }
    pub fn length(self) -> WorldDistance {
        WorldDistance::from_raw(integer_sqrt(self.length_squared()).min(i64::MAX as u128) as i64)
    }
    pub fn scaled_by_distance(self, distance: WorldDistance, denominator: WorldDistance) -> Self {
        if denominator.is_zero() {
            return Self::ZERO;
        }
        let denominator = denominator.raw() as i128;
        let scale = distance.raw() as i128;
        let round = |component: i64| {
            let value = (component as i128).saturating_mul(scale);
            let value = if value >= 0 {
                value.saturating_add(denominator / 2) / denominator
            } else {
                (value.saturating_sub(denominator / 2)) / denominator
            };
            value.clamp(i64::MIN as i128, i64::MAX as i128) as i64
        };
        Self::new(round(self.x), round(self.y))
    }
    pub fn scaled_by_ratio(self, ratio: crate::FixedRatio) -> Self {
        let value = |component: i64| {
            ((component as i128).saturating_mul(ratio.raw() as i128)
                / crate::combat_number::RATIO_SCALE as i128)
                .clamp(i64::MIN as i128, i64::MAX as i128) as i64
        };
        Self::new(value(self.x), value(self.y))
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
    let radius_sq = (radius.raw() as i128).unsigned_abs().saturating_pow(2);
    if ab_len_sq == 0 {
        return ap.length_squared() <= radius_sq;
    }
    let dot = (ap.x as i128)
        .saturating_mul(ab.x as i128)
        .saturating_add((ap.y as i128).saturating_mul(ab.y as i128));
    if dot <= 0 {
        return ap.length_squared() <= radius_sq;
    }
    if dot >= ab_len_sq as i128 {
        return (point - end).length_squared() <= radius_sq;
    }
    let cross = (ab.x as i128)
        .saturating_mul(ap.y as i128)
        .saturating_sub((ab.y as i128).saturating_mul(ap.x as i128));
    cross.unsigned_abs().saturating_pow(2) <= radius_sq.saturating_mul(ab_len_sq)
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
impl fmt::Display for WorldDistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.0 as f64 / WORLD_UNITS_PER_TILE as f64)
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
        Ok(Self::from_raw(rounded as i64))
    }
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
    }
}
