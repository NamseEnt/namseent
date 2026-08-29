//! Fixed-point values used by authoritative gameplay.
//!
//! Serialization intentionally remains in game units so existing snapshots and
//! JSON files keep their field representation. Rendering-specific formatting
//! belongs to the headed crate.

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
        value.floor() + f64::from((value.fract() >= 0.5) as u8)
    } else {
        value.ceil() - f64::from((value.fract().abs() >= 0.5) as u8)
    };
    let rounded = rounded as i128;
    (i64::MIN as i128..=i64::MAX as i128)
        .contains(&rounded)
        .then_some(rounded as i64)
}

fn parse_scaled<E: DeError>(value: f64, scale: i64) -> Result<i64, E> {
    round_ties_away_from_zero(value * scale as f64)
        .ok_or_else(|| E::custom("fixed-point value is invalid or out of range"))
}

fn serialize_scaled<S: SerdeSerializer>(
    raw: i64,
    scale: i64,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(raw as f64 / scale as f64)
}

macro_rules! unsigned_amount {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(i64);

        impl $name {
            pub const ZERO: Self = Self(0);
            pub const MAX: Self = Self(i64::MAX);

            pub const fn from_raw(raw: i64) -> Self {
                Self(if raw < 0 { 0 } else { raw })
            }

            pub const fn from_integer(value: i64) -> Self {
                Self::from_raw(value.saturating_mul(AMOUNT_SCALE))
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
                (raw >= 0)
                    .then_some(Self(raw))
                    .ok_or_else(|| "amount cannot be negative".to_string())
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
                Self::from_raw(self.0.saturating_add(rhs.0))
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self::from_raw(self.0.saturating_sub(rhs.0))
            }

            pub fn min(self, rhs: Self) -> Self {
                Self(if self.0 < rhs.0 { self.0 } else { rhs.0 })
            }

            pub fn max(self, rhs: Self) -> Self {
                Self(if self.0 > rhs.0 { self.0 } else { rhs.0 })
            }

            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.max(min.0).min(max.0))
            }

            pub fn scaled_by(self, ratio: FixedRatio) -> Self {
                Self::from_raw(RatioProduct::one().with(ratio).apply_raw(self.0))
            }

            pub fn scaled_by_all(self, ratios: impl IntoIterator<Item = FixedRatio>) -> Self {
                Self::from_raw(
                    ratios
                        .into_iter()
                        .fold(RatioProduct::one(), RatioProduct::with)
                        .apply_raw(self.0),
                )
            }

            pub fn scaled_by_product(self, product: &RatioProduct) -> Self {
                Self::from_raw(product.apply_raw(self.0))
            }

            pub fn ratio_of(self, denominator: Self) -> FixedRatio {
                if denominator.is_zero() {
                    return FixedRatio::ZERO;
                }
                FixedRatio::from_raw(
                    div_round_positive(
                        i128::from(self.raw()).saturating_mul(i128::from(RATIO_SCALE)),
                        i128::from(denominator.raw()),
                    )
                    .min(i128::from(i64::MAX)) as i64,
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
                let raw = parse_scaled::<D::Error>(
                    <f64 as SerdeDeserialize>::deserialize(deserializer)?,
                    AMOUNT_SCALE,
                )?;
                (raw >= 0)
                    .then_some(Self(raw))
                    .ok_or_else(|| D::Error::custom("amount cannot be negative"))
            }
        }
    };
}

macro_rules! signed_amount {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

unsigned_amount!(Health);
unsigned_amount!(Damage);
unsigned_amount!(Shield);
signed_amount!(HealthDelta);
signed_amount!(DamageDelta);

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedRatio(i64);

impl FixedRatio {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(RATIO_SCALE);
    pub const MAX: Self = Self(i64::MAX);

    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }

    pub const fn from_integer(value: i64) -> Self {
        Self::from_raw(value.saturating_mul(RATIO_SCALE))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    pub fn from_f64(value: f64) -> Result<Self, String> {
        let raw = round_ties_away_from_zero(value * RATIO_SCALE as f64)
            .ok_or_else(|| "ratio is invalid or out of range".to_string())?;
        (raw >= 0)
            .then_some(Self(raw))
            .ok_or_else(|| "ratio cannot be negative".to_string())
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
                i128::from(RATIO_SCALE).saturating_mul(i128::from(RATIO_SCALE)),
                i128::from(self.raw()),
            )
            .clamp(0, i128::from(i64::MAX)) as i64,
        )
    }

    pub fn div_integer(self, divisor: i64) -> Self {
        if divisor <= 0 {
            return Self::ZERO;
        }
        Self::from_raw(
            div_round_positive(i128::from(self.raw()), i128::from(divisor))
                .clamp(0, i128::from(i64::MAX)) as i64,
        )
    }

    pub fn div_usize(self, divisor: usize) -> Self {
        self.div_integer(i64::try_from(divisor).unwrap_or(i64::MAX))
    }

    pub fn saturating_mul_usize(self, multiplier: usize) -> Self {
        Self::from_raw(
            i128::from(self.raw())
                .saturating_mul(multiplier as i128)
                .clamp(0, i128::from(i64::MAX)) as i64,
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

impl crate::RatioRaw for FixedRatio {
    fn raw(&self) -> i64 {
        FixedRatio::raw(*self)
    }
}

impl SerdeSerialize for FixedRatio {
    fn serialize<S: SerdeSerializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_scaled(self.0, RATIO_SCALE, serializer)
    }
}

impl<'de> SerdeDeserialize<'de> for FixedRatio {
    fn deserialize<D: SerdeDeserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = parse_scaled::<D::Error>(
            <f64 as SerdeDeserialize>::deserialize(deserializer)?,
            RATIO_SCALE,
        )?;
        (raw >= 0)
            .then_some(Self(raw))
            .ok_or_else(|| D::Error::custom("ratio cannot be negative"))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

    pub fn apply_raw(&self, amount: i64) -> i64 {
        if amount <= 0 || self.factors.iter().any(|factor| factor.is_zero()) {
            return 0;
        }
        if self.factors.is_empty() {
            return amount;
        }

        let mut factors = self.factors.clone();
        factors.sort_by_key(|factor| factor.raw());
        let mut numerator = i128::from(amount);
        let mut denominator = 1_i128;
        for factor in factors {
            let factor_numerator = i128::from(factor.raw());
            let gcd_left = gcd_i128(numerator, i128::from(RATIO_SCALE));
            numerator /= gcd_left;
            let scale = i128::from(RATIO_SCALE) / gcd_left;
            let gcd_right = gcd_i128(factor_numerator, denominator);
            numerator = match numerator.checked_mul(factor_numerator / gcd_right) {
                Some(value) => value,
                None => return i64::MAX,
            };
            denominator = match denominator
                .checked_div(gcd_right)
                .and_then(|value| value.checked_mul(scale))
            {
                Some(value) => value,
                None => return i64::MAX,
            };
        }
        div_round_positive(numerator, denominator).clamp(0, i128::from(i64::MAX)) as i64
    }
}

fn gcd_i128(mut left: i128, mut right: i128) -> i128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.abs().max(1)
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

impl fmt::Display for Health {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.*}",
            formatter.precision().unwrap_or(3),
            self.as_f64()
        )
    }
}

impl fmt::Display for Damage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.*}",
            formatter.precision().unwrap_or(3),
            self.as_f64()
        )
    }
}

impl fmt::Display for Shield {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.*}",
            formatter.precision().unwrap_or(3),
            self.as_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_golden_values_match_headed_implementation() {
        assert_eq!(Health::from_f64(1.2345).unwrap().raw(), 1_235);
        assert_eq!(FixedRatio::from_f64(1.0000005).unwrap().raw(), 1_000_001);
        assert_eq!(
            Damage::from_integer(100).scaled_by_all([
                FixedRatio::from_f64(1.25).unwrap(),
                FixedRatio::from_f64(0.8).unwrap(),
                FixedRatio::from_f64(1.1).unwrap(),
            ]),
            Damage::from_raw(110_000)
        );
    }

    #[test]
    fn snapshots_keep_floating_point_amount_shape() {
        assert_eq!(
            serde_json::to_string(&Health::from_raw(1_235)).unwrap(),
            "1.235"
        );
        assert_eq!(
            serde_json::from_str::<Health>("1.2345").unwrap(),
            Health::from_raw(1_235)
        );
    }
}
