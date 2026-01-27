use super::{Language, Locale, LocalizedRichText, rich_text_helpers::*};
use crate::theme::typography::TypographyBuilder;
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



impl LocalizedRichText for TowerSkillText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl TowerSkillText {
    fn apply_korean<'a>(self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => {
                builder.static_text("주변 타워 공격력 증가")
            }
            TowerSkillText::NearbyTowerDamageAddTitle => {
                builder.static_text("주변 타워 공격력 추가")
            }
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => {
                builder.static_text("주변 타워 공격 속도 추가")
            }
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => {
                builder.static_text("주변 타워 공격 속도 증가")
            }
            TowerSkillText::NearbyTowerAttackRangeAddTitle => {
                builder.static_text("주변 타워 공격 범위 추가")
            }
            TowerSkillText::NearbyMonsterSpeedMulTitle => {
                builder.static_text("주변 몬스터 속도 감소")
            }
            TowerSkillText::MoneyIncomeAddTitle => builder.static_text("돈 수입 증가"),
            TowerSkillText::TopCardBonusTitle => builder.static_text("탑 카드 보너스"),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => builder
                .static_text("주변 타워의 ")
                .with_attack_damage_icon("공격력")
                .static_text("를 ")
                .with_percentage_increase(format!("{:.0}", mul * 100.0))
                .static_text(" 증가시킵니다 (반경 ")
                .with_range(format!("{range_radius} 타일"))
                .static_text(")"),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => builder
                .static_text("주변 타워의 ")
                .with_attack_damage_icon("공격력")
                .static_text("를 ")
                .with_value_increase(format!("{add:.0}"))
                .static_text("만큼 증가시킵니다 (반경 ")
                .with_range(format!("{range_radius} 타일"))
                .static_text(")"),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => builder
                .static_text("주변 타워의 ")
                .with_attack_speed_icon("공격 속도")
                .static_text("를 ")
                .with_percentage_increase(format!("{:.0}", add * 100.0))
                .static_text(" 증가시킵니다 (반경 ")
                .with_range(format!("{range_radius} 타일"))
                .static_text(")"),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => builder
                .static_text("주변 타워의 ")
                .with_attack_speed_icon("공격 속도")
                .static_text("를 ")
                .with_multiplier(format!("{mul:.1}"))
                .static_text("배 증가시킵니다 (반경 ")
                .with_range(format!("{range_radius} 타일"))
                .static_text(")"),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => builder
                .static_text("주변 타워의 ")
                .with_attack_range_icon("공격 범위")
                .static_text("를 ")
                .with_value_increase(format!("{add:.0}"))
                .static_text(" 타일 증가시킵니다 (반경 ")
                .with_range(format!("{range_radius} 타일"))
                .static_text(")"),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => builder
                .static_text("주변 몬스터의 속도를 ")
                .with_percentage_decrease(format!("{:.0}", mul * 100.0))
                .static_text(" 감소시킵니다 (반경 ")
                .with_range(format!("{range_radius} 타일"))
                .static_text(")"),
            TowerSkillText::MoneyIncomeAddDesc { add } => builder
                .static_text("적 처치시 ")
                .with_gold_icon(format!("{add}"))
                .static_text(" 골드를 추가로 획득합니다"),
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => builder
                .static_text("탑 카드 보너스: ")
                .with_card_rank(&rank)
                .static_text(" (")
                .with_attack_damage_icon(format!("+{bonus_damage}"))
                .static_text(")"),
        }
    }

    fn apply_english<'a>(self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => {
                builder.static_text("Nearby Tower Damage Multiplier")
            }
            TowerSkillText::NearbyTowerDamageAddTitle => {
                builder.static_text("Nearby Tower Damage Addition")
            }
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => {
                builder.static_text("Nearby Tower Attack Speed Addition")
            }
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => {
                builder.static_text("Nearby Tower Attack Speed Multiplier")
            }
            TowerSkillText::NearbyTowerAttackRangeAddTitle => {
                builder.static_text("Nearby Tower Attack Range Addition")
            }
            TowerSkillText::NearbyMonsterSpeedMulTitle => {
                builder.static_text("Nearby Monster Speed Multiplier")
            }
            TowerSkillText::MoneyIncomeAddTitle => builder.static_text("Additional Money Income"),
            TowerSkillText::TopCardBonusTitle => builder.static_text("Top Card Bonus"),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => builder
                .static_text("Increases nearby towers' ")
                .with_attack_damage_icon("damage")
                .static_text(" by ")
                .with_percentage_increase(format!("{:.0}", mul * 100.0))
                .static_text(" (within ")
                .with_range(format!("{range_radius}"))
                .static_text(" tiles)"),
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => builder
                .static_text("Increases nearby towers' ")
                .with_attack_damage_icon("damage")
                .static_text(" by ")
                .with_value_increase(format!("{add:.0}"))
                .static_text(" (within ")
                .with_range(format!("{range_radius}"))
                .static_text(" tiles)"),
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => builder
                .static_text("Increases nearby towers' ")
                .with_attack_speed_icon("attack speed")
                .static_text(" by ")
                .with_percentage_increase(format!("{:.0}", add * 100.0))
                .static_text(" (within ")
                .with_range(format!("{range_radius}"))
                .static_text(" tiles)"),
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => builder
                .static_text("Increases nearby towers' ")
                .with_attack_speed_icon("attack speed")
                .static_text(" by ")
                .with_multiplier(format!("{mul:.1}"))
                .static_text(" (within ")
                .with_range(format!("{range_radius}"))
                .static_text(" tiles)"),
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => builder
                .static_text("Increases nearby towers' ")
                .with_attack_range_icon("attack range")
                .static_text(" by ")
                .with_value_increase(format!("{add:.0}"))
                .static_text(" tiles (within ")
                .with_range(format!("{range_radius}"))
                .static_text(" tiles)"),
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => builder
                .static_text("Decreases nearby monsters' speed by ")
                .with_percentage_decrease(format!("{:.0}", mul * 100.0))
                .static_text(" (within ")
                .with_range(format!("{range_radius}"))
                .static_text(" tiles)"),
            TowerSkillText::MoneyIncomeAddDesc { add } => builder
                .static_text("Gain an additional ")
                .with_gold_icon(format!("{add}"))
                .static_text(" when defeating enemies"),
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => builder
                .static_text("Top Card Bonus: ")
                .with_card_rank(&rank)
                .static_text(" (")
                .with_attack_damage_icon(format!("+{bonus_damage}"))
                .static_text(")"),
        }
    }
}

impl TowerSkillText {
    pub fn text_korean(self) -> String {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => "주변 타워 공격력 증가".to_string(),
            TowerSkillText::NearbyTowerDamageAddTitle => "주변 타워 공격력 추가".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => "주변 타워 공격 속도 추가".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => "주변 타워 공격 속도 증가".to_string(),
            TowerSkillText::NearbyTowerAttackRangeAddTitle => "주변 타워 공격 범위 추가".to_string(),
            TowerSkillText::NearbyMonsterSpeedMulTitle => "주변 몬스터 속도 감소".to_string(),
            TowerSkillText::MoneyIncomeAddTitle => "돈 수입 증가".to_string(),
            TowerSkillText::TopCardBonusTitle => "탑 카드 보너스".to_string(),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => {
                format!("주변 타워의 공격력를 {:.0}% 증가시킵니다 (반경 {} 타일)", mul * 100.0, range_radius)
            }
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => {
                format!("주변 타워의 공격력를 {:.0}만큼 증가시킵니다 (반경 {} 타일)", add, range_radius)
            }
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => {
                format!("주변 타워의 공격 속도를 {:.0}% 증가시킵니다 (반경 {} 타일)", add * 100.0, range_radius)
            }
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => {
                format!("주변 타워의 공격 속도를 {:.1}배 증가시킵니다 (반경 {} 타일)", mul, range_radius)
            }
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => {
                format!("주변 타워의 공격 범위를 {:.0} 타일 증가시킵니다 (반경 {} 타일)", add, range_radius)
            }
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => {
                format!("주변 몬스터의 속도를 {:.0}% 감소시킵니다 (반경 {} 타일)", mul * 100.0, range_radius)
            }
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!("적 처치시 {} 골드를 추가로 획득합니다", add)
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!("탑 카드 보너스: {} (+{})", rank, bonus_damage)
            }
        }
    }

    pub fn text_english(self) -> String {
        match self {
            TowerSkillText::NearbyTowerDamageMulTitle => "Nearby Tower Damage Multiplier".to_string(),
            TowerSkillText::NearbyTowerDamageAddTitle => "Nearby Tower Damage Addition".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedAddTitle => "Nearby Tower Attack Speed Addition".to_string(),
            TowerSkillText::NearbyTowerAttackSpeedMulTitle => "Nearby Tower Attack Speed Multiplier".to_string(),
            TowerSkillText::NearbyTowerAttackRangeAddTitle => "Nearby Tower Attack Range Addition".to_string(),
            TowerSkillText::NearbyMonsterSpeedMulTitle => "Nearby Monster Speed Multiplier".to_string(),
            TowerSkillText::MoneyIncomeAddTitle => "Additional Money Income".to_string(),
            TowerSkillText::TopCardBonusTitle => "Top Card Bonus".to_string(),
            TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius } => {
                format!("Increases nearby towers' damage by {:.0}% (within {} tiles)", mul * 100.0, range_radius)
            }
            TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius } => {
                format!("Increases nearby towers' damage by {:.0} (within {} tiles)", add, range_radius)
            }
            TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius } => {
                format!("Increases nearby towers' attack speed by {:.0}% (within {} tiles)", add * 100.0, range_radius)
            }
            TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius } => {
                format!("Increases nearby towers' attack speed by {:.1} (within {} tiles)", mul, range_radius)
            }
            TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius } => {
                format!("Increases nearby towers' attack range by {:.0} tiles (within {} tiles)", add, range_radius)
            }
            TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius } => {
                format!("Decreases nearby monsters' speed by {:.0}% (within {} tiles)", mul * 100.0, range_radius)
            }
            TowerSkillText::MoneyIncomeAddDesc { add } => {
                format!("Gain an additional {} when defeating enemies", add)
            }
            TowerSkillText::TopCardBonusDesc { rank, bonus_damage } => {
                format!("Top Card Bonus: {} (+{})", rank, bonus_damage)
            }
        }
    }
}
