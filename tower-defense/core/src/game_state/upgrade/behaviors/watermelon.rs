use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WatermelonUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const WATERMELON_HEAL_AMOUNT_RAW: i64 = 12_000;

fn cache_watermelon(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(8_000)
}

fn recovery_watermelon() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(WATERMELON_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Watermelon
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Watermelon(
            super::super::codec_impl::WatermelonUpgradeState,
        )
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_watermelon(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_watermelon()
    }
}
