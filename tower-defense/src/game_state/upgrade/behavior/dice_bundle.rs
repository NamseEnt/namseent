use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DiceBundleUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for DiceBundleUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::DICE_BUNDLE,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn thumbnail_overlay(
        &self,
        width_height: Wh<Px>,
        _game_state: &GameState,
    ) -> Option<RenderingTree> {
        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &format!("{}", self.add),
            crate::theme::palette::BLUE,
        ))
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::DiceBundle(upgrade) = &mut upgrade.upgrade {
                upgrade.add += self.add;
                return UpgradeUpdateFlags::NONE;
            }
        }

        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::NONE
    }

    fn dice_chance_plus(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Dice Bundle",
            crate::l10n::locale::Language::Korean => "주사위 꾸러미",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.with_dice_value(format!("Dice +{}", self.add))
            }
            crate::l10n::locale::Language::Korean => {
                builder.with_dice_value(format!("주사위 +{}", self.add))
            }
        };
    }
}

impl DiceBundleUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::DiceBundle(DiceBundleUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_rare,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    DiceBundleUpgrade::into_upgrade(1)
}
