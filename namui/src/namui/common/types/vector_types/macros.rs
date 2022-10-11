#[macro_export]
/// Please add serde into your Cargo.toml
/// ```toml
/// serde = { version = "1.0", features = ["derive"] }
/// ```
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

        impl<Lhs, Rhs> std::ops::Div<Rhs> for $type_name<Lhs>
        where
            Lhs: std::ops::Div<Rhs, Output = Lhs>,
            Rhs: $crate::types::Ratio + Clone,
        {
            type Output = $type_name<Lhs>;
            fn div(self, rhs: Rhs) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.div(rhs.clone())),*
                }
            }
        }

        impl<'a, Lhs, Rhs> std::ops::Div<Rhs> for &'a $type_name<Lhs>
        where
            Lhs: std::ops::Div<Rhs, Output = Lhs> + Clone,
            Rhs: $crate::types::Ratio + Clone,
        {
            type Output = $type_name<Lhs>;
            fn div(self, rhs: Rhs) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.clone().div(rhs.clone())),*
                }
            }
        }

        impl<Lhs, Rhs> std::ops::DivAssign<Rhs> for $type_name<Lhs>
        where
            Lhs: std::ops::DivAssign<Rhs>,
            Rhs: $crate::types::Ratio + Clone,
        {
            fn div_assign(&mut self, rhs: Rhs) {
                $( self.$field_ident.div_assign(rhs.clone()); )*
            }
        }

        impl<Lhs, Rhs> std::ops::Mul<Rhs> for $type_name<Lhs>
        where
            Lhs: std::ops::Mul<Rhs, Output = Lhs>,
            Rhs: $crate::types::Ratio + Clone,
        {
            type Output = $type_name<Lhs>;
            fn mul(self, rhs: Rhs) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.mul(rhs.clone())),*
                }
            }
        }

        impl<'a, Lhs, Rhs> std::ops::Mul<Rhs> for &'a $type_name<Lhs>
        where
            Lhs: std::ops::Mul<Rhs, Output = Lhs> + Clone,
            Rhs: $crate::types::Ratio + Clone,
        {
            type Output = $type_name<Lhs>;
            fn mul(self, rhs: Rhs) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.clone().mul(rhs.clone())),*
                }
            }
        }

        impl<Lhs, Rhs> std::ops::MulAssign<Rhs> for $type_name<Lhs>
        where
            Lhs: std::ops::MulAssign<Rhs>,
            Rhs: $crate::types::Ratio + Clone,
        {
            fn mul_assign(&mut self, rhs: Rhs) {
                $( self.$field_ident.mul_assign(rhs.clone()); )*
            }
        }


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

pub use vector_types;

#[macro_export]
macro_rules! overload_tuple_types_binary_operator {
    ($ops_trait: tt, $fn_name: ident, $type_name: ident, { $($field_ident:ident),* $(,)? }) => {
        impl<Lhs, Rhs, TOutput> std::ops::$ops_trait<$type_name<Rhs>> for $type_name<Lhs>
        where
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput>,
        {
            type Output = $type_name<TOutput>;
            fn $fn_name(self, rhs: $type_name<Rhs>) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.$fn_name(rhs.$field_ident) ),*
                }
            }
        }
        impl<'a, Lhs, Rhs, TOutput> std::ops::$ops_trait<$type_name<Rhs>> for &'a $type_name<Lhs>
        where
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput> + Clone,
        {
            type Output = $type_name<TOutput>;
            fn $fn_name(self, rhs: $type_name<Rhs>) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.clone().$fn_name(rhs.$field_ident) ),*
                }
            }
        }
        impl<'b, Lhs, Rhs, TOutput> std::ops::$ops_trait<&'b $type_name<Rhs>> for $type_name<Lhs>
        where
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput>,
            Rhs: Clone,
        {
            type Output = $type_name<TOutput>;
            fn $fn_name(self, rhs: &'b $type_name<Rhs>) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.$fn_name(rhs.$field_ident.clone()) ),*
                }
            }
        }
        impl<'a, 'b, Lhs, Rhs, TOutput> std::ops::$ops_trait<&'b $type_name<Rhs>> for &'a $type_name<Lhs>
        where
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput> + Clone,
            Rhs: Clone,
        {
            type Output = $type_name<TOutput>;
            fn $fn_name(self, rhs: &'b $type_name<Rhs>) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.clone().$fn_name(rhs.$field_ident.clone()) ),*
                }
            }
        }
    };
}

pub use overload_tuple_types_binary_operator;

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}
pub use count;
