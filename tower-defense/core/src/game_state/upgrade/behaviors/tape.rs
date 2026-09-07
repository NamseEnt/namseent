use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TapeUpgradeState {
    pub acquired_stage: usize,
}
use super::support::push_acquired_upgrade;

const TAPE_ENEMY_SPEED_MULTIPLIER_RAW: i64 = 750_000;

const TAPE_STAGE_INTERVAL: usize = 4;

fn acquire_stage_marker(
    core: &mut crate::CoreState,
    mut upgrade: super::super::UpgradeEntry,
) -> usize {
    upgrade.tape_mut().acquired_stage = core.progress.stage;
    push_acquired_upgrade(core, upgrade)
}

fn stage_start_tape(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    stage: usize,
) -> bool {
    let acquired_stage = entry.tape().acquired_stage;
    if stage.saturating_sub(acquired_stage) % TAPE_STAGE_INTERVAL == TAPE_STAGE_INTERVAL - 1 {
        context
            .stage_modifiers
            .enemy_speed_multipliers_raw
            .push(TAPE_ENEMY_SPEED_MULTIPLIER_RAW);
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Tape
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Tape(super::super::codec_impl::TapeUpgradeState {
            acquired_stage: 0,
        })
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_stage_marker(core, upgrade)
    }
    fn stage_start(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        stage: usize,
    ) -> bool {
        stage_start_tape(context, entry, stage)
    }
}
