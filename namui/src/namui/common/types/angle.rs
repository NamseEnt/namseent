use num::{FromPrimitive, ToPrimitive};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Angle {
    Radian(f32),
    Degree(f32),
}

impl Angle {
    pub fn as_radians(&self) -> f32 {
        match self {
            Angle::Radian(radian) => *radian,
            Angle::Degree(degree) => degree.to_radians(),
        }
    }

    pub fn sin(&self) -> f32 {
        self.as_radians().sin()
    }

    pub fn cos(&self) -> f32 {
        self.as_radians().cos()
    }

    pub fn tan(&self) -> f32 {
        self.as_radians().tan()
    }
}

impl std::ops::Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        match (self, other) {
            (Angle::Radian(a), Angle::Radian(b)) => Angle::Radian(a + b),
            (Angle::Degree(a), Angle::Degree(b)) => Angle::Degree(a + b),
            (Angle::Radian(a), Angle::Degree(b)) => Angle::Radian(a + b.to_radians()),
            (Angle::Degree(a), Angle::Radian(b)) => Angle::Degree(a + b.to_degrees()),
        }
    }
}

impl std::ops::Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        match self {
            Angle::Radian(a) => Angle::Radian(-a),
            Angle::Degree(a) => Angle::Degree(-a),
        }
    }
}

impl ToPrimitive for Angle {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Angle::Radian(radian) => radian.to_i64(),
            Angle::Degree(degree) => degree.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Angle::Radian(radian) => radian.to_u64(),
            Angle::Degree(degree) => degree.to_u64(),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            Angle::Radian(radian) => radian.to_f64(),
            Angle::Degree(degree) => degree.to_f64(),
        }
    }
}

impl FromPrimitive for Angle {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Angle::Radian(n as f32))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Angle::Radian(n as f32))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Angle::Radian(n as f32))
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Angle::Radian(x) | Angle::Degree(x) => x,
        };
        let unit = match self {
            Angle::Radian(_) => "",
            Angle::Degree(_) => "°",
        };
        write!(f, "{:.*?}{}", f.precision().unwrap_or(0), value, unit)
    }
}
