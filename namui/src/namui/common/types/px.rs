use std::fmt::Display;

super::common_for_f32_type!(Px);

pub const fn px(value: f32) -> Px {
    Px(value)
}

pub trait PxExt {
    fn px(self) -> Px;
}

impl PxExt for f32 {
    fn px(self) -> Px {
        Px(self)
    }
}

impl PxExt for i32 {
    fn px(self) -> Px {
        Px(self as f32)
    }
}

impl Display for Px {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}px", f.precision().unwrap_or(0), self.0)
    }
}
