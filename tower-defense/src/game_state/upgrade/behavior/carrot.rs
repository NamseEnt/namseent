use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const CARROT_HP_PLUS: f32 = 6.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CarrotUpgrade;

impl UpgradeBehavior for CarrotUpgrade {
    fn key(&self) -> &'static str {
        "carrot"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::CARROT,
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
        CARROT_HP_PLUS
    }

    fn recovery_on_acquire(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::ToFull
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Carrot",
            crate::l10n::locale::Language::Korean => "당근",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder.with_health_value(format!(
                "Increase max Health by {:.0} and fully recover Health.",
                CARROT_HP_PLUS
            )),
            crate::l10n::locale::Language::Korean => builder.with_health_value(format!(
                "최대 체력을 {:.0} 늘리고, 체력을 모두 회복합니다.",
                CARROT_HP_PLUS
            )),
        };
    }
}

impl CarrotUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Carrot(CarrotUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_legendary,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    CarrotUpgrade::into_upgrade()
}
