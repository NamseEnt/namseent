#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PixelSize(pub f32);
impl PixelSize {
    pub fn into_f32(&self) -> f32 {
        self.0
    }
}

#[macro_use]
macro_rules! overload_single_tuple_operator {
    ($type: tt, $ops_trait: tt, $fn_name: ident) => {
        use std::ops::*;
        impl $ops_trait for $type {
            type Output = $type;
            fn $fn_name(self, other: $type) -> $type {
                $type(self.0.$fn_name(other.0))
            }
        }
        impl<'a> $ops_trait<$type> for &'a $type {
            type Output = $type;
            fn $fn_name(self, other: $type) -> $type {
                $type(self.0.$fn_name(other.0))
            }
        }
        impl<'b> $ops_trait<&'b $type> for $type {
            type Output = $type;
            fn $fn_name(self, other: &'b $type) -> $type {
                $type(self.0.$fn_name(other.0))
            }
        }
        impl<'a, 'b> $ops_trait<&'b $type> for &'a $type {
            type Output = $type;
            fn $fn_name(self, other: &'b $type) -> $type {
                $type(self.0.$fn_name(other.0))
            }
        }
    };
}
overload_single_tuple_operator!(PixelSize, Add, add);
overload_single_tuple_operator!(PixelSize, Sub, sub);
overload_single_tuple_operator!(PixelSize, Mul, mul);
overload_single_tuple_operator!(PixelSize, Div, div);
