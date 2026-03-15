use super::{Language, Locale, LocalizedText, rich_text_helpers::*};
use crate::theme::typography::TypographyBuilder;
use crate::*;

#[derive(Debug, Clone, State)]
pub enum TowerSkillText {
    NearbyTowerDamageMulTitle,
    NearbyTowerDamageAddTitle,
    NearbyMonsterSpeedMulTitle,
    MoneyIncomeAddTitle,
    TopCardBonusTitle,
    NearbyTowerDamageMulDesc { mul: f32, range_radius: usize },
    NearbyTowerDamageAddDesc { add: f32, range_radius: usize },
    NearbyMonsterSpeedMulDesc { mul: f32, range_radius: usize },
    MoneyIncomeAddDesc { add: u32 },
    TopCardBonusDesc { rank: String, bonus_damage: usize },
}

impl LocalizedText for TowerSkillText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl TowerSkillText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => {
                builder.static_text("주변 타워 공격력 증가");
            }
            TowerSkillText::NearbyTowerDamageAddTitle => {
                builder.static_text("주변 타워 공격력 추가");
            }
            TowerSkillText::NearbyMonsterSpeedMulTitle => {
                builder.static_text("주변 몬스터 속도 감소");
            }
            TowerSkillText::MoneyIncomeAddTitle => {
                builder.static_text("돈 수입 증가");
            }
            TowerSkillText::TopCardBonusTitle => {
                builder.static_text("탑 카드 보너스");
            }
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => {
                builder
                    .static_text("주변 타워의 ")
                    .with_attack_damage_icon("공격력")
                    .static_text("를 ")
                    .with_percentage_increase(format!("{:.0}", mul * 100.0))
                    .static_text(" 증가시킵니다 (반경 ")
                    .with_range(format!("{range_radius} 타일"))
                    .static_text(")");
            }
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => {
                builder
                    .static_text("주변 타워의 ")
                    .with_attack_damage_icon("공격력")
                    .static_text("를 ")
                    .with_value_increase(format!("{add:.0}"))
                    .static_text("만큼 증가시킵니다 (반경 ")
                    .with_range(format!("{range_radius} 타일"))
                    .static_text(")");
            }
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => {
                builder
                    .static_text("주변 몬스터의 속도를 ")
                    .with_percentage_decrease(format!("{:.0}", mul * 100.0))
                    .static_text(" 감소시킵니다 (반경 ")
                    .with_range(format!("{range_radius} 타일"))
                    .static_text(")");
            }
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                builder
                    .static_text("적 처치시 ")
                    .with_gold_icon(format!("{add}"))
                    .static_text(" 골드를 추가로 획득합니다");
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                builder
                    .static_text("탑 카드 보너스: ")
                    .with_card_rank(&rank)
                    .static_text(" (")
                    .with_attack_damage_icon(format!("+{bonus_damage}"))
                    .static_text(")");
            }
        }
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => {
                builder.static_text("Nearby Tower Damage Multiplier");
            }
            TowerSkillText::NearbyTowerDamageAddTitle => {
                builder.static_text("Nearby Tower Damage Addition");
            }
            TowerSkillText::NearbyMonsterSpeedMulTitle => {
                builder.static_text("Nearby Monster Speed Multiplier");
            }
            TowerSkillText::MoneyIncomeAddTitle => {
                builder.static_text("Additional Money Income");
            }
            TowerSkillText::TopCardBonusTitle => {
                builder.static_text("Top Card Bonus");
            }
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => {
                builder
                    .static_text("Increases nearby towers' ")
                    .with_attack_damage_icon("damage")
                    .static_text(" by ")
                    .with_percentage_increase(format!("{:.0}", mul * 100.0))
                    .static_text(" (within ")
                    .with_range(format!("{range_radius}"))
                    .static_text(" tiles)");
            }
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => {
                builder
                    .static_text("Increases nearby towers' ")
                    .with_attack_damage_icon("damage")
                    .static_text(" by ")
                    .with_value_increase(format!("{add:.0}"))
                    .static_text(" (within ")
                    .with_range(format!("{range_radius}"))
                    .static_text(" tiles)");
            }
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => {
                builder
                    .static_text("Decreases nearby monsters' speed by ")
                    .with_percentage_decrease(format!("{:.0}", mul * 100.0))
                    .static_text(" (within ")
                    .with_range(format!("{range_radius}"))
                    .static_text(" tiles)");
            }
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                builder
                    .static_text("Gain an additional ")
                    .with_gold_icon(format!("{add}"))
                    .static_text(" when defeating enemies");
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                builder
                    .static_text("Top Card Bonus: ")
                    .with_card_rank(&rank)
                    .static_text(" (")
                    .with_attack_damage_icon(format!("+{bonus_damage}"))
                    .static_text(")");
            }
        }
    }
}
