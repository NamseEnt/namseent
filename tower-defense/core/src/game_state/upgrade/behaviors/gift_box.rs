use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GiftBoxUpgradeState {
    pub gold_per_item: usize,
}

fn stage_end_item_gold(
    _: &mut super::super::UpgradeTriggerContext,
    upgrade: &mut super::super::UpgradeEntry,
    _: bool,
    _: usize,
    item_count: usize,
) -> (bool, usize) {
    let gold_per_item = upgrade.gift_box().gold_per_item;
    (false, item_count.saturating_mul(gold_per_item))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::GiftBox
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::GiftBox(super::super::codec_impl::GiftBoxUpgradeState {
            gold_per_item: 10,
        })
    }
    fn stage_end(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (bool, usize) {
        stage_end_item_gold(context, entry, perfect_clear, gold, item_count)
    }
}
