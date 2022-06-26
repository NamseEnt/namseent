use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(Radian, f32);

impl Display for Radian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}", f.precision().unwrap_or(3), self.0)
    }
}

impl Radian {
    pub fn to_degree(&self) -> Degree {
        Degree(self.0.to_degrees())
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
