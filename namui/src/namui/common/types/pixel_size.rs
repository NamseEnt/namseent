use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(PixelSize, f32); // NOTE: `PixelSize` naming is for distinguishing from `PixelColor`.

impl Display for PixelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0)
    }
}
