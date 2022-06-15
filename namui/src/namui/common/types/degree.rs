use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(Degree, f32); // NOTE: `PixelSize` naming is for distinguishing from `PixelColor`.

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0}°", self.0)
    }
}
