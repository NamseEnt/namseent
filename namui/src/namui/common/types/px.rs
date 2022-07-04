use num::{FromPrimitive, ToPrimitive};
use std::fmt::Display;

super::common_for_f32_type!(Px);

impl Display for Px {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0)
    }
}

impl FromPrimitive for Px {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Px(n as f32))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Px(n as f32))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Px(n as f32))
    }
}

impl ToPrimitive for Px {
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
