use crate::game_state::upgrade::{Upgrade, UpgradeBehavior};
use crate::l10n::{Locale, LocalizedText};
use crate::theme::typography::TypographyBuilder;

pub enum UpgradeTypeText<'a> {
    Name(&'a Upgrade),
    DescriptionUpgrade(&'a Upgrade),
}

impl LocalizedText for UpgradeTypeText<'_> {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match self {
            UpgradeTypeText::Name(upgrade) => upgrade.l10n_name(builder, locale),
            UpgradeTypeText::DescriptionUpgrade(upgrade) => {
                upgrade.l10n_description(builder, locale)
            }
        }
    }
}
