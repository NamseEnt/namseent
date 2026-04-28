mod upgrade_candidate_table;

use super::*;
use crate::game_state::GameState;
use rand::{seq::SliceRandom, thread_rng};
use upgrade_candidate_table::{
    generate_tower_damage_upgrade_candidate_table, generate_treasure_upgrade_candidate_table,
};

fn select_upgrade_from_candidates(
    upgrade_candidates: Vec<upgrade_candidate_table::CandidateRow>,
) -> Upgrade {
    let candidate = upgrade_candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.weight)
        .unwrap();
    let kind = (candidate.kind_gen)();

    Upgrade {
        kind,
        value: 1.0.into(),
    }
}

pub fn generate_tower_damage_upgrade(game_state: &GameState) -> Upgrade {
    select_upgrade_from_candidates(generate_tower_damage_upgrade_candidate_table(game_state))
}

pub fn generate_treasure_upgrade(game_state: &GameState) -> Upgrade {
    select_upgrade_from_candidates(generate_treasure_upgrade_candidate_table(game_state))
}
