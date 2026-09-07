use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FourLeafCloverUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::base_cache;

fn max_straight_flush(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        usize::from(core.upgrades.upgrades.iter().any(|u| {
            u.upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::FourLeafClover)
        })),
        1,
    ))
}

fn cache_straight_flush(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        shorten_straight_flush_to_4_cards: true,
        ..base_cache()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::FourLeafClover
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::FourLeafClover(
            super::super::codec_impl::FourLeafCloverUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_straight_flush(entry)
    }
    fn current_and_max(&self, core: &crate::CoreState) -> Option<(usize, usize)> {
        max_straight_flush(core)
    }
}
