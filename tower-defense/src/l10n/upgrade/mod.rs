mod upgrade_kind;

use super::{Language, Locale, LocalizedText};

pub enum UpgradeKindText<'a> {
    Name(&'a crate::game_state::upgrade::UpgradeKind),
    Description(&'a crate::game_state::upgrade::UpgradeKind),
}

impl LocalizedText for UpgradeKindText<'_> {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}
