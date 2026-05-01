use crate::l10n::Locale;
use crate::l10n::upgrade::UpgradeTypeText;
use crate::l10n::upgrade::upgrade_kind::impls::UpgradeTypeL10n;
use crate::theme::typography::TypographyBuilder;

impl UpgradeTypeText<'_> {
    pub fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeTypeText::Name(upgrade) => upgrade.l10n_name(builder, &Locale::KOREAN),
            UpgradeTypeText::DescriptionUpgrade(upgrade) => {
                upgrade.l10n_description(builder, &Locale::KOREAN)
            }
        }
    }
}
