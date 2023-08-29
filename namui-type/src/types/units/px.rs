use std::fmt::Display;

super::common_for_f32_type!(Px, px, PxExt);

impl Display for Px {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0)
    }
}
