use super::*;
use std::fmt::Display;

define_singular_floating_tuple!(OneZero, f32, |value| num::clamp(value, 0.0, 1.0));

impl Display for OneZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}
