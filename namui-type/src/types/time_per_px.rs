use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimePerPx {
    time: Time,
}

auto_ops::impl_op!(*|lhs: TimePerPx, rhs: Px| -> Time { lhs.time * (rhs / 1.px()) });
auto_ops::impl_op!(*|lhs: &TimePerPx, rhs: Px| -> Time { lhs.time * (rhs / 1.px()) });
auto_ops::impl_op!(*|lhs: TimePerPx, rhs: &Px| -> Time { lhs.time * (rhs / 1.px()) });
auto_ops::impl_op!(*|lhs: &TimePerPx, rhs: &Px| -> Time { lhs.time * (rhs / 1.px()) });

auto_ops::impl_op!(*|lhs: Px, rhs: TimePerPx| -> Time { rhs * lhs });
auto_ops::impl_op!(*|lhs: &Px, rhs: TimePerPx| -> Time { rhs * lhs });
auto_ops::impl_op!(*|lhs: Px, rhs: &TimePerPx| -> Time { rhs * lhs });
auto_ops::impl_op!(*|lhs: &Px, rhs: &TimePerPx| -> Time { rhs * lhs });

auto_ops::impl_op!(/ |lhs: Time, rhs: Px| -> TimePerPx { TimePerPx {
    time: lhs / (rhs / 1.px()),
} });

auto_ops::impl_op!(/ |lhs: Time, rhs: TimePerPx| -> Px {
    px(lhs / rhs.time)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_div_px_should_work() {
        let time = Time::Ms(1000.0);
        let px = px(10.0);

        let result = time / px;

        assert_eq!(result, TimePerPx { time: time / 10.0 });
    }

    #[test]
    fn time_per_px_mul_px_should_work() {
        let time = Time::Ms(1000.0);
        let px = px(10.0);
        let time_per_px = time / px;

        let result = time_per_px * px;

        assert_eq!(result, time);
    }

    #[test]
    fn time_div_time_per_px_should_work() {
        let time = Time::Ms(1000.0);
        let px = px(10.0);
        let time_per_px = time / px;

        let result = time / time_per_px;

        assert_eq!(result, px);
    }
}
