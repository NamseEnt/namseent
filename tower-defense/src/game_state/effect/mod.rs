use crate::card::{Rank, Suit};
use crate::game_state::{
    GameState,
    user_status_effect::{UserStatusEffect, UserStatusEffectKind},
};
use crate::rarity::Rarity;
use namui::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    Heal {
        amount: f32,
    },
    Shield {
        amount: f32,
    },
    ExtraReroll,
    EarnGold {
        amount: usize,
    },
    Lottery {
        amount: f32,
        probability: f32,
    },
    DamageReduction {
        damage_multiply: f32,
        duration: Duration,
    },
    UserDamageReduction {
        multiply: f32,
        duration: Duration,
    },
    LoseHealth {
        amount: f32,
    },
    LoseGold {
        amount: usize,
    },
    GrantUpgrade {
        rarity: Rarity,
    },
    GrantItem {
        rarity: Rarity,
    },
    AddChallengeMonster,
    IncreaseAllTowersDamage {
        multiplier: f32,
    },
    DecreaseAllTowersDamage {
        multiplier: f32,
    },
    IncreaseIncomingDamage {
        multiplier: f32,
    },
    IncreaseAllTowersAttackSpeed {
        multiplier: f32,
    },
    IncreaseAllTowersRange {
        multiplier: f32,
    },
    DecreaseIncomingDamage {
        multiplier: f32,
    },
    IncreaseGoldGain {
        multiplier: f32,
    },
    DecreaseGoldGainPercentDuringContract {
        reduction_percentage: f32,
    },
    DisableItemAndUpgradePurchasesDuringContract,
    DisableItemUseDuringContract,
    IncreaseCardSelectionHandMaxSlots {
        bonus: usize,
    },
    DecreaseCardSelectionHandMaxSlots {
        penalty: usize,
    },
    IncreaseCardSelectionHandMaxRerolls {
        bonus: usize,
    },
    DecreaseCardSelectionHandMaxRerolls {
        penalty: usize,
    },
    IncreaseShopMaxRerolls {
        bonus: usize,
    },
    DecreaseShopMaxRerolls {
        penalty: usize,
    },
    AddCardSelectionHandRerollHealthCost {
        cost: usize,
    },
    AddShopRerollHealthCost {
        cost: usize,
    },
    DecreaseEnemyHealthPercentDuringContract {
        percentage: f32,
    },
    RankTowerDisableDuringContract {
        rank: Rank,
    },
    SuitTowerDisableDuringContract {
        suit: Suit,
    },
    AddBarricadeCardsToTowerPlacementHandEachStageDuringContract {
        count: usize,
    },
    GainShieldEachStageDuringContract {
        min_amount: f32,
        max_amount: f32,
    },
    HealHealthEachStageDuringContract {
        min_amount: f32,
        max_amount: f32,
    },
}

pub fn run_effect(game_state: &mut GameState, effect: &Effect) {
    match effect {
        Effect::Heal { amount } => {
            game_state.hp = (game_state.hp + amount).min(crate::game_state::MAX_HP);
        }
        Effect::Shield { amount } => {
            game_state.shield += amount;
        }
        Effect::ExtraReroll => {
            game_state.left_reroll_chance += 1;
        }
        Effect::EarnGold { amount } => {
            game_state.gold = game_state.gold.saturating_add(*amount);
        }
        Effect::Lottery {
            amount,
            probability,
        } => {
            use rand::{Rng, thread_rng};
            let is_winner = thread_rng().gen_bool(*probability as f64);
            let gold = if is_winner { *amount as usize } else { 0 };
            game_state.earn_gold(gold);
        }
        Effect::DamageReduction {
            damage_multiply,
            duration,
        } => {
            let status_effect = UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply: *damage_multiply,
                },
                end_at: game_state.now() + *duration,
            };
            game_state.user_status_effects.push(status_effect);
        }
        Effect::UserDamageReduction { multiply, duration } => {
            let status_effect = UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply: *multiply,
                },
                end_at: game_state.now() + *duration,
            };
            game_state.user_status_effects.push(status_effect);
        }
        Effect::LoseHealth { amount } => {
            game_state.hp = (game_state.hp - amount).max(1.0);
        }
        Effect::LoseGold { amount } => {
            if game_state.gold >= *amount {
                game_state.gold -= *amount;
            } else {
                let remaining = *amount - game_state.gold;
                game_state.gold = 0;
                game_state.hp = (game_state.hp - (remaining as f32 / 10.0)).max(1.0);
            }
        }
        Effect::GrantUpgrade { rarity } => {
            let upgrade = crate::game_state::upgrade::generate_upgrade(game_state, *rarity);
            game_state.upgrade_state.upgrade(upgrade);
        }
        Effect::GrantItem { rarity } => {
            let item = crate::game_state::item::generation::generate_item(*rarity);
            game_state.items.push(item);
        }
        Effect::AddChallengeMonster => {
            unimplemented!("AddChallengeMonster effect is not implemented yet");
        }
        Effect::IncreaseAllTowersDamage { multiplier } => {
            game_state
                .contract_state
                .apply_damage_multiplier(*multiplier);
        }
        Effect::DecreaseAllTowersDamage { multiplier } => {
            game_state
                .contract_state
                .apply_damage_multiplier(*multiplier);
        }
        Effect::IncreaseIncomingDamage { multiplier } => {
            game_state
                .contract_state
                .apply_incoming_damage_multiplier(*multiplier);
        }
        Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
            game_state
                .contract_state
                .apply_attack_speed_multiplier(*multiplier);
        }
        Effect::IncreaseAllTowersRange { multiplier } => {
            game_state
                .contract_state
                .apply_range_multiplier(*multiplier);
        }
        Effect::DecreaseIncomingDamage { multiplier } => {
            game_state
                .contract_state
                .apply_damage_reduction_multiplier(*multiplier);
        }
        Effect::IncreaseGoldGain { multiplier } => {
            game_state
                .contract_state
                .apply_gold_gain_multiplier(*multiplier);
        }
        Effect::DecreaseGoldGainPercentDuringContract {
            reduction_percentage,
        } => {
            game_state
                .contract_state
                .apply_gold_gain_multiplier(1.0 - *reduction_percentage);
        }
        Effect::DisableItemAndUpgradePurchasesDuringContract => {
            game_state
                .contract_state
                .disable_item_and_upgrade_purchases();
        }
        Effect::DisableItemUseDuringContract => {
            game_state.contract_state.disable_item_use();
        }
        Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
            game_state
                .contract_state
                .apply_card_selection_hand_max_slots_penalty(*penalty);
        }
        Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
            game_state
                .contract_state
                .apply_card_selection_hand_max_slots_bonus(*bonus);
        }
        Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
            game_state
                .contract_state
                .apply_card_selection_hand_max_rerolls_bonus(*bonus);
        }
        Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
            game_state
                .contract_state
                .apply_card_selection_hand_max_rerolls_penalty(*penalty);
        }
        Effect::IncreaseShopMaxRerolls { bonus } => {
            game_state
                .contract_state
                .apply_shop_max_rerolls_bonus(*bonus);
        }
        Effect::DecreaseShopMaxRerolls { penalty } => {
            game_state
                .contract_state
                .apply_shop_max_rerolls_penalty(*penalty);
        }
        Effect::AddCardSelectionHandRerollHealthCost { cost } => {
            game_state
                .contract_state
                .apply_card_selection_hand_reroll_health_cost(*cost);
        }
        Effect::AddShopRerollHealthCost { cost } => {
            game_state
                .contract_state
                .apply_shop_reroll_health_cost(*cost);
        }
        Effect::DecreaseEnemyHealthPercentDuringContract { percentage } => {
            let multiplier = 1.0 + percentage / 100.0;
            game_state
                .contract_state
                .apply_enemy_health_multiplier(multiplier);
        }
        Effect::RankTowerDisableDuringContract { rank } => {
            game_state.contract_state.disable_rank(*rank);
        }
        Effect::SuitTowerDisableDuringContract { suit } => {
            game_state.contract_state.disable_suit(*suit);
        }
        Effect::AddBarricadeCardsToTowerPlacementHandEachStageDuringContract { count } => {
            // This effect is handled in the stage start logic
            game_state
                .contract_state
                .set_barricade_cards_per_stage(*count);
        }
        Effect::GainShieldEachStageDuringContract {
            min_amount,
            max_amount,
        } => {
            use rand::{Rng, thread_rng};
            let mut rng = thread_rng();
            let shield_amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.shield += shield_amount;
        }
        Effect::HealHealthEachStageDuringContract {
            min_amount,
            max_amount,
        } => {
            use rand::{Rng, thread_rng};
            let mut rng = thread_rng();
            let heal_amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.hp = (game_state.hp + heal_amount).min(crate::game_state::MAX_HP);
        }
    }
}

impl Effect {
    pub fn name(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.effect_name(self)
    }

    pub fn description(&self, text_manager: &crate::l10n::TextManager) -> String {
        text_manager.effect_description(self)
    }
}
