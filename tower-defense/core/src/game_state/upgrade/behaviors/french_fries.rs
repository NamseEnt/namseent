use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrenchFriesUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const FRENCH_FRIES_HEAL_AMOUNT_RAW: i64 = 12_000;

fn cache_french_fries(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(-4_000)
}

fn recovery_french_fries() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(FRENCH_FRIES_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::FrenchFries
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::FrenchFries(
            super::super::codec_impl::FrenchFriesUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_french_fries(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_french_fries()
    }
}
