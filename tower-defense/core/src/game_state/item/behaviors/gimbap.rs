use super::super::definition::ItemDefinition;
use super::support::{
    always_can_use, apply_heal_and_shield, generate_item, no_prepare_use,
    validate_two_signed_values,
};
use crate::game_state::item::ItemEntryState;

fn generate() -> ItemEntryState {
    generate_item(crate::ItemKind::Gimbap, &[], &[9_000, 9_000])
}

pub(crate) const DEFINITION: ItemDefinition = ItemDefinition {
    kind: crate::ItemKind::Gimbap,
    rarity: crate::Rarity::Rare,
    generate,
    validate: validate_two_signed_values,
    can_use: always_can_use,
    prepare_use: no_prepare_use,
    apply_use: apply_heal_and_shield,
};
