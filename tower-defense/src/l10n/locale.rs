use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum Language {
    Korean,
    English,
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
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

/// Trait for localized text that can be integrated into TypographyBuilder chains
pub trait LocalizedText {
    /// Apply localized rich text formatting to a builder
    fn apply_to_builder<'a>(
        self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &Locale,
    );
}
