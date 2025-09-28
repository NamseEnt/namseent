use super::types::RewardGeneratorFn;
use crate::game_state::contract::{
    constants::*,
    util::{distribute_per_stage, rarity_based_amount, rarity_table_random},
};
use crate::{game_state::effect::Effect, rarity::Rarity};

pub(crate) fn list() -> &'static [RewardGeneratorFn] {
    const FNS: &[RewardGeneratorFn] = &[
        |_, rarity, _| Effect::AddBarricadeCardsToTowerPlacementHand {
            count: rarity_based_amount(rarity, 1.0, 2.0, 3.0, 4.0) as usize,
        },
        |_, rarity, _| Effect::GainShield {
            min_amount: match rarity {
                Rarity::Common => 1.0,
                Rarity::Rare => 2.0,
                Rarity::Epic => 4.0,
                Rarity::Legendary => 7.0,
            },
            max_amount: match rarity {
                Rarity::Common => 2.0,
                Rarity::Rare => 3.0,
                Rarity::Epic => 6.0,
                Rarity::Legendary => 10.0,
            },
        },
        |_, rarity, _| Effect::HealHealth {
            min_amount: match rarity {
                Rarity::Common => 10.0,
                Rarity::Rare => 20.0,
                Rarity::Epic => 30.0,
                Rarity::Legendary => 40.0,
            },
            max_amount: match rarity {
                Rarity::Common => 14.0,
                Rarity::Rare => 24.0,
                Rarity::Epic => 34.0,
                Rarity::Legendary => 45.0,
            },
        },
        |rng, rarity, duration| {
            let total_gold = rarity_table_random(rng, rarity, &REWARD_EARN_GOLD);
            let (min_amount, max_amount) = distribute_per_stage(total_gold, duration);
            Effect::GainGold {
                min_amount,
                max_amount,
            }
        },
    ];
    FNS
}
