#[macro_export]
/// Please add serde into your Cargo.toml
/// ```toml
/// serde = { version = "1.0", features = ["derive"] }
/// ```
macro_rules! common_for_f32_type {
    ($your_type: tt, $short_term: ident, $short_term_ext: ident) => {
        $crate::common_for_f32_type!($your_type, |lhs: $your_type| -> f32 {
            *lhs.0
        }, |rhs: f32| -> $your_type {
            $your_type(OrderedFloat::new(rhs))
        }, $short_term, $short_term_ext, not_ratio);
    };
    ($your_type: tt, $to_f32: expr_2021, $from: expr_2021, $short_term: ident, $short_term_ext: ident, ratio) => {
        $crate::common_for_f32_type!($your_type, $to_f32, $from, $short_term, $short_term_ext);
    };
    ($your_type: tt, $to_f32: expr_2021, $from: expr_2021, $short_term: ident, $short_term_ext: ident, not_ratio) => {
        $crate::common_for_f32_type!($your_type, $to_f32, $from, $short_term, $short_term_ext);
        $crate::impl_op_forward_ref!(/|x: $your_type, y: $your_type| -> f32 {
            *x.0 / *y.0
        });
    };
    ($your_type: tt, $to_f32: expr_2021, $from: expr_2021, $short_term: ident, $short_term_ext: ident) => {
        use $crate::*;

        #[type_derives(Default, PartialOrd, Copy, Eq, Hash, -Debug)]

        pub struct $your_type(OrderedFloat);

        impl std::fmt::Debug for $your_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($your_type))
                    .field(self.0.as_ref())
                    .finish()
            }
        }

        $crate::impl_single_trait!(from|lhs: $your_type| -> f32 {
            #[allow(clippy::redundant_closure_call)]
            $to_f32(lhs)
        });
        $crate::impl_single_trait!(from|lhs: &$your_type| -> f32 { f32::from(*lhs) });
        $crate::impl_single_trait!(from|lhs: f32| -> $your_type {
            #[allow(clippy::redundant_closure_call)]
            $from(lhs)
        });
        $crate::impl_single_trait!(from|lhs: i32| -> $your_type { From::from(lhs as f32) });
        $crate::impl_single_trait!(from|lhs: usize| -> $your_type { From::from(lhs as f32) });

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
                if *self.0 < 0.0 {
                    -*self
                } else {
                    *self
                }
            }
            pub fn is_finite(&self) -> bool {
                self.0.is_finite()
            }
            pub fn floor(&self) -> $your_type {
                self.0.floor().into()
            }
            pub fn ceil(&self) -> $your_type {
                self.0.ceil().into()
            }
            pub fn round(&self) -> $your_type {
                self.0.round().into()
            }
        }

        $crate::impl_op_forward_ref!(+|x: $your_type, y: $your_type| -> $your_type {
            (x.as_f32() + y.as_f32()).into()
        });
        $crate::impl_op_forward_ref!(-|x: $your_type, y: $your_type| -> $your_type {
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

        impl<T: $crate::Ratio> std::ops::Mul<T> for $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.as_f32() * rhs.as_f32()).into()
            }
        }

        impl<'a, T: $crate::Ratio> std::ops::Mul<T> for &'a $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.as_f32() * rhs.as_f32()).into()
            }
        }

        impl<T: $crate::Ratio> std::ops::MulAssign<T> for $your_type {
            fn mul_assign(&mut self, rhs: T) {
                *self = (self.as_f32() * rhs.as_f32()).into();
            }
        }

        impl<T: $crate::Ratio> std::ops::Div<T> for $your_type {
            type Output = $your_type;
            fn div(self, rhs: T) -> Self::Output {
                (self.as_f32() / rhs.as_f32()).into()
            }
        }

        impl<'a, T: $crate::Ratio> std::ops::Div<T> for &'a $your_type {
            type Output = $your_type;
            fn div(self, rhs: T) -> Self::Output {
                (self.as_f32() / rhs.as_f32()).into()
            }
        }

        impl<T: $crate::Ratio> std::ops::DivAssign<T> for $your_type {
            fn div_assign(&mut self, rhs: T) {
                *self = (self.as_f32() / rhs.as_f32()).into();
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

        $crate::impl_op_forward_ref_for_f32_i32_usize!(%|lhs: $your_type, rhs: f32| -> $your_type {
            (lhs.as_f32() % rhs).into()
        });

        pub const fn $short_term(value: f32) -> $your_type {
            $your_type(OrderedFloat::new(value))
        }

        pub trait $short_term_ext {
            fn $short_term(self) -> $your_type;
        }

        impl $short_term_ext for f32 {
            fn $short_term(self) -> $your_type {
                $your_type(OrderedFloat::new(self))
            }
        }

        impl $short_term_ext for i32 {
            fn $short_term(self) -> $your_type {
                $your_type(OrderedFloat::new(self as f32))
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

#[macro_export]
macro_rules! impl_op_forward_ref_reversed {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : $rhs:ty| -> $out:ty $body:block) => {
        $crate::impl_op_forward_ref!($op|$lhs_i : $lhs, $rhs_i : $rhs| -> $out $body);
        $crate::impl_op_forward_ref!($op|$rhs_i : $rhs, $lhs_i : $lhs| -> $out { $lhs_i $op $rhs_i });
    };
}

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

#[macro_export]
macro_rules! impl_op_forward_ref_reversed_for_f32_i32_usize {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : f32| -> $out:ty $body:block) => {
        $crate::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: f32| -> $lhs $body );
        $crate::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: i32| -> $lhs { $lhs_i $op $rhs_i as f32 });
        $crate::impl_op_forward_ref_reversed!($op|$lhs_i: $lhs, $rhs_i: usize| -> $lhs { $lhs_i $op $rhs_i as f32 });
    }
}

#[macro_export]
macro_rules! impl_op_forward_ref_for_f32_i32_usize {
    ($op:tt |$lhs_i:ident : $lhs:ty, $rhs_i:ident : f32| -> $out:ty $body:block) => {
        $crate::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: f32| -> $lhs $body );
        $crate::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: i32| -> $lhs { $lhs_i $op $rhs_i as f32 });
        $crate::impl_op_forward_ref!($op|$lhs_i: $lhs, $rhs_i: usize| -> $lhs { $lhs_i $op $rhs_i as f32 });
    }
}
