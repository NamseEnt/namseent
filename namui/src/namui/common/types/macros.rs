macro_rules! common_for_f32_type {
    ($your_type: tt) => {
        $crate::types::macros::common_for_f32_type!($your_type, |lhs: $your_type| -> f32 {
            lhs.0
        }, |rhs: f32| -> $your_type {
            $your_type(rhs)
        });
    };
    ($your_type: tt, $to_f32: expr, $from: expr) => {

        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Default)]
        pub struct $your_type(f32);

        $crate::types::macros::impl_single_trait!(from|lhs: $your_type| -> f32 { $to_f32(lhs) });
        $crate::types::macros::impl_single_trait!(from|lhs: &$your_type| -> f32 { f32::from(*lhs) });
        $crate::types::macros::impl_single_trait!(from|lhs: f32| -> $your_type { $from(lhs) });
        $crate::types::macros::impl_single_trait!(from|lhs: i32| -> $your_type { From::from(lhs as f32) });
        $crate::types::macros::impl_single_trait!(from|lhs: usize| -> $your_type { From::from(lhs as f32) });

        impl $your_type {
            pub fn max(&self, other: $your_type) -> $your_type {
                if self.0 > other.0 {
                    *self
                } else {
                    other
                }
            }
            pub fn min(&self, other: $your_type) -> $your_type {
                if self.0 < other.0 {
                    *self
                } else {
                    other
                }
            }
            pub fn clamp(&self, min: $your_type, max: $your_type) -> $your_type {
                if self.0 < min.0 {
                    min
                } else if self.0 > max.0 {
                    max
                } else {
                    *self
                }
            }
            pub fn as_f32(&self) -> f32 {
                f32::from(*self)
            }
            pub fn abs(&self) -> $your_type {
                if self.0 < 0.0 {
                    -*self
                } else {
                    *self
                }
            }
        }

        $crate::types::macros::impl_op_forward_ref!(+|x: $your_type, y: $your_type| -> $your_type {
            (x.as_f32() + y.as_f32()).into()
        });
        $crate::types::macros::impl_op_forward_ref!(-|x: $your_type, y: $your_type| -> $your_type {
            (x.as_f32() - y.as_f32()).into()
        });
        $crate::types::macros::impl_op_forward_ref!(/|x: $your_type, y: $your_type| -> f32 {
            x.0 / y.0
        });
        auto_ops::impl_op!(-|x: $your_type| -> $your_type {
            (-(x.as_f32())).into()
        });

        auto_ops::impl_op!(+=|x: &mut $your_type, y: $your_type| {
            x.0 = (*x + y).0;
        });

        auto_ops::impl_op!(-=|x: &mut $your_type, y: $your_type| {
            x.0 = (*x - y).0;
        });

        impl<T: $crate::types::Ratio> std::ops::Mul<T> for $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.0 * rhs.as_f32()).into()
            }
        }

        impl std::iter::Sum for $your_type {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::default(), |acc, x| acc + x)
            }
        }

        $crate::types::macros::impl_op_forward_ref_reversed_for_f32_i32_usize!(*|lhs: $your_type, rhs: f32| -> $your_type {
            (lhs.as_f32() * rhs).into()
        });

        $crate::types::macros::impl_op_forward_ref_for_f32_i32_usize!(/|lhs: $your_type, rhs: f32| -> $your_type {
            (lhs.as_f32() / rhs).into()
        });

        $crate::types::macros::impl_op_forward_ref_for_f32_i32_usize!(%|lhs: $your_type, rhs: f32| -> $your_type {
            (lhs.as_f32() % rhs).into()
        });


        auto_ops::impl_op!(*=|lhs: &mut $your_type, rhs: f32| {
            lhs.0 = (*lhs * rhs).0;
        });
        auto_ops::impl_op!(*=|lhs: &mut $your_type, rhs: i32| { *lhs *= (rhs as f32) });
        auto_ops::impl_op!(*=|lhs: &mut $your_type, rhs: usize| { *lhs *= (rhs as f32) });

        auto_ops::impl_op!(/=|lhs: &mut $your_type, rhs: f32| {
            lhs.0 = (*lhs / rhs).0;
        });
        auto_ops::impl_op!(/=|lhs: &mut $your_type, rhs: i32| { *lhs /= (rhs as f32) });
        auto_ops::impl_op!(/=|lhs: &mut $your_type, rhs: usize| { *lhs /= (rhs as f32) });
    };
}
pub(crate) use common_for_f32_type;

macro_rules! impl_op_forward_ref {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : $rhs:ty| -> $out:ty $body:block) => {
        auto_ops::impl_op!($op|$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
        auto_ops::impl_op!($op|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out { *$lhs_i $op $rhs_i });
        auto_ops::impl_op!($op|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out { $lhs_i $op *$rhs_i });
        auto_ops::impl_op!($op|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out { *$lhs_i $op *$rhs_i });
    };
}
pub(crate) use impl_op_forward_ref;

macro_rules! impl_op_forward_ref_reversed {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : $rhs:ty| -> $out:ty $body:block) => {
        $crate::types::macros::impl_op_forward_ref!($op|$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
        $crate::types::macros::impl_op_forward_ref!($op|$rhs_i : $rhs, $lhs_i : $lhs| -> $out { $lhs_i $op $rhs_i });
    };
}
pub(crate) use impl_op_forward_ref_reversed;

macro_rules! impl_single_trait {
    (from | $lhs_i:ident : $lhs:ty | -> $for_type: ty $body:block) => {
        impl From<$lhs> for $for_type {
            fn from($lhs_i: $lhs) -> Self {
                $body
            }
        }
    };
}
pub(crate) use impl_single_trait;

macro_rules! impl_op_forward_ref_reversed_for_f32_i32_usize {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : f32| -> $out:ty $body:block) => {
        $crate::types::macros::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: f32| -> $lhs $body );
        $crate::types::macros::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: i32| -> $lhs { $lhs_i $op $rhs_i as f32 });
        $crate::types::macros::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: usize| -> $lhs { $lhs_i $op $rhs_i as f32 });
    }
}
pub(crate) use impl_op_forward_ref_reversed_for_f32_i32_usize;

macro_rules! impl_op_forward_ref_for_f32_i32_usize {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : f32| -> $out:ty $body:block) => {
        $crate::types::macros::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: f32| -> $lhs $body );
        $crate::types::macros::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: i32| -> $lhs { $lhs_i $op $rhs_i as f32 });
        $crate::types::macros::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: usize| -> $lhs { $lhs_i $op $rhs_i as f32 });
    }
}
pub(crate) use impl_op_forward_ref_for_f32_i32_usize;
