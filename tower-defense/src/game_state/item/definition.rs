use crate::{Rarity, game_state::item::Item};
use rand::RngCore;

#[derive(Clone, Copy)]
pub(super) struct ItemDefinition {
    generate: fn(&mut dyn RngCore) -> Item,
    rarity: fn() -> Rarity,
}

impl ItemDefinition {
    pub(super) const fn new(
        generate: fn(&mut dyn RngCore) -> Item,
        rarity: fn() -> Rarity,
    ) -> Self {
        Self { generate, rarity }
    }

    pub(super) fn generate(self, rng: &mut dyn RngCore) -> Item {
        (self.generate)(rng)
    }

    pub(super) fn rarity(self) -> Rarity {
        (self.rarity)()
    }
}
