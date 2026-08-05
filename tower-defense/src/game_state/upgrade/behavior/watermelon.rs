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

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::WATERMELON,
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
            crate::l10n::locale::Language::English => builder.with_health_value(format!(
                "Increase max Health by {:.0} and recover {:.0} Health.",
                WATERMELON_HP_PLUS, WATERMELON_HEAL_AMOUNT
            )),
            crate::l10n::locale::Language::Korean => builder.with_health_value(format!(
                "최대 체력을 {:.0} 늘리고, 체력을 {:.0} 회복합니다.",
                WATERMELON_HP_PLUS, WATERMELON_HEAL_AMOUNT
            )),
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
