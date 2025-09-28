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
    DecreaseGoldGainPercent {
        reduction_percentage: f32,
    },
    DisableItemAndUpgradePurchases,
    DisableItemUse,
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
    DecreaseEnemyHealthPercent {
        percentage: f32,
    },
    RankTowerDisable {
        rank: Rank,
    },
    SuitTowerDisable {
        suit: Suit,
    },
    AddBarricadeCardsToTowerPlacementHand {
        count: usize,
    },
    GainShield {
        min_amount: f32,
        max_amount: f32,
    },
    HealHealth {
        min_amount: f32,
        max_amount: f32,
    },
    GainGold {
        min_amount: f32,
        max_amount: f32,
    },
    LoseHealthRange {
        min_amount: f32,
        max_amount: f32,
    },
    LoseGoldRange {
        min_amount: f32,
        max_amount: f32,
    },
    LoseHealthExpire {
        min_amount: f32,
        max_amount: f32,
    },
    LoseGoldExpire {
        min_amount: f32,
        max_amount: f32,
    },
}

pub fn run_effect(game_state: &mut GameState, effect: &Effect) {
    use rand::thread_rng;
    let mut rng = thread_rng();
    run_effect_with_rng(game_state, effect, &mut rng);
}

/// 테스트 및 결정적(Deterministic) 실행을 위해 RNG를 주입할 수 있는 버전.
/// 기존 `run_effect` 는 thread_rng() 를 사용하며, 이 함수는 재사용 가능한 코어 로직을 담는다.
pub fn run_effect_with_rng<R: rand::Rng + ?Sized>(
    game_state: &mut GameState,
    effect: &Effect,
    rng: &mut R,
) {
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
            let is_winner = rng.gen_bool(*probability as f64);
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
        Effect::LoseHealthRange {
            min_amount,
            max_amount,
        } => {
            let amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.hp = (game_state.hp - amount).max(1.0);
        }
        Effect::LoseGoldRange {
            min_amount,
            max_amount,
        } => {
            let amount = rng.gen_range(*min_amount..=*max_amount) as usize;
            if game_state.gold >= amount {
                game_state.gold -= amount;
            } else {
                let remaining = amount - game_state.gold;
                game_state.gold = 0;
                let health_penalty = (remaining as f32 / 10.0).max(1.0);
                game_state.hp = (game_state.hp - health_penalty).max(1.0);
            }
        }
        Effect::LoseHealthExpire {
            min_amount,
            max_amount,
        } => {
            let amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.hp = (game_state.hp - amount).max(1.0);
        }
        Effect::LoseGoldExpire {
            min_amount,
            max_amount,
        } => {
            let amount = rng.gen_range(*min_amount..=*max_amount) as usize;
            if game_state.gold >= amount {
                game_state.gold -= amount;
            } else {
                let remaining = amount - game_state.gold;
                game_state.gold = 0;
                let health_penalty = (remaining as f32 / 10.0).max(1.0);
                game_state.hp = (game_state.hp - health_penalty).max(1.0);
            }
        }
        Effect::LoseGold { amount } => {
            if game_state.gold >= *amount {
                game_state.gold -= *amount;
            } else {
                let remaining = *amount - game_state.gold;
                game_state.gold = 0;
                let health_penalty = (remaining as f32 / 10.0).max(1.0);
                game_state.hp = (game_state.hp - health_penalty).max(1.0);
            }
        }
        Effect::GrantUpgrade { rarity } => {
            let upgrade = crate::game_state::upgrade::generate_upgrade(game_state, *rarity);
            game_state.upgrade_state.upgrade(upgrade);
        }
        Effect::GrantItem { rarity } => {
            // 간단히 아이템 생성: 현재 아이템 생성 로직이 별도로 있다면 그 경로로 위임 고려
            // 여기서는 placeholder Effect 하나(ExtraReroll)로 rarity만 반영
            let item = crate::game_state::item::Item {
                effect: Effect::ExtraReroll,
                rarity: *rarity,
                value: 0.0.into(),
            };
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
        Effect::DecreaseGoldGainPercent {
            reduction_percentage,
        } => {
            game_state
                .contract_state
                .apply_gold_gain_multiplier(1.0 - *reduction_percentage);
        }
        Effect::DisableItemAndUpgradePurchases => {
            game_state
                .contract_state
                .disable_item_and_upgrade_purchases();
        }
        Effect::DisableItemUse => {
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
        Effect::DecreaseEnemyHealthPercent { percentage } => {
            let multiplier = 1.0 + percentage / 100.0;
            game_state
                .contract_state
                .apply_enemy_health_multiplier(multiplier);
        }
        Effect::RankTowerDisable { rank } => {
            game_state.contract_state.disable_rank(*rank);
        }
        Effect::SuitTowerDisable { suit } => {
            game_state.contract_state.disable_suit(*suit);
        }
        Effect::AddBarricadeCardsToTowerPlacementHand { count } => {
            // This effect is handled in the stage start logic
            game_state
                .contract_state
                .set_barricade_cards_per_stage(*count);
        }
        Effect::GainShield {
            min_amount,
            max_amount,
        } => {
            let shield_amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.shield += shield_amount;
        }
        Effect::HealHealth {
            min_amount,
            max_amount,
        } => {
            let heal_amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.hp = (game_state.hp + heal_amount).min(crate::game_state::MAX_HP);
        }
        Effect::GainGold {
            min_amount,
            max_amount,
        } => {
            let gold_amount = rng.gen_range(*min_amount..=*max_amount) as usize;
            game_state.gold += gold_amount;
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

// ============================= Test Helpers =============================
#[cfg(test)]
pub mod tests_support {
    use crate::game_state::{
        ContractState, GameState, MAP_SIZE, TRAVEL_POINTS, field_particle, flow::GameFlow,
        monster_spawn::MonsterSpawnState,
    };
    use namui::Instant;
    use std::num::NonZeroUsize; // use the same Instant type as production code

    /// 테스트용 GameState 생성 헬퍼.
    /// - Atom / 렌더 컨텍스트에 의존하지 않음.
    /// - 필요한 최소 필드만 초기화.
    #[allow(dead_code)]
    pub fn make_test_state() -> GameState {
        GameState {
            monsters: Default::default(),
            towers: Default::default(),
            camera: crate::game_state::camera::Camera::new(),
            route: crate::game_state::calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
            backgrounds: crate::game_state::generate_backgrounds(),
            upgrade_state: Default::default(),
            flow: GameFlow::Initializing,
            stage: 1,
            left_reroll_chance: 1,
            monster_spawn_state: MonsterSpawnState::Idle,
            projectiles: Default::default(),
            items: vec![],
            gold: 0,
            cursor_preview: Default::default(),
            hp: 100.0,
            shield: 0.0,
            user_status_effects: Default::default(),
            left_shop_refresh_chance: 0,
            left_quest_board_refresh_chance: 0,
            item_used: false,
            level: NonZeroUsize::new(1).unwrap(),
            game_now: Instant::now(),
            fast_forward_multiplier: Default::default(),
            rerolled_count: 0,
            selected_tower_id: None,
            field_particle_system_manager: field_particle::FieldParticleSystemManager::default(),
            locale: crate::l10n::Locale::KOREAN,
            play_history: crate::game_state::play_history::PlayHistory::new(),
            opened_modal: None,
            contracts: vec![],
            contract_state: ContractState::new(),
        }
    }
}

// Aggregate test modules sitting under `effect/tests/` directory
#[cfg(test)]
mod tests {
    mod card_selection_reroll_and_slots;
    mod random_effects_deterministic;
    mod run_effect_integration;
    mod shop_reroll;
}
