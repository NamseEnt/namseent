use crate::{game_state::effect::Effect, rarity::Rarity};
use rand::RngCore;

pub type RewardGeneratorFn =
    fn(rng: &mut dyn RngCore, rarity: Rarity, duration_stages: usize) -> Effect;
