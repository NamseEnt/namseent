use super::{Language, Locale, LocalizedText, rich_text_helpers::RichTextHelpers};
use crate::theme::typography::TypographyBuilder;
use crate::*;

#[derive(Debug, Clone, Copy, State)]
pub enum TopBarText {
    Run,
    Quest,
    Refresh,
    Accepted,
    Use,
    Settings,
    Shop,
    UseTower,
}

#[derive(Debug, Clone, Copy, State)]
pub enum ResultModalText {
    Title,
    RestartButton,
    MaxPerfectClearLabel,
    TotalGoldLabel,
    TotalDamageLabel,
    RerollCountLabel,
    NoTowerDamage,
}

impl LocalizedText for TopBarText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl TopBarText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            TopBarText::Run => "런",
            TopBarText::Quest => "퀘스트",
            TopBarText::Refresh => "새로고침",
            TopBarText::Accepted => "수락됨",
            TopBarText::Use => "사용",
            TopBarText::Settings => "설정",
            TopBarText::Shop => "상점",
            TopBarText::UseTower => "타워 사용",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            TopBarText::Run => "Run",
            TopBarText::Quest => "Quest",
            TopBarText::Refresh => "Refresh",
            TopBarText::Accepted => "Accepted",
            TopBarText::Use => "Use",
            TopBarText::Settings => "Settings",
            TopBarText::Shop => "Shop",
            TopBarText::UseTower => "Use Tower",
        }
    }
}

impl LocalizedText for ResultModalText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl ResultModalText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            ResultModalText::Title => "게임 결과",
            ResultModalText::RestartButton => "다시하기",
            ResultModalText::MaxPerfectClearLabel => "최대 연속 퍼펙트 클리어",
            ResultModalText::TotalGoldLabel => "총 획득 골드",
            ResultModalText::TotalDamageLabel => "총 데미지",
            ResultModalText::RerollCountLabel => "리롤 횟수",
            ResultModalText::NoTowerDamage => "타워 기여 기록이 없습니다",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            ResultModalText::Title => "Game Result",
            ResultModalText::RestartButton => "Restart",
            ResultModalText::MaxPerfectClearLabel => "Max Perfect Streak",
            ResultModalText::TotalGoldLabel => "Total Gold Earned",
            ResultModalText::TotalDamageLabel => "Total Damage",
            ResultModalText::RerollCountLabel => "Reroll Count",
            ResultModalText::NoTowerDamage => "No tower contribution recorded",
        }
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum RerollHealthCostDetailText {
    Damage(usize),
}

impl LocalizedText for RerollHealthCostDetailText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl RerollHealthCostDetailText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            RerollHealthCostDetailText::Damage(amount) => builder
                .text("체력을 ")
                .with_health_loss(format!("{}", amount))
                .text(" 잃습니다"),
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            RerollHealthCostDetailText::Damage(amount) => builder
                .text("Lose ")
                .with_health_loss(format!("{}", amount))
                .text(" health"),
        };
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum SettingsText {
    MasterVolume,
    EffectsVolume,
    UiVolume,
    AmbientVolume,
    MusicVolume,
}

impl LocalizedText for SettingsText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl SettingsText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            SettingsText::MasterVolume => "전체 볼륨",
            SettingsText::EffectsVolume => "효과음",
            SettingsText::UiVolume => "UI",
            SettingsText::AmbientVolume => "환경음",
            SettingsText::MusicVolume => "음악",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            SettingsText::MasterVolume => "Master",
            SettingsText::EffectsVolume => "Effects",
            SettingsText::UiVolume => "UI",
            SettingsText::AmbientVolume => "Ambient",
            SettingsText::MusicVolume => "Music",
        }
    }
}

#[derive(Debug, Clone, Copy, State)]
pub enum OperationPlanText {
    Title,
    SelectDifficulty,
}

impl LocalizedText for OperationPlanText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => {
                builder.static_text(self.to_korean());
            }
            Language::English => {
                builder.static_text(self.to_english());
            }
        }
    }
}

impl OperationPlanText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            OperationPlanText::Title => "작전 계획",
            OperationPlanText::SelectDifficulty => "난이도를 선택하세요",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            OperationPlanText::Title => "Operation Plan",
            OperationPlanText::SelectDifficulty => "Select Difficulty",
        }
    }
}
