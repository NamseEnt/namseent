use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const APPLE_HP_PLUS: f32 = 4.0;
const APPLE_HEAL_AMOUNT: f32 = 6.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct AppleUpgrade;

impl UpgradeBehavior for AppleUpgrade {
    fn key(&self) -> &'static str {
        "apple"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::APPLE,
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
        APPLE_HP_PLUS
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::Amount(APPLE_HEAL_AMOUNT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Apple",
            crate::l10n::locale::Language::Korean => "사과",
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
                APPLE_HP_PLUS, APPLE_HEAL_AMOUNT
            )),
            crate::l10n::locale::Language::Korean => builder.with_health_value(format!(
                "최대 체력을 {:.0} 늘리고, 체력을 {:.0} 회복합니다.",
                APPLE_HP_PLUS, APPLE_HEAL_AMOUNT
            )),
        };
    }
}

impl AppleUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Apple(AppleUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    AppleUpgrade::into_upgrade()
}
