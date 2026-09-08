use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RabbitUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::base_cache;

fn max_skip_rank(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        usize::from(core.upgrades.upgrades.iter().any(|u| {
            u.upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::Rabbit)
        })),
        1,
    ))
}

fn cache_skip_rank(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        skip_rank_for_straight: true,
        ..base_cache()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Rabbit
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Rabbit(super::super::codec_impl::RabbitUpgradeState)
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_skip_rank(entry)
    }
    fn current_and_max(&self, core: &crate::CoreState) -> Option<(usize, usize)> {
        max_skip_rank(core)
    }
}
