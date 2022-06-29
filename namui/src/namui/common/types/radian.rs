use super::*;
use num::{FromPrimitive, ToPrimitive};
use std::fmt::Display;

super::common_for_f32_type!(Radian);

impl Display for Radian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}", f.precision().unwrap_or(3), self.0)
    }
}

impl Radian {
    pub fn to_degrees(&self) -> Degree {
        self.into()
    }
    pub fn cos(&self) -> f32 {
        self.0.cos()
    }
    pub fn sin(&self) -> f32 {
        self.0.sin()
    }
    pub fn tan(&self) -> f32 {
        self.0.tan()
    }
    pub fn acos(&self) -> f32 {
        self.0.acos()
    }
    pub fn asin(&self) -> f32 {
        self.0.asin()
    }
    pub fn atan(&self) -> f32 {
        self.0.atan()
    }
    pub fn atan2(&self, other: Radian) -> f32 {
        self.0.atan2(other.0)
    }
}

impl ToPrimitive for Radian {
    fn to_i64(&self) -> Option<i64> {
        Some(self.0 as i64)
    }
    fn to_u64(&self) -> Option<u64> {
        Some(self.0 as u64)
    }
    fn to_f64(&self) -> Option<f64> {
        Some(self.0 as f64)
    }
}

impl FromPrimitive for Radian {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Radian(n as f32))
    }
    fn from_u64(n: u64) -> Option<Self> {
        Some(Radian(n as f32))
    }
    fn from_f64(n: f64) -> Option<Self> {
        Some(Radian(n as f32))
    }
}

impl Into<Degree> for &Radian {
    fn into(self) -> Degree {
        Degree::from_f32(self.0.to_degrees()).unwrap()
    }
}
