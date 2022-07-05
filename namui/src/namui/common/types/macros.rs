macro_rules! common_for_f32_type {
    ($your_type: tt) => {

        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
        pub struct $your_type(f32);

        auto_ops::impl_op!(+|x: $your_type, y: $your_type| -> $your_type {
            (x.0 + y.0).into()
        });
        auto_ops::impl_op!(-|x: $your_type, y: $your_type| -> $your_type { (x.0 - y.0).into() });
        auto_ops::impl_op!(/|x: $your_type, y: $your_type| -> f32 {
            x.0 / y.0
        });
        auto_ops::impl_op!(/|x: &$your_type, y: $your_type| -> f32 {
            x.0 / y.0
        });
        auto_ops::impl_op!(/|x: $your_type, y: &$your_type| -> f32 {
            x.0 / y.0
        });
        auto_ops::impl_op!(/|x: &$your_type, y: &$your_type| -> f32 {
            x.0 / y.0
        });
        auto_ops::impl_op!(-|x: $your_type| -> $your_type {
            (-x.0).into()
        });

        auto_ops::impl_op!(+=|x: &mut $your_type, y: $your_type| {
            x.0 = (*x + y).0;
        });

        auto_ops::impl_op!(-=|x: &mut $your_type, y: $your_type| {
            x.0 = (*x - y).0;
        });


        impl From<f32> for $your_type {
            fn from(value: f32) -> Self {
                num::FromPrimitive::from_f32(value).unwrap()
            }
        }

        impl Into<f32> for $your_type {
            fn into(self) -> f32 {
                num::ToPrimitive::to_f32(&self).unwrap()
            }
        }

        impl<T: $crate::types::Ratio> std::ops::Mul<T> for $your_type {
            type Output = $your_type;
            fn mul(self, rhs: T) -> Self::Output {
                (self.0 * rhs.as_f32()).into()
            }
        }

        auto_ops::impl_op!(*|lhs: $your_type, rhs: i8| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: u8| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: i16| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: u16| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: i32| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: u32| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: i64| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: u64| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: i128| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: u128| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: isize| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: usize| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: f32| -> $your_type { rhs * lhs });
        auto_ops::impl_op!(*|lhs: $your_type, rhs: f64| -> $your_type { rhs * lhs });


        auto_ops::impl_op!(*|lhs: i8, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: u8, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: i16, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: u16, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: i32, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: u32, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: i64, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: u64, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: i128, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: u128, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: isize, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: usize, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: f32, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
        auto_ops::impl_op!(*|lhs: f64, rhs: $your_type| -> $your_type {
            let rhs: f32 = rhs.into();
            (rhs* lhs as f32).into()
        });
    }
}

pub(crate) use common_for_f32_type;
