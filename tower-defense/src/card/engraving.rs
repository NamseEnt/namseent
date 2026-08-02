use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub enum Engraving {
    Magnet,
    Overcharge,
}

const OVERCHARGE_ATTACK_SPEED_MUL: f32 = 1.5;

impl Engraving {
    pub fn key(&self) -> &'static str {
        match self {
            Engraving::Magnet => "magnet",
            Engraving::Overcharge => "overcharge",
        }
    }

    pub fn thumbnail(&self) -> Image {
        match self {
            Engraving::Magnet => crate::asset::image::thumbnail::MAGNET,
            Engraving::Overcharge => crate::asset::image::thumbnail::BATTERY,
        }
    }

    pub fn tower_modifier(&self) -> TowerEngravingModifier {
        match self {
            Engraving::Magnet => TowerEngravingModifier::NONE,
            Engraving::Overcharge => TowerEngravingModifier {
                shoot_interval_mul: 1.0 / OVERCHARGE_ATTACK_SPEED_MUL,
                ..TowerEngravingModifier::NONE
            },
        }
    }

    pub fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match (self, locale.language) {
            (Engraving::Magnet, crate::l10n::Language::Korean) => "자석",
            (Engraving::Magnet, crate::l10n::Language::English) => "Magnet",
            (Engraving::Overcharge, crate::l10n::Language::Korean) => "과충전",
            (Engraving::Overcharge, crate::l10n::Language::English) => "Overcharge",
        });
    }

    pub fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        let overcharge_pct = ((OVERCHARGE_ATTACK_SPEED_MUL - 1.0) * 100.0).round();
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
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct TowerEngravingModifier {
    pub attack_range_mul: f32,
    pub shoot_interval_mul: f32,
    pub splash: Option<EngravingSplash>,
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct EngravingSplash {
    pub radius: f32,
    pub damage_pct: f32,
}

impl TowerEngravingModifier {
    pub const NONE: Self = Self {
        attack_range_mul: 1.0,
        shoot_interval_mul: 1.0,
        splash: None,
    };

    pub fn apply_attack_range(&self, base_radius: f32) -> f32 {
        base_radius * self.attack_range_mul
    }

    pub fn apply_shoot_interval(&self, base_interval: Duration) -> Duration {
        Duration::from_secs_f32(base_interval.as_secs_f32() * self.shoot_interval_mul)
    }

    pub fn combine(self, other: Self) -> Self {
        Self {
            attack_range_mul: self.attack_range_mul * other.attack_range_mul,
            shoot_interval_mul: self.shoot_interval_mul * other.shoot_interval_mul,
            splash: match (self.splash, other.splash) {
                (Some(a), Some(b)) => Some(EngravingSplash {
                    radius: a.radius.max(b.radius),
                    damage_pct: a.damage_pct.max(b.damage_pct),
                }),
                (splash, None) | (None, splash) => splash,
            },
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
        interval: f32,
        splash: Option<EngravingSplash>,
    ) -> TowerEngravingModifier {
        TowerEngravingModifier {
            attack_range_mul: range,
            shoot_interval_mul: interval,
            splash,
        }
    }

    #[test]
    fn none_is_the_identity_of_combine() {
        let target = modifier(
            1.5,
            0.8,
            Some(EngravingSplash {
                radius: 2.0,
                damage_pct: 0.4,
            }),
        );

        assert_eq!(TowerEngravingModifier::NONE.combine(target), target);
        assert_eq!(target.combine(TowerEngravingModifier::NONE), target);
    }

    #[test]
    fn combine_multiplies_multipliers() {
        let combined = modifier(1.5, 0.5, None).combine(modifier(2.0, 0.5, None));

        assert_eq!(combined.attack_range_mul, 3.0);
        assert_eq!(combined.shoot_interval_mul, 0.25);
    }

    #[test]
    fn apply_attack_range_scales_by_the_multiplier() {
        assert_eq!(modifier(1.5, 1.0, None).apply_attack_range(8.0), 12.0);
        assert_eq!(TowerEngravingModifier::NONE.apply_attack_range(8.0), 8.0);
    }

    #[test]
    fn apply_shoot_interval_scales_by_the_multiplier() {
        let base = Duration::from_secs(1);

        assert_eq!(
            modifier(1.0, 0.5, None).apply_shoot_interval(base),
            Duration::from_millis(500)
        );
        assert_eq!(
            TowerEngravingModifier::NONE.apply_shoot_interval(base),
            base
        );
    }

    #[test]
    fn combine_takes_the_larger_splash_of_each_field() {
        let combined = modifier(
            1.0,
            1.0,
            Some(EngravingSplash {
                radius: 3.0,
                damage_pct: 0.2,
            }),
        )
        .combine(modifier(
            1.0,
            1.0,
            Some(EngravingSplash {
                radius: 1.0,
                damage_pct: 0.9,
            }),
        ));

        assert_eq!(
            combined.splash,
            Some(EngravingSplash {
                radius: 3.0,
                damage_pct: 0.9,
            })
        );
    }
}
