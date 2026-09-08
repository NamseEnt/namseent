use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BackpackUpgradeState {
    pub shop_slot_expand: usize,
}
use super::super::UpgradeCacheContribution;
use super::support::base_cache;

const MAX_SHOP_SLOT_EXPAND: usize = 2;

fn max_shop_slots(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        core.upgrades
            .upgrades
            .iter()
            .filter(|u| u.kind() == crate::UpgradeKind::Backpack)
            .map(|u| u.backpack().shop_slot_expand)
            .sum(),
        MAX_SHOP_SLOT_EXPAND,
    ))
}

fn merge_backpack(core: &mut crate::CoreState, mut upgrade: super::super::UpgradeEntry) -> usize {
    let add = upgrade.backpack().shop_slot_expand;
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.kind() == crate::UpgradeKind::Backpack)
    {
        existing.backpack_mut().shop_slot_expand =
            existing.backpack().shop_slot_expand.saturating_add(add);
    } else {
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
    0
}

fn acquire_shop_slots(core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
    let add = upgrade.backpack().shop_slot_expand;
    if add > 0 {
        crate::game_state::shop::add_shop_slots(core, add);
    }
    merge_backpack(core, upgrade);
    add
}

fn cache_shop_slots(upgrade: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        shop_slot_expand: upgrade.backpack().shop_slot_expand,
        ..base_cache()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Backpack
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Backpack(
            super::super::codec_impl::BackpackUpgradeState {
                shop_slot_expand: 1,
            },
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_shop_slots(entry)
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_shop_slots(core, upgrade)
    }
    fn current_and_max(&self, core: &crate::CoreState) -> Option<(usize, usize)> {
        max_shop_slots(core)
    }
}
