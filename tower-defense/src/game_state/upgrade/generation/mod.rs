mod upgrade_candidate_table;

use super::*;
use crate::game_state::{
    GameState,
    upgrade::generation::upgrade_candidate_table::{
        UpgradeRarityWeights, generate_upgrade_candidate_table,
    },
};
use rand::seq::SliceRandom;

fn select_upgrade_from_candidates(
    upgrade_candidates: Vec<upgrade_candidate_table::CandidateRow>,
) -> Upgrade {
    upgrade_candidates
        .choose_weighted(&mut rand::thread_rng(), |x| x.weight)
        .unwrap()
        .upgrade
}

pub fn generate_shop_upgrade(game_state: &GameState) -> Upgrade {
    select_upgrade_from_candidates(generate_upgrade_candidate_table(
        game_state,
        UpgradeRarityWeights::shop(),
    ))
}

pub fn generate_boss_reward_upgrade(game_state: &GameState) -> Upgrade {
    select_upgrade_from_candidates(generate_upgrade_candidate_table(
        game_state,
        UpgradeRarityWeights::boss_reward(),
    ))
}
