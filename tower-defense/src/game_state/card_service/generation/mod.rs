mod candidate_table;

use crate::game_state::card_service::CardService;
use crate::game_state::card_service::generation::candidate_table::{
    CandidateRow, RarityWeights, generate_candidate_table,
};
use rand::seq::SliceRandom;

pub fn generate_shop_card_service() -> CardService {
    select_from_candidates(generate_candidate_table(RarityWeights::shop()))
}

fn select_from_candidates(upgrade_candidates: Vec<CandidateRow>) -> CardService {
    upgrade_candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.weight)
        .unwrap()
        .card_service
        .clone()
}
