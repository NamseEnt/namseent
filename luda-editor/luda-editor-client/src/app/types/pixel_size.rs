use auto_ops::impl_op;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PixelSize(pub f32);

impl From<f32> for PixelSize {
    fn from(f: f32) -> Self {
        PixelSize(f)
    }
}

impl From<PixelSize> for f32 {
    fn from(val: PixelSize) -> Self {
        val.0
    }
}
impl From<&PixelSize> for f32 {
    fn from(val: &PixelSize) -> Self {
        val.0
    }
}

macro_rules! overload_pixel_size_binary_operator_with_numeric {
    ($ops: tt, $numeric_type: tt) => {
        impl_op!($ops|lhs: PixelSize, rhs: $numeric_type| -> PixelSize { PixelSize(lhs.0 $ops rhs as f32) });
        impl_op!($ops|lhs: PixelSize, rhs: &$numeric_type| -> PixelSize { PixelSize(lhs.0 $ops *rhs as f32) });
        impl_op!($ops|lhs: &PixelSize, rhs: $numeric_type| -> PixelSize { PixelSize(lhs.0 $ops rhs as f32) });
        impl_op!($ops|lhs: &PixelSize, rhs: &$numeric_type| -> PixelSize { PixelSize(lhs.0 $ops *rhs as f32) });

        impl_op!($ops|lhs: $numeric_type, rhs: PixelSize| -> PixelSize { rhs $ops lhs as f32 });
        impl_op!($ops|lhs: $numeric_type, rhs: &PixelSize| -> PixelSize { rhs $ops lhs as f32 });
        impl_op!($ops|lhs: &$numeric_type, rhs: PixelSize| -> PixelSize { rhs $ops *lhs as f32 });
        impl_op!($ops|lhs: &$numeric_type, rhs: &PixelSize| -> PixelSize { rhs $ops *lhs as f32 });
    };
}

macro_rules! overload_pixel_size_arithmetic_operator_with_numeric {
    ($numeric_type: tt) => {
        overload_pixel_size_binary_operator_with_numeric!(+, $numeric_type);
        overload_pixel_size_binary_operator_with_numeric!(-, $numeric_type);
        overload_pixel_size_binary_operator_with_numeric!(*, $numeric_type);
        overload_pixel_size_binary_operator_with_numeric!(/, $numeric_type);
        overload_pixel_size_binary_operator_with_numeric!(%, $numeric_type);
    };
}

macro_rules! overload_pixel_size_binary_operator_with_self {
    ($ops: tt) => {
        impl_op!($ops|lhs: PixelSize, rhs: PixelSize| -> PixelSize { PixelSize(lhs.0 $ops rhs.0) });
        impl_op!($ops|lhs: PixelSize, rhs: &PixelSize| -> PixelSize { PixelSize(lhs.0 $ops rhs.0) });
        impl_op!($ops|lhs: &PixelSize, rhs: PixelSize| -> PixelSize { PixelSize(lhs.0 $ops rhs.0) });
        impl_op!($ops|lhs: &PixelSize, rhs: &PixelSize| -> PixelSize { PixelSize(lhs.0 $ops rhs.0) });
    };
}

macro_rules! overload_pixel_size_assignment_operator_with_self {
    ($ops: tt) => {
        impl_op!($ops|lhs: &mut PixelSize, rhs: PixelSize| { lhs.0 $ops rhs.0 });
        impl_op!($ops|lhs: &mut PixelSize, rhs: &PixelSize| { lhs.0 $ops rhs.0 });
    };
}

// PixelSize and PixelSize
overload_pixel_size_binary_operator_with_self!(+);
overload_pixel_size_binary_operator_with_self!(-);
overload_pixel_size_binary_operator_with_self!(*);
overload_pixel_size_binary_operator_with_self!(/);
overload_pixel_size_binary_operator_with_self!(%);

overload_pixel_size_assignment_operator_with_self!(+=);
overload_pixel_size_assignment_operator_with_self!(-=);
overload_pixel_size_assignment_operator_with_self!(*=);
overload_pixel_size_assignment_operator_with_self!(/=);
overload_pixel_size_assignment_operator_with_self!(%=);
// END: PixelSize and PixelSize

// numerics arithmetic binary operators overloading
overload_pixel_size_arithmetic_operator_with_numeric!(u8);
overload_pixel_size_arithmetic_operator_with_numeric!(u16);
overload_pixel_size_arithmetic_operator_with_numeric!(u32);
overload_pixel_size_arithmetic_operator_with_numeric!(u64);
overload_pixel_size_arithmetic_operator_with_numeric!(u128);
overload_pixel_size_arithmetic_operator_with_numeric!(usize);
overload_pixel_size_arithmetic_operator_with_numeric!(i8);
overload_pixel_size_arithmetic_operator_with_numeric!(i16);
overload_pixel_size_arithmetic_operator_with_numeric!(i32);
overload_pixel_size_arithmetic_operator_with_numeric!(i64);
overload_pixel_size_arithmetic_operator_with_numeric!(i128);
overload_pixel_size_arithmetic_operator_with_numeric!(isize);
overload_pixel_size_arithmetic_operator_with_numeric!(f32);
overload_pixel_size_arithmetic_operator_with_numeric!(f64);
// END: numerics arithmetic binary operators overloading
