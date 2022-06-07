use super::{PixelSize, Time};

#[derive(Debug, Clone, Copy)]
pub struct TimePerPixel {
    pub(super) time: Time,
    pub(super) pixel_size: PixelSize,
}
impl TimePerPixel {
    pub(crate) fn new(time: Time, pixel_size: PixelSize) -> TimePerPixel {
        TimePerPixel { time, pixel_size }
    }
    pub(crate) fn ms_per_pixel(&self) -> f32 {
        self.time.milliseconds / self.pixel_size.0
    }
    pub(crate) fn from_ms_per_pixel(ms_per_pixel: f32) -> Self {
        TimePerPixel {
            time: Time {
                milliseconds: ms_per_pixel,
            },
            pixel_size: PixelSize(1.0),
        }
    }
}

impl std::ops::Mul<TimePerPixel> for PixelSize {
    type Output = Time;
    fn mul(self, rhs: TimePerPixel) -> Self::Output {
        Time {
            milliseconds: (self.0 / rhs.pixel_size.0) * rhs.time.milliseconds,
        }
    }
}
impl<'a> std::ops::Mul<TimePerPixel> for &'a PixelSize {
    type Output = Time;
    fn mul(self, rhs: TimePerPixel) -> Self::Output {
        Time {
            milliseconds: (self.0 / rhs.pixel_size.0) * rhs.time.milliseconds,
        }
    }
}
impl<'b> std::ops::Mul<&'b TimePerPixel> for PixelSize {
    type Output = Time;
    fn mul(self, rhs: &'b TimePerPixel) -> Self::Output {
        Time {
            milliseconds: (self.0 / rhs.pixel_size.0) * rhs.time.milliseconds,
        }
    }
}
impl<'a, 'b> std::ops::Mul<&'b TimePerPixel> for &'a PixelSize {
    type Output = Time;
    fn mul(self, rhs: &'b TimePerPixel) -> Self::Output {
        Time {
            milliseconds: (self.0 / rhs.pixel_size.0) * rhs.time.milliseconds,
        }
    }
}
