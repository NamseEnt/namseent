use super::UpgradeKind;
use crate::l10n::upgrade::Template;

impl UpgradeKind {
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.upgrade(Template::from_kind(self, true))
    }
    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.upgrade(Template::from_kind(self, false))
    }
}
