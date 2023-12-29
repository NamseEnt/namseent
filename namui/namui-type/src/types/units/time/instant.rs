use crate::Duration;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Instant {
    #[cfg(not(target_family = "wasm"))]
    inner: std::time::Instant,
    #[cfg(target_family = "wasm")]
    inner: todo,
}

impl Instant {
    #[cfg(feature = "namui_internal")]
    pub fn new(inner: std::time::Instant) -> Self {
        Self { inner }
    }
}

auto_ops::impl_op!(-|lhs: Instant, rhs: Instant| -> Duration { sub_instant(lhs, rhs) });
auto_ops::impl_op!(-|lhs: &Instant, rhs: Instant| -> Duration { sub_instant(*lhs, rhs) });
auto_ops::impl_op!(-|lhs: Instant, rhs: &Instant| -> Duration { sub_instant(lhs, *rhs) });
auto_ops::impl_op!(-|lhs: &Instant, rhs: &Instant| -> Duration { sub_instant(*lhs, *rhs) });

#[cfg(not(target_family = "wasm"))]
fn sub_instant(lhs: Instant, rhs: Instant) -> Duration {
    if lhs.inner > rhs.inner {
        Duration::from_std(lhs.inner - rhs.inner).unwrap()
    } else {
        -Duration::from_std(rhs.inner - lhs.inner).unwrap()
    }
}

#[cfg(target_family = "wasm")]
fn sub_instant(lhs: Instant, rhs: Instant) -> Duration {
    todo!()
}
