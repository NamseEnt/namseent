use crate::l10n::Locale;
use crate::l10n::upgrade::UpgradeKindText;
use crate::l10n::upgrade::upgrade_kind::impls::UpgradeKindL10n;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => {
                upgrade_kind.l10n_name(builder, &Locale::ENGLISH)
            }
            UpgradeKindText::Description(upgrade_kind) => {
                upgrade_kind.l10n_description(builder, &Locale::ENGLISH)
            }
        }
    }
}
