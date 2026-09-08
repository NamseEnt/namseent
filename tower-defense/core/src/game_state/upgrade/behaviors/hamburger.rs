use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HamburgerUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const HAMBURGER_HEAL_AMOUNT_RAW: i64 = 18_000;

fn cache_hamburger(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(-6_000)
}

fn recovery_hamburger() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(HAMBURGER_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Hamburger
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Hamburger(
            super::super::codec_impl::HamburgerUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_hamburger(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_hamburger()
    }
}
