use num::cast::AsPrimitive;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Time {
    Ms(f32),
    Sec(f32),
    Minute(f32),
    Hour(f32),
    Day(f32),
    Week(f32),
}

pub trait TimeExt {
    fn ms(self) -> Time;
    fn sec(self) -> Time;
    fn minute(self) -> Time;
    fn hour(self) -> Time;
    fn day(self) -> Time;
    fn week(self) -> Time;
}

impl TimeExt for f32 {
    fn ms(self) -> Time {
        Time::Ms(self)
    }

    fn sec(self) -> Time {
        Time::Sec(self)
    }

    fn minute(self) -> Time {
        Time::Minute(self)
    }

    fn hour(self) -> Time {
        Time::Hour(self)
    }

    fn day(self) -> Time {
        Time::Day(self)
    }

    fn week(self) -> Time {
        Time::Week(self)
    }
}

impl Time {
    pub fn as_millis(&self) -> f32 {
        match self {
            Time::Ms(ms) => *ms,
            Time::Sec(s) => *s * 1000.0,
            Time::Minute(m) => *m * 60.0 * 1000.0,
            Time::Hour(h) => *h * 60.0 * 60.0 * 1000.0,
            Time::Day(d) => *d * 24.0 * 60.0 * 60.0 * 1000.0,
            Time::Week(w) => *w * 7.0 * 24.0 * 60.0 * 60.0 * 1000.0,
        }
    }

    pub fn as_seconds(&self) -> f32 {
        match self {
            Time::Ms(ms) => *ms / 1000.0,
            Time::Sec(s) => *s,
            Time::Minute(m) => *m * 60.0,
            Time::Hour(h) => *h * 60.0 * 60.0,
            Time::Day(d) => *d * 24.0 * 60.0 * 60.0,
            Time::Week(w) => *w * 7.0 * 24.0 * 60.0 * 60.0,
        }
    }

    pub fn as_minutes(&self) -> f32 {
        match self {
            Time::Ms(ms) => *ms / 1000.0 / 60.0,
            Time::Sec(s) => *s / 60.0,
            Time::Minute(m) => *m,
            Time::Hour(h) => *h * 60.0,
            Time::Day(d) => *d * 24.0 * 60.0,
            Time::Week(w) => *w * 7.0 * 24.0 * 60.0,
        }
    }

    pub fn as_hours(&self) -> f32 {
        match self {
            Time::Ms(ms) => *ms / 1000.0 / 60.0 / 60.0,
            Time::Sec(s) => *s / 60.0 / 60.0,
            Time::Minute(m) => *m / 60.0,
            Time::Hour(h) => *h,
            Time::Day(d) => *d * 24.0,
            Time::Week(w) => *w * 7.0 * 24.0,
        }
    }

    pub fn as_days(&self) -> f32 {
        match self {
            Time::Ms(ms) => *ms / 1000.0 / 60.0 / 60.0 / 24.0,
            Time::Sec(s) => *s / 60.0 / 60.0 / 24.0,
            Time::Minute(m) => *m / 60.0 / 24.0,
            Time::Hour(h) => *h / 24.0,
            Time::Day(d) => *d,
            Time::Week(w) => *w * 7.0,
        }
    }

    pub fn as_weeks(&self) -> f32 {
        match self {
            Time::Ms(ms) => *ms / 1000.0 / 60.0 / 60.0 / 24.0 / 7.0,
            Time::Sec(s) => *s / 60.0 / 60.0 / 24.0 / 7.0,
            Time::Minute(m) => *m / 60.0 / 24.0 / 7.0,
            Time::Hour(h) => *h / 24.0 / 7.0,
            Time::Day(d) => *d / 7.0,
            Time::Week(w) => *w,
        }
    }

    pub fn now() -> Self {
        Time::Ms(crate::now().as_millis() as f32)
    }
}

crate::types::macros::impl_op_forward_ref_reversed_for_f32_i32_usize!(*|lhs: Time,
                                                                        rhs: f32|
 -> Time {
    match lhs {
        Time::Ms(x) => Time::Ms(x * rhs),
        Time::Sec(x) => Time::Sec(x * rhs),
        Time::Minute(x) => Time::Minute(x * rhs),
        Time::Hour(x) => Time::Hour(x * rhs),
        Time::Day(x) => Time::Day(x * rhs),
        Time::Week(x) => Time::Week(x * rhs),
    }
});

impl<T: AsPrimitive<f32>> std::ops::Div<T> for Time {
    type Output = Time;

    fn div(self, rhs: T) -> Self::Output {
        match self {
            Time::Ms(x) => Time::Ms(x / rhs.as_()),
            Time::Sec(x) => Time::Sec(x / rhs.as_()),
            Time::Minute(x) => Time::Minute(x / rhs.as_()),
            Time::Hour(x) => Time::Hour(x / rhs.as_()),
            Time::Day(x) => Time::Day(x / rhs.as_()),
            Time::Week(x) => Time::Week(x / rhs.as_()),
        }
    }
}

impl std::ops::Div for Time {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Time::Ms(x) => x.div(rhs.as_millis()),
            Time::Sec(x) => x.div(rhs.as_seconds()),
            Time::Minute(x) => x.div(rhs.as_minutes()),
            Time::Hour(x) => x.div(rhs.as_hours()),
            Time::Day(x) => x.div(rhs.as_days()),
            Time::Week(x) => x.div(rhs.as_weeks()),
        }
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Time::Ms(x) => Time::Ms(x.add(rhs.as_millis())),
            Time::Sec(x) => Time::Sec(x.add(rhs.as_seconds())),
            Time::Minute(x) => Time::Minute(x.add(rhs.as_minutes())),
            Time::Hour(x) => Time::Hour(x.add(rhs.as_hours())),
            Time::Day(x) => Time::Day(x.add(rhs.as_days())),
            Time::Week(x) => Time::Week(x.add(rhs.as_weeks())),
        }
    }
}

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Time::Ms(x) => Time::Ms(x.sub(rhs.as_millis())),
            Time::Sec(x) => Time::Sec(x.sub(rhs.as_seconds())),
            Time::Minute(x) => Time::Minute(x.sub(rhs.as_minutes())),
            Time::Hour(x) => Time::Hour(x.sub(rhs.as_hours())),
            Time::Day(x) => Time::Day(x.sub(rhs.as_days())),
            Time::Week(x) => Time::Week(x.sub(rhs.as_weeks())),
        }
    }
}

impl std::ops::Rem for Time {
    type Output = Time;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Time::Ms(x) => Time::Ms(x.rem(rhs.as_millis())),
            Time::Sec(x) => Time::Sec(x.rem(rhs.as_seconds())),
            Time::Minute(x) => Time::Minute(x.rem(rhs.as_minutes())),
            Time::Hour(x) => Time::Hour(x.rem(rhs.as_hours())),
            Time::Day(x) => Time::Day(x.rem(rhs.as_days())),
            Time::Week(x) => Time::Week(x.rem(rhs.as_weeks())),
        }
    }
}

impl std::ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Time::Ms(x) => x.add_assign(rhs.as_millis()),
            Time::Sec(x) => x.add_assign(rhs.as_seconds()),
            Time::Minute(x) => x.add_assign(rhs.as_minutes()),
            Time::Hour(x) => x.add_assign(rhs.as_hours()),
            Time::Day(x) => x.add_assign(rhs.as_days()),
            Time::Week(x) => x.add_assign(rhs.as_weeks()),
        }
    }
}

impl std::ops::SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Time::Ms(x) => x.sub_assign(rhs.as_millis()),
            Time::Sec(x) => x.sub_assign(rhs.as_seconds()),
            Time::Minute(x) => x.sub_assign(rhs.as_minutes()),
            Time::Hour(x) => x.sub_assign(rhs.as_hours()),
            Time::Day(x) => x.sub_assign(rhs.as_days()),
            Time::Week(x) => x.sub_assign(rhs.as_weeks()),
        }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Time::Ms(x) => OrderedFloat(*x).eq(&OrderedFloat(other.as_millis())),
            Time::Sec(x) => OrderedFloat(*x).eq(&OrderedFloat(other.as_seconds())),
            Time::Minute(x) => OrderedFloat(*x).eq(&OrderedFloat(other.as_minutes())),
            Time::Hour(x) => OrderedFloat(*x).eq(&OrderedFloat(other.as_hours())),
            Time::Day(x) => OrderedFloat(*x).eq(&OrderedFloat(other.as_days())),
            Time::Week(x) => OrderedFloat(*x).eq(&OrderedFloat(other.as_weeks())),
        }
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Time::Ms(x) => OrderedFloat(*x).partial_cmp(&OrderedFloat(other.as_millis())),
            Time::Sec(x) => OrderedFloat(*x).partial_cmp(&OrderedFloat(other.as_seconds())),
            Time::Minute(x) => OrderedFloat(*x).partial_cmp(&OrderedFloat(other.as_minutes())),
            Time::Hour(x) => OrderedFloat(*x).partial_cmp(&OrderedFloat(other.as_hours())),
            Time::Day(x) => OrderedFloat(*x).partial_cmp(&OrderedFloat(other.as_days())),
            Time::Week(x) => OrderedFloat(*x).partial_cmp(&OrderedFloat(other.as_weeks())),
        }
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Time::Ms(x) => OrderedFloat(*x).cmp(&OrderedFloat(other.as_millis())),
            Time::Sec(x) => OrderedFloat(*x).cmp(&OrderedFloat(other.as_seconds())),
            Time::Minute(x) => OrderedFloat(*x).cmp(&OrderedFloat(other.as_minutes())),
            Time::Hour(x) => OrderedFloat(*x).cmp(&OrderedFloat(other.as_hours())),
            Time::Day(x) => OrderedFloat(*x).cmp(&OrderedFloat(other.as_days())),
            Time::Week(x) => OrderedFloat(*x).cmp(&OrderedFloat(other.as_weeks())),
        }
    }
}
