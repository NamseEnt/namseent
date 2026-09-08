use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShoppingBagUpgradeState;

fn shop_purchase_dice(
    context: &mut super::super::UpgradeTriggerContext,
    _: &mut super::super::UpgradeEntry,
    item_purchase: bool,
) -> bool {
    if item_purchase {
        context.progress.left_dice = context.progress.left_dice.saturating_add(1);
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::ShoppingBag
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::ShoppingBag(
            super::super::codec_impl::ShoppingBagUpgradeState,
        )
    }
    fn shop_purchase(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        item_purchase: bool,
    ) -> bool {
        shop_purchase_dice(context, entry, item_purchase)
    }
}
