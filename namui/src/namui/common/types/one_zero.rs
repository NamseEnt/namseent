use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(OneZero, f32, |value| num::clamp(value, 0.0, 1.0));

impl Display for OneZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let in_f32: f32 = self.into();
        write!(f, "{:.*?}", f.precision().unwrap_or(3), in_f32)
    }
}
