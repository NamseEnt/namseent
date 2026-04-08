use crate::game_state::GameState;
use crate::game_state::upgrade::Upgrade;
use rand::{Rng, RngCore};

/// Strategy for selecting treasure options.
pub trait TreasureStrategy: Send + Sync {
    fn name(&self) -> &str;
    fn select_treasure(
        &self,
        _game_state: &GameState,
        options: &[Upgrade],
        rng: &mut dyn RngCore,
    ) -> usize;
}

/// Randomly selects one of the available treasure options.
pub struct RandomTreasureStrategy;

impl TreasureStrategy for RandomTreasureStrategy {
    fn name(&self) -> &str {
        "random_treasure"
    }

    fn select_treasure(
        &self,
        _game_state: &GameState,
        options: &[Upgrade],
        rng: &mut dyn RngCore,
    ) -> usize {
        if options.is_empty() {
            0
        } else {
            rng.gen_range(0..options.len())
        }
    }
}
