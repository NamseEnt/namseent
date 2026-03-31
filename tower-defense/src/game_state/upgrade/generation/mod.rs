mod upgrade_candidate_table;

use super::*;
use crate::{game_state::GameState, rarity::Rarity};
use rand::{Rng, seq::SliceRandom, thread_rng};
use upgrade_candidate_table::{
    generate_tower_damage_upgrade_candidate_table, generate_treasure_upgrade_candidate_table,
};

fn select_upgrade_from_candidates(
    upgrade_candidates: Vec<upgrade_candidate_table::CandidateRow>,
    rarity: Rarity,
) -> Upgrade {
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

pub fn generate_tower_damage_upgrade(game_state: &GameState, rarity: Rarity) -> Upgrade {
    select_upgrade_from_candidates(
        generate_tower_damage_upgrade_candidate_table(game_state, rarity),
        rarity,
    )
}

pub fn generate_treasure_upgrade(game_state: &GameState, rarity: Rarity) -> Upgrade {
    select_upgrade_from_candidates(
        generate_treasure_upgrade_candidate_table(game_state, rarity),
        rarity,
    )
}
