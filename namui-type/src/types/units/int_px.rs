use crate::*;

#[type_derives(Copy, Eq, Hash)]
pub struct IntPx(pub(crate) i32);

pub const fn int_px(value: i32) -> IntPx {
    IntPx(value)
}

pub trait IntPxExt {
    fn int_px(self) -> IntPx;
}

impl IntPxExt for i32 {
    fn int_px(self) -> IntPx {
        IntPx(self)
    }
}

impl IntPx {
    pub fn into_px(self) -> Px {
        (self.0 as f32).px()
    }
    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for IntPx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}px", self.0)
    }
}

impl From<Px> for IntPx {
    fn from(px: Px) -> Self {
        IntPx(f32::from(px) as i32)
    }
}

impl Into<Px> for IntPx {
    fn into(self) -> Px {
        Px::from(self.0 as f32)
    }
}

crate::impl_op_forward_ref_reversed!(+|lhs: IntPx, rhs: Px| -> Px {
    crate::px(lhs.0 as f32 + rhs.as_f32())
});
crate::impl_op_forward_ref_reversed!(-|lhs: IntPx, rhs: Px| -> Px {
    crate::px(lhs.0 as f32 - rhs.as_f32())
});
