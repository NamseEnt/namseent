// 현대적 l10n API - 간결하고 효율적인 다국어 텍스트 관리

use super::{Language, Locale, LocalizedText, contract, effect, tower, ui, upgrade};
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

    pub fn result_modal(&self, text: ui::ResultModalText) -> &'static str {
        match self.locale.language {
            Language::Korean => text.to_korean(),
            Language::English => text.to_english(),
        }
    }
}

/// 아이템 텍스트 처리
impl TextManager {
    pub fn effect_name<'a>(
        &self,
        effect: &crate::game_state::effect::Effect,
        mut builder: crate::theme::typography::TypographyBuilder<'a>,
    ) -> crate::theme::typography::TypographyBuilder<'a> {
        effect::EffectText::Name(effect.clone()).apply_to_builder(&mut builder, &self.locale);
        builder
    }

    pub fn effect_description<'a>(
        &self,
        effect: &crate::game_state::effect::Effect,
        mut builder: crate::theme::typography::TypographyBuilder<'a>,
    ) -> crate::theme::typography::TypographyBuilder<'a> {
        effect::EffectText::Description(effect.clone())
            .apply_to_builder(&mut builder, &self.locale);
        builder
    }

    pub fn effect_execution_error<'a>(
        &self,
        error: &crate::game_state::effect::EffectExecutionError,
        mut builder: crate::theme::typography::TypographyBuilder<'a>,
    ) -> crate::theme::typography::TypographyBuilder<'a> {
        let text = effect::EffectExecutionErrorText(error.clone());
        text.apply_to_builder(&mut builder, &self.locale);
        builder
    }
}

/// 계약 텍스트 처리
impl TextManager {
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

/// 타워 텍스트 처리
impl TextManager {
    pub fn tower(&self, text: tower::TowerKindText) -> &'static str {
        match self.locale.language {
            Language::Korean => text.to_korean(),
            Language::English => text.to_english(),
        }
    }
}

/// 편의성을 위한 전역 상수
pub const KOREAN_TEXT: TextManager = TextManager::korean();
pub const ENGLISH_TEXT: TextManager = TextManager::english();
