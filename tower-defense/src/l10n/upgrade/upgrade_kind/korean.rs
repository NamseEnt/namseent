use crate::l10n::Locale;
use crate::l10n::upgrade::UpgradeKindText;
use crate::l10n::upgrade::upgrade_kind::impls::UpgradeKindL10n;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => upgrade_kind.l10n_name(builder, &Locale::KOREAN),
            UpgradeKindText::Description(upgrade_kind) => {
                upgrade_kind.l10n_description(builder, &Locale::KOREAN)
            }
        }
    }
}
