//! Fixed-point values used by authoritative combat state.
//!
//! Amounts use one thousand internal units per game unit and ratios use one
//! million internal units per multiplier. The only floating-point boundary
//! in this module is configuration or presentation serialization.

use namui::*;
use serde::{
    Deserialize as SerdeDeserialize, Deserializer as SerdeDeserializer,
    Serialize as SerdeSerialize, Serializer as SerdeSerializer, de::Error as DeError,
};
use std::fmt;

pub const AMOUNT_SCALE: i64 = 1_000;
pub const RATIO_SCALE: i64 = 1_000_000;

fn round_ties_away_from_zero(value: f64) -> Option<i64> {
    if !value.is_finite() {
        return None;
    }

    let rounded = if value >= 0.0 {
        value.floor() + if value.fract() >= 0.5 { 1.0 } else { 0.0 }
    } else {
        value.ceil() - if value.fract().abs() >= 0.5 { 1.0 } else { 0.0 }
    };

    let rounded = rounded as i128;
    if rounded < i64::MIN as i128 || rounded > i64::MAX as i128 {
        None
    } else {
        Some(rounded as i64)
    }
}

fn parse_scaled<E: DeError>(value: f64, scale: i64) -> Result<i64, E> {
    let scaled = value * scale as f64;
    round_ties_away_from_zero(scaled)
        .ok_or_else(|| E::custom("fixed-point value is invalid or out of range"))
}

fn serialize_scaled<S: SerdeSerializer>(
    raw: i64,
    scale: i64,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(raw as f64 / scale as f64)
}

macro_rules! amount_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
        pub struct $name(i64);

        impl $name {
            pub const ZERO: Self = Self(0);
            pub const MAX: Self = Self(i64::MAX);

            pub const fn from_raw(raw: i64) -> Self {
                Self(if raw < 0 { 0 } else { raw })
            }

            pub const fn from_integer(value: i64) -> Self {
                let raw = value.saturating_mul(AMOUNT_SCALE);
                Self(if raw < 0 { 0 } else { raw })
            }

            pub fn from_usize(value: usize) -> Self {
                Self::from_integer(i64::try_from(value).unwrap_or(i64::MAX))
            }

            pub const fn raw(self) -> i64 {
                self.0
            }

            pub fn from_f64(value: f64) -> Result<Self, String> {
                let raw = round_ties_away_from_zero(value * AMOUNT_SCALE as f64)
                    .ok_or_else(|| "fixed-point value is invalid or out of range".to_string())?;
                if raw < 0 {
                    return Err("amount cannot be negative".to_string());
                }
                Ok(Self(raw))
            }

            pub fn as_f32(self) -> f32 {
                self.0 as f32 / AMOUNT_SCALE as f32
            }

            pub fn as_f64(self) -> f64 {
                self.0 as f64 / AMOUNT_SCALE as f64
            }

            pub const fn is_zero(self) -> bool {
                self.0 == 0
            }

            pub fn saturating_add(self, rhs: Self) -> Self {
                let raw = self.0.saturating_add(rhs.0);
                Self(if raw < 0 { 0 } else { raw })
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                let raw = self.0.saturating_sub(rhs.0);
                Self(if raw < 0 { 0 } else { raw })
            }

            pub fn min(self, rhs: Self) -> Self {
                Self(self.0.min(rhs.0))
            }

            pub fn max(self, rhs: Self) -> Self {
                Self(self.0.max(rhs.0))
            }

            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }

            pub fn scaled_by(self, ratio: FixedRatio) -> Self {
                Self(RatioProduct::one().with(ratio).apply_raw(self.0))
            }

            pub fn scaled_by_all(self, ratios: impl IntoIterator<Item = FixedRatio>) -> Self {
                let product = ratios
                    .into_iter()
                    .fold(RatioProduct::one(), RatioProduct::with);
                Self(product.apply_raw(self.0))
            }

            pub fn scaled_by_product(self, product: &RatioProduct) -> Self {
                Self(product.apply_raw(self.0))
            }

            pub fn ratio_of(self, denominator: Self) -> FixedRatio {
                if denominator.is_zero() {
                    return FixedRatio::ZERO;
                }
                FixedRatio::from_raw(
                    div_round_positive(
                        (self.raw() as i128).saturating_mul(RATIO_SCALE as i128),
                        denominator.raw() as i128,
                    )
                    .min(i64::MAX as i128) as i64,
                )
            }
        }

        impl SerdeSerialize for $name {
            fn serialize<S: SerdeSerializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serialize_scaled(self.0, AMOUNT_SCALE, serializer)
            }
        }

        impl<'de> SerdeDeserialize<'de> for $name {
            fn deserialize<D: SerdeDeserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = <f64 as SerdeDeserialize>::deserialize(deserializer)?;
                let raw = parse_scaled::<D::Error>(value, AMOUNT_SCALE)?;
                if raw < 0 {
                    return Err(D::Error::custom("amount cannot be negative"));
                }
                Ok(Self(raw))
            }
        }
    };
}

amount_type!(Health);
amount_type!(Damage);
amount_type!(Shield);

macro_rules! display_fixed {
    ($name:ident, $scale:expr) => {
        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                let precision = formatter.precision().unwrap_or(3);
                write!(
                    formatter,
                    "{:.*}",
                    precision,
                    self.raw() as f64 / $scale as f64
                )
            }
        }
    };
}

display_fixed!(Health, AMOUNT_SCALE);
display_fixed!(Damage, AMOUNT_SCALE);
display_fixed!(Shield, AMOUNT_SCALE);

macro_rules! signed_amount_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
        pub struct $name(i64);

        impl $name {
            pub const ZERO: Self = Self(0);

            pub const fn from_raw(raw: i64) -> Self {
                Self(raw)
            }

            pub const fn from_integer(value: i64) -> Self {
                Self(value.saturating_mul(AMOUNT_SCALE))
            }

            pub fn from_usize(value: usize) -> Self {
                Self::from_integer(i64::try_from(value).unwrap_or(i64::MAX))
            }

            pub const fn raw(self) -> i64 {
                self.0
            }

            pub fn from_f64(value: f64) -> Result<Self, String> {
                round_ties_away_from_zero(value * AMOUNT_SCALE as f64)
                    .map(Self)
                    .ok_or_else(|| "fixed-point value is invalid or out of range".to_string())
            }

            pub fn as_f32(self) -> f32 {
                self.0 as f32 / AMOUNT_SCALE as f32
            }

            pub fn saturating_add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }
        }
    };
}

signed_amount_type!(HealthDelta);
signed_amount_type!(DamageDelta);
display_fixed!(HealthDelta, AMOUNT_SCALE);
display_fixed!(DamageDelta, AMOUNT_SCALE);
display_fixed!(FixedRatio, RATIO_SCALE);

impl Health {
    pub fn saturating_add_delta(self, delta: HealthDelta) -> Self {
        Self::from_raw(self.raw().saturating_add(delta.raw()))
    }
}

impl Damage {
    pub fn saturating_add_delta(self, delta: DamageDelta) -> Self {
        Self::from_raw(self.raw().saturating_add(delta.raw()))
    }

    pub fn split_evenly(self, parts: usize) -> Vec<Self> {
        if parts == 0 {
            return Vec::new();
        }
        let parts = parts as i64;
        let base = self.raw() / parts;
        let remainder = self.raw() % parts;
        (0..parts)
            .map(|index| Self::from_raw(base + i64::from(index < remainder)))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct FixedRatio(i64);

impl FixedRatio {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(RATIO_SCALE);
    pub const MAX: Self = Self(i64::MAX);

    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }

    pub const fn from_integer(value: i64) -> Self {
        let raw = value.saturating_mul(RATIO_SCALE);
        Self(if raw < 0 { 0 } else { raw })
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    pub fn from_f64(value: f64) -> Result<Self, String> {
        let raw = round_ties_away_from_zero(value * RATIO_SCALE as f64)
            .ok_or_else(|| "ratio is invalid or out of range".to_string())?;
        if raw < 0 {
            return Err("ratio cannot be negative".to_string());
        }
        Ok(Self(raw))
    }

    pub fn as_f32(self) -> f32 {
        self.0 as f32 / RATIO_SCALE as f32
    }

    pub fn as_f64(self) -> f64 {
        self.0 as f64 / RATIO_SCALE as f64
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn saturating_mul(self, rhs: Self) -> Self {
        Self::from_raw(RatioProduct::one().with(rhs).apply_raw(self.raw()))
    }

    pub fn saturating_add(self, rhs: Self) -> Self {
        Self::from_raw(self.raw().saturating_add(rhs.raw()))
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self::from_raw(self.raw().saturating_sub(rhs.raw()))
    }

    pub fn reciprocal(self) -> Self {
        if self.is_zero() {
            return Self::MAX;
        }
        Self::from_raw(
            div_round_positive(
                (RATIO_SCALE as i128).saturating_mul(RATIO_SCALE as i128),
                self.raw() as i128,
            )
            .clamp(0, i64::MAX as i128) as i64,
        )
    }

    pub fn div_integer(self, divisor: i64) -> Self {
        if divisor <= 0 {
            return Self::ZERO;
        }
        Self::from_raw(
            div_round_positive(self.raw() as i128, divisor as i128).clamp(0, i64::MAX as i128)
                as i64,
        )
    }

    pub fn div_usize(self, divisor: usize) -> Self {
        if divisor == 0 {
            return Self::ZERO;
        }
        Self::from_raw(
            div_round_positive(self.raw() as i128, divisor as i128).clamp(0, i64::MAX as i128)
                as i64,
        )
    }

    pub fn saturating_mul_usize(self, multiplier: usize) -> Self {
        Self::from_raw(
            (self.raw() as i128)
                .saturating_mul(multiplier as i128)
                .clamp(0, i64::MAX as i128) as i64,
        )
    }

    pub fn increased_by_percent(self, percentage: Self) -> Self {
        Self::from_raw(self.raw().saturating_add(percentage.div_integer(100).raw()))
    }

    pub fn decreased_by_percent(self, percentage: Self) -> Self {
        Self::from_raw(self.raw().saturating_sub(percentage.div_integer(100).raw()))
    }

    pub fn one_minus(self) -> Self {
        Self::from_raw(RATIO_SCALE.saturating_sub(self.raw()))
    }
}

impl SerdeSerialize for FixedRatio {
    fn serialize<S: SerdeSerializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_scaled(self.0, RATIO_SCALE, serializer)
    }
}

impl<'de> SerdeDeserialize<'de> for FixedRatio {
    fn deserialize<D: SerdeDeserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <f64 as SerdeDeserialize>::deserialize(deserializer)?;
        let raw = parse_scaled::<D::Error>(value, RATIO_SCALE)?;
        if raw < 0 {
            return Err(D::Error::custom("ratio cannot be negative"));
        }
        Ok(Self(raw))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct ClearRate(FixedRatio);

impl ClearRate {
    pub const ZERO: Self = Self(FixedRatio::ZERO);
    pub const FULL: Self = Self(FixedRatio::ONE);

    pub fn from_ratio(ratio: FixedRatio) -> Self {
        Self(FixedRatio(ratio.raw().min(RATIO_SCALE)))
    }

    pub const fn ratio(self) -> FixedRatio {
        self.0
    }

    pub const fn raw(self) -> i64 {
        self.0.raw()
    }

    pub fn as_percent_f32(self) -> f32 {
        self.raw() as f32 * 100.0 / RATIO_SCALE as f32
    }

    pub fn as_percent_f64(self) -> f64 {
        self.raw() as f64 * 100.0 / RATIO_SCALE as f64
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, State)]
pub struct RatioProduct {
    factors: Vec<FixedRatio>,
}

impl RatioProduct {
    pub fn one() -> Self {
        Self {
            factors: Vec::new(),
        }
    }

    pub fn with(mut self, factor: FixedRatio) -> Self {
        self.factors.push(factor);
        self
    }

    pub fn with_all(mut self, factors: impl IntoIterator<Item = FixedRatio>) -> Self {
        self.factors.extend(factors);
        self
    }

    pub fn push(&mut self, factor: FixedRatio) {
        self.factors.push(factor);
    }

    pub fn factors(&self) -> &[FixedRatio] {
        &self.factors
    }

    pub fn combined_ratio(&self) -> FixedRatio {
        FixedRatio::from_raw(self.apply_raw(RATIO_SCALE))
    }

    pub fn apply_usize(&self, amount: usize) -> usize {
        usize::try_from(self.apply_raw(i64::try_from(amount).unwrap_or(i64::MAX)))
            .unwrap_or(usize::MAX)
    }

    /// Applies all factors with one final rounding operation. Factors are
    /// sorted by raw value so insertion order cannot affect the result.
    pub fn apply_raw(&self, amount: i64) -> i64 {
        if amount <= 0 || self.factors.iter().any(|factor| factor.is_zero()) {
            return 0;
        }
        if self.factors.is_empty() {
            return amount;
        }

        let mut factors = self.factors.clone();
        factors.sort_by_key(|factor| factor.raw());

        let mut numerator = amount as i128;
        let mut denominator = 1_i128;
        for factor in factors {
            let factor_numerator = factor.raw() as i128;
            let gcd_left = gcd_i128(numerator, RATIO_SCALE as i128);
            numerator /= gcd_left;
            let scale = RATIO_SCALE as i128 / gcd_left;
            let gcd_right = gcd_i128(factor_numerator, denominator);
            let factor_numerator = factor_numerator / gcd_right;
            denominator /= gcd_right;
            numerator = match numerator.checked_mul(factor_numerator) {
                Some(value) => value,
                None => return i64::MAX,
            };
            denominator = match denominator.checked_mul(scale) {
                Some(value) => value,
                None => return i64::MAX,
            };
        }

        div_round_positive(numerator, denominator).clamp(0, i64::MAX as i128) as i64
    }
}

fn gcd_i128(mut left: i128, mut right: i128) -> i128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn div_round_positive(numerator: i128, denominator: i128) -> i128 {
    debug_assert!(numerator >= 0 && denominator > 0);
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    if remainder.saturating_mul(2) >= denominator {
        quotient.saturating_add(1)
    } else {
        quotient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_conversion_uses_nearest_ties_away_from_zero() {
        assert_eq!(Health::from_f64(1.2345).unwrap().raw(), 1_235);
        assert_eq!(FixedRatio::from_f64(1.0000005).unwrap().raw(), 1_000_001);
        assert!(round_ties_away_from_zero(i64::MAX as f64).is_none());
    }

    #[test]
    fn ratio_product_is_insertion_order_independent() {
        let first = RatioProduct::one()
            .with(FixedRatio::from_f64(1.5).unwrap())
            .with(FixedRatio::from_f64(0.75).unwrap());
        let second = RatioProduct::one()
            .with(FixedRatio::from_f64(0.75).unwrap())
            .with(FixedRatio::from_f64(1.5).unwrap());

        assert_eq!(first.apply_raw(10_000), second.apply_raw(10_000));
        assert_eq!(first.apply_raw(10_000), 11_250);
    }

    #[test]
    fn zero_and_nonzero_small_damage_are_distinct() {
        assert_eq!(Damage::from_raw(1).scaled_by(FixedRatio::ZERO).raw(), 0);
        assert_eq!(Damage::from_raw(1).scaled_by(FixedRatio::ONE).raw(), 1);
        assert_eq!(Health::from_raw(-1), Health::ZERO);
        assert_eq!(Damage::from_raw(-1), Damage::ZERO);
        assert_eq!(FixedRatio::from_raw(-1), FixedRatio::ZERO);
        assert!(Health::from_f64(-0.001).is_err());
        assert!(FixedRatio::from_f64(-0.000_001).is_err());
    }

    #[test]
    fn combat_amounts_use_nearest_rounding_and_saturating_bounds() {
        let half = FixedRatio::from_raw(500_000);
        assert_eq!(Damage::from_integer(3).scaled_by(half).raw(), 1_500);
        assert_eq!(Shield::from_integer(2).scaled_by(half).raw(), 1_000);
        assert_eq!(Health::from_integer(4).scaled_by(half).raw(), 2_000);
        assert_eq!(
            Health::MAX.saturating_add(Health::from_integer(1)),
            Health::MAX
        );
        assert_eq!(
            Health::ZERO.saturating_sub(Health::from_integer(1)),
            Health::ZERO
        );
        assert_eq!(Health::from_usize(usize::MAX), Health::MAX);
        assert_eq!(DamageDelta::from_usize(usize::MAX).raw(), i64::MAX);
    }

    #[test]
    fn large_late_game_health_scaling_saturates_without_overflow() {
        let late_game_health = Health::from_raw(i64::MAX - 1);
        let scaled = late_game_health.scaled_by(FixedRatio::from_integer(2));
        assert_eq!(scaled, Health::MAX);
    }

    #[test]
    fn nested_multiplier_golden_value_is_order_independent() {
        let first = Damage::from_integer(100).scaled_by_all([
            FixedRatio::from_f64(1.25).unwrap(),
            FixedRatio::from_f64(0.8).unwrap(),
            FixedRatio::from_f64(1.1).unwrap(),
        ]);
        let second = Damage::from_integer(100).scaled_by_all([
            FixedRatio::from_f64(1.1).unwrap(),
            FixedRatio::from_f64(1.25).unwrap(),
            FixedRatio::from_f64(0.8).unwrap(),
        ]);
        assert_eq!(first, second);
        assert_eq!(first.raw(), 110_000);
    }

    #[test]
    fn clear_rate_is_bounded() {
        assert_eq!(
            ClearRate::from_ratio(FixedRatio::from_integer(2)),
            ClearRate::FULL
        );
    }
}
