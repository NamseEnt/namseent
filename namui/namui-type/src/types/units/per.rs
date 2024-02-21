use crate::*;

#[type_derives(-Debug, -PartialEq, Copy)]
pub struct Per<TNumerator, TDenominator> {
    numerator: TNumerator,
    denominator: TDenominator,
}

impl<TNumerator, TDenominator> Per<TNumerator, TDenominator> {
    pub fn new(numerator: TNumerator, denominator: TDenominator) -> Self {
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

impl<TNumerator, TDenominator> Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator>,
{
    pub fn mul_to_numerator(self, rhs: f32) -> Self {
        Self {
            numerator: self.numerator * rhs,
            denominator: self.denominator,
        }
    }
}

impl<TNumerator, TDenominator, T> std::ops::Mul<TDenominator> for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<T, Output = TNumerator>,
    TDenominator: std::ops::Div<TDenominator, Output = T>,
{
    type Output = TNumerator;

    fn mul(self, rhs: TDenominator) -> Self::Output {
        self.numerator * (rhs / self.denominator)
    }
}

impl<TNumerator, TDenominator> std::ops::Add<Self> for Per<TNumerator, TDenominator>
where
    TNumerator:
        std::ops::Mul<f32, Output = TNumerator> + std::ops::Add<TNumerator, Output = TNumerator>,
    TDenominator: std::ops::Div<Output = f32> + Clone,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator
                + (rhs.numerator * (self.denominator.clone() / rhs.denominator)),
            denominator: self.denominator,
        }
    }
}

impl<TNumerator, TDenominator> std::ops::AddAssign for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator>
        + std::ops::Add<TNumerator, Output = TNumerator>
        + Clone,
    TDenominator: std::ops::Div<Output = f32> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            numerator: self.numerator.clone()
                + (rhs.numerator * (self.denominator.clone() / rhs.denominator)),
            denominator: self.denominator.clone(),
        };
    }
}

impl<TNumerator, TDenominator> std::ops::SubAssign for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator>
        + std::ops::Sub<TNumerator, Output = TNumerator>
        + Clone,
    TDenominator: std::ops::Div<Output = f32> + Clone,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            numerator: self.numerator.clone()
                - (rhs.numerator * (self.denominator.clone() / rhs.denominator)),
            denominator: self.denominator.clone(),
        };
    }
}

impl<TNumerator, TDenominator> std::fmt::Debug for Per<TNumerator, TDenominator>
where
    TNumerator: std::fmt::Debug,
    TDenominator: std::fmt::Debug,
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
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + Ord + Copy,
    TDenominator: std::ops::Div<Output = f32> + Copy,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let denominator_ratio = self.denominator / other.denominator;
        let common_other_numerator = other.numerator * denominator_ratio;
        self.numerator.cmp(&common_other_numerator)
    }
}
impl<TNumerator, TDenominator> Eq for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + PartialOrd + PartialEq + Copy,
    TDenominator: std::ops::Div<Output = f32> + Copy,
{
}
impl<TNumerator, TDenominator> PartialOrd for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + PartialOrd + Copy,
    TDenominator: std::ops::Div<Output = f32> + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let denominator_ratio = self.denominator / other.denominator;
        let common_other_numerator = other.numerator * denominator_ratio;
        self.numerator.partial_cmp(&common_other_numerator)
    }
}
impl<TNumerator, TDenominator> PartialEq for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator> + PartialEq + Copy,
    TDenominator: std::ops::Div<Output = f32> + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        let denominator_ratio = self.denominator / other.denominator;
        let common_other_numerator = other.numerator * denominator_ratio;
        self.numerator == common_other_numerator
    }
}

impl<TNumerator, TDenominator> std::fmt::Display for Per<TNumerator, TDenominator>
where
    TNumerator: std::fmt::Display,
    TDenominator: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl<TNumerator, TDenominator> crate::SimpleSigned for Per<TNumerator, TDenominator>
where
    TNumerator: crate::SimpleSigned,
    TDenominator: crate::SimpleSigned,
{
    fn is_sign_positive(&self) -> bool {
        !self.is_sign_negative()
    }

    fn is_sign_negative(&self) -> bool {
        self.numerator.is_sign_positive() ^ self.denominator.is_sign_positive()
    }
}
