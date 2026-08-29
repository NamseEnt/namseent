use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const SIM_TICKS_PER_SECOND: u64 = 60;
pub const RATIO_SCALE: i64 = 1_000_000;

pub trait RatioRaw {
    fn raw(&self) -> i64;
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct SimTick(u64);

impl SimTick {
    pub const ZERO: Self = Self(0);

    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    pub const fn ticks(self) -> u64 {
        self.0
    }

    pub const fn saturating_add(self, span: SimTickSpan) -> Self {
        Self(self.0.saturating_add(span.0))
    }
}

impl Add<SimTickSpan> for SimTick {
    type Output = Self;

    fn add(self, rhs: SimTickSpan) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl AddAssign<SimTickSpan> for SimTick {
    fn add_assign(&mut self, rhs: SimTickSpan) {
        *self = *self + rhs;
    }
}

impl Sub for SimTick {
    type Output = SimTickSpan;

    fn sub(self, rhs: Self) -> Self::Output {
        SimTickSpan(self.0.saturating_sub(rhs.0))
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct SimTickSpan(u64);

impl SimTickSpan {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    pub const fn ticks(self) -> u64 {
        self.0
    }

    pub const fn from_millis_ceil(millis: u64) -> Self {
        Self(millis.saturating_mul(SIM_TICKS_PER_SECOND).div_ceil(1_000))
    }

    pub fn from_duration_ceil(duration: std::time::Duration) -> Self {
        Self::from_seconds_ceil(duration.as_secs_f64())
    }

    pub fn from_seconds_ceil(seconds: f64) -> Self {
        if seconds <= 0.0 {
            return Self::ZERO;
        }

        let ticks = (seconds * SIM_TICKS_PER_SECOND as f64).ceil();
        Self(ticks.min(u64::MAX as f64) as u64)
    }

    pub fn scale_ceil(self, multiplier: f32) -> Self {
        Self::from_seconds_ceil(self.as_seconds() * multiplier as f64)
    }

    pub fn scale_ratio_ceil(self, multiplier: impl RatioRaw) -> Self {
        if self.0 == 0 || multiplier.raw() <= 0 {
            return Self::ZERO;
        }
        let numerator = self.0 as u128 * multiplier.raw() as u128;
        let ticks = numerator.div_ceil(RATIO_SCALE as u128);
        Self(ticks.min(u64::MAX as u128) as u64)
    }

    pub fn as_secs_f32(self) -> f32 {
        self.as_seconds() as f32
    }

    pub fn as_seconds(self) -> f64 {
        self.0 as f64 / SIM_TICKS_PER_SECOND as f64
    }

    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Add for SimTickSpan {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl AddAssign for SimTickSpan {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for SimTickSpan {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl SubAssign for SimTickSpan {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
