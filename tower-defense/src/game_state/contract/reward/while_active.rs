use super::types::RewardGeneratorFn;
use crate::game_state::contract::{constants::*, util::rarity_table_random};
use crate::game_state::effect::Effect;

pub(crate) fn list() -> &'static [RewardGeneratorFn] {
    const FNS: &[RewardGeneratorFn] = &[
        |rng, rarity, _| Effect::IncreaseAllTowersDamage {
            multiplier: rarity_table_random(rng, rarity, &REWARD_INCREASE_TOWER_DAMAGE),
        },
        |rng, rarity, _| Effect::DecreaseIncomingDamage {
            multiplier: rarity_table_random(rng, rarity, &REWARD_DECREASE_INCOMING_DAMAGE),
        },
        |rng, rarity, _| Effect::IncreaseGoldGain {
            multiplier: rarity_table_random(rng, rarity, &REWARD_INCREASE_GOLD_GAIN),
        },
        |_, _, _| Effect::IncreaseMaxHandSlots { bonus: 1 },
        |_, _, _| Effect::IncreaseMaxRerolls { bonus: 1 },
        |_, _, _| Effect::IncreaseMaxRerolls { bonus: 1 },
    ];
    FNS
}
