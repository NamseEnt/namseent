use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(Radian, f32);

impl Display for Radian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0}°", self.0)
    }
}

impl Radian {
    pub fn to_degree(&self) -> Degree {
        Degree(self.0.to_degrees())
    }
}
