use crate::*;
use std::{
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Neg, Sub},
};

#[type_derives(Copy, Default)]
#[repr(transparent)]
pub struct OrderedFloat {
    inner: f32,
}

impl OrderedFloat {
    pub const fn new(inner: f32) -> Self {
        Self { inner }
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.partial_cmp(&other.inner).unwrap()
    }
}

impl Eq for OrderedFloat {}
impl Hash for OrderedFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.to_bits().hash(state);
    }
}

impl AsRef<f32> for OrderedFloat {
    fn as_ref(&self) -> &f32 {
        &self.inner
    }
}

impl AsMut<f32> for OrderedFloat {
    fn as_mut(&mut self) -> &mut f32 {
        &mut self.inner
    }
}

impl Deref for OrderedFloat {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for OrderedFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<f32> for OrderedFloat {
    fn from(inner: f32) -> Self {
        Self::new(inner)
    }
}

impl Mul<f32> for OrderedFloat {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.inner * rhs)
    }
}
impl Mul<OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn mul(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self * rhs.inner)
    }
}
impl MulAssign<f32> for OrderedFloat {
    fn mul_assign(&mut self, rhs: f32) {
        self.inner *= rhs;
    }
}
impl MulAssign<OrderedFloat> for f32 {
    fn mul_assign(&mut self, rhs: OrderedFloat) {
        *self *= rhs.inner;
    }
}
impl Mul<f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn mul(self, rhs: f32) -> Self::Output {
        OrderedFloat::new(self.inner * rhs)
    }
}
impl Mul<&OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn mul(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(self * rhs.inner)
    }
}
impl Mul<&f32> for OrderedFloat {
    type Output = OrderedFloat;

    fn mul(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner * *rhs)
    }
}
impl Mul<OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn mul(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self * rhs.inner)
    }
}
impl Mul<&OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn mul(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self * rhs.inner)
    }
}
impl Mul<&f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn mul(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner * *rhs)
    }
}
impl Mul<OrderedFloat> for OrderedFloat {
    type Output = OrderedFloat;

    fn mul(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self.inner * rhs.inner)
    }
}
impl MulAssign<OrderedFloat> for OrderedFloat {
    fn mul_assign(&mut self, rhs: OrderedFloat) {
        self.inner *= rhs.inner;
    }
}
impl Add<f32> for OrderedFloat {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.inner + rhs)
    }
}
impl Add<OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn add(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self + rhs.inner)
    }
}
impl Add<f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn add(self, rhs: f32) -> Self::Output {
        OrderedFloat::new(self.inner + rhs)
    }
}
impl Add<&OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn add(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(self + rhs.inner)
    }
}
impl Add<&f32> for OrderedFloat {
    type Output = OrderedFloat;

    fn add(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner + *rhs)
    }
}
impl Add<OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn add(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self + rhs.inner)
    }
}
impl Add<&OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn add(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self + rhs.inner)
    }
}
impl Add<&f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn add(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner + *rhs)
    }
}
impl Add<OrderedFloat> for OrderedFloat {
    type Output = OrderedFloat;

    fn add(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self.inner + rhs.inner)
    }
}

impl AddAssign<f32> for OrderedFloat {
    fn add_assign(&mut self, rhs: f32) {
        self.inner += rhs;
    }
}
impl AddAssign<OrderedFloat> for f32 {
    fn add_assign(&mut self, rhs: OrderedFloat) {
        *self += rhs.inner;
    }
}
impl AddAssign<OrderedFloat> for OrderedFloat {
    fn add_assign(&mut self, rhs: OrderedFloat) {
        self.inner += rhs.inner;
    }
}

impl Sub<f32> for OrderedFloat {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.inner - rhs)
    }
}
impl Sub<OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn sub(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self - rhs.inner)
    }
}
impl Sub<f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn sub(self, rhs: f32) -> Self::Output {
        OrderedFloat::new(self.inner - rhs)
    }
}
impl Sub<&OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn sub(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(self - rhs.inner)
    }
}
impl Sub<&f32> for OrderedFloat {
    type Output = OrderedFloat;

    fn sub(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner - *rhs)
    }
}
impl Sub<OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn sub(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self - rhs.inner)
    }
}
impl Sub<&OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn sub(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self - rhs.inner)
    }
}
impl Sub<&f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn sub(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner - *rhs)
    }
}
impl Sub<OrderedFloat> for OrderedFloat {
    type Output = OrderedFloat;

    fn sub(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self.inner - rhs.inner)
    }
}

impl Div<f32> for OrderedFloat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.inner / rhs)
    }
}
impl Div<OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn div(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self / rhs.inner)
    }
}
impl Div<f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn div(self, rhs: f32) -> Self::Output {
        OrderedFloat::new(self.inner / rhs)
    }
}
impl Div<&OrderedFloat> for f32 {
    type Output = OrderedFloat;

    fn div(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(self / rhs.inner)
    }
}
impl Div<&f32> for OrderedFloat {
    type Output = OrderedFloat;

    fn div(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner / *rhs)
    }
}
impl Div<OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn div(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self / rhs.inner)
    }
}
impl Div<&OrderedFloat> for &f32 {
    type Output = OrderedFloat;

    fn div(self, rhs: &OrderedFloat) -> Self::Output {
        OrderedFloat::new(*self / rhs.inner)
    }
}
impl Div<&f32> for &OrderedFloat {
    type Output = OrderedFloat;

    fn div(self, rhs: &f32) -> Self::Output {
        OrderedFloat::new(self.inner / *rhs)
    }
}
impl Div<OrderedFloat> for OrderedFloat {
    type Output = OrderedFloat;

    fn div(self, rhs: OrderedFloat) -> Self::Output {
        OrderedFloat::new(self.inner / rhs.inner)
    }
}

impl Neg for OrderedFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.inner)
    }
}
