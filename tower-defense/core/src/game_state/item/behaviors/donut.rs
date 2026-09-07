use super::support::{always_can_use, apply_heal, no_prepare_use};
use super::{ItemBehavior, ItemRuntimeState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DonutItemState {
    pub heal_raw: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl ItemBehavior for Behavior {
    fn kind(&self) -> crate::ItemKind {
        crate::ItemKind::Donut
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generated_state(&self) -> ItemRuntimeState {
        ItemRuntimeState::Donut(DonutItemState { heal_raw: 7_000 })
    }
    fn can_use(&self, core: &crate::CoreState) -> bool {
        always_can_use(core)
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
        _: Option<crate::TowerTemplateState>,
    ) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
        let ItemRuntimeState::Donut(state) = state else {
            return Err(crate::CommandError::Rejected);
        };
        apply_heal(core, state.heal_raw)
    }
}
