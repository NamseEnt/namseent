use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(OneZero, f32, |value| num::clamp(value, 0.0, 1.0));

impl Display for OneZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let in_f32: f32 = self.into();
        write!(f, "{:.*?}", f.precision().unwrap_or(3), in_f32)
    }
}

impl num::FromPrimitive for OneZero {
    fn from_i64(n: i64) -> Option<Self> {
        Some(OneZero(num::clamp(n as f32, 0.0, 1.0)))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(OneZero(num::clamp(n as f32, 0.0, 1.0)))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(OneZero(num::clamp(n as f32, 0.0, 1.0)))
    }
}

impl num::ToPrimitive for OneZero {
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
