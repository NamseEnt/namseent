use super::Locale;
use crate::game_state::monster::skill::MonsterSkillKind;

#[derive(Clone)]
pub enum MonsterSkillText {
    Description(MonsterSkillKind),
}

impl crate::l10n::LocalizedText for MonsterSkillText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            crate::l10n::Language::Korean => self.to_korean(),
            crate::l10n::Language::English => self.to_english(),
        }
    }
}

impl MonsterSkillText {
    pub fn to_korean(&self) -> String {
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

    pub fn to_english(&self) -> String {
        match self {
            MonsterSkillText::Description(skill) => match skill {
                MonsterSkillKind::Invincible => "Becomes invincible".to_string(),
                MonsterSkillKind::SpeedMul { mul } => format!("Movement speed becomes {}x", mul),
                MonsterSkillKind::ImmuneToSlow => "Immune to slow effects".to_string(),
                MonsterSkillKind::HealByMaxHp { ratio } => {
                    format!("Heals {}x of max HP", ratio)
                }
            },
        }
    }
}
