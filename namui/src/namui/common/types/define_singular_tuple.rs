macro_rules! overload_binary_operator_with_numeric {
    ($self_type: tt, $ops: tt, $numeric_type: tt) => {
        auto_ops::impl_op!($ops|lhs: $self_type, rhs: $numeric_type| -> $self_type { $self_type(lhs.0 $ops rhs as f32) });
        auto_ops::impl_op!($ops|lhs: $self_type, rhs: &$numeric_type| -> $self_type { $self_type(lhs.0 $ops *rhs as f32) });
        auto_ops::impl_op!($ops|lhs: &$self_type, rhs: $numeric_type| -> $self_type { $self_type(lhs.0 $ops rhs as f32) });
        auto_ops::impl_op!($ops|lhs: &$self_type, rhs: &$numeric_type| -> $self_type { $self_type(lhs.0 $ops *rhs as f32) });

        auto_ops::impl_op!($ops|lhs: $numeric_type, rhs: $self_type| -> $self_type { rhs $ops lhs as f32 });
        auto_ops::impl_op!($ops|lhs: $numeric_type, rhs: &$self_type| -> $self_type { rhs $ops lhs as f32 });
        auto_ops::impl_op!($ops|lhs: &$numeric_type, rhs: $self_type| -> $self_type { rhs $ops *lhs as f32 });
        auto_ops::impl_op!($ops|lhs: &$numeric_type, rhs: &$self_type| -> $self_type { rhs $ops *lhs as f32 });
    };
}

macro_rules! overload_arithmetic_operator_with_numeric {
    ($self_type: tt, $numeric_type: tt) => {
        overload_binary_operator_with_numeric!($self_type, +, $numeric_type);
        overload_binary_operator_with_numeric!($self_type, -, $numeric_type);
        overload_binary_operator_with_numeric!($self_type, *, $numeric_type);
        overload_binary_operator_with_numeric!($self_type, /, $numeric_type);
        overload_binary_operator_with_numeric!($self_type, %, $numeric_type);
    };
}

macro_rules! overload_binary_operator_with_self {
    ($self_type: tt, $ops: tt) => {
        auto_ops::impl_op!($ops|lhs: $self_type, rhs: $self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
        auto_ops::impl_op!($ops|lhs: $self_type, rhs: &$self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
        auto_ops::impl_op!($ops|lhs: &$self_type, rhs: $self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
        auto_ops::impl_op!($ops|lhs: &$self_type, rhs: &$self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
    };
}

macro_rules! overload_assignment_operator_with_self {
    ($self_type: tt, $ops: tt) => {
        auto_ops::impl_op!($ops|lhs: &mut $self_type, rhs: $self_type| { lhs.0 $ops rhs.0 });
        auto_ops::impl_op!($ops|lhs: &mut $self_type, rhs: &$self_type| { lhs.0 $ops rhs.0 });
    };
}

macro_rules! overload_binary_operator_with_self_with_type {
    ($self_type: tt, $ops: tt, $type: tt) => {
        auto_ops::impl_op!($ops|lhs: $self_type, rhs: $self_type| -> $type { lhs.0 $ops rhs.0 });
        auto_ops::impl_op!($ops|lhs: $self_type, rhs: &$self_type| -> $type { lhs.0 $ops rhs.0 });
        auto_ops::impl_op!($ops|lhs: &$self_type, rhs: $self_type| -> $type { lhs.0 $ops rhs.0 });
        auto_ops::impl_op!($ops|lhs: &$self_type, rhs: &$self_type| -> $type { lhs.0 $ops rhs.0 });
    };
}

#[macro_export]
macro_rules! define_singular_tuple {
    ($name: ident, $type: tt) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(pub $type);

        impl From<$type> for $name {
            fn from(f: $type) -> Self {
                $name(f)
            }
        }

        impl From<$name> for $type {
            fn from(val: $name) -> Self {
                val.0
            }
        }
        impl From<&$name> for $type {
            fn from(val: &$name) -> Self {
                val.0
            }
        }

        // Self and Self
        overload_binary_operator_with_self!($name, +);
        overload_binary_operator_with_self!($name, -);
        overload_binary_operator_with_self_with_type!($name, /, $type);

        overload_assignment_operator_with_self!($name, +=);
        overload_assignment_operator_with_self!($name, -=);
        // END: Self and Self

        // numerics arithmetic binary operators overloading
        overload_arithmetic_operator_with_numeric!($name, u8);
        overload_arithmetic_operator_with_numeric!($name, u16);
        overload_arithmetic_operator_with_numeric!($name, u32);
        overload_arithmetic_operator_with_numeric!($name, u64);
        overload_arithmetic_operator_with_numeric!($name, u128);
        overload_arithmetic_operator_with_numeric!($name, usize);
        overload_arithmetic_operator_with_numeric!($name, i8);
        overload_arithmetic_operator_with_numeric!($name, i16);
        overload_arithmetic_operator_with_numeric!($name, i32);
        overload_arithmetic_operator_with_numeric!($name, i64);
        overload_arithmetic_operator_with_numeric!($name, i128);
        overload_arithmetic_operator_with_numeric!($name, isize);
        overload_arithmetic_operator_with_numeric!($name, f32);
        overload_arithmetic_operator_with_numeric!($name, f64);
        // END: numerics arithmetic binary operators overloading

    }

}
