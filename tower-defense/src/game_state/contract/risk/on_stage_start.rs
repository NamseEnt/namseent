use crate::game_state::effect::Effect;
use super::types::RiskGeneratorFn;
use crate::game_state::contract::{constants::*, util::{rarity_table_random, distribute_per_stage}};

pub(crate) fn list() -> &'static [RiskGeneratorFn] {
    const FNS: &[RiskGeneratorFn] = &[
        |rng, rarity, duration| {
            let total = rarity_table_random(rng, rarity, &RISK_LOSE_HEALTH);
            let (min_amount, max_amount) = distribute_per_stage(total, duration);
            Effect::LoseHealthRange { min_amount, max_amount }
        },
        |rng, rarity, duration| {
            let total = rarity_table_random(rng, rarity, &RISK_STAGE_LOSE_GOLD);
            let (min_amount, max_amount) = distribute_per_stage(total, duration);
            Effect::LoseGoldRange { min_amount, max_amount }
        },
    ];
    FNS
}
