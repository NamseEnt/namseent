use crate::{Rarity, game_state::item::ItemDiscriminants};
use strum::IntoEnumIterator;

pub(super) fn generate_item_rarity_candidate_table(rarity: Rarity) -> Vec<ItemDiscriminants> {
    ItemDiscriminants::iter()
        .filter(|discriminant| discriminant.rarity() == rarity)
        .collect()
}
