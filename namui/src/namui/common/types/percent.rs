use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Percent(pub(crate) f32);

impl<T: num::ToPrimitive + num::FromPrimitive> std::ops::Mul<T> for Percent {
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        T::from_f32((self.0 / 100.0).mul(rhs.to_f32().unwrap())).unwrap()
    }
}

impl<T: num::ToPrimitive + num::FromPrimitive> std::ops::Div<T> for Percent {
    type Output = T;
    fn div(self, rhs: T) -> Self::Output {
        T::from_f32((self.0 / 100.0).div(rhs.to_f32().unwrap()).into()).unwrap()
    }
}

impl num::FromPrimitive for Percent {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Percent((n * 100) as f32))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Percent((n * 100) as f32))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Percent((n * 100.0) as f32))
    }
}

impl num::ToPrimitive for Percent {
    fn to_i64(&self) -> Option<i64> {
        Some((self.0 / 100.0) as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some((self.0 / 100.0) as u64)
    }

    fn to_f64(&self) -> Option<f64> {
        Some((self.0 / 100.0) as f64)
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}%", f.precision().unwrap_or(1), self.0)
    }
}

impl Percent {
    pub fn new<T: num::cast::AsPrimitive<f32>>(percent: T) -> Percent {
        Percent(percent.as_())
    }
    pub fn from<T>(decimal: T) -> Percent
    where
        T: num::Float,
    {
        Percent(decimal.to_f32().unwrap() * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn percent_multiply_should_work() {
        let a = 4.0_f32;
        let b = Percent::new(150.0);
        let c = 6.0_f32;
        let a_b = b * a;

        assert_eq!(c, a_b);
    }

    #[test]
    #[wasm_bindgen_test]
    fn percent_display_should_work() {
        let b = Percent::new(150.0);

        assert_eq!(format!("{}", b), "150.0%");
    }
}
