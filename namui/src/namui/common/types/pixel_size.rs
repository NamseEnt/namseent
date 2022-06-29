use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(PixelSize, f32); // NOTE: `PixelSize` naming is for distinguishing from `PixelColor`.

impl Display for PixelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0)
    }
}

impl num::FromPrimitive for PixelSize {
    fn from_i64(n: i64) -> Option<Self> {
        Some(PixelSize(n as f32))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(PixelSize(n as f32))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(PixelSize(n as f32))
    }
}

impl num::ToPrimitive for PixelSize {
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
