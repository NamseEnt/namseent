use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const BANANA_HP_PLUS: f32 = 6.0;
const BANANA_HEAL_AMOUNT: f32 = 9.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BananaUpgrade;

impl UpgradeBehavior for BananaUpgrade {
    fn key(&self) -> &'static str {
        "banana"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BANANA)
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::REVISION | UpgradeUpdateFlags::CACHE
    }

    fn max_hp_plus(&self) -> f32 {
        BANANA_HP_PLUS
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::Amount(BANANA_HEAL_AMOUNT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Banana",
            crate::l10n::locale::Language::Korean => "바나나",
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
                .static_text(" increased by ")
                .with_health_value(format!("{:.0}", BANANA_HP_PLUS))
                .static_text(", ")
                .with_heal_icon("Health")
                .static_text(" recovered by ")
                .with_health_value(format!("{:.0}", BANANA_HEAL_AMOUNT))
                .static_text("."),
            crate::l10n::locale::Language::Korean => builder
                .with_heal_icon("최대 체력")
                .static_text("을 ")
                .with_health_value(format!("{:.0}", BANANA_HP_PLUS))
                .static_text(" 늘리고, ")
                .with_heal_icon("체력을 ")
                .with_health_value(format!("{:.0}", BANANA_HEAL_AMOUNT))
                .static_text(" 회복합니다."),
        };
    }
}

impl BananaUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Banana(BananaUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_rare,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BananaUpgrade::into_upgrade()
}
