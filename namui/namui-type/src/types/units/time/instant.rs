use crate::Duration;
use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Instant {
    #[cfg(not(target_family = "wasm"))]
    inner: Duration,
    #[cfg(target_family = "wasm")]
    inner: todo,
}

impl Instant {
    pub fn new(inner: Duration) -> Self {
        Self { inner }
    }

    /// Make sure that your system using std::time for every Instant creation.
    pub fn from_std(std: std::time::Instant) -> Self {
        Self {
            inner: std.duration_since(std::time::Instant::now()).into(),
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

#[cfg(not(target_family = "wasm"))]
fn sub_instant(lhs: Instant, rhs: Instant) -> Duration {
    lhs.inner - rhs.inner
}

#[cfg(not(target_family = "wasm"))]
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
