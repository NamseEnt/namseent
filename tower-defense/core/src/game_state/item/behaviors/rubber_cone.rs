use super::support::always_can_use;
use super::{ItemBehavior, ItemRuntimeState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RubberConeItemState {
    pub count: usize,
}

fn prepare_use(
    core: &crate::CoreState,
) -> Result<Option<crate::TowerTemplateState>, crate::CommandError> {
    Ok(Some(core.rubber_cone_template()?))
}

fn apply_use(
    core: &mut crate::CoreState,
    state: ItemRuntimeState,
    prepared: Option<crate::TowerTemplateState>,
) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
    let super::ItemRuntimeState::RubberCone(state) = state else {
        return Err(crate::CommandError::Rejected);
    };
    let count = state.count;
    let tower = prepared.expect("rubber cone template was prepared for the item");
    if matches!(core.flow, crate::GameFlowState::PlacingTower) {
        for _ in 0..count {
            let id = core.hand.allocate_slot_id();
            core.hand.slots.push(crate::HandSlotState {
                id,
                item: crate::HandItemState::Tower(tower.clone()),
                selected: false,
            });
        }
        core.hand.sort_slots();
    } else {
        for _ in 0..count {
            core.stage_modifiers
                .extra_tower_cards
                .push(crate::StageModifierTowerCardState {
                    kind: 0,
                    suit: None,
                    rank: None,
                });
        }
    }
    Ok(vec![super::super::ItemUseEffect::GrantTowerCards {
        tower_kind: 0,
        count,
    }])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl ItemBehavior for Behavior {
    fn kind(&self) -> crate::ItemKind {
        crate::ItemKind::RubberCone
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generated_state(&self) -> ItemRuntimeState {
        ItemRuntimeState::RubberCone(RubberConeItemState { count: 4 })
    }
    fn can_use(&self, core: &crate::CoreState) -> bool {
        always_can_use(core)
    }
    fn prepare_use(
        &self,
        core: &crate::CoreState,
    ) -> Result<Option<crate::TowerTemplateState>, crate::CommandError> {
        prepare_use(core)
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
