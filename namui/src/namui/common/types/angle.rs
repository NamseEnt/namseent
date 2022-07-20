use num::cast::AsPrimitive;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Angle {
    Radian(f32),
    Degree(f32),
}

pub trait AngleExt {
    fn deg(self) -> Angle;
    fn rad(self) -> Angle;
}

impl AngleExt for f32 {
    fn deg(self) -> Angle {
        Angle::Degree(self)
    }
    fn rad(self) -> Angle {
        Angle::Radian(self)
    }
}

impl Angle {
    pub fn as_radians(&self) -> f32 {
        match self {
            Angle::Radian(angle) => *angle,
            Angle::Degree(degree) => degree.to_radians(),
        }
    }
    pub fn as_degrees(&self) -> f32 {
        match self {
            Angle::Radian(angle) => angle.to_degrees(),
            Angle::Degree(degree) => *degree,
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

    pub fn atan2(y: impl AsPrimitive<f32>, x: impl AsPrimitive<f32>) -> Self {
        Angle::Radian(y.as_().atan2(x.as_()))
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

impl std::ops::Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        self + (-other)
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

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Angle::Radian(a), Angle::Radian(b)) => a == b,
            (Angle::Degree(a), Angle::Degree(b)) => a == b,
            (Angle::Radian(a), Angle::Degree(b)) => *a == b.to_radians(),
            (Angle::Degree(a), Angle::Radian(b)) => *a == b.to_degrees(),
        }
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Angle::Radian(a), Angle::Radian(b)) => a.partial_cmp(&b),
            (Angle::Degree(a), Angle::Degree(b)) => a.partial_cmp(&b),
            (Angle::Radian(a), Angle::Degree(b)) => a.partial_cmp(&b.to_radians()),
            (Angle::Degree(a), Angle::Radian(b)) => a.partial_cmp(&b.to_degrees()),
        }
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

super::impl_op_forward_ref!(*|lhs: Angle, rhs: f32| -> Angle {
    match lhs {
        Angle::Radian(x) => Angle::Radian(x * rhs),
        Angle::Degree(x) => Angle::Degree(x * rhs),
    }
});

super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i8| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u8| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i16| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u16| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i32| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u32| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i64| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u64| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i128| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u128| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: isize| -> Angle { lhs * rhs as f32 });
super::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: usize| -> Angle { lhs * rhs as f32 });

super::impl_op_forward_ref!(/|lhs: Angle, rhs: f32| -> Angle {
    match lhs {
        Angle::Radian(x) => Angle::Radian(x / rhs),
        Angle::Degree(x) => Angle::Degree(x / rhs),
    }
});

super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i8| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u8| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i16| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u16| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i32| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u32| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i64| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u64| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i128| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u128| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: isize| -> Angle { lhs / rhs as f32 });
super::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: usize| -> Angle { lhs / rhs as f32 });

auto_ops::impl_op!(+=|lhs: &mut Angle, rhs: Angle| {
    match lhs {
        Angle::Radian(x) => *x += rhs.as_radians(),
        Angle::Degree(x) => *x += rhs.as_degrees(),
    };
});
