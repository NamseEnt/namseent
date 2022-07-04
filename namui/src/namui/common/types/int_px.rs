use num::ToPrimitive;

use super::Px;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, serde::Serialize)]
pub struct IntPx(pub i32);

impl From<Px> for IntPx {
    fn from(px: Px) -> Self {
        IntPx(px.to_i32().unwrap())
    }
}
