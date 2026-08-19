use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const STRAWBERRY_HP_PLUS: f32 = 2.0;
const STRAWBERRY_HEAL_AMOUNT: f32 = 3.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct StrawberryUpgrade;

impl UpgradeBehavior for StrawberryUpgrade {
    fn key(&self) -> &'static str {
        "strawberry"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::STRAWBERRY)
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::REVISION | UpgradeUpdateFlags::CACHE
    }

    fn max_hp_plus(&self) -> f32 {
        STRAWBERRY_HP_PLUS
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::Amount(STRAWBERRY_HEAL_AMOUNT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Strawberry",
            crate::l10n::locale::Language::Korean => "딸기",
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
                .with_bold(format!("{:.0}", STRAWBERRY_HP_PLUS))
                .static_text(", ")
                .with_bold("Health")
                .static_text(" recovered by ")
                .with_bold(format!("{:.0}", STRAWBERRY_HEAL_AMOUNT))
                .static_text("."),
            crate::l10n::locale::Language::Korean => builder
                .with_bold("최대 체력")
                .static_text("을 ")
                .with_bold(format!("{:.0}", STRAWBERRY_HP_PLUS))
                .static_text(" 늘리고, ")
                .with_bold("체력을 ")
                .with_bold(format!("{:.0}", STRAWBERRY_HEAL_AMOUNT))
                .static_text(" 회복합니다."),
        };
    }
}

impl StrawberryUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Strawberry(StrawberryUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    StrawberryUpgrade::into_upgrade()
}
