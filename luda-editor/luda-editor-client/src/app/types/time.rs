use super::PixelSize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time {
    pub milliseconds: i64,
}
impl Time {
    pub fn zero() -> Self {
        Self { milliseconds: 0 }
    }
    pub fn from_ms(milliseconds: i64) -> Self {
        Self { milliseconds }
    }
}
impl std::ops::Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Time) -> Self::Output {
        Time {
            milliseconds: self.milliseconds - rhs.milliseconds,
        }
    }
}
impl std::ops::Add for Time {
    type Output = Time;
    fn add(self, rhs: Time) -> Self::Output {
        Time {
            milliseconds: self.milliseconds + rhs.milliseconds,
        }
    }
}
impl std::ops::Div<TimePerPixel> for Time {
    type Output = PixelSize;
    fn div(self, rhs: TimePerPixel) -> Self::Output {
        let milliseconds = self.milliseconds as f64 / rhs.time.milliseconds as f64;
        PixelSize(milliseconds as f32 * rhs.pixel_size.0)
    }
}
impl std::ops::Div<i64> for Time {
    type Output = Time;
    fn div(self, rhs: i64) -> Self::Output {
        let milliseconds = self.milliseconds / rhs;
        Time { milliseconds }
    }
}
impl Time {
    pub fn sec(seconds: i64) -> Time {
        Time {
            milliseconds: seconds * 1000,
        }
    }
    pub fn ms(milliseconds: i64) -> Time {
        Time { milliseconds }
    }
}
impl TimePerPixel {
    pub(crate) fn new(time: Time, pixel_size: PixelSize) -> TimePerPixel {
        TimePerPixel { time, pixel_size }
    }
}
impl std::ops::Mul<TimePerPixel> for PixelSize {
    type Output = Time;
    fn mul(self, rhs: TimePerPixel) -> Self::Output {
        Time {
            milliseconds: ((self.0 / rhs.pixel_size.0) * (rhs.time.milliseconds as f32)) as i64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimePerPixel {
    time: Time,
    pixel_size: PixelSize,
}
