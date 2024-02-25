use namui::prelude::*;
use num_traits::AsPrimitive;
use std::ops::{Div, Mul, Neg};

macro_rules! create_f32_type {
    ($type_name:ident, $comment:literal) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            PartialOrd,
            num_derive::Float,
            num_derive::FromPrimitive,
            num_derive::Num,
            num_derive::NumCast,
            num_derive::NumOps,
            num_derive::One,
            num_derive::Signed,
            num_derive::ToPrimitive,
            num_derive::Unsigned,
            num_derive::Zero,
        )]
        #[doc = $comment]
        pub struct $type_name(pub f32);
        impl Neg for $type_name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                $type_name(self.0.neg())
            }
        }

        impl namui::Ratio for $type_name {
            fn as_f32(&self) -> f32 {
                self.0
            }
        }

        impl Mul<f32> for $type_name {
            type Output = $type_name;
            fn mul(self, rhs: f32) -> Self::Output {
                $type_name(self.0 * rhs)
            }
        }
        impl Mul<$type_name> for f32 {
            type Output = $type_name;
            fn mul(self, rhs: $type_name) -> Self::Output {
                $type_name(self * rhs.0)
            }
        }

        impl Div<f32> for $type_name {
            type Output = $type_name;
            fn div(self, rhs: f32) -> Self::Output {
                $type_name(self.0 / rhs)
            }
        }

        impl<P> std::convert::From<P> for $type_name
        where
            P: AsPrimitive<f32>,
        {
            fn from(value: P) -> Self {
                $type_name(value.as_())
            }
        }
        impl std::convert::From<$type_name> for f32 {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }
    };
}

create_f32_type!(Meter, "inner value is meter");
pub trait MeterExt {
    fn meter(self) -> Meter;
}
impl<P> MeterExt for P
where
    P: AsPrimitive<f32>,
{
    fn meter(self) -> Meter {
        Meter(self.as_())
    }
}

create_f32_type!(Speed, "inner value is meter per second");
pub trait SpeedExt {
    fn mps(self) -> Speed;
}
impl<P> SpeedExt for P
where
    P: AsPrimitive<f32>,
{
    fn mps(self) -> Speed {
        Speed(self.as_())
    }
}
impl Mul<Duration> for Speed {
    type Output = Meter;
    fn mul(self, duration: Duration) -> Self::Output {
        Meter(self.0 * duration.as_secs_f32())
    }
}
impl Div<Acceleration> for Speed {
    type Output = Duration;
    fn div(self, acc: Acceleration) -> Self::Output {
        Duration::from_secs_f32(self.0 / acc.0)
    }
}

create_f32_type!(Acceleration, "inner value is meter per second squared");
pub trait AccelerationExt {
    fn mpsps(self) -> Acceleration;
}
impl<P> AccelerationExt for P
where
    P: AsPrimitive<f32>,
{
    fn mpsps(self) -> Acceleration {
        Acceleration(self.as_())
    }
}
impl Mul<Duration> for Acceleration {
    type Output = Speed;
    fn mul(self, duration: Duration) -> Self::Output {
        Speed(self.0 * duration.as_secs_f32())
    }
}
