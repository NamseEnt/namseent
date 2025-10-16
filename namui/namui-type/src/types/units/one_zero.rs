use std::fmt::Display;

super::common_for_f32_type!(
    OneZero,
    |lhs: OneZero| -> f32 { *lhs.0 },
    |lhs: f32| -> OneZero { OneZero(OrderedFloat::new(lhs.clamp(0.0, 1.0))) },
    one_zero,
    OneZeroExt
);

impl Display for OneZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}", f.precision().unwrap_or(3), f32::from(self))
    }
}
