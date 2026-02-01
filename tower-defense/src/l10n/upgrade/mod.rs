mod upgrade_kind;

use super::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;

pub enum UpgradeKindText<'a> {
    Name(&'a crate::game_state::upgrade::UpgradeKind),
    Description(&'a crate::game_state::upgrade::UpgradeKind),
}



impl LocalizedText for UpgradeKindText<'_> {
    fn apply_to_builder<'a>(
        self,
        builder: &mut TypographyBuilder<'a>,
        locale: &Locale,
    ) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl UpgradeKindText<'_> {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        let text = self.to_korean();
        builder.text(text);
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        let text = self.to_english();
        builder.text(text);
    }
}
