use num::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::ops::*;

use crate::types::Angle;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Xy<T> {
    pub x: T,
    pub y: T,
}
impl<T: Clone> Xy<T> {
    pub fn single(value: T) -> Xy<T> {
        Xy {
            x: value.clone(),
            y: value.clone(),
        }
    }
}

impl<T: Clone> Xy<T> {
    pub fn into_type<U>(&self) -> Xy<U>
    where
        T: Into<U>,
    {
        Xy {
            x: self.x.clone().into(),
            y: self.y.clone().into(),
        }
    }
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

impl<T: Mul<f32, Output = T>> Mul<Xy<T>> for f32 {
    type Output = Xy<T>;
    fn mul(self, rhs: Xy<T>) -> Self::Output {
        Xy {
            x: rhs.x.mul(self),
            y: rhs.y.mul(self),
        }
    }
}

impl<T: Div<f32, Output = T>> Div<Xy<T>> for f32 {
    type Output = Xy<T>;
    fn div(self, rhs: Xy<T>) -> Self::Output {
        Xy {
            x: rhs.x.div(self),
            y: rhs.y.div(self),
        }
    }
}

impl<T> Xy<T>
where
    T: ToPrimitive + FromPrimitive + Copy,
{
    pub fn length(&self) -> T {
        let x = self.x.to_f32().unwrap();
        let y = self.y.to_f32().unwrap();
        FromPrimitive::from_f32((x * x + y * y).sqrt()).unwrap()
    }
    pub fn angle_to(&self, rhs: Xy<T>) -> Angle {
        let x = self.x.to_f32().unwrap();
        let y = self.y.to_f32().unwrap();
        let rhs_x = rhs.x.to_f32().unwrap();
        let rhs_y = rhs.y.to_f32().unwrap();
        Angle::Radian(
            (x * rhs_y - y * rhs_x)
                .atan2(x * rhs_x + y * rhs_y)
                .to_f32()
                .unwrap(),
        )
    }
    pub fn atan2(&self) -> Angle {
        Angle::Radian(
            self.y
                .to_f32()
                .unwrap()
                .atan2(self.x.to_f32().unwrap())
                .to_f32()
                .unwrap(),
        )
    }
}
impl<T> Xy<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone,
{
    pub fn dot(&self, rhs: Xy<T>) -> T {
        self.x.clone() * rhs.x + self.y.clone() * rhs.y
    }
}
