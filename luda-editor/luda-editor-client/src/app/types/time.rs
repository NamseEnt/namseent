use super::{Deserialize, PixelSize, Serialize, TimePerPixel};
use auto_ops::impl_op;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Time {
    pub(super) milliseconds: f32,
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

macro_rules! overload_time_binary_operator_with_numeric {
    ($ops: tt, $numeric_type: tt) => {
        impl_op!($ops|lhs: Time, rhs: $numeric_type| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs as f32 } });
        impl_op!($ops|lhs: Time, rhs: &$numeric_type| -> Time { Time { milliseconds: lhs.milliseconds $ops *rhs as f32 } });
        impl_op!($ops|lhs: &Time, rhs: $numeric_type| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs as f32 } });
        impl_op!($ops|lhs: &Time, rhs: &$numeric_type| -> Time { Time { milliseconds: lhs.milliseconds $ops *rhs as f32 } });

        impl_op!($ops|lhs: $numeric_type, rhs: Time| -> Time { rhs $ops lhs as f32 });
        impl_op!($ops|lhs: $numeric_type, rhs: &Time| -> Time { rhs $ops lhs as f32 });
        impl_op!($ops|lhs: &$numeric_type, rhs: Time| -> Time { rhs $ops *lhs as f32 });
        impl_op!($ops|lhs: &$numeric_type, rhs: &Time| -> Time { rhs $ops *lhs as f32 });
    };
}

macro_rules! overload_time_arithmetic_operator_with_numeric {
    ($numeric_type: tt) => {
        overload_time_binary_operator_with_numeric!(+, $numeric_type);
        overload_time_binary_operator_with_numeric!(-, $numeric_type);
        overload_time_binary_operator_with_numeric!(*, $numeric_type);
        overload_time_binary_operator_with_numeric!(/, $numeric_type);
        overload_time_binary_operator_with_numeric!(%, $numeric_type);
    };
}

macro_rules! overload_time_binary_operator_with_self {
    ($ops: tt) => {
        impl_op!($ops|lhs: Time, rhs: Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
        impl_op!($ops|lhs: Time, rhs: &Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
        impl_op!($ops|lhs: &Time, rhs: Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
        impl_op!($ops|lhs: &Time, rhs: &Time| -> Time { Time { milliseconds: lhs.milliseconds $ops rhs.milliseconds } });
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
overload_time_binary_operator_with_self!(/);
overload_time_binary_operator_with_self!(%);

overload_time_assignment_operator_with_self!(+=);
overload_time_assignment_operator_with_self!(-=);
overload_time_assignment_operator_with_self!(*=);
overload_time_assignment_operator_with_self!(/=);
overload_time_assignment_operator_with_self!(%=);
// END: Time and Time

// numerics arithmetic binary operators overloading
overload_time_arithmetic_operator_with_numeric!(u8);
overload_time_arithmetic_operator_with_numeric!(u16);
overload_time_arithmetic_operator_with_numeric!(u32);
overload_time_arithmetic_operator_with_numeric!(u64);
overload_time_arithmetic_operator_with_numeric!(u128);
overload_time_arithmetic_operator_with_numeric!(usize);
overload_time_arithmetic_operator_with_numeric!(i8);
overload_time_arithmetic_operator_with_numeric!(i16);
overload_time_arithmetic_operator_with_numeric!(i32);
overload_time_arithmetic_operator_with_numeric!(i64);
overload_time_arithmetic_operator_with_numeric!(i128);
overload_time_arithmetic_operator_with_numeric!(isize);
overload_time_arithmetic_operator_with_numeric!(f32);
overload_time_arithmetic_operator_with_numeric!(f64);
// END: numerics arithmetic binary operators overloading

// Time and PixelPerTime
impl_op!(/|lhs: Time, rhs: TimePerPixel| -> PixelSize { PixelSize (lhs.milliseconds / rhs.time.milliseconds * rhs.pixel_size.0) });
impl_op!(/|lhs: Time, rhs: &TimePerPixel| -> PixelSize { PixelSize (lhs.milliseconds / rhs.time.milliseconds * rhs.pixel_size.0) });
impl_op!(/|lhs: &Time, rhs: TimePerPixel| -> PixelSize { PixelSize (lhs.milliseconds / rhs.time.milliseconds * rhs.pixel_size.0) });
impl_op!(/|lhs: &Time, rhs: &TimePerPixel| -> PixelSize { PixelSize (lhs.milliseconds / rhs.time.milliseconds * rhs.pixel_size.0) });
// END: Time and PixelPerTime
