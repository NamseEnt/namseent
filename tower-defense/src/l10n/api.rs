// 현대적 l10n API - 간결하고 효율적인 다국어 텍스트 관리

use super::{
    Language, Locale, LocalizedText, contract, effect, event, quest, tower, tower_skill, ui,
    upgrade, upgrade_board,
};
use crate::*;

/// 통합 다국어 텍스트 관리자
#[derive(Debug, Clone, Copy, State)]
pub struct TextManager {
    locale: Locale,
}

impl TextManager {
    /// 새로운 텍스트 관리자 생성
    pub const fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// 현재 언어 반환
    pub fn language(&self) -> crate::l10n::Language {
        self.locale.language
    }

    /// 한국어 텍스트 관리자
    pub const fn korean() -> Self {
        Self::new(Locale::KOREAN)
    }

    /// 영어 텍스트 관리자
    pub const fn english() -> Self {
        Self::new(Locale::ENGLISH)
    }

    /// 현재 로케일 반환
    pub const fn locale(&self) -> Locale {
        self.locale
    }

    /// 로케일 변경
    pub const fn with_locale(self, locale: Locale) -> Self {
        Self::new(locale)
    }
}

/// UI 텍스트 처리
impl TextManager {
    pub fn ui(&self, text: ui::TopBarText) -> &'static str {
        match self.locale.language {
            Language::Korean => text.to_korean(),
            Language::English => text.to_english(),
        }
    }

    pub fn start_confirm_modal(&self, text: ui::StartConfirmModalText) -> &'static str {
        match self.locale.language {
            Language::Korean => text.to_korean(),
            Language::English => text.to_english(),
        }
    }
}

/// 퀘스트 텍스트 처리
impl TextManager {
    pub fn quest(&self, text: quest::QuestText) -> String {
        text.localized_text(&self.locale)
    }

    pub fn quest_reward(&self, text: quest::QuestRewardText) -> String {
        text.localized_text(&self.locale)
    }
}

/// 아이템 텍스트 처리
impl TextManager {
    pub fn effect_name(&self, effect: &crate::game_state::effect::Effect) -> String {
        effect::EffectText::Name(effect.clone()).localized_text(&self.locale)
    }

    pub fn effect_description(&self, effect: &crate::game_state::effect::Effect) -> String {
        effect::EffectText::Description(effect.clone()).localized_text(&self.locale)
    }

    pub fn effect_execution_error(
        &self,
        error: &crate::game_state::effect::EffectExecutionError,
    ) -> String {
        effect::EffectExecutionErrorText(error.clone()).localized_text(&self.locale)
    }
}

/// 계약 텍스트 처리
impl TextManager {
    pub fn contract(&self, text: contract::ContractText) -> String {
        text.localized_text(&self.locale)
    }

    pub fn contract_name(&self, text: contract::ContractNameText) -> &'static str {
        match self.locale.language {
            Language::Korean => text.to_korean(),
            Language::English => text.to_english(),
        }
    }

    pub fn contract_duration(
        &self,
        status: &crate::game_state::contract::ContractStatus,
    ) -> String {
        match self.locale.language {
            Language::Korean => contract::duration_korean(status),
            Language::English => contract::duration_english(status),
        }
    }
}

/// 이벤트 텍스트 처리
impl TextManager {
    pub fn event(&self, text: event::EventText) -> String {
        text.localized_text(&self.locale)
    }
}

/// 타워 텍스트 처리
impl TextManager {
    pub fn tower(&self, text: tower::TowerKindText) -> &'static str {
        match self.locale.language {
            Language::Korean => text.to_korean(),
            Language::English => text.to_english(),
        }
    }
}

/// 타워 스킬 텍스트 처리
impl TextManager {
    pub fn tower_skill(&self, text: tower_skill::TowerSkillText) -> String {
        text.localized_text(&self.locale)
    }
}

/// 업그레이드 텍스트 처리
impl TextManager {
    pub fn upgrade_kind(&self, text: upgrade::UpgradeKindText) -> String {
        text.localized_text(&self.locale)
    }
}

/// 업그레이드 보드 텍스트 처리
impl TextManager {
    pub fn upgrade_board(&self, text: upgrade_board::UpgradeBoardText) -> String {
        text.localized_text(&self.locale)
    }
}

/// 편의성을 위한 전역 상수
pub const KOREAN_TEXT: TextManager = TextManager::korean();
pub const ENGLISH_TEXT: TextManager = TextManager::english();

// ===== 레거시 호환성 지원 (점진적 마이그레이션용) =====

/// 기존 호환성을 위한 레거시 API
#[derive(Debug, Clone, State)]
pub enum LegacyLocales {
    KoKR(LegacyKoKRLocale),
}

#[derive(Debug, Clone, State)]
pub struct LegacyKoKRLocale;

impl LegacyKoKRLocale {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for LegacyKoKRLocale {
    fn default() -> Self {
        Self::new()
    }
}

impl LegacyLocales {
    /// 텍스트 매니저로 변환
    pub fn as_text_manager(&self) -> TextManager {
        match self {
            LegacyLocales::KoKR(_) => TextManager::korean(),
        }
    }

    /// 기존 호환성을 위한 메서드들 - 새로운 TextManager로 위임
    pub fn ui_text(&self, text: ui::TopBarText) -> &'static str {
        self.as_text_manager().ui(text)
    }

    pub fn quest_text(&self, text: &quest::QuestText) -> String {
        self.as_text_manager().quest(text.clone())
    }

    pub fn tower_kind_text(&self, text: &tower::TowerKindText) -> &'static str {
        self.as_text_manager().tower(*text)
    }

    pub fn tower_skill_text(&self, text: &tower_skill::TowerSkillText) -> String {
        self.as_text_manager().tower_skill(text.clone())
    }

    pub fn upgrade_kind_text(&self, text: upgrade::UpgradeKindText) -> String {
        self.as_text_manager().upgrade_kind(text)
    }

    pub fn upgrade_board_text(&self, text: &upgrade_board::UpgradeBoardText) -> String {
        self.as_text_manager().upgrade_board(text.clone())
    }
}
