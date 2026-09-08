use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorUpgradeState {
    pub pending: bool,
}

fn tower_placed_free_item(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    _: u64,
    _: bool,
    tower_template: &crate::TowerTemplateState,
    _: &mut usize,
) -> bool {
    if entry.mirror().pending {
        let id = context.hand.allocate_slot_id();
        context.hand.slots.push(crate::HandSlotState {
            id,
            item: crate::HandItemState::Tower(tower_template.clone()),
            selected: false,
        });
        context.hand.sort_slots();
        entry.mirror_mut().pending = false;
        return true;
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Mirror
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Mirror(super::super::codec_impl::MirrorUpgradeState {
            pending: true,
        })
    }
    fn tower_placed(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        tower_id: u64,
        is_face: bool,
        template: &crate::TowerTemplateState,
        reward: &mut usize,
    ) -> bool {
        tower_placed_free_item(context, entry, tower_id, is_face, template, reward)
    }
}
