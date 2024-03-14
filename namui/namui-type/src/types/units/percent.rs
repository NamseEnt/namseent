use super::{ratio::Ratio, *};
use std::fmt::Display;

common_for_f32_type!(
    Percent,
    |lhs: Percent| -> f32 { *lhs.0 / 100.0 },
    |lhs: f32| -> Percent { Percent(ordered_float::OrderedFloat(lhs / 100.0)) },
    percent,
    PercentExt,
    ratio
);

impl Ratio for Percent {
    fn as_f32(&self) -> f32 {
        *self.0 / 100.0
    }
}

impl Percent {
    pub fn from<T>(decimal: T) -> Percent
    where
        T: num::Float,
    {
        Percent(ordered_float::OrderedFloat(
            decimal.to_f32().unwrap() * 100.0,
        ))
    }
    pub fn from_percent<T>(percent: T) -> Percent
    where
        T: num::cast::AsPrimitive<f32>,
    {
        Percent(ordered_float::OrderedFloat(percent.as_()))
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}%", f.precision().unwrap_or(1), self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_multiply_should_work() {
        let a = 4.0_f32;
        let b = Percent::from_percent(150.0_f32);
        let c = Percent::from_percent(600.0_f32);
        let b_a = b * a;

        assert_eq!(c, b_a);
    }

    #[test]
    fn percent_addition_should_work() {
        let a = 50.percent();
        let b = 25.percent();
        let c = a + b;

        assert_eq!(c, 75.percent());
        assert_eq!(format!("{}", c), "75.0%");
    }

    #[test]
    fn percent_display_should_work() {
        let b = Percent::from_percent(150.0);

        assert_eq!(format!("{}", b), "150.0%");
    }
}
