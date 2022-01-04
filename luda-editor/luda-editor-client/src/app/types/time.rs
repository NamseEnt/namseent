use super::{Deserialize, PixelSize, Serialize, TimePerPixel};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Time {
    pub(super) milliseconds: f32,
}
impl Time {
    pub fn zero() -> Self {
        Self { milliseconds: 0.0 }
    }
    pub fn from_ms(milliseconds: f32) -> Self {
        Self { milliseconds }
    }
    pub fn from_sec(seconds: f32) -> Time {
        Time {
            milliseconds: seconds * 1000.0,
        }
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
        let milliseconds = self.milliseconds / rhs.time.milliseconds;
        PixelSize(milliseconds * rhs.pixel_size.0)
    }
}
impl std::ops::Div<f32> for Time {
    type Output = Time;
    fn div(self, rhs: f32) -> Self::Output {
        let milliseconds = self.milliseconds / rhs;
        Time { milliseconds }
    }
}
impl std::ops::Mul<&Time> for usize {
    type Output = Time;

    fn mul(self, rhs: &Time) -> Self::Output {
        Time {
            milliseconds: self as f32 * rhs.milliseconds,
        }
    }
}

impl std::ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Time) {
        self.milliseconds.add_assign(rhs.milliseconds);
    }
}
impl std::ops::SubAssign for Time {
    fn sub_assign(&mut self, rhs: Time) {
        self.milliseconds.sub_assign(rhs.milliseconds);
    }
}
impl std::ops::MulAssign for Time {
    fn mul_assign(&mut self, rhs: Time) {
        self.milliseconds.mul_assign(rhs.milliseconds);
    }
}
impl std::ops::DivAssign for Time {
    fn div_assign(&mut self, rhs: Time) {
        self.milliseconds.div_assign(rhs.milliseconds);
    }
}
