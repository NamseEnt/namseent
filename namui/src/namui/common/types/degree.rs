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

impl num::FromPrimitive for Degree {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Degree(n as f32))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Degree(n as f32))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Degree(n as f32))
    }
}

impl num::ToPrimitive for Degree {
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
