use super::support::{always_can_use, apply_heal, no_prepare_use};
use super::{ItemBehavior, ItemRuntimeState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CandyItemState {
    pub heal_raw: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl ItemBehavior for Behavior {
    fn kind(&self) -> crate::ItemKind {
        crate::ItemKind::Candy
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generated_state(&self) -> ItemRuntimeState {
        ItemRuntimeState::Candy(CandyItemState { heal_raw: 3_000 })
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
        prepared: Option<crate::TowerTemplateState>,
    ) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
        apply_heal(core, state, prepared)
    }
}
