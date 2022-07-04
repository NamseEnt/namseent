use super::Px;
use num::{cast::AsPrimitive, ToPrimitive};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, serde::Serialize)]
pub struct IntPx(pub(crate) i32);

pub fn int_px(value: impl AsPrimitive<i32>) -> IntPx {
    IntPx(value.as_())
}

impl From<Px> for IntPx {
    fn from(px: Px) -> Self {
        IntPx(px.to_i32().unwrap())
    }
}
