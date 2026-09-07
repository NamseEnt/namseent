use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MembershipCardUpgradeState {
    pub pending: bool,
}

fn stage_start_free_shop(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    _: usize,
) -> bool {
    if entry.membership_card().pending {
        context.stage_modifiers.free_shop_this_stage = true;
        entry.membership_card_mut().pending = false;
        return true;
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::MembershipCard
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::MembershipCard(
            super::super::codec_impl::MembershipCardUpgradeState { pending: true },
        )
    }
    fn stage_start(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        stage: usize,
    ) -> bool {
        stage_start_free_shop(context, entry, stage)
    }
}
