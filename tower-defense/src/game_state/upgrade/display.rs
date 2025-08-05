use super::UpgradeKind;
use crate::l10n::upgrade::UpgradeKindText;

impl UpgradeKind {
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.upgrade_kind(UpgradeKindText::Name(self))
    }

    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.upgrade_kind(UpgradeKindText::Description(self))
    }
}
