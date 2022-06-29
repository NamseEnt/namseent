mod degree;
mod one_zero;
mod percent;
mod pixel_size;
mod radian;
mod time;
mod time_per_pixel;

pub use degree::*;
pub use one_zero::*;
pub use percent::*;
pub use pixel_size::*;
pub use radian::*;
pub use time::*;
pub use time_per_pixel::*;

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
        auto_ops::impl_op!(-|x: $your_type| -> $your_type {
            (-x.0).into()
        });

        impl<T: num::Float> From<T> for $your_type {
            fn from(value: T) -> Self {
                num::FromPrimitive::from_f32(value.to_f32().unwrap()).unwrap()
            }
        }

        impl Into<f32> for $your_type {
            fn into(self) -> f32 {
                num::ToPrimitive::to_f32(&self).unwrap()
            }
        }
    }
}

pub(crate) use common_for_f32_type;
