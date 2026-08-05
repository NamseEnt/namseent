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

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::BANANA,
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
            crate::l10n::locale::Language::English => builder.with_health_value(format!(
                "Increase max Health by {:.0} and recover {:.0} Health.",
                BANANA_HP_PLUS, BANANA_HEAL_AMOUNT
            )),
            crate::l10n::locale::Language::Korean => builder.with_health_value(format!(
                "최대 체력을 {:.0} 늘리고, 체력을 {:.0} 회복합니다.",
                BANANA_HP_PLUS, BANANA_HEAL_AMOUNT
            )),
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
