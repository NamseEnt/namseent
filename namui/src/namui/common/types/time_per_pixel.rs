use super::{PixelSize, Time};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimePerPixel {
    pub(super) time: Time,
    pub(super) pixel_size: PixelSize,
}

impl TimePerPixel {
    pub fn new(time: Time, pixel_size: PixelSize) -> TimePerPixel {
        TimePerPixel { time, pixel_size }
    }
}

auto_ops::impl_op!(*|lhs: TimePerPixel, rhs: PixelSize| -> Time {
    lhs.time * (rhs / lhs.pixel_size)
});

auto_ops::impl_op!(/ |lhs: Time, rhs: PixelSize| -> TimePerPixel { TimePerPixel {
    time: lhs,
    pixel_size: rhs,
} });

auto_ops::impl_op!(/ |lhs: Time, rhs: TimePerPixel| -> PixelSize {
    rhs.pixel_size * (lhs / rhs.time)
});

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn time_div_pixel_should_work() {
        let time = Time::from_ms(1000.0);
        let pixel_size = PixelSize::from_f32(10.0).unwrap();

        let result = time / pixel_size;

        assert_eq!(result, TimePerPixel { time, pixel_size });
    }

    #[test]
    #[wasm_bindgen_test]
    fn time_per_pixel_mul_pixel_should_work() {
        let time = Time::from_ms(1000.0);
        let pixel_size = PixelSize::from_f32(10.0).unwrap();
        let time_per_pixel = TimePerPixel { time, pixel_size };

        let result = time_per_pixel * pixel_size;

        assert_eq!(result, time);
    }

    #[test]
    #[wasm_bindgen_test]
    fn time_div_time_per_pixel_should_work() {
        let time = Time::from_ms(1000.0);
        let pixel_size = PixelSize::from_f32(10.0).unwrap();
        let time_per_pixel = TimePerPixel { time, pixel_size };

        let result = time / time_per_pixel;

        assert_eq!(result, pixel_size);
    }
}
