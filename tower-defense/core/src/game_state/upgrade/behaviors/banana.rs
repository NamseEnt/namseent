use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BananaUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const BANANA_HEAL_AMOUNT_RAW: i64 = 9_000;

fn cache_banana(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(6_000)
}

fn recovery_banana() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(BANANA_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Banana
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Banana(super::super::codec_impl::BananaUpgradeState)
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_banana(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_banana()
    }
}
