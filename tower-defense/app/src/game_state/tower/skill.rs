use super::*;
use crate::{SimTick, SimTickSpan, WorldDistance};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct TowerSkillTemplate {
    pub kind: TowerSkillKind,
    pub cooldown: SimTickSpan,
    pub duration: SimTickSpan,
}
impl TowerSkillTemplate {
    pub fn new_passive(kind: TowerSkillKind) -> Self {
        Self {
            kind,
            cooldown: SimTickSpan::from_millis_ceil(1_000),
            duration: SimTickSpan::from_millis_ceil(1_000),
        }
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct TowerSkill {
    pub last_used_at: SimTick,
    pub template: TowerSkillTemplate,
}

impl TowerSkill {
    pub fn new(template: TowerSkillTemplate, sim_tick: SimTick) -> Self {
        Self {
            last_used_at: sim_tick,
            template,
        }
    }
}

impl Deref for TowerSkill {
    type Target = TowerSkillTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

impl TowerSkill {
    pub(crate) fn to_core_skill(&self) -> td_core::TowerSkill {
        td_core::TowerSkill {
            last_used_at: self.last_used_at.ticks(),
            template: td_core::TowerSkillTemplate {
                kind: self.template.kind.to_core_kind(),
                cooldown: self.template.cooldown.ticks(),
                duration: self.template.duration.ticks(),
            },
        }
    }

    pub(crate) fn from_core_skill(skill: td_core::TowerSkill) -> Self {
        Self {
            last_used_at: SimTick::from_ticks(skill.last_used_at),
            template: TowerSkillTemplate {
                kind: TowerSkillKind::from_core_kind(skill.template.kind),
                cooldown: SimTickSpan::from_ticks(skill.template.cooldown),
                duration: SimTickSpan::from_ticks(skill.template.duration),
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, State)]
pub enum TowerSkillKind {
    NearbyTowerDamageMul {
        mul: FixedRatio,
        range_radius: WorldDistance,
    },
    NearbyTowerDamageAdd {
        add: DamageDelta,
        range_radius: WorldDistance,
    },
    NearbyMonsterSpeedMul {
        mul: FixedRatio,
        range_radius: WorldDistance,
    },
    MoneyIncomeAdd {
        add: u32,
    },
    TopCardBonus {
        rank: Rank,
        bonus_damage: usize,
    },
}

impl TowerSkillKind {
    fn to_core_kind(self) -> td_core::TowerSkillKind {
        match self {
            Self::NearbyTowerDamageMul { mul, range_radius } => {
                td_core::TowerSkillKind::NearbyTowerDamageMul {
                    mul_raw: mul.raw(),
                    range_radius_raw: range_radius.raw(),
                }
            }
            Self::NearbyTowerDamageAdd { add, range_radius } => {
                td_core::TowerSkillKind::NearbyTowerDamageAdd {
                    add_raw: add.raw(),
                    range_radius_raw: range_radius.raw(),
                }
            }
            Self::NearbyMonsterSpeedMul { mul, range_radius } => {
                td_core::TowerSkillKind::NearbyMonsterSpeedMul {
                    mul_raw: mul.raw(),
                    range_radius_raw: range_radius.raw(),
                }
            }
            Self::MoneyIncomeAdd { add } => td_core::TowerSkillKind::MoneyIncomeAdd { add },
            Self::TopCardBonus { rank, bonus_damage } => td_core::TowerSkillKind::TopCardBonus {
                rank: rank as u8,
                bonus_damage,
            },
        }
    }

    fn from_core_kind(kind: td_core::TowerSkillKind) -> Self {
        match kind {
            td_core::TowerSkillKind::NearbyTowerDamageMul {
                mul_raw,
                range_radius_raw,
            } => Self::NearbyTowerDamageMul {
                mul: FixedRatio::from_raw(mul_raw),
                range_radius: WorldDistance::from_raw(range_radius_raw),
            },
            td_core::TowerSkillKind::NearbyTowerDamageAdd {
                add_raw,
                range_radius_raw,
            } => Self::NearbyTowerDamageAdd {
                add: DamageDelta::from_raw(add_raw),
                range_radius: WorldDistance::from_raw(range_radius_raw),
            },
            td_core::TowerSkillKind::NearbyMonsterSpeedMul {
                mul_raw,
                range_radius_raw,
            } => Self::NearbyMonsterSpeedMul {
                mul: FixedRatio::from_raw(mul_raw),
                range_radius: WorldDistance::from_raw(range_radius_raw),
            },
            td_core::TowerSkillKind::MoneyIncomeAdd { add } => Self::MoneyIncomeAdd { add },
            td_core::TowerSkillKind::TopCardBonus { rank, bonus_damage } => Self::TopCardBonus {
                rank: rank_from_raw(rank),
                bonus_damage,
            },
        }
    }
}

fn rank_from_raw(value: u8) -> Rank {
    match value {
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::Jack,
        10 => Rank::Queen,
        11 => Rank::King,
        12 => Rank::Ace,
        _ => panic!("invalid tower skill rank raw value: {value}"),
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct TowerStatusEffect {
    pub kind: TowerStatusEffectKind,
    pub end_at: TowerStatusEffectEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, State)]
pub enum TowerStatusEffectKind {
    DamageMul { mul: FixedRatio },
    DamageAdd { add: DamageDelta },
}

impl TowerStatusEffectKind {
    pub fn affects_damage(&self) -> bool {
        matches!(
            self,
            TowerStatusEffectKind::DamageMul { .. } | TowerStatusEffectKind::DamageAdd { .. }
        )
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub enum TowerStatusEffectEnd {
    Time { end_at: SimTick },
    NeverEnd,
}

impl TowerStatusEffect {
    pub(crate) fn to_core_status_effect(&self) -> td_core::TowerStatusEffect {
        td_core::TowerStatusEffect {
            kind: match self.kind {
                TowerStatusEffectKind::DamageMul { mul } => {
                    td_core::TowerStatusEffectKind::DamageMul { mul_raw: mul.raw() }
                }
                TowerStatusEffectKind::DamageAdd { add } => {
                    td_core::TowerStatusEffectKind::DamageAdd { add_raw: add.raw() }
                }
            },
            end: match self.end_at {
                TowerStatusEffectEnd::Time { end_at } => td_core::TowerStatusEffectEnd::Time {
                    end_at: end_at.ticks(),
                },
                TowerStatusEffectEnd::NeverEnd => td_core::TowerStatusEffectEnd::NeverEnd,
            },
        }
    }

    pub(crate) fn from_core_status_effect(effect: td_core::TowerStatusEffect) -> Self {
        Self {
            kind: match effect.kind {
                td_core::TowerStatusEffectKind::DamageMul { mul_raw } => {
                    TowerStatusEffectKind::DamageMul {
                        mul: FixedRatio::from_raw(mul_raw),
                    }
                }
                td_core::TowerStatusEffectKind::DamageAdd { add_raw } => {
                    TowerStatusEffectKind::DamageAdd {
                        add: DamageDelta::from_raw(add_raw),
                    }
                }
            },
            end_at: match effect.end {
                td_core::TowerStatusEffectEnd::Time { end_at } => TowerStatusEffectEnd::Time {
                    end_at: SimTick::from_ticks(end_at),
                },
                td_core::TowerStatusEffectEnd::NeverEnd => TowerStatusEffectEnd::NeverEnd,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_status_effect_snapshot_round_trips_without_host_types() {
        let effects = [
            TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageMul {
                    mul: FixedRatio::from_raw(800_000),
                },
                end_at: TowerStatusEffectEnd::Time {
                    end_at: SimTick::from_ticks(18),
                },
            },
            TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageAdd {
                    add: DamageDelta::from_integer(15),
                },
                end_at: TowerStatusEffectEnd::NeverEnd,
            },
        ];

        let restored = effects
            .iter()
            .map(TowerStatusEffect::to_core_status_effect)
            .map(TowerStatusEffect::from_core_status_effect)
            .collect::<Vec<_>>();

        assert!(matches!(
            restored[0].kind,
            TowerStatusEffectKind::DamageMul { mul }
                if mul == FixedRatio::from_raw(800_000)
        ));
        assert!(matches!(
            restored[0].end_at,
            TowerStatusEffectEnd::Time { end_at }
                if end_at == SimTick::from_ticks(18)
        ));
        assert!(matches!(
            restored[1].kind,
            TowerStatusEffectKind::DamageAdd { add }
                if add == DamageDelta::from_integer(15)
        ));
        assert!(matches!(restored[1].end_at, TowerStatusEffectEnd::NeverEnd));
    }

    #[test]
    fn tower_skill_snapshot_round_trips_without_host_types() {
        let skills = [
            TowerSkill::new(
                TowerSkillTemplate {
                    kind: TowerSkillKind::NearbyTowerDamageMul {
                        mul: FixedRatio::from_raw(800_000),
                        range_radius: WorldDistance::from_raw(2_500),
                    },
                    cooldown: SimTickSpan::from_ticks(30),
                    duration: SimTickSpan::from_ticks(90),
                },
                SimTick::from_ticks(12),
            ),
            TowerSkill::new(
                TowerSkillTemplate {
                    kind: TowerSkillKind::TopCardBonus {
                        rank: Rank::Queen,
                        bonus_damage: 15,
                    },
                    cooldown: SimTickSpan::from_ticks(60),
                    duration: SimTickSpan::from_ticks(120),
                },
                SimTick::from_ticks(24),
            ),
        ];

        let restored = skills
            .iter()
            .map(TowerSkill::to_core_skill)
            .map(TowerSkill::from_core_skill)
            .collect::<Vec<_>>();

        assert_eq!(restored[0].last_used_at, SimTick::from_ticks(12));
        assert_eq!(restored[0].cooldown, SimTickSpan::from_ticks(30));
        assert!(matches!(
            restored[0].kind,
            TowerSkillKind::NearbyTowerDamageMul {
                mul,
                range_radius,
            } if mul == FixedRatio::from_raw(800_000)
                && range_radius == WorldDistance::from_raw(2_500)
        ));
        assert!(matches!(
            restored[1].kind,
            TowerSkillKind::TopCardBonus {
                rank: Rank::Queen,
                bonus_damage: 15,
            }
        ));
    }
}
