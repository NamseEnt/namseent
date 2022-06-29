use crate::namui;
use auto_ops::impl_op;
use num::Float;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Time {
    milliseconds: f32,
}
impl Time {
    pub fn zero() -> Self {
        Self { milliseconds: 0.0 }
    }
    pub fn from_ms(milliseconds: f32) -> Self {
        Self { milliseconds }
    }
    pub fn from_sec(seconds: f32) -> Time {
        Time {
            milliseconds: seconds * 1000.0,
        }
    }
    pub fn get_total_milliseconds(&self) -> f32 {
        self.milliseconds
    }
    pub fn now() -> Self {
        namui::now().into()
    }
}
impl Into<std::time::Duration> for Time {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_millis(self.milliseconds as u64)
    }
}
impl Into<Time> for std::time::Duration {
    fn into(self) -> Time {
        Time::from_ms(self.as_millis() as f32)
    }
}
impl Eq for Time {}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderedFloat(self.milliseconds).cmp(&OrderedFloat(other.milliseconds))
    }
}
impl<'a> PartialEq<&'a Time> for Time {
    fn eq(&self, other: &&'a Time) -> bool {
        self.milliseconds == other.milliseconds
    }
}
impl<'a> PartialEq<Time> for &'a Time {
    fn eq(&self, other: &Time) -> bool {
        self.milliseconds == other.milliseconds
    }
}
impl<'a> PartialOrd<&'a Time> for Time {
    fn partial_cmp(&self, other: &&'a Time) -> Option<std::cmp::Ordering> {
        self.milliseconds.partial_cmp(&other.milliseconds)
    }
}

impl<'a> PartialOrd<Time> for &'a Time {
    fn partial_cmp(&self, other: &Time) -> Option<std::cmp::Ordering> {
        self.milliseconds.partial_cmp(&other.milliseconds)
    }
}

impl std::hash::Hash for Time {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        bincode::serialize(&self.milliseconds).unwrap().hash(state);
    }
}

macro_rules! overload_time_binary_operator_with_self {
    ($ops: tt) => {
        impl_op!($ops|lhs: Time, rhs: Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
        impl_op!($ops|lhs: Time, rhs: &Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
        impl_op!($ops|lhs: &Time, rhs: Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
        impl_op!($ops|lhs: &Time, rhs: &Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
    };
}

macro_rules! overload_time_binary_operator_with_self_with_type {
    ($ops: tt, $type: tt) => {
        impl_op!($ops|lhs: Time, rhs: Time| -> $type { lhs.milliseconds $ops rhs.milliseconds });
        impl_op!($ops|lhs: Time, rhs: &Time| -> $type { lhs.milliseconds $ops rhs.milliseconds });
        impl_op!($ops|lhs: &Time, rhs: Time| -> $type { lhs.milliseconds $ops rhs.milliseconds });
        impl_op!($ops|lhs: &Time, rhs: &Time| -> $type { lhs.milliseconds $ops rhs.milliseconds });
    };
}

macro_rules! overload_time_assignment_operator_with_self {
    ($ops: tt) => {
        impl_op!($ops|lhs: &mut Time, rhs: Time| { lhs.milliseconds $ops rhs.milliseconds });
        impl_op!($ops|lhs: &mut Time, rhs: &Time| { lhs.milliseconds $ops rhs.milliseconds });
    };
}

// Unary
impl_op!(-|lhs: Time| -> Time {
    Time {
        milliseconds: -lhs.milliseconds,
    }
});
impl_op!(-|lhs: &Time| -> Time {
    Time {
        milliseconds: -lhs.milliseconds,
    }
});
// END: Unary

// Time and Time
overload_time_binary_operator_with_self!(+);
overload_time_binary_operator_with_self!(-);
overload_time_binary_operator_with_self!(*);
overload_time_binary_operator_with_self_with_type!(/, f32);
overload_time_binary_operator_with_self!(%);

overload_time_assignment_operator_with_self!(+=);
overload_time_assignment_operator_with_self!(-=);
overload_time_assignment_operator_with_self!(*=);
overload_time_assignment_operator_with_self!(/=);
overload_time_assignment_operator_with_self!(%=);
// END: Time and Time

impl<T: Float> std::ops::Mul<T> for Time {
    type Output = Time;
    fn mul(self, rhs: T) -> Self::Output {
        Time {
            milliseconds: self.milliseconds.mul(rhs.to_f32().unwrap()),
        }
    }
}

impl<T: Float> std::ops::Mul<T> for &Time {
    type Output = Time;
    fn mul(self, rhs: T) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl<T: Float> std::ops::Div<T> for Time {
    type Output = Time;
    fn div(self, rhs: T) -> Self::Output {
        Time {
            milliseconds: self.milliseconds.div(rhs.to_f32().unwrap()),
        }
    }
}

impl<T: Float> std::ops::Div<T> for &Time {
    type Output = Time;
    fn div(self, rhs: T) -> Self::Output {
        (*self).div(rhs)
    }
}
