use super::{Language, Locale, LocalizedText, rich_text_helpers::*};
use crate::*;

#[derive(Debug, Clone, State)]
pub enum TowerSkillText {
    NearbyTowerDamageMulTitle,
    NearbyTowerDamageAddTitle,
    NearbyTowerAttackSpeedAddTitle,
    NearbyTowerAttackSpeedMulTitle,
    NearbyTowerAttackRangeAddTitle,
    NearbyMonsterSpeedMulTitle,
    MoneyIncomeAddTitle,
    TopCardBonusTitle,
    NearbyTowerDamageMulDesc { mul: f32, range_radius: usize },
    NearbyTowerDamageAddDesc { add: f32, range_radius: usize },
    NearbyTowerAttackSpeedAddDesc { add: f32, range_radius: usize },
    NearbyTowerAttackSpeedMulDesc { mul: f32, range_radius: usize },
    NearbyTowerAttackRangeAddDesc { add: f32, range_radius: usize },
    NearbyMonsterSpeedMulDesc { mul: f32, range_radius: usize },
    MoneyIncomeAddDesc { add: u32 },
    TopCardBonusDesc { rank: String, bonus_damage: usize },
}

impl LocalizedText for TowerSkillText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl TowerSkillText {
    pub(super) fn to_korean(&self) -> String {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => "주변 타워 공격력 증가".to_string(),
            TowerSkillText::NearbyTowerDamageAddTitle => "주변 타워 공격력 추가".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => {
                "주변 타워 공격 속도 추가".to_string()
            }
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => {
                "주변 타워 공격 속도 증가".to_string()
            }
            TowerSkillText::NearbyTowerAttackRangeAddTitle => {
                "주변 타워 공격 범위 추가".to_string()
            }
            TowerSkillText::NearbyMonsterSpeedMulTitle => "주변 몬스터 속도 감소".to_string(),
            TowerSkillText::MoneyIncomeAddTitle => "돈 수입 증가".to_string(),
            TowerSkillText::TopCardBonusTitle => "탑 카드 보너스".to_string(),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => format!(
                "주변 타워의 {}를 {} 증가시킵니다 (반경 {})",
                attack_damage_icon_small("공격력"),
                percentage_increase(format!("{:.0}", mul * 100.0)),
                range(format!("{range_radius} 타일"))
            ),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => format!(
                "주변 타워의 {}를 {}만큼 증가시킵니다 (반경 {})",
                attack_damage_icon_small("공격력"),
                value_increase(format!("{add:.0}")),
                range(format!("{range_radius} 타일"))
            ),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => format!(
                "주변 타워의 {}를 {} 증가시킵니다 (반경 {})",
                attack_speed_icon_small("공격 속도"),
                percentage_increase(format!("{:.0}", add * 100.0)),
                range(format!("{range_radius} 타일"))
            ),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => format!(
                "주변 타워의 {}를 {}배 증가시킵니다 (반경 {})",
                attack_speed_icon_small("공격 속도"),
                multiplier_value(format!("{mul:.1}")),
                range(format!("{range_radius} 타일"))
            ),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => format!(
                "주변 타워의 {}를 {} 타일 증가시킵니다 (반경 {})",
                attack_range_icon_small("공격 범위"),
                value_increase(format!("{add:.0}")),
                range(format!("{range_radius} 타일"))
            ),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => format!(
                "주변 몬스터의 속도를 {} 감소시킵니다 (반경 {})",
                percentage_decrease(format!("{:.0}", mul * 100.0)),
                range(format!("{range_radius} 타일"))
            ),
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!(
                    "적 처치시 {} 골드를 추가로 획득합니다",
                    gold_icon_small(add)
                )
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!(
                    "탑 카드 보너스: {} ({})",
                    card_rank(rank),
                    attack_damage_icon_small(format!("+{bonus_damage}"))
                )
            }
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => {
                "Nearby Tower Damage Multiplier".to_string()
            }
            TowerSkillText::NearbyTowerDamageAddTitle => "Nearby Tower Damage Addition".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => {
                "Nearby Tower Attack Speed Addition".to_string()
            }
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => {
                "Nearby Tower Attack Speed Multiplier".to_string()
            }
            TowerSkillText::NearbyTowerAttackRangeAddTitle => {
                "Nearby Tower Attack Range Addition".to_string()
            }
            TowerSkillText::NearbyMonsterSpeedMulTitle => {
                "Nearby Monster Speed Multiplier".to_string()
            }
            TowerSkillText::MoneyIncomeAddTitle => "Additional Money Income".to_string(),
            TowerSkillText::TopCardBonusTitle => "Top Card Bonus".to_string(),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => format!(
                "Increases nearby towers' {} by {} (within {} tiles)",
                attack_damage_icon_small("damage"),
                percentage_increase(format!("{:.0}", mul * 100.0)),
                range(format!("{range_radius}"))
            ),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => format!(
                "Increases nearby towers' {} by {} (within {} tiles)",
                attack_damage_icon_small("damage"),
                value_increase(format!("{add:.0}")),
                range(format!("{range_radius}"))
            ),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => format!(
                "Increases nearby towers' {} by {} (within {} tiles)",
                attack_speed_icon_small("attack speed"),
                percentage_increase(format!("{:.0}", add * 100.0)),
                range(format!("{range_radius}"))
            ),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => format!(
                "Increases nearby towers' {} by {} (within {} tiles)",
                attack_speed_icon_small("attack speed"),
                multiplier_value(format!("{mul:.1}")),
                range(format!("{range_radius}"))
            ),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => format!(
                "Increases nearby towers' {} by {} tiles (within {} tiles)",
                attack_range_icon_small("attack range"),
                value_increase(format!("{add:.0}")),
                range(format!("{range_radius}"))
            ),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => format!(
                "Decreases nearby monsters' speed by {} (within {} tiles)",
                percentage_decrease(format!("{:.0}", mul * 100.0)),
                range(format!("{range_radius}"))
            ),
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!(
                    "Gain an additional {} when defeating enemies",
                    gold_icon_small(add)
                )
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!(
                    "Top Card Bonus: {} ({})",
                    card_rank(rank),
                    attack_damage_icon_small(format!("+{bonus_damage}"))
                )
            }
        }
    }
}
