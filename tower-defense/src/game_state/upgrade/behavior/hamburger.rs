use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const HAMBURGER_MAX_HP_DECREASE: f32 = 6.0;
const HAMBURGER_HEAL_AMOUNT: f32 = 18.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct HamburgerUpgrade;

impl UpgradeBehavior for HamburgerUpgrade {
    fn key(&self) -> &'static str {
        "hamburger"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::HAMBURGER,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::REVISION | UpgradeUpdateFlags::CACHE
    }

    fn max_hp_plus(&self) -> f32 {
        -HAMBURGER_MAX_HP_DECREASE
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::Amount(HAMBURGER_HEAL_AMOUNT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Hamburger",
            crate::l10n::locale::Language::Korean => "햄버거",
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
                .with_health_value(format!("{:.0}", HAMBURGER_MAX_HP_DECREASE))
                .static_text(", ")
                .with_heal_icon("Health")
                .static_text(" recovered by ")
                .with_health_value(format!("{:.0}", HAMBURGER_HEAL_AMOUNT))
                .static_text("."),
            crate::l10n::locale::Language::Korean => builder
                .with_heal_icon("최대 체력")
                .static_text("을 ")
                .with_health_value(format!("{:.0}", HAMBURGER_MAX_HP_DECREASE))
                .static_text(" 줄이고, ")
                .with_heal_icon("체력을 ")
                .with_health_value(format!("{:.0}", HAMBURGER_HEAL_AMOUNT))
                .static_text(" 회복합니다."),
        };
    }
}

impl HamburgerUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Hamburger(HamburgerUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_rare,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    HamburgerUpgrade::into_upgrade()
}
