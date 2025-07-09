use super::UpgradeKind;
use crate::l10n::upgrade::Locales;
use crate::l10n::upgrade::Template;

impl UpgradeKind {
    pub fn name(&self, locale: &Locales) -> String {
        locale.text(Template::from_kind(self, true))
    }
    pub fn description(&self, locale: &Locales) -> String {
        locale.text(Template::from_kind(self, false))
    }
}
