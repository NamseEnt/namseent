use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const FRENCH_FRIES_MAX_HP_DECREASE: f32 = 4.0;
const FRENCH_FRIES_HEAL_AMOUNT: f32 = 12.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FrenchFriesUpgrade;

impl UpgradeBehavior for FrenchFriesUpgrade {
    fn key(&self) -> &'static str {
        "french_fries"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::FRENCH_FRIES)
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::REVISION | UpgradeUpdateFlags::CACHE
    }

    fn max_hp_plus(&self) -> f32 {
        -FRENCH_FRIES_MAX_HP_DECREASE
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::Amount(FRENCH_FRIES_HEAL_AMOUNT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "French Fries",
            crate::l10n::locale::Language::Korean => "감자튀김",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .with_heal_icon("Max Health")
                .static_text(" decreased by ")
                .with_health_value(format!("{:.0}", FRENCH_FRIES_MAX_HP_DECREASE))
                .static_text(", ")
                .with_heal_icon("Health")
                .static_text(" recovered by ")
                .with_health_value(format!("{:.0}", FRENCH_FRIES_HEAL_AMOUNT))
                .static_text("."),
            crate::l10n::locale::Language::Korean => builder
                .with_heal_icon("최대 체력")
                .static_text("을 ")
                .with_health_value(format!("{:.0}", FRENCH_FRIES_MAX_HP_DECREASE))
                .static_text(" 줄이고, ")
                .with_heal_icon("체력을 ")
                .with_health_value(format!("{:.0}", FRENCH_FRIES_HEAL_AMOUNT))
                .static_text(" 회복합니다."),
        };
    }
}

impl FrenchFriesUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::FrenchFries(FrenchFriesUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    FrenchFriesUpgrade::into_upgrade()
}
