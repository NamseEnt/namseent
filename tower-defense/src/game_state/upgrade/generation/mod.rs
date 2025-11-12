mod upgrade_candidate_table;

use super::*;
use crate::{game_state::GameState, rarity::Rarity};
use rand::{Rng, seq::SliceRandom, thread_rng};
use upgrade_candidate_table::generate_upgrade_candidate_table;

pub fn generate_upgrade(game_state: &GameState, rarity: Rarity) -> Upgrade {
    let upgrade_candidates = generate_upgrade_candidate_table(game_state, rarity);
    let candidate = upgrade_candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.weight)
        .unwrap();
    let kind = (candidate.kind_gen)(rarity);
    let value = thread_rng().gen_range(0.0..=1.0);

    Upgrade {
        kind,
        rarity,
        value: value.into(),
    }
}
