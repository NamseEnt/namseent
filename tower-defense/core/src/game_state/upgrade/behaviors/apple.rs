use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppleUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const APPLE_HEAL_AMOUNT_RAW: i64 = 6_000;

fn cache_apple(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(4_000)
}

fn recovery_apple() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(APPLE_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Apple
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Apple(super::super::codec_impl::AppleUpgradeState)
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_apple(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_apple()
    }
}
