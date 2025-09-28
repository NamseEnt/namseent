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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};
    use crate::rarity::Rarity;

    #[test]
    fn reward_on_sign_generators_return_expected_variants() {
        let gens: &[RewardGeneratorFn] = list();
        assert!(!gens.is_empty());
        for (i, generator) in gens.iter().enumerate() {
            let mut rng = StdRng::seed_from_u64(1000 + i as u64);
            let effect = generator(&mut rng, Rarity::Common, 3);
            match effect {
                crate::game_state::effect::Effect::Heal { amount } => assert!(amount > 0.0),
                crate::game_state::effect::Effect::EarnGold { amount } => assert!(amount > 0),
                crate::game_state::effect::Effect::GrantUpgrade { .. } => {}
                crate::game_state::effect::Effect::GrantItem { .. } => {}
                other => panic!("Unexpected effect variant: {:?}", other),
            }
        }
    }
}
