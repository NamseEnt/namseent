use super::types::RiskGeneratorFn;
use crate::game_state::contract::util::rarity_based_amount;
use crate::game_state::effect::Effect;

pub(crate) fn list() -> &'static [RiskGeneratorFn] {
    const FNS: &[RiskGeneratorFn] = &[
        |_, rarity, _| Effect::LoseHealthExpire {
            min_amount: rarity_based_amount(rarity, 5.0, 10.0, 15.0, 20.0),
            max_amount: rarity_based_amount(rarity, 9.0, 14.0, 19.0, 25.0),
        },
        |_, rarity, _| Effect::LoseGoldExpire {
            min_amount: rarity_based_amount(rarity, 125.0, 250.0, 500.0, 1000.0),
            max_amount: rarity_based_amount(rarity, 150.0, 300.0, 750.0, 1500.0),
        },
    ];
    FNS
}
