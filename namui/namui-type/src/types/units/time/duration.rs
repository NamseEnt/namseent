use crate::*;
use std::fmt::Debug;

#[type_derives(-Debug, Copy, PartialOrd, Eq, Ord)]
pub struct Duration {
    secs: OrderedFloat,
}

impl Duration {
    pub const ZERO: Duration = Duration {
        secs: OrderedFloat::new(0.0),
    };
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            secs: OrderedFloat::new(0.0),
        }
    }
}

impl Debug for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.secs.get();
        if secs < 0.0 {
            f.write_fmt(format_args!(
                "-{:?}",
                std::time::Duration::from_secs_f32(-secs)
            ))
        } else {
            f.write_fmt(format_args!(
                "{:?}",
                std::time::Duration::from_secs_f32(secs)
            ))
        }
    }
}

impl Duration {
    pub fn from_secs_f32(secs: f32) -> Self {
        Self {
            secs: OrderedFloat::new(secs),
        }
    }
    pub fn from_secs_f64(secs: f64) -> Self {
        Self {
            secs: OrderedFloat::new(secs as f32),
        }
    }
    pub const fn from_millis(millis: i64) -> Self {
        Self {
            secs: OrderedFloat::new(millis as f32 / 1000.0),
        }
    }
    pub const fn from_micros(micros: i64) -> Self {
        Self {
            secs: OrderedFloat::new(micros as f32 / 1_000_000.0),
        }
    }
    pub const fn from_secs(secs: i64) -> Self {
        Self {
            secs: OrderedFloat::new(secs as f32),
        }
    }
    pub const fn abs(self) -> Self {
        let s = self.secs.get();
        Self {
            secs: OrderedFloat::new(if s < 0.0 { -s } else { s }),
        }
    }
    pub const fn is_positive(&self) -> bool {
        self.secs.get() >= 0.0
    }
    pub const fn as_secs(&self) -> i64 {
        self.secs.get() as i64
    }
    pub const fn as_millis(&self) -> i64 {
        (self.secs.get() * 1000.0) as i64
    }
    pub const fn as_micros(&self) -> i64 {
        (self.secs.get() * 1_000_000.0) as i64
    }
    pub const fn as_secs_f32(&self) -> f32 {
        self.secs.get()
    }
    pub const fn as_secs_f64(&self) -> f64 {
        self.secs.get() as f64
    }
}

impl std::ops::Neg for Duration {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            secs: OrderedFloat::new(-self.secs.get()),
        }
    }
}
impl std::ops::Neg for &Duration {
    type Output = Duration;
    fn neg(self) -> Self::Output {
        Duration {
            secs: OrderedFloat::new(-self.secs.get()),
        }
    }
}

impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        Self {
            secs: OrderedFloat::new(duration.as_secs_f32()),
        }
    }
}

auto_ops::impl_op!(+|lhs: Duration, rhs: Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });
auto_ops::impl_op!(+|lhs: &Duration, rhs: Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });
auto_ops::impl_op!(+|lhs: Duration, rhs: &Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });
auto_ops::impl_op!(+|lhs: &Duration, rhs: &Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });

auto_ops::impl_op!(-|lhs: Duration, rhs: Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});
auto_ops::impl_op!(-|lhs: &Duration, rhs: Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});
auto_ops::impl_op!(-|lhs: Duration, rhs: &Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});
auto_ops::impl_op!(-|lhs: &Duration, rhs: &Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});

auto_ops::impl_op!(/|lhs: Duration, rhs: Duration| -> f32 { lhs.secs.get() / rhs.secs.get() });
auto_ops::impl_op!(/|lhs: &Duration, rhs: Duration| -> f32 { lhs.secs.get() / rhs.secs.get() });
auto_ops::impl_op!(/|lhs: Duration, rhs: &Duration| -> f32 { lhs.secs.get() / rhs.secs.get() });
auto_ops::impl_op!(/|lhs: &Duration, rhs: &Duration| -> f32 { lhs.secs.get() / rhs.secs.get() });

auto_ops::impl_op!(%|lhs: Duration, rhs: Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() % rhs.secs.get()) } });
auto_ops::impl_op!(%|lhs: &Duration, rhs: Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() % rhs.secs.get()) } });
auto_ops::impl_op!(%|lhs: Duration, rhs: &Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() % rhs.secs.get()) } });
auto_ops::impl_op!(%|lhs: &Duration, rhs: &Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() % rhs.secs.get()) } });

auto_ops::impl_op!(*|lhs: Duration, rhs: f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * rhs),
    }
});
auto_ops::impl_op!(*|lhs: &Duration, rhs: f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * rhs),
    }
});
auto_ops::impl_op!(*|lhs: Duration, rhs: &f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs),
    }
});
auto_ops::impl_op!(*|lhs: &Duration, rhs: &f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs),
    }
});

auto_ops::impl_op!(/|lhs: Duration, rhs: f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs) } });
auto_ops::impl_op!(/|lhs: &Duration, rhs: f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs) } });
auto_ops::impl_op!(/|lhs: Duration, rhs: &f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs) } });
auto_ops::impl_op!(/|lhs: &Duration, rhs: &f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs) } });

auto_ops::impl_op!(*|lhs: Duration, rhs: i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * rhs as f32),
    }
});
auto_ops::impl_op!(*|lhs: &Duration, rhs: i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * rhs as f32),
    }
});
auto_ops::impl_op!(*|lhs: Duration, rhs: &i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs as f32),
    }
});
auto_ops::impl_op!(*|lhs: &Duration, rhs: &i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs as f32),
    }
});

auto_ops::impl_op!(/|lhs: Duration, rhs: i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs as f32) } });
auto_ops::impl_op!(/|lhs: &Duration, rhs: i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs as f32) } });
auto_ops::impl_op!(/|lhs: Duration, rhs: &i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });
auto_ops::impl_op!(/|lhs: &Duration, rhs: &i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });

auto_ops::impl_op!(/|lhs: Duration, rhs: usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs as f32) } });
auto_ops::impl_op!(/|lhs: &Duration, rhs: usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs as f32) } });
auto_ops::impl_op!(/|lhs: Duration, rhs: &usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });
auto_ops::impl_op!(/|lhs: &Duration, rhs: &usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });

auto_ops::impl_op!(+=|lhs: &mut Duration, rhs: Duration| { *lhs.secs += rhs.secs.get() });
auto_ops::impl_op!(+=|lhs: &mut Duration, rhs: &Duration| { *lhs.secs += rhs.secs.get() });

auto_ops::impl_op!(-=|lhs: &mut Duration, rhs: Duration| { *lhs.secs -= rhs.secs.get() });
auto_ops::impl_op!(-=|lhs: &mut Duration, rhs: &Duration| { *lhs.secs -= rhs.secs.get() });

auto_ops::impl_op!(*=|lhs: &mut Duration, rhs: f32| { *lhs.secs *= rhs });
auto_ops::impl_op!(*=|lhs: &mut Duration, rhs: &f32| { *lhs.secs *= *rhs });

auto_ops::impl_op!(/=|lhs: &mut Duration, rhs: f32| { *lhs.secs /= rhs });
auto_ops::impl_op!(/=|lhs: &mut Duration, rhs: &f32| { *lhs.secs /= *rhs });

auto_ops::impl_op!(*=|lhs: &mut Duration, rhs: i32| { *lhs.secs *= rhs as f32 });
auto_ops::impl_op!(*=|lhs: &mut Duration, rhs: &i32| { *lhs.secs *= *rhs as f32 });

auto_ops::impl_op!(/=|lhs: &mut Duration, rhs: i32| { *lhs.secs /= rhs as f32 });
auto_ops::impl_op!(/=|lhs: &mut Duration, rhs: &i32| { *lhs.secs /= *rhs as f32 });

auto_ops::impl_op!(/=|lhs: &mut Duration, rhs: usize| { *lhs.secs /= rhs as f32 });
auto_ops::impl_op!(/=|lhs: &mut Duration, rhs: &usize| { *lhs.secs /= *rhs as f32 });

//

auto_ops::impl_op!(+|lhs: &mut Duration, rhs: Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });
auto_ops::impl_op!(+|lhs: Duration, rhs: &mut Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });
auto_ops::impl_op!(+|lhs: &mut Duration, rhs: &mut Duration| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() + rhs.secs.get()) } });

auto_ops::impl_op!(-|lhs: &mut Duration, rhs: Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});
auto_ops::impl_op!(-|lhs: Duration, rhs: &mut Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});
auto_ops::impl_op!(-|lhs: &mut Duration, rhs: &mut Duration| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() - rhs.secs.get()),
    }
});

auto_ops::impl_op!(*|lhs: &mut Duration, rhs: f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * rhs),
    }
});
auto_ops::impl_op!(*|lhs: Duration, rhs: &mut f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs),
    }
});
auto_ops::impl_op!(*|lhs: &mut Duration, rhs: &mut f32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs),
    }
});

auto_ops::impl_op!(/|lhs: &mut Duration, rhs: f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs) } });
auto_ops::impl_op!(/|lhs: Duration, rhs: &mut f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs) } });
auto_ops::impl_op!(/|lhs: &mut Duration, rhs: &mut f32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs) } });

auto_ops::impl_op!(*|lhs: &mut Duration, rhs: i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * rhs as f32),
    }
});
auto_ops::impl_op!(*|lhs: Duration, rhs: &mut i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs as f32),
    }
});
auto_ops::impl_op!(*|lhs: &mut Duration, rhs: &mut i32| -> Duration {
    Duration {
        secs: OrderedFloat::new(lhs.secs.get() * *rhs as f32),
    }
});

auto_ops::impl_op!(/|lhs: &mut Duration, rhs: i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs as f32) } });
auto_ops::impl_op!(/|lhs: Duration, rhs: &mut i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });
auto_ops::impl_op!(/|lhs: &mut Duration, rhs: &mut i32| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });

auto_ops::impl_op!(/|lhs: &mut Duration, rhs: usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / rhs as f32) } });
auto_ops::impl_op!(/|lhs: Duration, rhs: &mut usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });
auto_ops::impl_op!(/|lhs: &mut Duration, rhs: &mut usize| -> Duration { Duration { secs: OrderedFloat::new(lhs.secs.get() / *rhs as f32) } });

auto_ops::impl_op!(+=|lhs: &mut Duration, rhs: &mut Duration| { *lhs.secs += rhs.secs.get() });

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
        assert_eq!(1.sec() - 2.sec(), (-1).sec());
    }

    #[test]
    fn test_duration_div() {
        assert_eq!(1.sec() / 1.sec(), 1.0);
        assert_eq!(1.sec() / 2.sec(), 0.5);
        assert_eq!(1.sec() / (-1).sec(), -1.0);
        assert_eq!(1.sec() / (-2).sec(), -0.5);

        assert_eq!(Per::new(1.px(), 1.sec()) * (-1).sec(), (-1).px());
    }

    #[test]
    fn test_duration_mul_f32() {
        assert_eq!(1.sec() * 0.0, 0.sec());
        assert_eq!(1.sec() * 1.0, 1.sec());
        assert_eq!(1.sec() * 2.0, 2.sec());
        assert_eq!(1.sec() * -1.0, (-1).sec());
        assert_eq!(1.sec() * -2.0, (-2).sec());
    }

    #[test]
    fn test_duration_div_f32() {
        assert_eq!(1.sec() / 1.0, 1.sec());
        assert_eq!(1.sec() / 2.0, 0.5.sec());
        assert_eq!(1.sec() / -1.0, (-1).sec());
        assert_eq!(1.sec() / -2.0, (-0.5).sec());
    }

    #[test]
    #[allow(clippy::erasing_op)]
    fn test_duration_mul_i32() {
        assert_eq!(1.sec() * 0, 0.sec());
        assert_eq!(1.sec() * 1, 1.sec());
        assert_eq!(1.sec() * 2, 2.sec());
        assert_eq!(1.sec() * -1, (-1).sec());
        assert_eq!(1.sec() * -2, (-2).sec());
    }

    #[test]
    fn test_duration_div_i32() {
        assert_eq!(1.sec() / 1, 1.sec());
        assert_eq!(1.sec() / 2, 0.5.sec());
        assert_eq!(1.sec() / -1, (-1).sec());
        assert_eq!(1.sec() / -2, (-0.5).sec());
    }

    #[test]
    fn test_duration_neg() {
        assert_eq!(-(1.sec()), (-1).sec());
        assert_eq!(-((-1).sec()), 1.sec());
    }

    #[test]
    fn test_duration_rem() {
        // Basic positive cases
        assert_eq!(5.sec() % 3.sec(), 2.sec());
        assert_eq!(7.sec() % 3.sec(), 1.sec());
        assert_eq!(6.sec() % 3.sec(), 0.sec());

        // Cases with decimals
        assert_eq!(5.5.sec() % 2.sec(), 1.5.sec());
        assert_eq!(7.25.sec() % 2.5.sec(), 2.25.sec());

        // Mixed sign cases (following standard remainder behavior)
        // 5 % (-3) = 2 (result has same sign as dividend)
        assert_eq!(5.sec() % (-3).sec(), 2.sec());
        // (-5) % 3 = -2 (result has same sign as dividend)
        assert_eq!((-5).sec() % 3.sec(), (-2).sec());
        // (-5) % (-3) = -2 (result has same sign as dividend)
        assert_eq!((-5).sec() % (-3).sec(), (-2).sec());

        // Edge cases
        assert_eq!(0.sec() % 3.sec(), 0.sec());
        assert_eq!(1.sec() % 2.sec(), 1.sec());

        // Millisecond precision
        assert_eq!(
            Duration::from_millis(1500) % Duration::from_millis(1000),
            Duration::from_millis(500)
        );
        assert_eq!(
            Duration::from_millis(2500) % Duration::from_millis(1000),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn test_duration_div_duration() {
        // Basic positive division
        assert_eq!(6.sec() / 2.sec(), 3.0);
        assert_eq!(5.sec() / 2.sec(), 2.5);
        assert_eq!(1.sec() / 1.sec(), 1.0);

        // Division with decimals
        assert_eq!(2.5.sec() / 0.5.sec(), 5.0);
        assert_eq!(7.5.sec() / 2.5.sec(), 3.0);

        // Mixed sign cases
        assert_eq!(6.sec() / (-2).sec(), -3.0);
        assert_eq!((-6).sec() / 2.sec(), -3.0);
        assert_eq!((-6).sec() / (-2).sec(), 3.0);

        // Small values
        assert_eq!(Duration::from_millis(100) / Duration::from_millis(50), 2.0);
        assert_eq!(1.sec() / Duration::from_millis(1000), 1.0);

        // Fractional results
        assert_eq!(1.sec() / 3.sec(), 1.0 / 3.0);
        assert_eq!(2.sec() / 3.sec(), 2.0 / 3.0);

        // Zero dividend
        assert_eq!(0.sec() / 1.sec(), 0.0);
    }
}
