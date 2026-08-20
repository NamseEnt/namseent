mod upgrade_candidate_table;

use super::*;
use crate::game_state::{
    GameState,
    upgrade::generation::upgrade_candidate_table::{
        UpgradeRarityWeights, generate_upgrade_candidate_table,
    },
};
use rand::Rng;
use rand::seq::SliceRandom;

fn select_upgrade_from_candidates<R: Rng>(
    upgrade_candidates: Vec<upgrade_candidate_table::CandidateRow>,
    rng: &mut R,
) -> Upgrade {
    upgrade_candidates
        .choose_weighted(rng, |x| x.weight)
        .unwrap()
        .upgrade
}

pub fn generate_boss_reward_upgrade(game_state: &mut GameState) -> Upgrade {
    let mut rng = game_state.rng.next_rng(
        crate::deterministic_rng::domain::REWARD_UPGRADE,
        &[game_state.stage as u64],
    );
    select_upgrade_from_candidates(
        generate_upgrade_candidate_table(game_state, UpgradeRarityWeights::boss_reward()),
        &mut rng,
    )
}
