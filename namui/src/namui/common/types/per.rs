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

impl<TNumerator, TDenominator> std::ops::Mul<TDenominator> for Per<TNumerator, TDenominator>
where
    TNumerator: std::ops::Mul<f32, Output = TNumerator>,
    TDenominator: std::ops::Div<Output = f32>,
{
    type Output = TNumerator;

    fn mul(self, rhs: TDenominator) -> Self::Output {
        self.numerator * (rhs / self.denominator)
    }
}

impl<TNumerator, TDenominator> Clone for Per<TNumerator, TDenominator>
where
    TNumerator: Clone,
    TDenominator: Clone,
{
    fn clone(&self) -> Self {
        Self {
            numerator: self.numerator.clone(),
            denominator: self.denominator.clone(),
        }
    }
}

impl<TNumerator, TDenominator> Copy for Per<TNumerator, TDenominator>
where
    TNumerator: Copy,
    TDenominator: Copy,
{
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
