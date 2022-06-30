use serde::{Deserialize, Serialize};
use std::ops::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Xy<T> {
    pub x: T,
    pub y: T,
}

macro_rules! overload_operator {
    ($ops_trait: tt, $fn_name: ident) => {
        impl<T> $ops_trait for Xy<T>
        where
            T: $ops_trait<Output = T>,
        {
            type Output = Xy<T>;
            fn $fn_name(self, other: Xy<T>) -> Xy<T> {
                Xy {
                    x: self.x.$fn_name(other.x),
                    y: self.y.$fn_name(other.y),
                }
            }
        }
        impl<'a, T> $ops_trait<Xy<T>> for &'a Xy<T>
        where
            T: $ops_trait<Output = T> + Copy,
        {
            type Output = Xy<T>;
            fn $fn_name(self, other: Xy<T>) -> Xy<T> {
                Xy {
                    x: self.x.$fn_name(other.x),
                    y: self.y.$fn_name(other.y),
                }
            }
        }
        impl<'b, T> $ops_trait<&'b Xy<T>> for Xy<T>
        where
            T: $ops_trait<Output = T> + Copy,
        {
            type Output = Xy<T>;
            fn $fn_name(self, other: &'b Xy<T>) -> Xy<T> {
                Xy {
                    x: self.x.$fn_name(other.x),
                    y: self.y.$fn_name(other.y),
                }
            }
        }
        impl<'a, 'b, T> $ops_trait<&'b Xy<T>> for &'a Xy<T>
        where
            T: $ops_trait<Output = T> + Copy,
        {
            type Output = Xy<T>;
            fn $fn_name(self, other: &'b Xy<T>) -> Xy<T> {
                Xy {
                    x: self.x.$fn_name(other.x),
                    y: self.y.$fn_name(other.y),
                }
            }
        }
    };
}
overload_operator!(Add, add);
overload_operator!(Sub, sub);
overload_operator!(Mul, mul);
overload_operator!(Div, div);

impl Mul<Xy<f32>> for f32 {
    type Output = Xy<f32>;
    fn mul(self, rhs: Xy<f32>) -> Self::Output {
        Xy {
            x: rhs.x.mul(self),
            y: rhs.y.mul(self),
        }
    }
}
impl Div<Xy<f32>> for f32 {
    type Output = Xy<f32>;
    fn div(self, rhs: Xy<f32>) -> Self::Output {
        Xy {
            x: rhs.x.div(self),
            y: rhs.y.div(self),
        }
    }
}

impl Xy<f32> {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
impl Xy<f64> {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
