macro_rules! vector_types {
    ($type_name: ident, {
        $($field_ident:ident),* $(,)?
    }) => {
        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
        pub struct $type_name<T> {
            $( pub $field_ident: T ),*
        }
        impl<T> $type_name<T> {
            pub fn new($($field_ident: T),*) -> Self {
                Self { $($field_ident),* }
            }
        }
        impl<T: Clone> $type_name<T> {
            pub fn single(value: T) -> $type_name<T> {
                $type_name {
                    $( $field_ident: value.clone() ),*
                }
            }
            pub fn into_type<U>(&self) -> $type_name<U>
            where
                T: Into<U>,
            {
                $type_name {
                    $( $field_ident: self.$field_ident.clone().into() ),*
                }
            }
            pub fn as_slice(&self) -> [T; $crate::count!($($field_ident)*)] {
                [$(self.$field_ident.clone()),*]
            }
            pub fn into_slice<U>(&self) -> [U; $crate::count!($($field_ident)*)]
            where
                T: Into<U>,
            {
                [$( self.$field_ident.clone().into() ),*]
            }
        }
        $crate::overload_tuple_types_binary_operator!(Add, add, $type_name, {
            $( $field_ident ),*
        });
        $crate::overload_tuple_types_binary_operator!(Sub, sub, $type_name, {
            $( $field_ident ),*
        });
        $crate::overload_tuple_types_binary_operator!(Div, div, $type_name, {
            $( $field_ident ),*
        });
        $crate::overload_tuple_types_binary_operator!(Mul, mul, $type_name, {
            $( $field_ident ),*
        });
        impl<T> $type_name<T>
        where
            T: From<f32>,
        {
            pub fn zero() -> $type_name<T> {
                $type_name {
                    $( $field_ident: 0.0.into() ),*
                }
            }
            pub fn one() -> $type_name<T> {
                $type_name {
                    $( $field_ident: 1.0.into() ),*
                }
            }
        }
        impl<T> $type_name<T>
        where
            T: From<f32> + Into<f32> + Copy,
        {
            pub fn length(&self) -> T {
                let length_in_f32 = {
                    let mut sum = 0.0;
                    $(
                        sum += self.$field_ident.into().powi(2);
                    )*
                    sum.sqrt()
                };
                T::from(length_in_f32)
            }
        }
        impl<T: std::ops::Mul<f32, Output = T>> std::ops::Mul<$type_name<T>> for f32 {
            type Output = $type_name<T>;
            fn mul(self, rhs: $type_name<T>) -> Self::Output {
                $type_name {
                    $( $field_ident: rhs.$field_ident.mul(self) ),*
                }
            }
        }
        impl<T: std::ops::Div<f32, Output = T>> std::ops::Div<$type_name<T>> for f32 {
            type Output = $type_name<T>;
            fn div(self, rhs: $type_name<T>) -> Self::Output {
                $type_name {
                    $( $field_ident: rhs.$field_ident.div(self) ),*
                }
            }
        }
        impl<T> $type_name<T>
        where
            T: std::ops::Mul<Output = T> + std::ops::AddAssign + Clone + Default,
        {
            pub fn dot(&self, rhs: &$type_name<T>) -> T {
                let mut sum = T::default();
                $(
                    sum += self.$field_ident.clone().mul(rhs.$field_ident.clone());
                )*
                sum
            }
        }
    };
}

pub(crate) use vector_types;

macro_rules! overload_tuple_types_binary_operator {
    ($ops_trait: tt, $fn_name: ident, $type_name: ident, { $($field_ident:ident),* $(,)? }) => {
        impl<T> std::ops::$ops_trait for $type_name<T>
        where
            T: std::ops::$ops_trait<Output = T>,
        {
            type Output = $type_name<T>;
            fn $fn_name(self, other: $type_name<T>) -> $type_name<T> {
                $type_name {
                    $( $field_ident: self.$field_ident.$fn_name(other.$field_ident) ),*
                }
            }
        }
        impl<'a, T> std::ops::$ops_trait<$type_name<T>> for &'a $type_name<T>
        where
            T: std::ops::$ops_trait<Output = T> + Copy,
        {
            type Output = $type_name<T>;
            fn $fn_name(self, other: $type_name<T>) -> $type_name<T> {
                $type_name {
                    $( $field_ident: self.$field_ident.$fn_name(other.$field_ident) ),*
                }
            }
        }
        impl<'b, T> std::ops::$ops_trait<&'b $type_name<T>> for $type_name<T>
        where
            T: std::ops::$ops_trait<Output = T> + Copy,
        {
            type Output = $type_name<T>;
            fn $fn_name(self, other: &'b $type_name<T>) -> $type_name<T> {
                $type_name {
                    $( $field_ident: self.$field_ident.$fn_name(other.$field_ident) ),*
                }
            }
        }
        impl<'a, 'b, T> std::ops::$ops_trait<&'b $type_name<T>> for &'a $type_name<T>
        where
            T: std::ops::$ops_trait<Output = T> + Copy,
        {
            type Output = $type_name<T>;
            fn $fn_name(self, other: &'b $type_name<T>) -> $type_name<T> {
                $type_name {
                    $( $field_ident: self.$field_ident.$fn_name(other.$field_ident) ),*
                }
            }
        }
    };
}

pub(crate) use overload_tuple_types_binary_operator;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}
pub(crate) use count;
