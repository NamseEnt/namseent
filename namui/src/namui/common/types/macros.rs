#[macro_export]
/// Please add serde into your Cargo.toml
/// ```toml
/// serde = { version = "1.0", features = ["derive"] }
/// ```
macro_rules! common_for_f32_type {
    ($your_type: tt, $short_term: ident, $short_term_ext: ident) => {
        $crate::types::common_for_f32_type!($your_type, |lhs: $your_type| -> f32 {
            lhs.0
        }, |rhs: f32| -> $your_type {
            $your_type(rhs)
        }, $short_term, $short_term_ext, not_ratio);
    };
    ($your_type: tt, $to_f32: expr, $from: expr, $short_term: ident, $short_term_ext: ident, ratio) => {
        $crate::types::common_for_f32_type!($your_type, $to_f32, $from, $short_term, $short_term_ext);
    };
    ($your_type: tt, $to_f32: expr, $from: expr, $short_term: ident, $short_term_ext: ident, not_ratio) => {
        $crate::types::common_for_f32_type!($your_type, $to_f32, $from, $short_term, $short_term_ext);
        $crate::types::impl_op_forward_ref!(/|x: $your_type, y: $your_type| -> f32 {
            x.0 / y.0
        });
    };
    ($your_type: tt, $to_f32: expr, $from: expr, $short_term: ident, $short_term_ext: ident) => {
        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Default)]
        pub struct $your_type(f32);

        $crate::types::impl_single_trait!(from|lhs: $your_type| -> f32 { $to_f32(lhs) });
        $crate::types::impl_single_trait!(from|lhs: &$your_type| -> f32 { f32::from(*lhs) });
        $crate::types::impl_single_trait!(from|lhs: f32| -> $your_type { $from(lhs) });
        $crate::types::impl_single_trait!(from|lhs: i32| -> $your_type { From::from(lhs as f32) });
        $crate::types::impl_single_trait!(from|lhs: usize| -> $your_type { From::from(lhs as f32) });

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
            pub fn is_finite(&self) -> bool {
                self.0.is_finite()
            }
        }

        $crate::types::impl_op_forward_ref!(+|x: $your_type, y: $your_type| -> $your_type {
            (x.as_f32() + y.as_f32()).into()
        });
        $crate::types::impl_op_forward_ref!(-|x: $your_type, y: $your_type| -> $your_type {
            (x.as_f32() - y.as_f32()).into()
        });
        $crate::auto_ops::impl_op!(-|x: $your_type| -> $your_type {
            (-(x.as_f32())).into()
        });

        $crate::auto_ops::impl_op!(+=|x: &mut $your_type, y: $your_type| {
            x.0 = (*x + y).0;
        });

        $crate::auto_ops::impl_op!(-=|x: &mut $your_type, y: $your_type| {
            x.0 = (*x - y).0;
        });

        impl<T: $crate::types::Ratio> std::ops::Mul<T> for $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.as_f32() * rhs.as_f32()).into()
            }
        }

        impl<'a, T: $crate::types::Ratio> std::ops::Mul<T> for &'a $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.as_f32() * rhs.as_f32()).into()
            }
        }

        impl<T: $crate::types::Ratio> std::ops::MulAssign<T> for $your_type {
            fn mul_assign(&mut self, rhs: T) {
                *self = (self.as_f32() * rhs.as_f32()).into();
            }
        }

        impl<T: $crate::types::Ratio> std::ops::Div<T> for $your_type {
            type Output = $your_type;
            fn div(self, rhs: T) -> Self::Output {
                (self.as_f32() / rhs.as_f32()).into()
            }
        }

        impl<'a, T: $crate::types::Ratio> std::ops::Div<T> for &'a $your_type {
            type Output = $your_type;
            fn div(self, rhs: T) -> Self::Output {
                (self.as_f32() / rhs.as_f32()).into()
            }
        }

        impl<T: $crate::types::Ratio> std::ops::DivAssign<T> for $your_type {
            fn div_assign(&mut self, rhs: T) {
                *self = (self.as_f32() / rhs.as_f32()).into();
            }
        }

        impl<'a, T: $crate::types::Ratio> std::ops::Mul<T> for &'a $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.0 * rhs.as_f32()).into()
            }
        }

        impl<T: $crate::types::Ratio> std::ops::MulAssign<T> for $your_type {
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs.as_f32();
            }
        }

        impl std::iter::Sum for $your_type {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::default(), |acc, x| acc + x)
            }
        }

        impl<'a> std::iter::Sum<&'a $your_type> for $your_type {
            fn sum<I: Iterator<Item = &'a $your_type>>(iter: I) -> Self {
                iter.fold(Self::default(), |acc, x| acc + x)
            }
        }

        $crate::types::impl_op_forward_ref_for_f32_i32_usize!(%|lhs: $your_type, rhs: f32| -> $your_type {
            (lhs.as_f32() % rhs).into()
        });

        pub const fn $short_term(value: f32) -> $your_type {
            $your_type(value)
        }

        pub trait $short_term_ext {
            fn $short_term(self) -> $your_type;
        }

        impl $short_term_ext for f32 {
            fn $short_term(self) -> $your_type {
                $your_type(self)
            }
        }

        impl $short_term_ext for i32 {
            fn $short_term(self) -> $your_type {
                $your_type(self as f32)
            }
        }

        impl $crate::SimpleSigned for $your_type {
            fn is_sign_positive(&self) -> bool {
                self.0.is_sign_positive()
            }

            fn is_sign_negative(&self) -> bool {
                self.0.is_sign_negative()
            }
        }
    };
}
pub use common_for_f32_type;

#[macro_export]
macro_rules! impl_op_forward_ref {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : $rhs:ty| -> $out:ty $body:block) => {
        $crate::auto_ops::impl_op!($op|$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
        $crate::auto_ops::impl_op!($op|$lhs_i : &$lhs, $rhs_i : $rhs| -> $out { *$lhs_i $op $rhs_i });
        $crate::auto_ops::impl_op!($op|$lhs_i : $lhs, $rhs_i : &$rhs| -> $out { $lhs_i $op *$rhs_i });
        $crate::auto_ops::impl_op!($op|$lhs_i : &$lhs, $rhs_i : &$rhs| -> $out { *$lhs_i $op *$rhs_i });
    };
}
pub use impl_op_forward_ref;

#[macro_export]
macro_rules! impl_op_forward_ref_reversed {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : $rhs:ty| -> $out:ty $body:block) => {
        $crate::types::impl_op_forward_ref!($op|$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
        $crate::types::impl_op_forward_ref!($op|$rhs_i : $rhs, $lhs_i : $lhs| -> $out { $lhs_i $op $rhs_i });
    };
}
pub use impl_op_forward_ref_reversed;

#[macro_export]
macro_rules! impl_single_trait {
    (from | $lhs_i:ident : $lhs:ty | -> $for_type: ty $body:block) => {
        impl From<$lhs> for $for_type {
            fn from($lhs_i: $lhs) -> Self {
                $body
            }
        }
    };
}
pub use impl_single_trait;

#[macro_export]
macro_rules! impl_op_forward_ref_reversed_for_f32_i32_usize {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : f32| -> $out:ty $body:block) => {
        $crate::types::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: f32| -> $lhs $body );
        $crate::types::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: i32| -> $lhs { $lhs_i $op $rhs_i as f32 });
        $crate::types::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: usize| -> $lhs { $lhs_i $op $rhs_i as f32 });
    }
}
pub use impl_op_forward_ref_reversed_for_f32_i32_usize;

#[macro_export]
macro_rules! impl_op_forward_ref_for_f32_i32_usize {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : f32| -> $out:ty $body:block) => {
        $crate::types::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: f32| -> $lhs $body );
        $crate::types::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: i32| -> $lhs { $lhs_i $op $rhs_i as f32 });
        $crate::types::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: usize| -> $lhs { $lhs_i $op $rhs_i as f32 });
    }
}
pub use impl_op_forward_ref_for_f32_i32_usize;
