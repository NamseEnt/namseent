use crate::*;
use num::cast::AsPrimitive;
use std::fmt::Display;

#[type_derives(Copy, Eq, Hash)]
pub struct Angle {
    radians: ordered_float::OrderedFloat<f32>,
}

pub trait AngleExt {
    fn deg(self) -> Angle;
    fn rad(self) -> Angle;
}

impl AngleExt for f32 {
    fn deg(self) -> Angle {
        Angle {
            radians: self.to_radians().into(),
        }
    }
    fn rad(self) -> Angle {
        Angle {
            radians: self.into(),
        }
    }
}

impl AngleExt for i32 {
    fn deg(self) -> Angle {
        (self as f32).deg()
    }
    fn rad(self) -> Angle {
        (self as f32).rad()
    }
}

impl Angle {
    pub fn as_radians(&self) -> f32 {
        *self.radians
    }
    pub fn as_degrees(&self) -> f32 {
        self.radians.to_degrees()
    }

    pub fn sin(&self) -> f32 {
        self.radians.sin()
    }

    pub fn cos(&self) -> f32 {
        self.radians.cos()
    }

    pub fn tan(&self) -> f32 {
        self.radians.tan()
    }

    pub fn atan2(y: impl AsPrimitive<f32>, x: impl AsPrimitive<f32>) -> Self {
        OrderedFloat(y.as_().atan2(x.as_())).rad()
    }
}

impl std::ops::Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle {
            radians: self.radians + other.radians,
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
        Self::Output {
            radians: -self.radians,
        }
    }
}

impl<'a> std::ops::Neg for &'a Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        Self::Output {
            radians: -self.radians,
        }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let degree = self.as_degrees();
        write!(f, "{:.*?}°", f.precision().unwrap_or(0), degree)
    }
}

// i32 and f32 versions are implemented with trait Ratio.
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i8| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u8| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i16| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u16| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u32| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i64| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u64| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: i128| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: u128| -> Angle { lhs * rhs as f32 });
crate::impl_op_forward_ref_reversed!(*|lhs: Angle, rhs: isize| -> Angle { lhs * rhs as f32 });

crate::impl_op_forward_ref!(/|lhs: Angle, rhs: f32| -> Angle {
    Self {
        radians: lhs.radians / rhs,
    }
});

crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i8| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u8| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i16| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u16| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i32| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u32| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i64| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u64| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: i128| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: u128| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: isize| -> Angle { lhs / rhs as f32 });
crate::impl_op_forward_ref_reversed!(/|lhs: Angle, rhs: usize| -> Angle { lhs / rhs as f32 });

auto_ops::impl_op!(+=|lhs: &mut Angle, rhs: Angle| {
    lhs.radians += rhs.radians;
});

impl<Rhs> std::ops::Mul<Rhs> for Angle
where
    Rhs: Ratio + Clone,
{
    type Output = Angle;
    fn mul(self, rhs: Rhs) -> Self::Output {
        Angle {
            radians: self.radians * rhs.as_f32(),
        }
    }
}

impl<'a, Rhs> std::ops::Mul<Rhs> for &'a Angle
where
    Rhs: Ratio + Clone,
{
    type Output = Angle;
    fn mul(self, rhs: Rhs) -> Self::Output {
        Angle {
            radians: self.radians * rhs.as_f32(),
        }
    }
}

impl<Rhs> std::ops::MulAssign<Rhs> for Angle
where
    Rhs: Ratio + Clone,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.radians *= rhs.as_f32();
    }
}
