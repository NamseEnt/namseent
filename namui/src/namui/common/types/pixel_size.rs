use num::{Float, FromPrimitive, ToPrimitive};
use std::fmt::Display;

super::common_for_f32_type!(PixelSize);

impl Display for PixelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0)
    }
}

impl FromPrimitive for PixelSize {
    fn from_i64(n: i64) -> Option<Self> {
        Some(PixelSize(n as f32))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(PixelSize(n as f32))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(PixelSize(n as f32))
    }
}

impl ToPrimitive for PixelSize {
    fn to_i64(&self) -> Option<i64> {
        Some(self.0 as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.0 as u64)
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.0 as f64)
    }
}

impl<T: Float> std::ops::Div<T> for PixelSize {
    type Output = PixelSize;
    fn div(self, rhs: T) -> Self::Output {
        PixelSize::from_f32(self.0.div(rhs.to_f32().unwrap())).unwrap()
    }
}

impl<T: Float> std::ops::Div<T> for &PixelSize {
    type Output = PixelSize;
    fn div(self, rhs: T) -> Self::Output {
        (*self).div(rhs)
    }
}

impl<T: Float> std::ops::Mul<T> for PixelSize {
    type Output = PixelSize;
    fn mul(self, rhs: T) -> Self::Output {
        PixelSize::from_f32(self.0.mul(rhs.to_f32().unwrap())).unwrap()
    }
}

impl<T: Float> std::ops::Mul<T> for &PixelSize {
    type Output = PixelSize;
    fn mul(self, rhs: T) -> Self::Output {
        (*self).mul(rhs)
    }
}
