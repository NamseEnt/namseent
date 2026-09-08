use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PizzaUpgradeState;
use super::super::UpgradeCacheContribution;
use super::support::{cache_hp, recovery_amount};

const PIZZA_HEAL_AMOUNT_RAW: i64 = 24_000;

fn cache_pizza(_: &super::super::UpgradeEntry) -> UpgradeCacheContribution {
    cache_hp(-8_000)
}

fn recovery_pizza() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(PIZZA_HEAL_AMOUNT_RAW)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Pizza
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Pizza(super::super::codec_impl::PizzaUpgradeState)
    }
    fn cache(&self, entry: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        cache_pizza(entry)
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        recovery_pizza()
    }
}
