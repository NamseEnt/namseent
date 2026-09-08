//! Namui state and presentation adapters for `td_core::combat_number`.

use namui::*;
use serde::{
    Deserialize as SerdeDeserialize, Deserializer as SerdeDeserializer,
    Serialize as SerdeSerialize, Serializer as SerdeSerializer, de::Error as DeError,
};
use std::fmt;

pub const AMOUNT_SCALE: i64 = td_core::AMOUNT_SCALE;
pub const RATIO_SCALE: i64 = td_core::RATIO_SCALE;

fn serialize_scaled<S: SerdeSerializer>(
    raw: i64,
    scale: i64,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(raw as f64 / scale as f64)
}

fn parse_scaled<E: DeError>(value: f64, scale: i64) -> Result<i64, E> {
    if !value.is_finite() {
        return Err(E::custom("fixed-point value is invalid or out of range"));
    }
    let scaled = value * scale as f64;
    let rounded = if scaled >= 0.0 {
        scaled.floor() + f64::from((scaled.fract() >= 0.5) as u8)
    } else {
        scaled.ceil() - f64::from((scaled.fract().abs() >= 0.5) as u8)
    };
    let rounded = rounded as i128;
    (i64::MIN as i128..=i64::MAX as i128)
        .contains(&rounded)
        .then_some(rounded as i64)
        .ok_or_else(|| E::custom("fixed-point value is invalid or out of range"))
}

macro_rules! amount_type {
    ($name:ident, $core_name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
        pub struct $name(i64);

        impl $name {
            pub const ZERO: Self = Self(0);
            pub const MAX: Self = Self(i64::MAX);

            pub const fn from_raw(raw: i64) -> Self {
                Self::from_core(td_core::$core_name::from_raw(raw))
            }

            pub const fn from_integer(value: i64) -> Self {
                Self::from_core(td_core::$core_name::from_integer(value))
            }

            pub fn from_usize(value: usize) -> Self {
                Self::from_core(td_core::$core_name::from_usize(value))
            }

            pub const fn raw(self) -> i64 {
                self.0
            }

            pub fn from_f64(value: f64) -> Result<Self, String> {
                td_core::$core_name::from_f64(value).map(Self::from_core)
            }

            pub fn as_f32(self) -> f32 {
                self.to_core().as_f32()
            }

            pub fn as_f64(self) -> f64 {
                self.to_core().as_f64()
            }

            pub const fn is_zero(self) -> bool {
                self.0 == 0
            }

            pub fn saturating_add(self, rhs: Self) -> Self {
                Self::from_core(self.to_core().saturating_add(rhs.to_core()))
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self::from_core(self.to_core().saturating_sub(rhs.to_core()))
            }

            pub fn min(self, rhs: Self) -> Self {
                Self::from_core(self.to_core().min(rhs.to_core()))
            }

            pub fn max(self, rhs: Self) -> Self {
                Self::from_core(self.to_core().max(rhs.to_core()))
            }

            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self::from_core(self.to_core().clamp(min.to_core(), max.to_core()))
            }

            pub fn scaled_by(self, ratio: FixedRatio) -> Self {
                Self::from_core(
                    self.to_core()
                        .scaled_by(td_core::FixedRatio::from_raw(ratio.raw())),
                )
            }

            pub fn scaled_by_all(self, ratios: impl IntoIterator<Item = FixedRatio>) -> Self {
                Self::from_core(
                    self.to_core().scaled_by_all(
                        ratios
                            .into_iter()
                            .map(|ratio| td_core::FixedRatio::from_raw(ratio.raw())),
                    ),
                )
            }

            pub fn scaled_by_product(self, product: &RatioProduct) -> Self {
                Self::from_core(self.to_core().scaled_by_product(&product.to_core()))
            }

            pub fn ratio_of(self, denominator: Self) -> FixedRatio {
                FixedRatio::from_core(self.to_core().ratio_of(denominator.to_core()))
            }

            fn to_core(self) -> td_core::$core_name {
                td_core::$core_name::from_raw(self.0)
            }

            const fn from_core(value: td_core::$core_name) -> Self {
                Self(value.raw())
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

amount_type!(Health, Health);
amount_type!(Damage, Damage);
amount_type!(Shield, Shield);

macro_rules! signed_amount_type {
    ($name:ident, $core_name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
        pub struct $name(i64);

        impl $name {
            pub const ZERO: Self = Self(0);

            pub const fn from_raw(raw: i64) -> Self {
                Self(raw)
            }

            pub const fn from_integer(value: i64) -> Self {
                Self::from_core(td_core::$core_name::from_integer(value))
            }

            pub fn from_usize(value: usize) -> Self {
                Self::from_core(td_core::$core_name::from_usize(value))
            }

            pub const fn raw(self) -> i64 {
                self.0
            }

            pub fn from_f64(value: f64) -> Result<Self, String> {
                td_core::$core_name::from_f64(value).map(Self::from_core)
            }

            pub fn as_f32(self) -> f32 {
                self.to_core().as_f32()
            }

            pub fn saturating_add(self, rhs: Self) -> Self {
                Self::from_core(self.to_core().saturating_add(rhs.to_core()))
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self::from_core(self.to_core().saturating_sub(rhs.to_core()))
            }

            fn to_core(self) -> td_core::$core_name {
                td_core::$core_name::from_raw(self.0)
            }

            const fn from_core(value: td_core::$core_name) -> Self {
                Self(value.raw())
            }
        }
    };
}

signed_amount_type!(HealthDelta, HealthDelta);
signed_amount_type!(DamageDelta, DamageDelta);

impl Health {
    pub fn saturating_add_delta(self, delta: HealthDelta) -> Self {
        Self::from_raw(
            self.to_core()
                .saturating_add_delta(td_core::HealthDelta::from_raw(delta.raw()))
                .raw(),
        )
    }
}

impl Damage {
    pub fn saturating_add_delta(self, delta: DamageDelta) -> Self {
        Self::from_raw(
            self.to_core()
                .saturating_add_delta(td_core::DamageDelta::from_raw(delta.raw()))
                .raw(),
        )
    }

    pub fn split_evenly(self, parts: usize) -> Vec<Self> {
        self.to_core()
            .split_evenly(parts)
            .into_iter()
            .map(Self::from_core)
            .collect()
    }
}

impl fmt::Display for Health {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_core().fmt(formatter)
    }
}

impl fmt::Display for Damage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_core().fmt(formatter)
    }
}

impl fmt::Display for Shield {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_core().fmt(formatter)
    }
}

impl fmt::Display for HealthDelta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.*}",
            formatter.precision().unwrap_or(3),
            self.raw() as f64 / AMOUNT_SCALE as f64
        )
    }
}

impl fmt::Display for DamageDelta {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.*}",
            formatter.precision().unwrap_or(3),
            self.raw() as f64 / AMOUNT_SCALE as f64
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct FixedRatio(i64);

impl FixedRatio {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(RATIO_SCALE);
    pub const MAX: Self = Self(i64::MAX);

    pub const fn from_raw(raw: i64) -> Self {
        Self::from_core(td_core::FixedRatio::from_raw(raw))
    }

    pub const fn from_integer(value: i64) -> Self {
        Self::from_core(td_core::FixedRatio::from_integer(value))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    pub fn from_f64(value: f64) -> Result<Self, String> {
        td_core::FixedRatio::from_f64(value).map(Self::from_core)
    }

    pub fn as_f32(self) -> f32 {
        self.to_core().as_f32()
    }

    pub fn as_f64(self) -> f64 {
        self.to_core().as_f64()
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn saturating_mul(self, rhs: Self) -> Self {
        Self::from_core(
            self.to_core()
                .saturating_mul(td_core::FixedRatio::from_raw(rhs.raw())),
        )
    }

    pub fn saturating_add(self, rhs: Self) -> Self {
        Self::from_core(self.to_core().saturating_add(rhs.to_core()))
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self::from_core(self.to_core().saturating_sub(rhs.to_core()))
    }

    pub fn reciprocal(self) -> Self {
        Self::from_core(self.to_core().reciprocal())
    }

    pub fn div_integer(self, divisor: i64) -> Self {
        Self::from_core(self.to_core().div_integer(divisor))
    }

    pub fn div_usize(self, divisor: usize) -> Self {
        Self::from_core(self.to_core().div_usize(divisor))
    }

    pub fn saturating_mul_usize(self, multiplier: usize) -> Self {
        Self::from_core(self.to_core().saturating_mul_usize(multiplier))
    }

    pub fn increased_by_percent(self, percentage: Self) -> Self {
        Self::from_core(
            self.to_core()
                .increased_by_percent(td_core::FixedRatio::from_raw(percentage.raw())),
        )
    }

    pub fn decreased_by_percent(self, percentage: Self) -> Self {
        Self::from_core(
            self.to_core()
                .decreased_by_percent(td_core::FixedRatio::from_raw(percentage.raw())),
        )
    }

    pub fn one_minus(self) -> Self {
        Self::from_core(self.to_core().one_minus())
    }

    fn to_core(self) -> td_core::FixedRatio {
        td_core::FixedRatio::from_raw(self.0)
    }

    const fn from_core(value: td_core::FixedRatio) -> Self {
        Self(value.raw())
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

impl fmt::Display for FixedRatio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:.*}",
            formatter.precision().unwrap_or(3),
            self.as_f64()
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
pub struct ClearRate(FixedRatio);

impl ClearRate {
    pub const ZERO: Self = Self(FixedRatio::ZERO);
    pub const FULL: Self = Self(FixedRatio::ONE);

    pub fn from_ratio(ratio: FixedRatio) -> Self {
        Self(FixedRatio::from_core(
            td_core::ClearRate::from_ratio(ratio.to_core()).ratio(),
        ))
    }

    pub const fn ratio(self) -> FixedRatio {
        self.0
    }

    pub const fn raw(self) -> i64 {
        self.0.raw()
    }

    pub fn as_percent_f32(self) -> f32 {
        td_core::ClearRate::from_ratio(self.0.to_core()).as_percent_f32()
    }

    pub fn as_percent_f64(self) -> f64 {
        td_core::ClearRate::from_ratio(self.0.to_core()).as_percent_f64()
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
        FixedRatio::from_core(self.to_core().combined_ratio())
    }

    pub fn apply_usize(&self, amount: usize) -> usize {
        self.to_core().apply_usize(amount)
    }

    pub fn apply_raw(&self, amount: i64) -> i64 {
        self.to_core().apply_raw(amount)
    }

    fn to_core(&self) -> td_core::RatioProduct {
        self.factors
            .iter()
            .fold(td_core::RatioProduct::one(), |product, factor| {
                product.with(td_core::FixedRatio::from_raw(factor.raw()))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_conversion_uses_nearest_ties_away_from_zero() {
        assert_eq!(Health::from_f64(1.2345).unwrap().raw(), 1_235);
        assert_eq!(FixedRatio::from_f64(1.0000005).unwrap().raw(), 1_000_001);
        assert!(Health::from_f64(f64::NAN).is_err());
    }

    #[test]
    fn wrapper_arithmetic_matches_core() {
        let amount = Damage::from_integer(100);
        let ratio = FixedRatio::from_f64(1.25).unwrap();
        assert_eq!(
            amount.scaled_by(ratio).raw(),
            td_core::Damage::from_integer(100)
                .scaled_by(td_core::FixedRatio::from_raw(ratio.raw()))
                .raw()
        );
        assert_eq!(
            FixedRatio::from_raw(400_000).reciprocal().raw(),
            td_core::FixedRatio::from_raw(400_000).reciprocal().raw()
        );
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
    fn clear_rate_is_bounded() {
        assert_eq!(
            ClearRate::from_ratio(FixedRatio::from_integer(2)),
            ClearRate::FULL
        );
    }
}
