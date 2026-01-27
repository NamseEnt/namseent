use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum Language {
    Korean,
    English,
}

#[derive(Debug, Clone, Copy, State)]
pub struct Locale {
    pub language: Language,
}

impl Locale {
    pub const KOREAN: Self = Self {
        language: Language::Korean,
    };

    pub const ENGLISH: Self = Self {
        language: Language::English,
    };
}

impl Default for Locale {
    fn default() -> Self {
        Self::KOREAN
    }
}

pub trait LocalizedText {
    fn localized_text(&self, locale: &Locale) -> String;
}

pub trait LocalizedStaticText {
    fn localized_text(&self, locale: &Locale) -> &'static str;
}

/// Trait for localized text that can be integrated into TypographyBuilder chains
pub trait LocalizedRichText {
    /// Apply localized rich text formatting to a builder
    fn apply_to_builder<'a>(
        self,
        builder: crate::theme::typography::TypographyBuilder<'a>,
        locale: &Locale,
    ) -> crate::theme::typography::TypographyBuilder<'a>;
}
