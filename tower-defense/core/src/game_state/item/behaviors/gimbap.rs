use super::support::{always_can_use, apply_heal_and_shield, no_prepare_use};
use super::{ItemBehavior, ItemRuntimeState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GimbapItemState {
    pub heal_raw: i64,
    pub shield_raw: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl ItemBehavior for Behavior {
    fn kind(&self) -> crate::ItemKind {
        crate::ItemKind::Gimbap
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generated_state(&self) -> ItemRuntimeState {
        ItemRuntimeState::Gimbap(GimbapItemState {
            heal_raw: 9_000,
            shield_raw: 9_000,
        })
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
        let ItemRuntimeState::Gimbap(state) = state else {
            return Err(crate::CommandError::Rejected);
        };
        apply_heal_and_shield(core, state.heal_raw, state.shield_raw)
    }
}
