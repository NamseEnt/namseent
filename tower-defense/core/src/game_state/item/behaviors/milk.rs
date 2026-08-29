use super::super::definition::ItemDefinition;
use super::support::{
    always_can_use, apply_heal, generate_item, no_prepare_use, validate_one_signed_value,
};
use crate::game_state::item::ItemEntryState;

fn generate() -> ItemEntryState {
    generate_item(crate::ItemKind::Milk, &[], &[12_000])
}

pub(crate) const DEFINITION: ItemDefinition = ItemDefinition {
    kind: crate::ItemKind::Milk,
    rarity: crate::Rarity::Rare,
    generate,
    validate: validate_one_signed_value,
    can_use: always_can_use,
    prepare_use: no_prepare_use,
    apply_use: apply_heal,
};
