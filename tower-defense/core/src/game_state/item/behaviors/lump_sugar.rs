use super::super::definition::ItemDefinition;
use super::support::{generate_item, no_prepare_use, validate_one_scalar_value};
use crate::game_state::item::ItemEntryState;

fn generate() -> ItemEntryState {
    generate_item(crate::ItemKind::LumpSugar, &[1], &[])
}

fn can_use(core: &crate::CoreState) -> bool {
    matches!(
        core.flow,
        crate::GameFlowState::SelectingTower | crate::GameFlowState::Shopping(_)
    )
}

fn apply_use(
    core: &mut crate::CoreState,
    item: &ItemEntryState,
    _: Option<crate::TowerTemplateState>,
) -> Result<Vec<super::super::ItemUseEffect>, crate::CommandError> {
    let amount = super::support::scalar(item, 0)?;
    core.progress.left_dice = core.progress.left_dice.saturating_add(amount);
    Ok(vec![super::super::ItemUseEffect::GainRerolls { amount }])
}

pub(crate) const DEFINITION: ItemDefinition = ItemDefinition {
    kind: crate::ItemKind::LumpSugar,
    rarity: crate::Rarity::Epic,
    generate,
    validate: validate_one_scalar_value,
    can_use,
    prepare_use: no_prepare_use,
    apply_use,
};
