use crate::game_state::effect::Effect;
use super::types::RiskGeneratorFn;
use crate::game_state::contract::{constants::*, util::rarity_table_random};

pub(crate) fn list() -> &'static [RiskGeneratorFn] {
    const FNS: &[RiskGeneratorFn] = &[
        |rng, rarity, _| Effect::LoseHealth { amount: rarity_table_random(rng, rarity, &RISK_LOSE_HEALTH) },
        |rng, rarity, _| Effect::LoseGold { amount: rarity_table_random(rng, rarity, &RISK_LOSE_GOLD) as usize },
        |_, _, _| Effect::AddChallengeMonster,
    ];
    FNS
}
