use crate::{Rarity, game_state::card_service::CardService};

#[derive(Clone, Copy)]
pub(super) struct CardServiceDefinition {
    generate: fn() -> CardService,
    rarity: fn() -> Rarity,
}

impl CardServiceDefinition {
    pub(super) const fn new(generate: fn() -> CardService, rarity: fn() -> Rarity) -> Self {
        Self { generate, rarity }
    }

    pub fn generate(self) -> CardService {
        (self.generate)()
    }

    pub(super) fn rarity(self) -> Rarity {
        (self.rarity)()
    }
}
