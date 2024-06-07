use crate::Duration;
use std::{fmt::Debug, sync::OnceLock};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Instant {
    inner: Duration,
}

impl Instant {
    pub fn new(inner: Duration) -> Self {
        Self { inner }
    }

    pub fn now() -> Self {
        static START: OnceLock<std::time::Instant> = OnceLock::new();
        Self {
            inner: std::time::Instant::now()
                .duration_since(*START.get_or_init(std::time::Instant::now))
                .into(),
        }
    }
}

impl Debug for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

auto_ops::impl_op!(-|lhs: Instant, rhs: Instant| -> Duration { sub_instant(lhs, rhs) });
auto_ops::impl_op!(-|lhs: &Instant, rhs: Instant| -> Duration { sub_instant(*lhs, rhs) });
auto_ops::impl_op!(-|lhs: Instant, rhs: &Instant| -> Duration { sub_instant(lhs, *rhs) });
auto_ops::impl_op!(-|lhs: &Instant, rhs: &Instant| -> Duration { sub_instant(*lhs, *rhs) });

auto_ops::impl_op!(+|lhs: Instant, rhs: Duration| -> Instant { add_duration(lhs, rhs) });
auto_ops::impl_op!(+|lhs: &Instant, rhs: Duration| -> Instant { add_duration(*lhs, rhs) });
auto_ops::impl_op!(+|lhs: Instant, rhs: &Duration| -> Instant { add_duration(lhs, *rhs) });
auto_ops::impl_op!(+|lhs: &Instant, rhs: &Duration| -> Instant { add_duration(*lhs, *rhs) });

auto_ops::impl_op!(-|lhs: Instant, rhs: Duration| -> Instant { add_duration(lhs, -rhs) });
auto_ops::impl_op!(-|lhs: &Instant, rhs: Duration| -> Instant { add_duration(*lhs, -rhs) });
auto_ops::impl_op!(-|lhs: Instant, rhs: &Duration| -> Instant { add_duration(lhs, -*rhs) });
auto_ops::impl_op!(-|lhs: &Instant, rhs: &Duration| -> Instant { add_duration(*lhs, -*rhs) });

auto_ops::impl_op!(-|lhs: &mut Instant, rhs: Instant| -> Duration { sub_instant(*lhs, rhs) });
auto_ops::impl_op!(-|lhs: Instant, rhs: &mut Instant| -> Duration { sub_instant(lhs, *rhs) });
auto_ops::impl_op!(-|lhs: &mut Instant, rhs: &mut Instant| -> Duration { sub_instant(*lhs, *rhs) });

auto_ops::impl_op!(+|lhs: &mut Instant, rhs: Duration| -> Instant { add_duration(*lhs, rhs) });
auto_ops::impl_op!(+|lhs: Instant, rhs: &mut Duration| -> Instant { add_duration(lhs, *rhs) });
auto_ops::impl_op!(+|lhs: &mut Instant, rhs: &mut Duration| -> Instant { add_duration(*lhs, *rhs) });

auto_ops::impl_op!(-|lhs: &mut Instant, rhs: Duration| -> Instant { add_duration(*lhs, -rhs) });
auto_ops::impl_op!(-|lhs: Instant, rhs: &mut Duration| -> Instant { add_duration(lhs, -*rhs) });
auto_ops::impl_op!(-|lhs: &mut Instant, rhs: &mut Duration| -> Instant {
    add_duration(*lhs, -*rhs)
});

fn sub_instant(lhs: Instant, rhs: Instant) -> Duration {
    lhs.inner - rhs.inner
}

fn add_duration(lhs: Instant, rhs: Duration) -> Instant {
    Instant {
        inner: lhs.inner + rhs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_instant_sub() {
        let a = Instant {
            inner: Duration::from_secs(0),
        };
        let b = Instant {
            inner: Duration::from_secs(1),
        };

        assert_eq!(b - a, 1.sec());
        assert_eq!(a - b, (-1).sec());
    }

    #[test]
    fn test_instant_add_duration() {
        let a = Instant {
            inner: Duration::from_secs(0),
        };
        let b = a + 1.sec();

        assert_eq!(b - a, 1.sec());
        assert_eq!(a - b, (-1).sec());
    }
}
