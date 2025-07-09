use super::UpgradeKind;
use crate::l10n::upgrade::Locales;
use crate::l10n::upgrade::Template;

impl UpgradeKind {
    pub fn name(&self) -> String {
        Locales::KoKR(crate::l10n::upgrade::KoKRLocale).text(Template::from_kind(self, true))
    }
    pub fn description(&self) -> String {
        Locales::KoKR(crate::l10n::upgrade::KoKRLocale).text(Template::from_kind(self, false))
    }
}
