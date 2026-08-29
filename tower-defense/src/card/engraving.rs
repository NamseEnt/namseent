use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;
use crate::{FixedRatio, RatioProduct, SimTickSpan, WorldDistance};
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub enum Engraving {
    Magnet,
    Overcharge,
    Cactus,
    SpinningTop,
}

const CACTUS_SPLASH_RADIUS: WorldDistance = WorldDistance::from_tiles(2);
const CACTUS_SPLASH_DAMAGE_PCT: FixedRatio = FixedRatio::from_raw(300_000);
const OVERCHARGE_ATTACK_SPEED_MUL: FixedRatio = FixedRatio::from_raw(1_500_000);

impl Engraving {
    pub fn key(&self) -> &'static str {
        match self {
            Engraving::Magnet => "magnet",
            Engraving::Overcharge => "overcharge",
            Engraving::Cactus => "cactus",
            Engraving::SpinningTop => "spinning_top",
        }
    }

    pub fn thumbnail(&self) -> Image {
        match self {
            Engraving::Magnet => crate::asset::image::thumbnail::MAGNET,
            Engraving::Overcharge => crate::asset::image::thumbnail::BATTERY,
            Engraving::Cactus => crate::asset::image::thumbnail::CACTUS,
            Engraving::SpinningTop => crate::asset::image::thumbnail::SPINNING_TOP,
        }
    }

    pub fn tower_modifier(&self) -> TowerEngravingModifier {
        match self {
            Engraving::Magnet => TowerEngravingModifier::NONE,
            Engraving::Overcharge => TowerEngravingModifier {
                shoot_interval_mul: OVERCHARGE_ATTACK_SPEED_MUL.reciprocal(),
                ..TowerEngravingModifier::NONE
            },
            Engraving::Cactus => TowerEngravingModifier {
                on_attack_splashes: vec![EngravingSplash {
                    radius: CACTUS_SPLASH_RADIUS,
                    damage_pct: CACTUS_SPLASH_DAMAGE_PCT,
                }],
                ..TowerEngravingModifier::NONE
            },
            Engraving::SpinningTop => TowerEngravingModifier::NONE,
        }
    }

    pub fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match (self, locale.language) {
            (Engraving::Magnet, crate::l10n::Language::Korean) => "자석",
            (Engraving::Magnet, crate::l10n::Language::English) => "Magnet",
            (Engraving::Overcharge, crate::l10n::Language::Korean) => "과충전",
            (Engraving::Overcharge, crate::l10n::Language::English) => "Overcharge",
            (Engraving::Cactus, crate::l10n::Language::Korean) => "선인장",
            (Engraving::Cactus, crate::l10n::Language::English) => "Cactus",
            (Engraving::SpinningTop, crate::l10n::Language::Korean) => "팽이",
            (Engraving::SpinningTop, crate::l10n::Language::English) => "Spinning Top",
        });
    }

    pub fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        let overcharge_pct = ((OVERCHARGE_ATTACK_SPEED_MUL.as_f32() - 1.0) * 100.0).round();
        match (self, locale.language) {
            (Engraving::Magnet, crate::l10n::Language::Korean) => builder.static_text(
                "이 카드를 뽑으면 뽑을 카드 더미에 있는 자석이 각인된 카드를 모두 손으로 가져옵니다",
            ),
            (Engraving::Magnet, crate::l10n::Language::English) => builder.static_text(
                "When you draw this card, every magnet-engraved card left in the draw pile is pulled into your hand",
            ),
            (Engraving::Overcharge, crate::l10n::Language::Korean) => {
                builder.text(format!("공격속도를 {overcharge_pct}% 증가시킵니다"))
            }
            (Engraving::Overcharge, crate::l10n::Language::English) => {
                builder.text(format!("Increases attack speed by {overcharge_pct}%"))
            }
            (Engraving::Cactus, crate::l10n::Language::Korean) => {
                builder.static_text("공격 시 타워 주변 적들에게 타워 데미지의 30%를 입힙니다")
            }
            (Engraving::Cactus, crate::l10n::Language::English) => {
                builder.static_text("When attacking, deals 30% of tower damage to nearby enemies")
            }
            (Engraving::SpinningTop, crate::l10n::Language::Korean) => {
                builder.static_text("이 카드를 뽑으면 카드를 1장 더 뽑습니다")
            }
            (Engraving::SpinningTop, crate::l10n::Language::English) => {
                builder.static_text("When this card is drawn, draw 1 additional card")
            }
        };
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct TowerEngravingModifier {
    pub attack_range_mul: FixedRatio,
    pub shoot_interval_mul: FixedRatio,
    pub on_hit_splashes: Vec<EngravingSplash>,
    pub on_attack_splashes: Vec<EngravingSplash>,
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct EngravingSplash {
    pub radius: WorldDistance,
    pub damage_pct: FixedRatio,
}

impl TowerEngravingModifier {
    pub const NONE: Self = Self {
        attack_range_mul: FixedRatio::ONE,
        shoot_interval_mul: FixedRatio::ONE,
        on_hit_splashes: Vec::new(),
        on_attack_splashes: Vec::new(),
    };

    pub fn apply_attack_range(&self, base_radius: WorldDistance) -> WorldDistance {
        base_radius.scaled_by(self.attack_range_mul)
    }

    pub fn apply_shoot_interval(&self, base_interval: SimTickSpan) -> SimTickSpan {
        base_interval.scale_ratio_ceil(self.shoot_interval_mul)
    }

    pub fn combine(self, other: Self) -> Self {
        let mut on_hit_splashes = self.on_hit_splashes;
        on_hit_splashes.extend(other.on_hit_splashes);
        let mut on_attack_splashes = self.on_attack_splashes;
        on_attack_splashes.extend(other.on_attack_splashes);

        Self {
            attack_range_mul: FixedRatio::from_raw(
                RatioProduct::one()
                    .with(self.attack_range_mul)
                    .with(other.attack_range_mul)
                    .apply_raw(FixedRatio::ONE.raw()),
            ),
            shoot_interval_mul: FixedRatio::from_raw(
                crate::RatioProduct::one()
                    .with(self.shoot_interval_mul)
                    .with(other.shoot_interval_mul)
                    .apply_raw(FixedRatio::ONE.raw()),
            ),
            on_hit_splashes,
            on_attack_splashes,
        }
    }
}

impl Default for TowerEngravingModifier {
    fn default() -> Self {
        Self::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modifier(
        range: f32,
        interval: FixedRatio,
        splash: Option<EngravingSplash>,
    ) -> TowerEngravingModifier {
        TowerEngravingModifier {
            attack_range_mul: FixedRatio::from_f64(range as f64).unwrap(),
            shoot_interval_mul: interval,
            on_hit_splashes: splash.into_iter().collect(),
            on_attack_splashes: Vec::new(),
        }
    }

    #[test]
    fn none_is_the_identity_of_combine() {
        let target = modifier(
            1.5,
            FixedRatio::from_raw(800_000),
            Some(EngravingSplash {
                radius: WorldDistance::from_tiles(2),
                damage_pct: FixedRatio::from_raw(400_000),
            }),
        );

        assert_eq!(TowerEngravingModifier::NONE.combine(target.clone()), target);
        assert_eq!(target.clone().combine(TowerEngravingModifier::NONE), target);
    }

    #[test]
    fn combine_multiplies_multipliers() {
        let combined = modifier(1.5, FixedRatio::from_raw(500_000), None).combine(modifier(
            2.0,
            FixedRatio::from_raw(500_000),
            None,
        ));

        assert_eq!(combined.attack_range_mul, FixedRatio::from_integer(3));
        assert_eq!(combined.shoot_interval_mul, FixedRatio::from_raw(250_000));
    }

    #[test]
    fn apply_attack_range_scales_by_the_multiplier() {
        assert_eq!(
            modifier(1.5, FixedRatio::ONE, None).apply_attack_range(WorldDistance::from_tiles(8)),
            WorldDistance::from_tiles(12)
        );
        assert_eq!(
            TowerEngravingModifier::NONE.apply_attack_range(WorldDistance::from_tiles(8)),
            WorldDistance::from_tiles(8)
        );
    }

    #[test]
    fn apply_shoot_interval_scales_by_the_multiplier() {
        let base = SimTickSpan::from_millis_ceil(1_000);

        assert_eq!(
            modifier(1.0, FixedRatio::from_raw(500_000), None).apply_shoot_interval(base),
            SimTickSpan::from_millis_ceil(500)
        );
        assert_eq!(
            TowerEngravingModifier::NONE.apply_shoot_interval(base),
            base
        );
    }

    #[test]
    fn combine_preserves_duplicate_splashes() {
        let first = EngravingSplash {
            radius: WorldDistance::from_tiles(3),
            damage_pct: FixedRatio::from_raw(200_000),
        };
        let second = EngravingSplash {
            radius: WorldDistance::from_tiles(1),
            damage_pct: FixedRatio::from_raw(900_000),
        };
        let combined = modifier(1.0, FixedRatio::ONE, Some(first)).combine(modifier(
            1.0,
            FixedRatio::ONE,
            Some(second),
        ));

        assert_eq!(combined.on_hit_splashes, vec![first, second]);
    }
}
