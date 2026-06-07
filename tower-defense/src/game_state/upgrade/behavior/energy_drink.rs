use super::*;
use crate::{game_state::flow::GameFlow, l10n::rich_text_helpers::RichTextHelpers};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EnergyDrinkUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for EnergyDrinkUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::ENERGY_DRINK,
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
            &format!("-{}", self.add),
            crate::theme::palette::YELLOW,
        ))
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        if let GameFlow::Shopping(flow) = &mut game_state.flow {
            for slot in &mut flow.shop.slots {
                let cost = match &mut slot.slot {
                    crate::shop::ShopSlot::Item { cost, .. } => cost,
                    crate::shop::ShopSlot::Upgrade { cost, .. } => cost,
                };
                *cost = cost.saturating_sub(self.add);
            }
        };

        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::EnergyDrink(upgrade) = &mut upgrade.upgrade {
                upgrade.add += self.add;
                return UpgradeUpdateFlags::CACHE | UpgradeUpdateFlags::REVISION;
            }
        }

        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::CACHE | UpgradeUpdateFlags::REVISION
    }

    fn shop_item_price_minus(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Energy Drink",
            crate::l10n::locale::Language::Korean => "에너지드링크",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Shop price ")
                .with_gold_loss(format!("-{}", self.add)),
            crate::l10n::locale::Language::Korean => builder
                .static_text("상점 가격 ")
                .with_gold_loss(format!("-{}", self.add)),
        };
    }
}

impl EnergyDrinkUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::EnergyDrink(EnergyDrinkUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    EnergyDrinkUpgrade::into_upgrade(5)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.shop_item_price_minus(),
        super::MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
    ))
}
