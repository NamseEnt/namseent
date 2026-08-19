use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const WATERMELON_HP_PLUS: f32 = 8.0;
const WATERMELON_HEAL_AMOUNT: f32 = 12.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct WatermelonUpgrade;

impl UpgradeBehavior for WatermelonUpgrade {
    fn key(&self) -> &'static str {
        "watermelon"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::WATERMELON)
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::REVISION | UpgradeUpdateFlags::CACHE
    }

    fn max_hp_plus(&self) -> f32 {
        WATERMELON_HP_PLUS
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::Amount(WATERMELON_HEAL_AMOUNT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Watermelon",
            crate::l10n::locale::Language::Korean => "수박",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .with_bold("Max Health")
                .static_text(" increased by ")
                .with_bold(format!("{:.0}", WATERMELON_HP_PLUS))
                .static_text(", ")
                .with_bold("Health")
                .static_text(" recovered by ")
                .with_bold(format!("{:.0}", WATERMELON_HEAL_AMOUNT))
                .static_text("."),
            crate::l10n::locale::Language::Korean => builder
                .with_bold("최대 체력")
                .static_text("을 ")
                .with_bold(format!("{:.0}", WATERMELON_HP_PLUS))
                .static_text(" 늘리고, ")
                .with_bold("체력을 ")
                .with_bold(format!("{:.0}", WATERMELON_HEAL_AMOUNT))
                .static_text(" 회복합니다."),
        };
    }
}

impl WatermelonUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Watermelon(WatermelonUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_epic,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    WatermelonUpgrade::into_upgrade()
}
