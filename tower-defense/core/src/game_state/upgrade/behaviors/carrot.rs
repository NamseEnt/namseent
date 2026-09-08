use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CarrotUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::cache_hp;

fn cache_carrot(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(6_000)
}

fn recovery_full() -> super::super::UpgradeAcquireRecovery {
    super::super::UpgradeAcquireRecovery::ToFull
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Carrot
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Carrot(super::super::codec_impl::CarrotUpgradeState)
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_carrot(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_full()
    }
}
