use crate::game_state::effect::Effect;
use super::types::RewardGeneratorFn;
use crate::game_state::contract::{constants::*, util::rarity_table_random};

pub(crate) fn list() -> &'static [RewardGeneratorFn] {
    const FNS: &[RewardGeneratorFn] = &[
        |rng, rarity, _| Effect::Heal { amount: rarity_table_random(rng, rarity, &REWARD_HEAL_ON_SIGN) },
        |rng, rarity, _| Effect::EarnGold { amount: rarity_table_random(rng, rarity, &REWARD_EARN_GOLD) as usize },
        |_, rarity, _| Effect::GrantUpgrade { rarity },
        |_, rarity, _| Effect::GrantItem { rarity },
    ];
    FNS
}
