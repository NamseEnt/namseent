use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BlackWhiteUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::base_cache;

fn max_same_suits(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        usize::from(core.upgrades.upgrades.iter().any(|u| {
            u.upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::BlackWhite)
        })),
        1,
    ))
}

fn cache_same_suits(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        treat_suits_as_same: true,
        ..base_cache()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::BlackWhite
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::BlackWhite(
            super::super::codec_impl::BlackWhiteUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_same_suits(entry)
    }
    fn current_and_max(&self, core: &crate::CoreState) -> Option<(usize, usize)> {
        max_same_suits(core)
    }
}
