use crate::*;
use std::fmt::Debug;

#[type_derives(-Debug, -PartialEq, Copy)]
pub struct Per<TNumerator, TDenominator>
where
    TNumerator: Debug,
    TDenominator: Debug,
{
    numerator: TNumerator,
    denominator: TDenominator,
}

impl<TNumerator, TDenominator> Per<TNumerator, TDenominator>
where
    TNumerator: Debug,
    TDenominator: Debug,
{
    pub const fn new(numerator: TNumerator, denominator: TDenominator) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn invert(self) -> Per<TDenominator, TNumerator> {
        Per {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}

impl<TNumerator, TDenominator, T> std::ops::Mul<TDenominator> for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<T, Output = TNumerator> + Debug,
    TDenominator: std::ops::Div<TDenominator, Output = T> + Debug,
{
    type Output = TNumerator;

    fn mul(self, rhs: TDenominator) -> Self::Output {
        self.numerator * (rhs / self.denominator)
    }
}

impl<TNumerator, TDenominator> std::fmt::Debug for Per<TNumerator, TDenominator>
where
    TNumerator: Debug,
    TDenominator: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Per")
            .field("numerator", &self.numerator)
            .field("denominator", &self.denominator)
            .finish()
    }
}

impl<TNumerator, TDenominator> Ord for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + Ord + Copy + Debug,
    TDenominator: std::ops::Div<Output = f32> + Copy + Debug,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let denominator_ratio = self.denominator / other.denominator;
        let common_other_numerator = other.numerator * denominator_ratio;
        self.numerator.cmp(&common_other_numerator)
    }
}
impl<TNumerator, TDenominator> Eq for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + PartialOrd + PartialEq + Copy + Debug,
    TDenominator: std::ops::Div<Output = f32> + Copy + Debug,
{
}
impl<TNumerator, TDenominator> PartialOrd for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + PartialOrd + Copy + Debug,
    TDenominator: std::ops::Div<Output = f32> + Copy + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let denominator_ratio = self.denominator / other.denominator;
        let common_other_numerator = other.numerator * denominator_ratio;
        self.numerator.partial_cmp(&common_other_numerator)
    }
}
impl<TNumerator, TDenominator> PartialEq for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + PartialEq + Copy + Debug,
    TDenominator: std::ops::Div<Output = f32> + Copy + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        let denominator_ratio = self.denominator / other.denominator;
        let common_other_numerator = other.numerator * denominator_ratio;
        self.numerator == common_other_numerator
    }
}

impl<TNumerator, TDenominator> std::fmt::Display for Per<TNumerator, TDenominator>
where
    TNumerator: std::fmt::Display + Debug,
    TDenominator: std::fmt::Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl<TNumerator, TDenominator> crate::SimpleSigned for Per<TNumerator, TDenominator>
where
    TNumerator: crate::SimpleSigned + Debug,
    TDenominator: crate::SimpleSigned + Debug,
{
    fn is_sign_positive(&self) -> bool {
        !self.is_sign_negative()
    }

    fn is_sign_negative(&self) -> bool {
        self.numerator.is_sign_positive() ^ self.denominator.is_sign_positive()
    }
}
