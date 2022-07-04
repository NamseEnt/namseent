use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PxPerTime {
    time: Time,
}

impl_op_forward_ref_reversed!(*|lhs: PxPerTime, rhs: Time| -> Px { Px::from(lhs.time / rhs) });

impl_op_forward_ref!(/ |lhs: Px, rhs: Time| -> PxPerTime { PxPerTime {
    time: (lhs / Px::from(1.0f32)) * rhs,
} });

impl_op_forward_ref!(/ |lhs: Px, rhs: PxPerTime| -> Time {
    lhs / Px::from(1.0f32) * rhs.time
});
