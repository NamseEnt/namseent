use super::*;
use num::FromPrimitive;
use std::fmt::Display;

super::common_for_f32_type!(Degree);

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}°", f.precision().unwrap_or(0), self.0)
    }
}

impl Degree {
    pub fn to_radians(&self) -> Radian {
        self.into()
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

impl Into<Radian> for &Degree {
    fn into(self) -> Radian {
        Radian::from_f32(self.0.to_radians()).unwrap()
    }
}
