use crate::Duration;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SystemTime {
    #[cfg(not(target_family = "wasm"))]
    inner: std::time::SystemTime,
    #[cfg(target_family = "wasm")]
    inner: todo,
}

impl SystemTime {
    #[cfg(feature = "namui_internal")]
    pub fn new(inner: std::time::SystemTime) -> Self {
        Self { inner }
    }
}

auto_ops::impl_op!(-|lhs: SystemTime, rhs: SystemTime| -> Duration { sub_system_time(lhs, rhs) });
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: SystemTime| -> Duration { sub_system_time(*lhs, rhs) });
auto_ops::impl_op!(-|lhs: SystemTime, rhs: &SystemTime| -> Duration { sub_system_time(lhs, *rhs) });
auto_ops::impl_op!(-|lhs: &SystemTime, rhs: &SystemTime| -> Duration {
    sub_system_time(*lhs, *rhs)
});

#[cfg(not(target_family = "wasm"))]
fn sub_system_time(lhs: SystemTime, rhs: SystemTime) -> Duration {
    let duration = match lhs.inner.duration_since(rhs.inner) {
        Ok(duration) => duration,
        Err(err) => err.duration(),
    };

    let duration = Duration::from_std(duration).unwrap();

    if lhs.inner > rhs.inner {
        duration
    } else {
        -duration
    }
}

#[cfg(target_family = "wasm")]
fn sub_system_time(lhs: SystemTime, rhs: SystemTime) -> Duration {
    todo!()
}
