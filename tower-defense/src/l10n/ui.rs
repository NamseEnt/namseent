use super::{Language, Locale, LocalizedStaticText};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum StartConfirmModalText {
    Title,
    Message,
    No,
    Yes,
}

impl LocalizedStaticText for TopBarText {
    fn localized_text(&self, locale: &Locale) -> &'static str {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
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

impl StartConfirmModalText {
    pub(super) fn to_korean(self) -> &'static str {
        match self {
            StartConfirmModalText::Title => "확인",
            StartConfirmModalText::Message => "아직 사용할 수 있는 타워가 남았습니다.\n그래도 정말 시작하시겠습니까?",
            StartConfirmModalText::No => "아니오",
            StartConfirmModalText::Yes => "예",
        }
    }

    pub(super) fn to_english(self) -> &'static str {
        match self {
            StartConfirmModalText::Title => "Confirm",
            StartConfirmModalText::Message => "You still have towers available.\nAre you sure you want to start?",
            StartConfirmModalText::No => "No",
            StartConfirmModalText::Yes => "Yes",
        }
    }
}
