use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ShoppingBagUpgrade;

impl UpgradeBehavior for ShoppingBagUpgrade {
    fn key(&self) -> &'static str {
        "shopping_bag"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::SHOPPING_BAG)
    }

    fn on_item_bought(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state.action(crate::game_state::GameStateAction::GainRerolls(1));
        UpgradeUpdateFlags::NONE
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Shopping Bag",
            crate::l10n::locale::Language::Korean => "쇼핑백",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder
                    .l10n(Word::Dice.name(), locale)
                    .with_dice_value(" +1")
                    .static_text(" for each purchased ")
                    .l10n(Word::Item.name(), locale);
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .l10n(Word::Item.name(), locale)
                    .static_text(" 구매 시 ")
                    .l10n(Word::Dice.name(), locale)
                    .with_dice_value(" +1");
            }
        }
    }
}

impl ShoppingBagUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::ShoppingBag(ShoppingBagUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_legendary,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    ShoppingBagUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    use crate::game_state::upgrade::*;

    #[test]
    fn shopping_bag_grants_one_reroll_when_an_item_is_purchased() {
        use crate::game_state::GameFlow;
        use crate::game_state::item::LumpSugarItem;
        use crate::game_state::upgrade::tests::support;
        use crate::shop::ShopSlot;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(),
            None,
        ));
        game_state.left_dice = 0;

        let slot_id = if let GameFlow::Shopping(flow) = &mut game_state.flow {
            flow.shop.push(ShopSlot::Item {
                item: LumpSugarItem::standard().into_item(),
                cost: 0,
            });
            flow.shop.slots.last().unwrap().id
        } else {
            panic!("expected shopping flow");
        };

        game_state.action(crate::game_state::GameStateAction::PurchaseShopItem(
            slot_id,
        ));

        assert_eq!(game_state.left_dice, 1);
    }

    #[test]
    fn shopping_bag_does_not_increase_tower_damage() {
        let state = UpgradeState::with_upgrades(vec![
            crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(),
        ]);

        assert!(state.tower_upgrade_damage_bonuses().is_empty());
    }
}
