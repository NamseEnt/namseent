use crate::*;
use rkyv::{Deserialize, Infallible};
use std::fmt::Debug;

#[type_derives(-Debug, Copy, PartialOrd, Eq, Ord)]
pub struct SystemTime {
    #[with(rkyv::with::UnixTimestamp)]
    inner: std::time::SystemTime,
}
impl Clone for ArchivedSystemTime {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for ArchivedSystemTime {}
impl From<ArchivedSystemTime> for SystemTime {
    fn from(value: ArchivedSystemTime) -> Self {
        value.deserialize(&mut Infallible).unwrap()
    }
}

impl SystemTime {
    pub fn now() -> Self {
        Self {
            inner: std::time::SystemTime::now(),
        }
    }
}

impl Debug for SystemTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

auto_ops::impl_op!(-|lhs: SystemTime, rhs: SystemTime| -> Duration { sub_system_time(lhs, rhs) });
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: SystemTime| -> Duration { sub_system_time(*lhs, rhs) });
auto_ops::impl_op!(-|lhs: SystemTime, rhs: &SystemTime| -> Duration { sub_system_time(lhs, *rhs) });
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: &SystemTime| -> Duration {
    sub_system_time(*lhs, *rhs)
});

auto_ops::impl_op!(-|lhs: SystemTime, rhs: ArchivedSystemTime| -> Duration {
    sub_system_time(lhs, rhs.into())
});
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: ArchivedSystemTime| -> Duration {
    sub_system_time(*lhs, rhs.into())
});
auto_ops::impl_op!(-|lhs: SystemTime, rhs: &ArchivedSystemTime| -> Duration {
    sub_system_time(lhs, (*rhs).into())
});
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: &ArchivedSystemTime| -> Duration {
    sub_system_time(*lhs, (*rhs).into())
});

auto_ops::impl_op!(+|lhs: SystemTime, rhs: Duration| -> SystemTime { add_duration(lhs, rhs) });
auto_ops::impl_op!(+|lhs: &SystemTime, rhs: Duration| -> SystemTime { add_duration(*lhs, rhs) });
auto_ops::impl_op!(+|lhs: SystemTime, rhs: &Duration| -> SystemTime { add_duration(lhs, *rhs) });
auto_ops::impl_op!(+|lhs: &SystemTime, rhs: &Duration| -> SystemTime { add_duration(*lhs, *rhs) });

auto_ops::impl_op!(-|lhs: SystemTime, rhs: Duration| -> SystemTime { add_duration(lhs, -rhs) });
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: Duration| -> SystemTime { add_duration(*lhs, -rhs) });
auto_ops::impl_op!(-|lhs: SystemTime, rhs: &Duration| -> SystemTime { add_duration(lhs, -*rhs) });
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: &Duration| -> SystemTime { add_duration(*lhs, -*rhs) });

auto_ops::impl_op!(+|lhs: SystemTime, rhs: std::time::Duration| -> SystemTime { add_std_duration(lhs, rhs) });
auto_ops::impl_op!(+|lhs: &SystemTime, rhs: std::time::Duration| -> SystemTime { add_std_duration(*lhs, rhs) });
auto_ops::impl_op!(+|lhs: SystemTime, rhs: &std::time::Duration| -> SystemTime { add_std_duration(lhs, *rhs) });
auto_ops::impl_op!(+|lhs: &SystemTime, rhs: &std::time::Duration| -> SystemTime { add_std_duration(*lhs, *rhs) });

fn sub_system_time(lhs: SystemTime, rhs: SystemTime) -> Duration {
    let duration = match lhs.inner.duration_since(rhs.inner) {
        Ok(duration) => duration,
        Err(err) => err.duration(),
    };

    let sign = lhs.inner > rhs.inner;
    Duration::from_std(sign, duration)
}

fn add_duration(lhs: SystemTime, rhs: Duration) -> SystemTime {
    match rhs.sign {
        true => SystemTime {
            inner: lhs.inner + rhs.inner,
        },
        false => SystemTime {
            inner: lhs.inner - rhs.inner,
        },
    }
}

fn add_std_duration(lhs: SystemTime, rhs: std::time::Duration) -> SystemTime {
    SystemTime {
        inner: lhs.inner + rhs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_time_sub() {
        let std_system_time_now = std::time::SystemTime::now();
        let std_system_time_1s_ago = std_system_time_now - std::time::Duration::from_secs(1);
        let std_system_time_2s_ago = std_system_time_now - std::time::Duration::from_secs(2);
        let std_system_time_3s_ago = std_system_time_now - std::time::Duration::from_secs(3);

        let system_time_now = SystemTime {
            inner: std_system_time_now,
        };
        let system_time_1s_ago = SystemTime {
            inner: std_system_time_1s_ago,
        };
        let system_time_2s_ago = SystemTime {
            inner: std_system_time_2s_ago,
        };
        let system_time_3s_ago = SystemTime {
            inner: std_system_time_3s_ago,
        };

        assert_eq!(system_time_now - system_time_1s_ago, Duration::from_secs(1));
        assert_eq!(system_time_now - system_time_2s_ago, Duration::from_secs(2));
        assert_eq!(system_time_now - system_time_3s_ago, Duration::from_secs(3));

        assert_eq!(
            system_time_1s_ago - system_time_now,
            Duration::from_secs(-1)
        );
        assert_eq!(
            system_time_2s_ago - system_time_now,
            Duration::from_secs(-2)
        );
        assert_eq!(
            system_time_3s_ago - system_time_now,
            Duration::from_secs(-3)
        );
    }

    #[test]
    fn test_system_time_add_duration() {
        let std_system_time_now = std::time::SystemTime::now();
        let std_system_time_1s_ago = std_system_time_now + std::time::Duration::from_secs(1);
        let std_system_time_2s_ago = std_system_time_now + std::time::Duration::from_secs(2);
        let std_system_time_3s_ago = std_system_time_now + std::time::Duration::from_secs(3);

        let system_time_now = SystemTime {
            inner: std_system_time_now,
        };
        let system_time_1s_ago = SystemTime {
            inner: std_system_time_1s_ago,
        };
        let system_time_2s_ago = SystemTime {
            inner: std_system_time_2s_ago,
        };
        let system_time_3s_ago = SystemTime {
            inner: std_system_time_3s_ago,
        };

        assert_eq!(system_time_now + 1.sec(), system_time_1s_ago);
        assert_eq!(system_time_now + 2.sec(), system_time_2s_ago);
        assert_eq!(system_time_now + 3.sec(), system_time_3s_ago);

        assert_eq!(system_time_1s_ago + 1.sec(), system_time_2s_ago);
        assert_eq!(system_time_1s_ago + 2.sec(), system_time_3s_ago);
        assert_eq!(system_time_2s_ago + 1.sec(), system_time_3s_ago);
    }
}
