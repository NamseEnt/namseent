use rand::prelude::*;
use crate::{game_state::effect::Effect, card::{Rank, SUITS}};
use super::types::RiskGeneratorFn;
use crate::game_state::contract::constants::*;

pub(crate) fn list() -> &'static [RiskGeneratorFn] {
    const FNS: &[RiskGeneratorFn] = &[
        |rng, _, _| Effect::DecreaseAllTowersDamage { multiplier: rng.gen_range(RISK_DECREASE_TOWER_DAMAGE.0..RISK_DECREASE_TOWER_DAMAGE.1) },
        |rng, _, _| Effect::IncreaseIncomingDamage { multiplier: rng.gen_range(RISK_INCREASE_INCOMING_DAMAGE.0..RISK_INCREASE_INCOMING_DAMAGE.1) },
        |rng, _, _| Effect::DecreaseGoldGainPercent { reduction_percentage: rng.gen_range(RISK_DECREASE_GOLD_GAIN_PERCENT.0..RISK_DECREASE_GOLD_GAIN_PERCENT.1) },
        |_, _, _| Effect::DisableItemAndUpgradePurchases,
        |_, _, _| Effect::DisableItemUse,
        |_, _, _| Effect::DecreaseCardSelectionHandMaxSlots { penalty: 1 },
        |_, _, _| Effect::DecreaseCardSelectionHandMaxRerolls { penalty: 1 },
        |_, _, _| Effect::DecreaseShopMaxRerolls { penalty: 1 },
        |rng, _, _| Effect::AddCardSelectionHandRerollHealthCost { cost: rng.gen_range(RISK_REROLL_HEALTH_COST.0..=RISK_REROLL_HEALTH_COST.1) as usize },
        |rng, _, _| Effect::AddShopRerollHealthCost { cost: rng.gen_range(RISK_REROLL_HEALTH_COST.0..=RISK_REROLL_HEALTH_COST.1) as usize },
        |_, _, _| Effect::DecreaseEnemyHealthPercent { percentage: RISK_DECREASE_ENEMY_HEALTH_PERCENT },
        |rng, _, _| {
            let ranks = [Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
            let rank = ranks.choose(rng).unwrap();
            Effect::RankTowerDisable { rank: *rank }
        },
        |rng, _, _| {
            let suit = SUITS.choose(rng).unwrap();
            Effect::SuitTowerDisable { suit: *suit }
        },
    ];
    FNS
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};
    use crate::rarity::Rarity;

    #[test]
    fn risk_while_active_generators_produce_effects() {
        let gens = list();
        assert!(!gens.is_empty());
        for (i, g) in gens.iter().enumerate() {
            let mut rng = StdRng::seed_from_u64(5000 + i as u64);
            let effect = g(&mut rng, Rarity::Common, 2);
            // basic sanity: matches one of expected variants by manual pattern subset
            match effect {
                crate::game_state::effect::Effect::DecreaseAllTowersDamage { .. }
                | crate::game_state::effect::Effect::IncreaseIncomingDamage { .. }
                | crate::game_state::effect::Effect::DecreaseGoldGainPercent { .. }
                | crate::game_state::effect::Effect::DisableItemAndUpgradePurchases
                | crate::game_state::effect::Effect::DisableItemUse
                | crate::game_state::effect::Effect::DecreaseCardSelectionHandMaxSlots { .. }
                | crate::game_state::effect::Effect::DecreaseCardSelectionHandMaxRerolls { .. }
                | crate::game_state::effect::Effect::DecreaseShopMaxRerolls { .. }
                | crate::game_state::effect::Effect::AddCardSelectionHandRerollHealthCost { .. }
                | crate::game_state::effect::Effect::AddShopRerollHealthCost { .. }
                | crate::game_state::effect::Effect::DecreaseEnemyHealthPercent { .. }
                | crate::game_state::effect::Effect::RankTowerDisable { .. }
                | crate::game_state::effect::Effect::SuitTowerDisable { .. } => {}
                other => panic!("Unexpected while_active risk effect: {:?}", other),
            }
        }
    }
}
