use super::support::no_prepare_use;
use super::{ItemBehavior, ItemRuntimeState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LumpSugarItemState {
    pub amount: usize,
}

fn can_use(core: &crate::CoreState) -> bool {
    matches!(
        core.flow,
        crate::GameFlowState::SelectingTower | crate::GameFlowState::Shopping(_)
    )
}

fn apply_use(
    core: &mut crate::CoreState,
    state: super::ItemRuntimeState,
    _: Option<crate::TowerTemplateState>,
) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
    let super::ItemRuntimeState::LumpSugar(state) = state else {
        return Err(crate::CommandError::Rejected);
    };
    let amount = state.amount;
    core.progress.left_dice = core.progress.left_dice.saturating_add(amount);
    Ok(vec![super::super::ItemUseEffect::GainRerolls { amount }])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl ItemBehavior for Behavior {
    fn kind(&self) -> crate::ItemKind {
        crate::ItemKind::LumpSugar
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generated_state(&self) -> ItemRuntimeState {
        ItemRuntimeState::LumpSugar(LumpSugarItemState { amount: 1 })
    }
    fn can_use(&self, core: &crate::CoreState) -> bool {
        can_use(core)
    }
    fn prepare_use(
        &self,
        core: &crate::CoreState,
    ) -> Result<Option<crate::TowerTemplateState>, crate::CommandError> {
        no_prepare_use(core)
    }
    fn apply_use(
        &self,
        core: &mut crate::CoreState,
        state: ItemRuntimeState,
        prepared: Option<crate::TowerTemplateState>,
    ) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
        apply_use(core, state, prepared)
    }
}
