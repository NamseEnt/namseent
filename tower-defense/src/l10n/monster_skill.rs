use super::{Locale, LocalizedText};
use crate::game_state::monster::skill::MonsterSkillKind;
use crate::theme::typography::TypographyBuilder;

#[derive(Clone)]
pub enum MonsterSkillText {
    Description(MonsterSkillKind),
}



impl LocalizedText for MonsterSkillText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match locale.language {
            crate::l10n::Language::Korean => self.apply_korean(builder),
            crate::l10n::Language::English => self.apply_english(builder),
        }
    }
}

impl MonsterSkillText {
    pub fn text_korean(self) -> String {
        match self {
            MonsterSkillText::Description(skill) => match skill {
                MonsterSkillKind::Invincible => "무적 상태가 됩니다".to_string(),
                MonsterSkillKind::SpeedMul { mul } => format!("이동 속도가 {}배가 됩니다", mul),
                MonsterSkillKind::ImmuneToSlow => "둔화 효과에 면역이 됩니다".to_string(),
                MonsterSkillKind::HealByMaxHp { ratio } => {
                    format!("최대 체력의 {}배를 회복합니다", ratio)
                }
            },
        }
    }

    pub fn text_english(self) -> String {
        match self {
            MonsterSkillText::Description(skill) => match skill {
                MonsterSkillKind::Invincible => "Becomes invincible".to_string(),
                MonsterSkillKind::SpeedMul { mul } => {
                    format!("Movement speed becomes {}x", mul)
                }
                MonsterSkillKind::ImmuneToSlow => "Immune to slow effects".to_string(),
                MonsterSkillKind::HealByMaxHp { ratio } => {
                    format!("Heals {}x of max HP", ratio)
                }
            },
        }
    }

    fn apply_korean<'a>(self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            MonsterSkillText::Description(skill) => match skill {
                MonsterSkillKind::Invincible => builder.static_text("무적 상태가 됩니다"),
                MonsterSkillKind::SpeedMul { mul } => builder
                    .static_text("이동 속도가 ")
                    .text(format!("{}", mul))
                    .static_text("배가 됩니다"),
                MonsterSkillKind::ImmuneToSlow => builder.static_text("둔화 효과에 면역이 됩니다"),
                MonsterSkillKind::HealByMaxHp { ratio } => builder
                    .static_text("최대 체력의 ")
                    .text(format!("{}", ratio))
                    .static_text("배를 회복합니다"),
            },
        }
    }

    fn apply_english<'a>(self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            MonsterSkillText::Description(skill) => match skill {
                MonsterSkillKind::Invincible => builder.static_text("Becomes invincible"),
                MonsterSkillKind::SpeedMul { mul } => builder
                    .static_text("Movement speed becomes ")
                    .text(format!("{}x", mul)),
                MonsterSkillKind::ImmuneToSlow => builder.static_text("Immune to slow effects"),
                MonsterSkillKind::HealByMaxHp { ratio } => builder
                    .static_text("Heals ")
                    .text(format!("{}x", ratio))
                    .static_text(" of max HP"),
            },
        }
    }
}
