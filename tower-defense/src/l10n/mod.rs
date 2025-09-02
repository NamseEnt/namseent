// 통합 l10n 모듈 - 모든 다국어 기능의 중앙 진입점

pub mod api;
pub mod contract;
pub mod effect;
pub mod item;
pub mod locale;
pub mod quest;
pub mod rich_text_helpers;
pub mod tower;
pub mod tower_skill;
pub mod ui;
pub mod upgrade;
pub mod upgrade_board;

// 핵심 타입들 재export
pub use locale::{Language, Locale, LocalizedStaticText, LocalizedText};

// 현대적 API (권장)
pub use api::{ENGLISH_TEXT, KOREAN_TEXT, TextManager};

// 레거시 호환성 타입들 (점진적 마이그레이션용)
pub use api::{LegacyKoKRLocale as KoKRLocale, LegacyLocales as Locales};

// 편의 함수들
/// 기본 텍스트 매니저 (한국어)
pub fn text() -> TextManager {
    TextManager::korean()
}

/// 한국어 텍스트 매니저
pub fn korean_text() -> TextManager {
    TextManager::korean()
}

/// 영어 텍스트 매니저
pub fn english_text() -> TextManager {
    TextManager::english()
}
