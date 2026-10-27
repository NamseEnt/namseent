use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrainingMelonUpgradeState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::TrainingMelon
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::TrainingMelon(
            super::super::codec_impl::TrainingMelonUpgradeState,
        )
    }
    fn cache(&self, _: &super::super::UpgradeEntry) -> super::super::UpgradeCacheContribution {
        super::super::UpgradeCacheContribution {
            max_hp_plus_raw: 8_000,
            clear_shield_on_stage_start: true,
            ..Default::default()
        }
    }
    fn recovery(&self) -> super::super::UpgradeAcquireRecovery {
        super::super::UpgradeAcquireRecovery::Amount(12_000)
    }
}
