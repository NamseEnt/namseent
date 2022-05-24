use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(Degree, f32);

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}°", f.precision().unwrap_or(0), self.0)
    }
}

impl Degree {
    pub fn to_radian(&self) -> Radian {
        Radian(self.0.to_radians())
    }
}
