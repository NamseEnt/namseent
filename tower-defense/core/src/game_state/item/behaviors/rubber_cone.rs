use super::super::definition::ItemDefinition;
use super::support::{always_can_use, generate_item, validate_one_scalar_value};
use crate::game_state::item::ItemEntryState;

fn generate() -> ItemEntryState {
    generate_item(crate::ItemKind::RubberCone, &[4], &[])
}

fn prepare_use(
    core: &crate::CoreState,
    _: &ItemEntryState,
) -> Result<Option<crate::TowerTemplateState>, crate::CommandError> {
    Ok(Some(core.rubber_cone_template()?))
}

fn apply_use(
    core: &mut crate::CoreState,
    item: &ItemEntryState,
    prepared: Option<crate::TowerTemplateState>,
) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
    let count = super::support::scalar(item, 0)?;
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

pub(crate) const DEFINITION: ItemDefinition = ItemDefinition {
    kind: crate::ItemKind::RubberCone,
    rarity: crate::Rarity::Rare,
    generate,
    validate: validate_one_scalar_value,
    can_use: always_can_use,
    prepare_use,
    apply_use,
};
