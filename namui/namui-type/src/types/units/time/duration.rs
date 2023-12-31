use anyhow::Result;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    pub(crate) sign: bool,
    pub(crate) inner: std::time::Duration,
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            sign: true,
            inner: Default::default(),
        }
    }
}

impl Debug for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{:?}",
            if self.sign { "" } else { "-" },
            self.inner
        ))
    }
}

impl Duration {
    pub fn from_secs_f64(secs: f64) -> Self {
        Self {
            sign: secs >= 0.0,
            inner: std::time::Duration::from_secs_f64(secs.abs()),
        }
    }
    pub fn from_secs_f32(secs: f32) -> Self {
        Self {
            sign: secs >= 0.0,
            inner: std::time::Duration::from_secs_f32(secs.abs()),
        }
    }
    pub fn from_millis(millis: i64) -> Self {
        Self {
            sign: millis >= 0,
            inner: std::time::Duration::from_millis(millis.unsigned_abs()),
        }
    }
    pub fn from_micros(micros: i64) -> Self {
        Self {
            sign: micros >= 0,
            inner: std::time::Duration::from_micros(micros.unsigned_abs()),
        }
    }
    pub fn from_secs(secs: i64) -> Self {
        Self {
            sign: secs >= 0,
            inner: std::time::Duration::from_secs(secs.unsigned_abs()),
        }
    }
    pub fn from_std(sign: bool, duration: std::time::Duration) -> Self {
        Self {
            sign,
            inner: duration,
        }
    }
    pub fn abs(self) -> Self {
        Self {
            sign: true,
            inner: self.inner,
        }
    }
    /// `Err` if `self` is negative
    pub fn to_std(&self) -> Result<std::time::Duration> {
        if self.sign {
            Ok(self.inner)
        } else {
            Err(anyhow::anyhow!("negative duration"))
        }
    }

    /// CAUTION: You should take care about -0 case.
    pub fn is_positive(&self) -> bool {
        self.sign
    }

    pub fn as_secs(&self) -> i64 {
        self.inner.as_secs() as i64 * if self.sign { 1 } else { -1 }
    }
    pub fn as_millis(&self) -> i128 {
        self.inner.as_millis() as i128 * if self.sign { 1 } else { -1 }
    }
    pub fn as_micros(&self) -> i128 {
        self.inner.as_micros() as i128 * if self.sign { 1 } else { -1 }
    }
    pub fn as_nanos(&self) -> i128 {
        self.inner.as_nanos() as i128 * if self.sign { 1 } else { -1 }
    }
    pub fn as_secs_f64(&self) -> f64 {
        self.inner.as_secs_f64() * if self.sign { 1.0 } else { -1.0 }
    }
    pub fn as_secs_f32(&self) -> f32 {
        self.inner.as_secs_f32() * if self.sign { 1.0 } else { -1.0 }
    }
}

impl std::ops::Neg for Duration {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            sign: !self.sign,
            inner: self.inner,
        }
    }
}
impl std::ops::Neg for &Duration {
    type Output = Duration;
    fn neg(self) -> Self::Output {
        Duration {
            sign: !self.sign,
            inner: self.inner,
        }
    }
}

impl std::ops::Div<Duration> for Duration {
    type Output = f32;
    fn div(self, rhs: Self) -> Self::Output {
        let lhs_secs = self.inner.as_secs_f64();
        let rhs_secs = rhs.inner.as_secs_f64();
        if rhs_secs == 0.0 {
            panic!("divide by zero")
        }
        (lhs_secs / rhs_secs) as f32 * if self.sign == rhs.sign { 1.0 } else { -1.0 }
    }
}

auto_ops::impl_op!(+|lhs: Duration, rhs: Duration| -> Duration { add(lhs, rhs) });
auto_ops::impl_op!(+|lhs: &Duration, rhs: Duration| -> Duration { add(*lhs, rhs) });
auto_ops::impl_op!(+|lhs: Duration, rhs: &Duration| -> Duration { add(lhs, *rhs) });
auto_ops::impl_op!(+|lhs: &Duration, rhs: &Duration| -> Duration { add(*lhs, *rhs) });

auto_ops::impl_op!(-|lhs: Duration, rhs: Duration| -> Duration { add(lhs, -rhs) });
auto_ops::impl_op!(-|lhs: &Duration, rhs: Duration| -> Duration { add(*lhs, -rhs) });
auto_ops::impl_op!(-|lhs: Duration, rhs: &Duration| -> Duration { add(lhs, -*rhs) });
auto_ops::impl_op!(-|lhs: &Duration, rhs: &Duration| -> Duration { add(*lhs, -*rhs) });

auto_ops::impl_op!(*|lhs: Duration, rhs: f32| -> Duration { mul_f32(lhs, rhs) });
auto_ops::impl_op!(*|lhs: &Duration, rhs: f32| -> Duration { mul_f32(*lhs, rhs) });
auto_ops::impl_op!(*|lhs: Duration, rhs: &f32| -> Duration { mul_f32(lhs, *rhs) });
auto_ops::impl_op!(*|lhs: &Duration, rhs: &f32| -> Duration { mul_f32(*lhs, *rhs) });

auto_ops::impl_op!(/|lhs: Duration, rhs: f32| -> Duration { mul_f32(lhs, 1.0 / rhs) });
auto_ops::impl_op!(/|lhs: &Duration, rhs: f32| -> Duration { mul_f32(*lhs,1.0 /  rhs) });
auto_ops::impl_op!(/|lhs: Duration, rhs: &f32| -> Duration { mul_f32(lhs, 1.0 / *rhs) });
auto_ops::impl_op!(/|lhs: &Duration, rhs: &f32| -> Duration { mul_f32(*lhs,1.0 /  *rhs) });

auto_ops::impl_op!(*|lhs: Duration, rhs: i32| -> Duration { mul_i32(lhs, rhs) });
auto_ops::impl_op!(*|lhs: &Duration, rhs: i32| -> Duration { mul_i32(*lhs, rhs) });
auto_ops::impl_op!(*|lhs: Duration, rhs: &i32| -> Duration { mul_i32(lhs, *rhs) });
auto_ops::impl_op!(*|lhs: &Duration, rhs: &i32| -> Duration { mul_i32(*lhs, *rhs) });

auto_ops::impl_op!(/|lhs: Duration, rhs: i32| -> Duration { div_i32(lhs, rhs) });
auto_ops::impl_op!(/|lhs: &Duration, rhs: i32| -> Duration { div_i32(*lhs, rhs) });
auto_ops::impl_op!(/|lhs: Duration, rhs: &i32| -> Duration { div_i32(lhs, *rhs) });
auto_ops::impl_op!(/|lhs: &Duration, rhs: &i32| -> Duration { div_i32(*lhs, *rhs) });

fn add(lhs: Duration, rhs: Duration) -> Duration {
    if lhs.sign == rhs.sign {
        Duration {
            sign: lhs.sign,
            inner: lhs.inner + rhs.inner,
        }
    } else {
        let (abs_bigger, abs_smaller) = if lhs.inner > rhs.inner {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        Duration {
            sign: abs_bigger.sign,
            inner: abs_bigger.inner - abs_smaller.inner,
        }
    }
}

fn mul_f32(lhs: Duration, rhs: f32) -> Duration {
    if rhs == 0.0 {
        Duration {
            sign: true,
            inner: std::time::Duration::from_secs(0),
        }
    } else {
        Duration {
            sign: lhs.sign == (rhs >= 0.0),
            inner: std::time::Duration::from_secs_f64(lhs.inner.as_secs_f64() * rhs.abs() as f64),
        }
    }
}

fn mul_i32(lhs: Duration, rhs: i32) -> Duration {
    if rhs == 0 {
        Duration {
            sign: true,
            inner: std::time::Duration::from_secs(0),
        }
    } else {
        Duration {
            sign: lhs.sign == (rhs >= 0),
            inner: std::time::Duration::from_secs_f64(lhs.inner.as_secs_f64() * rhs.abs() as f64),
        }
    }
}

fn div_i32(lhs: Duration, rhs: i32) -> Duration {
    if rhs == 0 {
        panic!("divide by zero")
    } else {
        Duration {
            sign: lhs.sign == (rhs >= 0),
            inner: std::time::Duration::from_secs_f64(lhs.inner.as_secs_f64() / rhs.abs() as f64),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_duration_add() {
        assert_eq!(1.sec() + 1.sec(), 2.sec());
        assert_eq!(1.sec() + 2.sec(), 3.sec());
    }

    #[test]
    fn test_duration_sub() {
        assert_eq!((1.sec() - 1.sec()).abs(), (0.sec()).abs());
        assert_eq!(1.sec() - 2.sec(), -1.sec());
    }

    #[test]
    fn test_duration_div() {
        assert_eq!(1.sec() / 1.sec(), 1.0);
        assert_eq!(1.sec() / 2.sec(), 0.5);
        assert_eq!(1.sec() / -1.sec(), -1.0);
        assert_eq!(1.sec() / -2.sec(), -0.5);

        assert_eq!(Per::new(1.px(), 1.sec()) * (-1).sec(), -1.px());
    }

    #[test]
    fn test_duration_mul_f32() {
        assert_eq!(1.sec() * 0.0, 0.sec());
        assert_eq!(1.sec() * 1.0, 1.sec());
        assert_eq!(1.sec() * 2.0, 2.sec());
        assert_eq!(1.sec() * -1.0, -1.sec());
        assert_eq!(1.sec() * -2.0, -2.sec());
    }

    #[test]
    fn test_duration_div_f32() {
        assert_eq!(1.sec() / 1.0, 1.sec());
        assert_eq!(1.sec() / 2.0, 0.5.sec());
        assert_eq!(1.sec() / -1.0, -1.sec());
        assert_eq!(1.sec() / -2.0, -0.5.sec());
    }

    #[test]
    fn test_duration_mul_i32() {
        assert_eq!(1.sec() * 0, 0.sec());
        assert_eq!(1.sec() * 1, 1.sec());
        assert_eq!(1.sec() * 2, 2.sec());
        assert_eq!(1.sec() * -1, -1.sec());
        assert_eq!(1.sec() * -2, -2.sec());
    }

    #[test]
    fn test_duration_div_i32() {
        assert_eq!(1.sec() / 1, 1.sec());
        assert_eq!(1.sec() / 2, 0.5.sec());
        assert_eq!(1.sec() / -1, -1.sec());
        assert_eq!(1.sec() / -2, -0.5.sec());
    }

    #[test]
    fn test_duration_neg() {
        assert_eq!(-1.sec(), -1.sec());
        assert_eq!(-(-1.sec()), 1.sec());
    }
}
