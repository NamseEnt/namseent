use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MetronomeUpgradeState {
    pub acquired_stage: usize,
}
use super::support::push_acquired_upgrade;

const DICE_BONUS: usize = 2;

const METRONOME_STAGE_INTERVAL: usize = 2;

fn acquire_stage_marker(
    core: &mut crate::CoreState,
    mut upgrade: super::super::UpgradeEntry,
) -> usize {
    upgrade.metronome_mut().acquired_stage = core.progress.stage;
    push_acquired_upgrade(core, upgrade)
}

fn stage_start_metronome(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    stage: usize,
) -> bool {
    let acquired_stage = entry.metronome().acquired_stage;
    if stage.saturating_sub(acquired_stage) % METRONOME_STAGE_INTERVAL
        == METRONOME_STAGE_INTERVAL - 1
    {
        context.progress.left_dice = context.progress.left_dice.saturating_add(DICE_BONUS);
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Metronome
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Metronome(
            super::super::codec_impl::MetronomeUpgradeState { acquired_stage: 0 },
        )
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
        stage_start_metronome(context, entry, stage)
    }
}
