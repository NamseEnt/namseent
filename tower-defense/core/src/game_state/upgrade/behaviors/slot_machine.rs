use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotMachineUpgradeState {
    pub dice: usize,
}

fn acquire_slot_machine(core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
    let add = upgrade.slot_machine().dice;
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.kind() == crate::UpgradeKind::SlotMachine)
    {
        existing.slot_machine_mut().dice = existing.slot_machine().dice.saturating_add(add);
    } else {
        let mut upgrade = upgrade;
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
    0
}

fn stage_start_slot_machine(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    _: usize,
) -> bool {
    let dice = entry.slot_machine().dice;
    if dice > 0 {
        context.progress.left_dice = context.progress.left_dice.saturating_add(dice);
        entry.slot_machine_mut().dice = 0;
        return true;
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::SlotMachine
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::SlotMachine(
            super::super::codec_impl::SlotMachineUpgradeState { dice: 10 },
        )
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_slot_machine(core, upgrade)
    }
    fn stage_start(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        stage: usize,
    ) -> bool {
        stage_start_slot_machine(context, entry, stage)
    }
}
