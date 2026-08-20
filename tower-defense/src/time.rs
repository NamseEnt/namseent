use crate::FixedRatio;
use namui::*;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const SIM_TICKS_PER_SECOND: u64 = 60;
const NANOS_PER_SECOND: u128 = 1_000_000_000;
pub const INTERPOLATION_UNITS_PER_TICK: u64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
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

    pub fn from_duration_ceil(duration: Duration) -> Self {
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

    pub fn scale_ratio_ceil(self, multiplier: FixedRatio) -> Self {
        if self.0 == 0 || multiplier.is_zero() {
            return Self::ZERO;
        }
        let numerator = self.0 as u128 * multiplier.raw() as u128;
        let ticks = numerator.div_ceil(crate::combat_number::RATIO_SCALE as u128);
        Self(ticks.min(u64::MAX as u128) as u64)
    }

    pub fn as_seconds(self) -> f64 {
        self.0 as f64 / SIM_TICKS_PER_SECOND as f64
    }

    pub fn as_secs_f32(self) -> f32 {
        self.as_seconds() as f32
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

/// Fraction of the way from the previous fixed simulation tick to the current
/// fixed simulation tick. This value is presentation-only.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, State)]
pub struct InterpolationAlpha(f32);

impl InterpolationAlpha {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);

    pub fn from_fractional_units(units: u64) -> Self {
        let units = units.min(INTERPOLATION_UNITS_PER_TICK);
        Self(units as f32 / INTERPOLATION_UNITS_PER_TICK as f32)
    }

    pub fn from_f32(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub const fn as_f32(self) -> f32 {
        self.0
    }
}

/// The presentation time between two authoritative fixed simulation ticks.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, State)]
pub struct SimRenderTime {
    pub tick: SimTick,
    pub alpha: InterpolationAlpha,
}

impl SimRenderTime {
    pub const fn new(tick: SimTick, alpha: InterpolationAlpha) -> Self {
        Self { tick, alpha }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, State)]
pub struct PresentationInstant(Instant);

impl PresentationInstant {
    pub fn zero() -> Self {
        Self(Instant::new(Duration::ZERO))
    }

    pub fn capture() -> Self {
        Self(Instant::now())
    }

    pub const fn from_namui(instant: Instant) -> Self {
        Self(instant)
    }

    pub const fn as_namui(self) -> Instant {
        self.0
    }

    pub fn delta_since(self, earlier: Self) -> PresentationDelta {
        PresentationDelta::from_namui(self.0 - earlier.0)
    }
}

impl Sub for PresentationInstant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct PresentationDelta(u64);

impl PresentationDelta {
    pub const ZERO: Self = Self(0);

    pub const fn from_nanos(nanos: u64) -> Self {
        Self(nanos)
    }

    pub fn from_namui(duration: Duration) -> Self {
        if !duration.is_positive() {
            return Self::ZERO;
        }

        let nanos = (duration.as_secs_f64().max(0.0) * NANOS_PER_SECOND as f64).round();
        Self(nanos.min(u64::MAX as f64) as u64)
    }

    pub const fn as_nanos(self) -> u64 {
        self.0
    }

    pub fn as_secs_f32(self) -> f32 {
        self.0 as f32 / NANOS_PER_SECOND as f32
    }

    pub fn as_secs_f64(self) -> f64 {
        self.0 as f64 / NANOS_PER_SECOND as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_ticks_and_spans_are_distinct_domains() {
        let tick = SimTick::from_ticks(10);
        let later = tick + SimTickSpan::from_ticks(5);

        assert_eq!((later - tick).ticks(), 5);
        assert_eq!(
            SimTickSpan::from_millis_ceil(1_000),
            SimTickSpan::from_ticks(60)
        );
    }

    #[test]
    fn positive_simulation_durations_round_up_to_a_tick() {
        assert_eq!(SimTickSpan::from_millis_ceil(1), SimTickSpan::ONE);
        assert_eq!(SimTickSpan::from_seconds_ceil(0.0), SimTickSpan::ZERO);
        assert_eq!(SimTickSpan::from_seconds_ceil(0.000_001), SimTickSpan::ONE);
    }

    #[test]
    fn presentation_delta_is_monotonic_and_integer_based() {
        let first = PresentationInstant::from_namui(Instant::new(Duration::from_secs(1)));
        let second = PresentationInstant::from_namui(Instant::new(Duration::from_secs_f32(1.25)));

        assert_eq!(second.delta_since(first).as_nanos(), 250_000_000);
        assert_eq!(first.delta_since(second), PresentationDelta::ZERO);
    }

    #[test]
    fn interpolation_alpha_is_bounded_and_uses_fractional_units() {
        assert_eq!(
            InterpolationAlpha::from_fractional_units(0),
            InterpolationAlpha::ZERO
        );
        assert_eq!(
            InterpolationAlpha::from_fractional_units(INTERPOLATION_UNITS_PER_TICK / 2).as_f32(),
            0.5
        );
        assert_eq!(
            InterpolationAlpha::from_fractional_units(INTERPOLATION_UNITS_PER_TICK + 1),
            InterpolationAlpha::ONE
        );
        assert_eq!(InterpolationAlpha::from_f32(-1.0), InterpolationAlpha::ZERO);
        assert_eq!(InterpolationAlpha::from_f32(2.0), InterpolationAlpha::ONE);
    }
}
