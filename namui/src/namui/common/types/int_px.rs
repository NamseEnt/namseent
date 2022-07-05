use super::Px;
use num::{FromPrimitive, ToPrimitive};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, serde::Serialize)]
pub struct IntPx(pub(crate) i32);

pub const fn int_px(value: i32) -> IntPx {
    IntPx(value)
}

impl From<Px> for IntPx {
    fn from(px: Px) -> Self {
        IntPx(px.to_i32().unwrap())
    }
}

impl Into<Px> for IntPx {
    fn into(self) -> Px {
        Px::from_i32(self.0).unwrap()
    }
}

crate::namui::common::types::macros::impl_op_forward_ref_reversed!(+|lhs: IntPx, rhs: Px| -> Px {
    crate::px(lhs.0 as f32 + rhs.as_f32())
});
crate::namui::common::types::macros::impl_op_forward_ref_reversed!(-|lhs: IntPx, rhs: Px| -> Px {
    crate::px(lhs.0 as f32 - rhs.as_f32())
});
