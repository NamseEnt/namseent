use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyDrinkUpgradeState {
    pub discount: usize,
}
use super::super::UpgradeCacheContribution;
use super::support::base_cache;

const MAX_SHOP_ITEM_PRICE_MINUS: usize = 15;

fn max_shop_discount(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        core.upgrades
            .upgrades
            .iter()
            .filter(|u| u.kind() == crate::UpgradeKind::EnergyDrink)
            .map(|u| u.energy_drink().discount)
            .sum(),
        MAX_SHOP_ITEM_PRICE_MINUS,
    ))
}

fn merge_energy_drink(core: &mut crate::CoreState, mut upgrade: super::super::UpgradeEntry) {
    let add = upgrade.energy_drink().discount;
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.kind() == crate::UpgradeKind::EnergyDrink)
    {
        existing.energy_drink_mut().discount = existing.energy_drink().discount.saturating_add(add);
    } else {
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
}

fn acquire_discount(core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
    let discount = upgrade.energy_drink().discount;
    if let crate::GameFlowState::Shopping(shop) = &mut core.flow {
        for slot in &mut shop.slots {
            let cost = match &mut slot.slot {
                crate::ShopSlotState::Item { cost, .. }
                | crate::ShopSlotState::Upgrade { cost, .. }
                | crate::ShopSlotState::CardService { cost, .. } => cost,
            };
            *cost = cost.saturating_sub(discount);
        }
    }
    merge_energy_drink(core, upgrade);
    0
}

fn cache_discount(upgrade: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        shop_item_price_minus: upgrade.energy_drink().discount,
        ..base_cache()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::EnergyDrink
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::EnergyDrink(
            super::super::codec_impl::EnergyDrinkUpgradeState { discount: 5 },
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_discount(entry)
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_discount(core, upgrade)
    }
    fn current_and_max(&self, core: &crate::CoreState) -> Option<(usize, usize)> {
        max_shop_discount(core)
    }
}
