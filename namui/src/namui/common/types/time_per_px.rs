use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimePerPx {
    time: Time,
}

auto_ops::impl_op!(*|lhs: TimePerPx, rhs: Px| -> Time { lhs.time * (rhs / Px::from(1.0f32)) });
auto_ops::impl_op!(*|lhs: &TimePerPx, rhs: Px| -> Time { lhs.time * (rhs / Px::from(1.0f32)) });
auto_ops::impl_op!(*|lhs: TimePerPx, rhs: &Px| -> Time { lhs.time * (rhs / Px::from(1.0f32)) });
auto_ops::impl_op!(*|lhs: &TimePerPx, rhs: &Px| -> Time { lhs.time * (rhs / Px::from(1.0f32)) });

auto_ops::impl_op!(*|lhs: Px, rhs: TimePerPx| -> Time { rhs * lhs });
auto_ops::impl_op!(*|lhs: &Px, rhs: TimePerPx| -> Time { rhs * lhs });
auto_ops::impl_op!(*|lhs: Px, rhs: &TimePerPx| -> Time { rhs * lhs });
auto_ops::impl_op!(*|lhs: &Px, rhs: &TimePerPx| -> Time { rhs * lhs });

auto_ops::impl_op!(/ |lhs: Time, rhs: Px| -> TimePerPx { TimePerPx {
    time: lhs / (rhs / Px::from(1.0f32)),
} });

auto_ops::impl_op!(/ |lhs: Time, rhs: TimePerPx| -> Px {
    Px::from(lhs / rhs.time)
});

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn time_div_px_should_work() {
        let time = Time::Ms(1000.0);
        let px = Px::from_f32(10.0).unwrap();

        let result = time / px;

        assert_eq!(result, TimePerPx { time: time / 10.0 });
    }

    #[test]
    #[wasm_bindgen_test]
    fn time_per_px_mul_px_should_work() {
        let time = Time::Ms(1000.0);
        let px = Px::from_f32(10.0).unwrap();
        let time_per_px = time / px;

        let result = time_per_px * px;

        assert_eq!(result, time);
    }

    #[test]
    #[wasm_bindgen_test]
    fn time_div_time_per_px_should_work() {
        let time = Time::Ms(1000.0);
        let px = Px::from_f32(10.0).unwrap();
        let time_per_px = time / px;

        let result = time / time_per_px;

        assert_eq!(result, px);
    }
}
