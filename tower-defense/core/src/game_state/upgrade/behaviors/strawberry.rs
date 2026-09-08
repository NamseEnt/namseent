use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StrawberryUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const STRAWBERRY_HEAL_AMOUNT_RAW: i64 = 3_000;

fn cache_strawberry(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(2_000)
}

fn recovery_strawberry() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(STRAWBERRY_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Strawberry
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Strawberry(
            super::super::codec_impl::StrawberryUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_strawberry(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_strawberry()
    }
}
