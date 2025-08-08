use std::fmt::Display;

super::common_for_f32_type!(Px, px, PxExt);

impl Display for Px {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0.as_f32())
    }
}

#[macro_export]
macro_rules! assert_px_eq {
    ($left:expr, $right:expr $(,)?) => {
        let left = $left;
        let right = $right;
        let epsilon = f32::EPSILON.px();

        if (left - right).abs() > epsilon {
            assert_eq!(left, right);
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        let left = $left;
        let right = $right;
        let epsilon = f32::EPSILON.px();

        if (left - right).abs() > epsilon {
            assert_eq!(left, right, $($arg)+);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", 1.0.px()), "Px(1.0)");
    }
}
