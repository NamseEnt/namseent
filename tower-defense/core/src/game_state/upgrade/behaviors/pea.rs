use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PeaUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::cache_hp;

fn cache_pea(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(3_000)
}

fn recovery_full() -> super::super::UpgradeAcquireRecovery {
    super::super::UpgradeAcquireRecovery::ToFull
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Pea
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Pea(super::super::codec_impl::PeaUpgradeState)
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_pea(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_full()
    }
}
