#[macro_export]
/// Please add serde into your Cargo.toml
/// ```toml
/// serde = { version = "1.0", features = ["derive"] }
/// ```
macro_rules! vector_types {
    ($type_name: ident, {
        $($field_ident:ident),* $(,)?
    }) => {
        use $crate::*;

        #[type_derives(Copy, Eq, Hash)]
        pub struct $type_name<T>
        where
            T: std::fmt::Debug + State,
        {
            $( pub $field_ident: T ),*
        }
        impl<T> $type_name<T>
        where
            T: std::fmt::Debug + State,
        {
            #[inline(always)]
            pub const fn new($($field_ident: T),*) -> Self {
                Self { $($field_ident),* }
            }

            pub fn map<U, F>(self, f: F) -> $type_name<U>
            where
                F: Fn(T) -> U,
                U: std::fmt::Debug + State,
            {
                $type_name {
                    $( $field_ident: f(self.$field_ident) ),*
                }
            }
        }
        impl<T> $type_name<T>
        where
            T: Clone + std::fmt::Debug + State,
        {
            pub fn single(value: T) -> $type_name<T> {
                $type_name {
                    $( $field_ident: value.clone() ),*
                }
            }
            pub fn into_type<U>(&self) -> $type_name<U>
            where
                T: Into<U>,
                U: std::fmt::Debug + State,
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
            Lhs: std::ops::Div<Rhs, Output = Lhs> + std::fmt::Debug + State,
            Rhs: $crate::Ratio + Clone + std::fmt::Debug + State,
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
            Lhs: std::ops::Div<Rhs, Output = Lhs> + Clone + std::fmt::Debug + State,
            Rhs: $crate::Ratio + Clone + std::fmt::Debug + State,
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
            Lhs: std::ops::DivAssign<Rhs> + std::fmt::Debug + State,
            Rhs: $crate::Ratio + Clone + std::fmt::Debug + State,
        {
            fn div_assign(&mut self, rhs: Rhs) {
                $( self.$field_ident.div_assign(rhs.clone()); )*
            }
        }

        impl<Lhs, Rhs> std::ops::Mul<Rhs> for $type_name<Lhs>
        where
            Lhs: std::ops::Mul<Rhs, Output = Lhs> + std::fmt::Debug + State,
            Rhs: $crate::Ratio + Clone + std::fmt::Debug + State,
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
            Lhs: std::ops::Mul<Rhs, Output = Lhs> + Clone + std::fmt::Debug + State,
            Rhs: $crate::Ratio + Clone + std::fmt::Debug + State,
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
            Lhs: std::ops::MulAssign<Rhs> + std::fmt::Debug + State,
            Rhs: $crate::Ratio + Clone + std::fmt::Debug + State,
        {
            fn mul_assign(&mut self, rhs: Rhs) {
                $( self.$field_ident.mul_assign(rhs.clone()); )*
            }
        }

        impl<T> std::ops::AddAssign<$type_name<T>> for $type_name<T>
        where
            T: std::ops::AddAssign + Clone + std::fmt::Debug + State,
        {
            fn add_assign(&mut self, rhs: $type_name<T>) {
                $( self.$field_ident.add_assign(rhs.$field_ident.clone()); )*
            }
        }

        impl<'a, T> std::ops::AddAssign<&'a $type_name<T>> for $type_name<T>
        where
            T: std::ops::AddAssign + Clone + std::fmt::Debug + State,
        {
            fn add_assign(&mut self, rhs: &$type_name<T>) {
                $( self.$field_ident.add_assign(rhs.$field_ident.clone()); )*
            }
        }

        impl<T> std::ops::SubAssign<$type_name<T>> for $type_name<T>
        where
            T: std::ops::SubAssign + Clone + std::fmt::Debug + State,
        {
            fn sub_assign(&mut self, rhs: $type_name<T>) {
                $( self.$field_ident.sub_assign(rhs.$field_ident.clone()); )*
            }
        }

        impl<'a, T> std::ops::SubAssign<&'a $type_name<T>> for $type_name<T>
        where
            T: std::ops::SubAssign + Clone + std::fmt::Debug + State,
        {
            fn sub_assign(&mut self, rhs: &$type_name<T>) {
                $( self.$field_ident.sub_assign(rhs.$field_ident.clone()); )*
            }
        }

        impl<T> std::ops::Neg for $type_name<T>
        where
            T: std::ops::Neg<Output = T> + std::fmt::Debug + State,
        {
            type Output = $type_name<T>;
            fn neg(self) -> Self::Output {
                $type_name {
                    $( $field_ident: self.$field_ident.neg()),*
                }
            }
        }


        impl<T> $type_name<T>
        where
            T: From<f32> + std::fmt::Debug + State,
        {
            #[inline(always)]
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
            T: From<f32> + Into<f32> + Copy + std::fmt::Debug + State,
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
            T: From<f32> + Into<f32> + Copy + std::fmt::Debug + State,
        {
            pub fn length_squared(&self) -> T {
                let mut sum = 0.0;
                $(
                    sum += self.$field_ident.into().powi(2);
                )*
                T::from(sum)
            }
        }
        impl<T> $type_name<T>
        where
            T: From<f32> + Into<f32> + Copy + std::fmt::Debug + std::ops::Sub<Output = T> + State,
        {
            pub fn distance(&self, rhs: $type_name<T>) -> T {
                (self - rhs).length()
            }
        }
        impl<T> $type_name<T>
        where
            T: std::ops::Div<f32, Output = T>
                + $crate::Ratio
                + Clone
                + std::fmt::Debug
                + From<f32> + Into<f32> + Copy + std::fmt::Debug + State,
        {
            pub fn normalize(&self) -> $type_name<T> {
                let length: f32 = self.length().into();
                if length == 0.0 {
                    return $type_name::zero();
                }
                $type_name {
                    $( $field_ident: self.$field_ident.clone() / length ),*
                }
            }
        }

        impl<T> $type_name<T>
        where
            T: std::ops::Mul<Output = T> + std::ops::AddAssign + Clone + Default + std::fmt::Debug + State,
        {
            pub fn dot(&self, rhs: &$type_name<T>) -> T {
                let mut sum = T::default();
                $(
                    sum += self.$field_ident.clone().mul(rhs.$field_ident.clone());
                )*
                sum
            }
        }

        impl<T> AsRef<$type_name<T>> for $type_name<T>
        where
            T: std::fmt::Debug + State,
        {
            fn as_ref(&self) -> &$type_name<T> {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! overload_tuple_types_binary_operator {
    ($ops_trait: tt, $fn_name: ident, $type_name: ident, { $($field_ident:ident),* $(,)? }) => {
        impl<Lhs, Rhs, TOutput> std::ops::$ops_trait<$type_name<Rhs>> for $type_name<Lhs>
        where
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput> + std::fmt::Debug + State,
            Rhs: std::fmt::Debug + State,
            TOutput: std::fmt::Debug + State,
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
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput> + Clone + std::fmt::Debug + State,
            Rhs: std::fmt::Debug + State,
            TOutput: std::fmt::Debug + State,
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
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput> + std::fmt::Debug + State,
            Rhs: Clone + std::fmt::Debug + State,
            TOutput: std::fmt::Debug + State,
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
            Lhs: std::ops::$ops_trait<Rhs, Output = TOutput> + Clone + std::fmt::Debug + State,
            Rhs: Clone + std::fmt::Debug + State,
            TOutput: std::fmt::Debug + State,
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

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}
