mod behavior;
pub mod generation;

pub use behavior::*;

impl Item {
    pub fn name_text(&self) -> crate::l10n::item_kind::ItemText {
        crate::l10n::item_kind::ItemText::Name(self.clone())
    }

    pub fn description_text(&self) -> crate::l10n::item_kind::ItemText {
        crate::l10n::item_kind::ItemText::Description(self.clone())
    }
}
