mod upgrade_kind;

use super::{Language, Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;

pub enum UpgradeTypeText<'a> {
    Name(&'a crate::game_state::upgrade::Upgrade),
    DescriptionUpgrade(&'a crate::game_state::upgrade::Upgrade),
}

impl LocalizedText for UpgradeTypeText<'_> {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}
