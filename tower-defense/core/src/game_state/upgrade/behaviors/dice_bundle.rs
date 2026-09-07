use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiceBundleUpgradeState {
    pub dice_chance_plus: usize,
}
use super::super::UpgradeCacheContribution;
use super::support::base_cache;

fn acquire_dice(core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
    let add = upgrade.dice_bundle().dice_chance_plus;
    core.progress.left_dice = core.progress.left_dice.saturating_add(add);
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.kind() == crate::UpgradeKind::DiceBundle)
    {
        existing.dice_bundle_mut().dice_chance_plus =
            existing.dice_bundle().dice_chance_plus.saturating_add(add);
    } else {
        let mut upgrade = upgrade;
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
    0
}

fn cache_dice(upgrade: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        dice_chance_plus: upgrade.dice_bundle().dice_chance_plus,
        ..base_cache()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::DiceBundle
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::DiceBundle(
            super::super::codec_impl::DiceBundleUpgradeState {
                dice_chance_plus: 1,
            },
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_dice(entry)
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_dice(core, upgrade)
    }
}
