use super::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;
use crate::*;

#[derive(Debug, Clone, Copy, State)]
pub enum TopBarText {
    Stage,
    Quest,
    Refresh,
    Accepted,
    Use,
    Settings,
    Shop,
    SoldOut,
    UseTower,
}

#[derive(Debug, Clone, Copy, State)]
pub enum ResultModalText {
    Title,
    RestartButton,
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
            TopBarText::Stage => "스테이지",
            TopBarText::Quest => "퀘스트",
            TopBarText::Refresh => "새로고침",
            TopBarText::Accepted => "수락됨",
            TopBarText::Use => "사용",
            TopBarText::Settings => "설정",
            TopBarText::Shop => "상점",
            TopBarText::SoldOut => "품절",
            TopBarText::UseTower => "타워 사용",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            TopBarText::Stage => "Stage",
            TopBarText::Quest => "Quest",
            TopBarText::Refresh => "Refresh",
            TopBarText::Accepted => "Accepted",
            TopBarText::Use => "Use",
            TopBarText::Settings => "Settings",
            TopBarText::Shop => "Shop",
            TopBarText::SoldOut => "Sold Out",
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
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            ResultModalText::Title => "Game Result",
            ResultModalText::RestartButton => "Restart",
        }
    }
}
