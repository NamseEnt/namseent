use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CupNoodlesUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const CUP_NOODLES_HEAL_AMOUNT_RAW: i64 = 6_000;

fn cache_cup_noodles(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(-2_000)
}

fn recovery_cup_noodles() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(CUP_NOODLES_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::CupNoodles
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::CupNoodles(
            super::super::codec_impl::CupNoodlesUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_cup_noodles(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_cup_noodles()
    }
}
