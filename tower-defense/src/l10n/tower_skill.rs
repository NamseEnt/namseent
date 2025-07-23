use super::{Language, Locale, LocalizedText};
use crate::icon::{Icon, IconKind};

// --- Rich text 헬퍼 함수 (tower_skill 전용) ---
fn attack_damage_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackDamage).size(crate::icon::IconSize::Small);
    format!(
        "|attack_damage_color|{}{value}|/attack_damage_color|",
        icon.as_tag()
    )
}
fn attack_speed_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackSpeed).size(crate::icon::IconSize::Small);
    format!(
        "|attack_speed_color|{}{value}|/attack_speed_color|",
        icon.as_tag()
    )
}
fn attack_range_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackRange).size(crate::icon::IconSize::Small);
    format!(
        "|attack_range_color|{}{value}|/attack_range_color|",
        icon.as_tag()
    )
}
fn gold_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::Gold).size(crate::icon::IconSize::Small);
    format!("|gold_color|{}{value}|/gold_color|", icon.as_tag())
}
fn blue<T: std::fmt::Display>(value: T) -> String {
    format!("|blue|{value}|/blue|")
}

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
                "주변 타워의 {}를 |green|+{:.0}%|/green| 증가시킵니다 (반경 {} 타일)",
                attack_damage_icon("공격력"),
                mul * 100.0,
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => format!(
                "주변 타워의 {}를 |green|+{add:.0}|/green|만큼 증가시킵니다 (반경 {} 타일)",
                attack_damage_icon("공격력"),
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => format!(
                "주변 타워의 {}를 |green|+{:.0}%|/green| 증가시킵니다 (반경 {} 타일)",
                attack_speed_icon("공격 속도"),
                add * 100.0,
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => format!(
                "주변 타워의 {}를 |green|×{mul:.1}|/green|배 증가시킵니다 (반경 {} 타일)",
                attack_speed_icon("공격 속도"),
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => format!(
                "주변 타워의 {}를 |green|+{add:.0}|/green| 타일 증가시킵니다 (반경 {} 타일)",
                attack_range_icon("공격 범위"),
                blue(*range_radius)
            ),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => format!(
                "주변 몬스터의 속도를 |red|-{:.0}%|/red| 감소시킵니다 (반경 {} 타일)",
                mul * 100.0,
                blue(*range_radius)
            ),
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!("적 처치시 {} 골드를 추가로 획득합니다", gold_icon(add))
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!(
                    "탑 카드 보너스: |purple|{rank}|/purple| ({})",
                    attack_damage_icon(format!("+{bonus_damage}"))
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
                "Increases nearby towers' {} by |green|+{:.0}%|/green| (within {} tiles)",
                attack_damage_icon("damage"),
                mul * 100.0,
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => format!(
                "Increases nearby towers' {} by |green|+{add:.0}|/green| (within {} tiles)",
                attack_damage_icon("damage"),
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => format!(
                "Increases nearby towers' {} by |green|+{:.0}%|/green| (within {} tiles)",
                attack_speed_icon("attack speed"),
                add * 100.0,
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => format!(
                "Increases nearby towers' {} by |green|×{mul:.1}|/green| (within {} tiles)",
                attack_speed_icon("attack speed"),
                blue(*range_radius)
            ),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => format!(
                "Increases nearby towers' {} by |green|+{add:.0}|/green| tiles (within {} tiles)",
                attack_range_icon("attack range"),
                blue(*range_radius)
            ),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => format!(
                "Decreases nearby monsters' speed by |red|-{:.0}%|/red| (within {} tiles)",
                mul * 100.0,
                blue(*range_radius)
            ),
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!(
                    "Gain an additional {} when defeating enemies",
                    gold_icon(add)
                )
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!(
                    "Top Card Bonus: |purple|{rank}|/purple| ({})",
                    attack_damage_icon(format!("+{bonus_damage}"))
                )
            }
        }
    }
}
