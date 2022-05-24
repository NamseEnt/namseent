macro_rules! overload_binary_operator_rhs {
    ($self_type: tt, $floating_type: tt, $ops_trait: tt, $ops_method: tt, $ops_sign: tt, [$($lhs_type: tt),*]) => {
        $(overload_binary_operator_rhs!($self_type, $floating_type, $ops_trait, $ops_method, $ops_sign, $lhs_type);)*
    };
    ($self_type: tt, $floating_type: tt, $ops_trait: tt, $ops_method: tt, $ops_sign: tt, $lhs_type: tt) => {
        impl std::ops::$ops_trait<$self_type> for $lhs_type {
            type Output = $self_type;

            fn $ops_method(self, rhs: $self_type) -> $self_type {
                $self_type(self as $floating_type $ops_sign rhs.0)
            }
        }
        impl std::ops::$ops_trait<&$self_type> for $lhs_type {
            type Output = $self_type;

            fn $ops_method(self, rhs: &$self_type) -> $self_type {
                $self_type(self as $floating_type $ops_sign rhs.0)
            }
        }
    };
}

macro_rules! overload_binary_operator {
    ($self_type: tt, $floating_type: tt, *) => { overload_binary_operator!($self_type, $floating_type, Mul, mul, *); };
    ($self_type: tt, $floating_type: tt, /) => { overload_binary_operator!($self_type, $floating_type, Div, div, /); };
    ($self_type: tt, $floating_type: tt, $ops_trait: tt, $ops_method: tt, $ops_sign: tt) => {
        impl<T> std::ops::$ops_trait<T> for $self_type
        where
            T: num::NumCast,
        {
            type Output = $self_type;

            fn $ops_method(self, rhs: T) -> $self_type {
                let rhs: $floating_type = num::NumCast::from::<T>(rhs).unwrap();
                $self_type (self.0 $ops_sign rhs)
            }
        }
        impl<T> std::ops::$ops_trait<T> for &$self_type
        where
            T: num::NumCast,
        {
            type Output = $self_type;

            fn $ops_method(self, rhs: T) -> $self_type {
                let rhs: $floating_type = num::NumCast::from::<T>(rhs).unwrap();
                $self_type (self.0 $ops_sign rhs)
            }
        }
        overload_binary_operator_rhs!($self_type, $floating_type, $ops_trait, $ops_method, $ops_sign, [
            usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64
        ]);
    };
}

macro_rules! overload_arithmetic_operator {
    ($self_type: tt, $floating_type: tt) => {
        overload_binary_operator!($self_type, $floating_type, *);
        overload_binary_operator!($self_type, $floating_type, /);
    };
}

macro_rules! overload_binary_operator_with_self {
    ($self_type: tt, $ops: tt) => {
        auto_ops::impl_op!($ops |lhs: $self_type, rhs: $self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
        auto_ops::impl_op!($ops |lhs: $self_type, rhs: &$self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
        auto_ops::impl_op!($ops |lhs: &$self_type, rhs: $self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
        auto_ops::impl_op!($ops |lhs: &$self_type, rhs: &$self_type| -> $self_type { $self_type(lhs.0 $ops rhs.0) });
    };
}

macro_rules! overload_assignment_operator_with_self {
    ($self_type: tt, $ops: tt) => {
        auto_ops::impl_op!($ops |lhs: &mut $self_type, rhs: $self_type| { lhs.0 $ops rhs.0; *lhs = $self_type(lhs.0) });
        auto_ops::impl_op!($ops |lhs: &mut $self_type, rhs: &$self_type| { lhs.0 $ops rhs.0; *lhs = $self_type(lhs.0) });
    };
}

macro_rules! overload_binary_operator_with_self_with_type {
    ($self_type: tt, $ops: tt, $type: tt) => {
        auto_ops::impl_op!($ops |lhs: $self_type, rhs: $self_type| -> $type { Into::<$type>::into(lhs.0) $ops Into::<$type>::into(rhs.0) });
        auto_ops::impl_op!($ops |lhs: $self_type, rhs: &$self_type| -> $type { Into::<$type>::into(lhs.0) $ops Into::<$type>::into(rhs.0) });
        auto_ops::impl_op!($ops |lhs: &$self_type, rhs: $self_type| -> $type { Into::<$type>::into(lhs.0) $ops Into::<$type>::into(rhs.0) });
        auto_ops::impl_op!($ops |lhs: &$self_type, rhs: &$self_type| -> $type { Into::<$type>::into(lhs.0) $ops Into::<$type>::into(rhs.0) });
    };
}

macro_rules! impl_froms {
    ($self_type: tt, $floating_type: tt, $into_type_fn: expr, $from_type_fn: expr, [$($from_type: tt),*]) => {
        $(impl_froms!($self_type, $floating_type, $into_type_fn, $from_type_fn, $from_type);)*
    };
    ($self_type: tt, $floating_type: tt, $into_type_fn: expr, $from_type_fn: expr, $from_type: tt) => {
        impl From<$from_type> for $self_type {
            fn from(value: $from_type) -> Self {
                $self_type($into_type_fn(value as $floating_type))
            }
        }
        impl From<&$from_type> for $self_type {
            fn from(value: &$from_type) -> Self {
                $self_type($into_type_fn(*value as $floating_type))
            }
        }
        impl From<$self_type> for $from_type {
            fn from(value: $self_type) -> Self {
                $from_type_fn(value.0) as $from_type
            }
        }
        impl From<&$self_type> for $from_type {
            fn from(value: &$self_type) -> Self {
                $from_type_fn(value.0) as $from_type
            }
        }
    };
}

#[macro_export]
macro_rules! define_singular_floating_tuple {
    ($name: ident, $type: tt) => {
        define_singular_floating_tuple!($name, $type, |value| {
            value
        }, |value| {
            value
        });
    };
    ($name: ident, $type: tt, $into_type_fn: expr) => {
        define_singular_floating_tuple!($name, $type, $into_type_fn, |value| {
            value
        });
    };
    ($name: ident, $type: tt, $into_type_fn: expr, $from_type_fn: expr) => {
        #[derive(Debug, Clone, Copy, PartialOrd, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub(crate) $type);

        impl $name {
            pub fn from(value: $type) -> Self {
                $name($into_type_fn(value))
            }
        }
        impl_froms!($name, $type, $into_type_fn, $from_type_fn, [usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64]);

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $into_type_fn(self.0) == $into_type_fn(other.0)
            }
        }

        impl Eq for $name {}
        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                ordered_float::OrderedFloat(self.0).cmp(&ordered_float::OrderedFloat(other.0))
            }
        }
        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self {
                (-self.0).into()
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
        overload_arithmetic_operator!($name, $type);
        // END: numerics arithmetic binary operators overloading
    };
}
