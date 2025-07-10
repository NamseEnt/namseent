use super::{Locale, Language, LocalizedText};

#[derive(Debug, Clone)]
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
                "주변 타워의 공격력을 {:.0}% 증가시킵니다 (반경 {} 타일)",
                mul * 100.0,
                range_radius
            ),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => format!(
                "주변 타워의 공격력을 {:.0}만큼 증가시킵니다 (반경 {} 타일)",
                add, range_radius
            ),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => format!(
                "주변 타워의 공격 속도를 {:.0}% 증가시킵니다 (반경 {} 타일)",
                add * 100.0,
                range_radius
            ),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => format!(
                "주변 타워의 공격 속도를 {:.1}배 증가시킵니다 (반경 {} 타일)",
                mul, range_radius
            ),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => format!(
                "주변 타워의 공격 범위를 {:.0} 타일 증가시킵니다 (반경 {} 타일)",
                add, range_radius
            ),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => format!(
                "주변 몬스터의 속도를 {:.0}% 감소시킵니다 (반경 {} 타일)",
                mul * 100.0,
                range_radius
            ),
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!("적 처치시 {add} 골드를 추가로 획득합니다")
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!("탑 카드 보너스: {rank} (공격력 +{bonus_damage})")
            }
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => "Nearby Tower Damage Multiplier".to_string(),
            TowerSkillText::NearbyTowerDamageAddTitle => "Nearby Tower Damage Addition".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => "Nearby Tower Attack Speed Addition".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => "Nearby Tower Attack Speed Multiplier".to_string(),
            TowerSkillText::NearbyTowerAttackRangeAddTitle => "Nearby Tower Attack Range Addition".to_string(),
            TowerSkillText::NearbyMonsterSpeedMulTitle => "Nearby Monster Speed Multiplier".to_string(),
            TowerSkillText::MoneyIncomeAddTitle => "Additional Money Income".to_string(),
            TowerSkillText::TopCardBonusTitle => "Top Card Bonus".to_string(),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => format!(
                "Increases nearby towers' damage by {:.0}% (within {} tiles)",
                mul * 100.0,
                range_radius
            ),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => format!(
                "Increases nearby towers' damage by {:.0} (within {} tiles)",
                add, range_radius
            ),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => format!(
                "Increases nearby towers' attack speed by {:.0}% (within {} tiles)",
                add * 100.0,
                range_radius
            ),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => format!(
                "Increases nearby towers' attack speed by {:.1}x (within {} tiles)",
                mul, range_radius
            ),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => format!(
                "Increases nearby towers' attack range by {:.0} tiles (within {} tiles)",
                add, range_radius
            ),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => format!(
                "Decreases nearby monsters' speed by {:.0}% (within {} tiles)",
                mul * 100.0,
                range_radius
            ),
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!("Gain an additional {add} gold when defeating enemies")
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!("Top Card Bonus: {rank} (Damage +{bonus_damage})")
            }
        }
    }
}
