use crate::game_state::{
    GameState,
    card::{Rank, Suit},
    stage_modifiers::StageModifiers,
    user_status_effect::{UserStatusEffect, UserStatusEffectKind},
};
use crate::{FixedRatio, Health, Shield, SimTickSpan};

use crate::rarity::Rarity;
use namui::*;

fn percentage_to_multiplier(percentage: FixedRatio) -> FixedRatio {
    FixedRatio::ONE.increased_by_percent(percentage)
}

#[derive(Clone, Debug, PartialEq, State)]
pub enum Effect {
    Heal {
        amount: Health,
    },
    Shield {
        amount: Shield,
    },
    EarnGold {
        amount: usize,
    },
    DamageReduction {
        damage_multiply: FixedRatio,
        duration: SimTickSpan,
    },
    LoseHealth {
        amount: Health,
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
    IncreaseAllTowersDamage {
        multiplier: FixedRatio,
    },
    DecreaseAllTowersDamage {
        multiplier: FixedRatio,
    },
    IncreaseIncomingDamage {
        multiplier: FixedRatio,
    },
    DecreaseIncomingDamage {
        multiplier: FixedRatio,
    },
    IncreaseGoldGain {
        multiplier: FixedRatio,
    },
    DecreaseGoldGainPercent {
        reduction_percentage: FixedRatio,
    },
    DisableItemAndUpgradePurchases,
    DisableItemUse,
    IncreaseMaxHandSlots {
        bonus: usize,
    },
    DecreaseMaxHandSlots {
        penalty: usize,
    },
    IncreaseMaxRerolls {
        bonus: usize,
    },
    DecreaseMaxRerolls {
        penalty: usize,
    },
    IncreaseEnemyHealthPercent {
        percentage: FixedRatio,
    },
    DecreaseEnemyHealthPercent {
        percentage: FixedRatio,
    },
    IncreaseEnemySpeed {
        multiplier: FixedRatio,
    },
    DecreaseEnemySpeed {
        multiplier: FixedRatio,
    },
    RankTowerDisable {
        rank: Rank,
    },
    SuitTowerDisable {
        suit: Suit,
    },
    GainShield {
        min_amount: Shield,
        max_amount: Shield,
    },
    HealHealth {
        min_amount: Health,
        max_amount: Health,
    },
    GainGold {
        min_amount: usize,
        max_amount: usize,
    },
}

pub fn run_effect(game_state: &mut GameState, effect: &Effect) {
    game_state.sync_raw_core_from_projection();
    let mut raw = game_state.raw_core.state().clone();
    let stage = raw.progress().stage as u64;
    let mut rng = None;
    raw.edit_snapshot(|parts| {
        rng = Some(
            parts
                .rng
                .next_rng(td_core::deterministic_rng::domain::EFFECT_PAYLOAD, &[stage]),
        );
    })
    .expect("effect RNG state must remain valid");
    let mut rng = rng.expect("effect RNG must be initialized");
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw effect RNG state must be restorable in headed adapter");
    run_effect_with_rng(game_state, effect, &mut rng);
}

/// 테스트 및 결정적(Deterministic) 실행을 위해 RNG를 주입할 수 있는 버전.
/// 기존 `run_effect` 는 thread_rng() 를 사용하며, 이 함수는 재사용 가능한 코어 로직을 담는다.
pub fn run_effect_with_rng<R: rand::Rng>(game_state: &mut GameState, effect: &Effect, rng: &mut R) {
    match effect {
        Effect::Heal { amount } => {
            game_state
                .apply_compatibility_action(crate::game_state::CompatibilityAction::Heal(*amount));
        }
        Effect::Shield { amount } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::GainShield(*amount),
            );
        }
        Effect::EarnGold { amount } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::EarnGold(*amount),
            );
        }
        Effect::DamageReduction {
            damage_multiply,
            duration,
        } => {
            let status_effect = UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply: *damage_multiply,
                },
                end_at: game_state.sim_tick() + *duration,
            };
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::ApplyUserStatusEffect(status_effect),
            );
        }
        Effect::LoseHealth { amount } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::LoseHealth(*amount),
            );
        }
        Effect::LoseGold { amount } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::LoseGold(*amount),
            );
        }
        Effect::GrantUpgrade { rarity: _ } => {
            let upgrade = crate::game_state::upgrade::generate_boss_reward_upgrade(game_state);
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
                upgrade, None,
            ));
        }
        Effect::GrantItem { rarity } => {
            let rarity = match rarity {
                Rarity::Common => td_core::Rarity::Common,
                Rarity::Rare => td_core::Rarity::Rare,
                Rarity::Epic => td_core::Rarity::Epic,
                Rarity::Legendary => td_core::Rarity::Legendary,
            };
            if let Some(item) = td_core::generate_item_of_rarity_with_rng(rarity, rng) {
                game_state.grant_core_item(item);
            }
        }
        Effect::IncreaseAllTowersDamage { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::Damage,
                    *multiplier,
                ),
            );
        }
        Effect::DecreaseAllTowersDamage { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::Damage,
                    *multiplier,
                ),
            );
        }
        Effect::IncreaseIncomingDamage { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::IncomingDamage,
                    *multiplier,
                ),
            );
        }
        Effect::DecreaseIncomingDamage { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::DamageReduction,
                    *multiplier,
                ),
            );
        }
        Effect::IncreaseGoldGain { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::GoldGain,
                    *multiplier,
                ),
            );
        }
        Effect::DecreaseGoldGainPercent {
            reduction_percentage,
        } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::GoldGain,
                    reduction_percentage.one_minus(),
                ),
            );
        }
        Effect::DisableItemAndUpgradePurchases => {
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::ApplyStageModifier(
                crate::game_state::compatibility_action::StageModifierMutation::DisableItemAndUpgradePurchases,
            ));
        }
        Effect::DisableItemUse => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::ApplyStageModifier(
                    crate::game_state::compatibility_action::StageModifierMutation::DisableItemUse,
                ),
            );
        }
        Effect::DecreaseMaxHandSlots { penalty } => {
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::ApplyStageModifier(
                crate::game_state::compatibility_action::StageModifierMutation::AddMaxHandSlotsPenalty(*penalty),
            ));
        }
        Effect::IncreaseMaxHandSlots { bonus } => {
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::ApplyStageModifier(
                crate::game_state::compatibility_action::StageModifierMutation::AddMaxHandSlotsBonus(*bonus),
            ));
        }
        Effect::IncreaseMaxRerolls { bonus } => {
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::ApplyStageModifier(
                crate::game_state::compatibility_action::StageModifierMutation::AddMaxRerollsBonus(*bonus),
            ));
        }
        Effect::DecreaseMaxRerolls { penalty } => {
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::ApplyStageModifier(
                crate::game_state::compatibility_action::StageModifierMutation::AddMaxRerollsPenalty(*penalty),
            ));
        }
        Effect::IncreaseEnemyHealthPercent { percentage } => {
            let multiplier = percentage_to_multiplier(*percentage);
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::EnemyHealth,
                    multiplier,
                ),
            );
        }
        Effect::DecreaseEnemyHealthPercent { percentage } => {
            let multiplier = FixedRatio::ONE.decreased_by_percent(*percentage);
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::EnemyHealth,
                    multiplier,
                ),
            );
        }
        Effect::IncreaseEnemySpeed { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::EnemySpeed,
                    *multiplier,
                ),
            );
        }
        Effect::DecreaseEnemySpeed { multiplier } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::AddStageMultiplier(
                    crate::game_state::compatibility_action::StageMultiplierKind::EnemySpeed,
                    *multiplier,
                ),
            );
        }
        Effect::RankTowerDisable { rank } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::ApplyStageModifier(
                    crate::game_state::compatibility_action::StageModifierMutation::DisableRank(
                        *rank,
                    ),
                ),
            );
        }
        Effect::SuitTowerDisable { suit } => {
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::ApplyStageModifier(
                    crate::game_state::compatibility_action::StageModifierMutation::DisableSuit(
                        *suit,
                    ),
                ),
            );
        }
        Effect::GainShield {
            min_amount,
            max_amount,
        } => {
            let shield_amount =
                Shield::from_raw(rng.gen_range(min_amount.raw()..=max_amount.raw()));
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::GainShield(shield_amount),
            );
        }
        Effect::HealHealth {
            min_amount,
            max_amount,
        } => {
            let heal_amount = Health::from_raw(rng.gen_range(min_amount.raw()..=max_amount.raw()));
            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Heal(
                heal_amount,
            ));
        }
        Effect::GainGold {
            min_amount,
            max_amount,
        } => {
            let gold_amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.apply_compatibility_action(
                crate::game_state::CompatibilityAction::EarnGold(gold_amount),
            );
        }
    }
}

impl Effect {
    pub fn description_text(&self) -> crate::l10n::effect::EffectText {
        crate::l10n::effect::EffectText::Description(self.clone())
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Effect::Heal { .. }
            | Effect::Shield { .. }
            | Effect::EarnGold { .. }
            | Effect::DamageReduction { .. }
            | Effect::GrantUpgrade { .. }
            | Effect::GrantItem { .. }
            | Effect::IncreaseAllTowersDamage { .. }
            | Effect::DecreaseIncomingDamage { .. }
            | Effect::IncreaseGoldGain { .. }
            | Effect::IncreaseMaxHandSlots { .. }
            | Effect::IncreaseMaxRerolls { .. }
            | Effect::DecreaseEnemyHealthPercent { .. }
            | Effect::DecreaseEnemySpeed { .. }
            | Effect::GainShield { .. }
            | Effect::HealHealth { .. }
            | Effect::GainGold { .. } => true,
            Effect::LoseHealth { .. }
            | Effect::LoseGold { .. }
            | Effect::DecreaseAllTowersDamage { .. }
            | Effect::IncreaseIncomingDamage { .. }
            | Effect::DecreaseGoldGainPercent { .. }
            | Effect::DisableItemAndUpgradePurchases
            | Effect::DisableItemUse
            | Effect::DecreaseMaxHandSlots { .. }
            | Effect::DecreaseMaxRerolls { .. }
            | Effect::IncreaseEnemyHealthPercent { .. }
            | Effect::IncreaseEnemySpeed { .. }
            | Effect::RankTowerDisable { .. }
            | Effect::SuitTowerDisable { .. } => false,
        }
    }

    pub fn apply_to_stage_modifiers(&self, modifiers: &mut StageModifiers) {
        match self {
            Effect::DecreaseEnemyHealthPercent { percentage } => {
                let multiplier = percentage_to_multiplier(*percentage);
                modifiers.apply_enemy_health_multiplier(multiplier);
            }
            Effect::IncreaseIncomingDamage { multiplier } => {
                modifiers.apply_incoming_damage_multiplier(*multiplier);
            }
            Effect::DecreaseIncomingDamage { multiplier } => {
                modifiers.apply_damage_reduction_multiplier(*multiplier);
            }
            Effect::IncreaseGoldGain { multiplier } => {
                modifiers.apply_gold_gain_multiplier(*multiplier);
            }
            Effect::DecreaseGoldGainPercent {
                reduction_percentage,
            } => {
                modifiers.apply_gold_gain_multiplier(reduction_percentage.one_minus());
            }
            Effect::DecreaseAllTowersDamage { multiplier }
            | Effect::IncreaseAllTowersDamage { multiplier } => {
                modifiers.apply_damage_multiplier(*multiplier);
            }
            Effect::DecreaseMaxHandSlots { penalty } => {
                modifiers.apply_max_hand_slots_penalty(*penalty);
            }
            Effect::IncreaseMaxHandSlots { bonus } => {
                modifiers.apply_max_hand_slots_bonus(*bonus);
            }
            Effect::DecreaseMaxRerolls { penalty } => {
                modifiers.apply_max_rerolls_penalty(*penalty);
            }
            Effect::IncreaseMaxRerolls { bonus } => {
                modifiers.apply_max_rerolls_bonus(*bonus);
            }
            Effect::DisableItemAndUpgradePurchases => {
                modifiers.disable_item_and_upgrade_purchases();
            }
            Effect::DisableItemUse => {
                modifiers.disable_item_use();
            }
            _ => {}
        }
    }
}

// ============================= Test Helpers =============================
#[cfg(test)]
pub mod tests_support {
    use crate::card::Deck;
    use crate::game_state::stage_modifiers::StageModifiers;
    use crate::game_state::{
        GameState, MAP_SIZE, TRAVEL_POINTS, flow::GameFlow, monster_spawn::MonsterSpawnState,
    };
    use crate::hand::{Hand, HandItem};

    /// 테스트용 GameState 생성 헬퍼.
    /// - Atom / 렌더 컨텍스트에 의존하지 않음.
    /// - 필요한 최소 필드만 초기화.
    pub fn make_test_state() -> GameState {
        let config = std::sync::Arc::new(crate::config::GameConfig::default_config());
        GameState {
            presentation_projection:
                crate::game_state::presentation_projection::LegacyProjectionCodec::new(
                    crate::SimTick::ZERO,
                    crate::game_state::GameRngState::new(0),
                    crate::game_state::calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
                    config.clone(),
                    StageModifiers::new(),
                    crate::game_state::upgrade::UpgradeState::default(),
                    Hand::new(std::iter::empty::<HandItem>()),
                    Deck::new(),
                    vec![],
                    MonsterSpawnState::idle(),
                    Vec::new(),
                    Vec::new(),
                    crate::game_state::entity_id::EntityIdAllocator::default(),
                    crate::game_state::GameMetrics {
                        total_gold_earned: 0,
                        total_gold_spent: 0,
                        current_consecutive_perfect_clears: 0,
                        max_consecutive_perfect_clears: 0,
                        tower_damage_stats: vec![],
                        total_rerolled_count: 0,
                        total_escaped_hp: crate::Health::ZERO,
                        total_player_damage: crate::Health::ZERO,
                        stage_damage: Vec::new(),
                    }
                    .to_core_metrics(),
                    GameFlow::Initializing,
                    1,
                    0,
                    crate::Health::from_integer(100),
                    crate::Shield::ZERO,
                    config.player.base_dice_chance,
                    0,
                    0,
                    false,
                    Default::default(),
                    Default::default(),
                    0,
                    Vec::new(),
                    Vec::new(),
                ),
            raw_core: crate::game_state::raw_core::HeadedRawCoreState::new(
                crate::game_state::presentation_projection::LegacyProjectionCodec::new(
                    crate::SimTick::ZERO,
                    crate::game_state::GameRngState::new(0),
                    crate::game_state::calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
                    config.clone(),
                    StageModifiers::new(),
                    crate::game_state::upgrade::UpgradeState::default(),
                    Hand::new(std::iter::empty::<HandItem>()),
                    Deck::new(),
                    vec![],
                    MonsterSpawnState::idle(),
                    Vec::new(),
                    Vec::new(),
                    crate::game_state::entity_id::EntityIdAllocator::default(),
                    crate::game_state::GameMetrics {
                        total_gold_earned: 0,
                        total_gold_spent: 0,
                        current_consecutive_perfect_clears: 0,
                        max_consecutive_perfect_clears: 0,
                        tower_damage_stats: vec![],
                        total_rerolled_count: 0,
                        total_escaped_hp: crate::Health::ZERO,
                        total_player_damage: crate::Health::ZERO,
                        stage_damage: Vec::new(),
                    }
                    .to_core_metrics(),
                    GameFlow::Initializing,
                    1,
                    0,
                    crate::Health::from_integer(100),
                    crate::Shield::ZERO,
                    config.player.base_dice_chance,
                    0,
                    0,
                    false,
                    Default::default(),
                    Default::default(),
                    0,
                    Vec::new(),
                    Vec::new(),
                )
                .to_td_core_state(),
            ),
            presentation_metadata: Default::default(),
            monster_animation_runtime: Vec::new(),
            presentation_hand: Hand::new(std::iter::empty::<HandItem>()),
            presentation_flow: GameFlow::Initializing,
            presentation_inventory: Default::default(),
            presentation_upgrades: Default::default(),
            presentation_deck: Default::default(),
            pending_presentation_events: crate::game_state::PresentationEventQueue::default(),
            locale: crate::l10n::Locale::KOREAN,
            pending_history_events: Vec::new(),
            pending_card_service_notifications: Vec::new(),
            pending_modals: crate::game_state::modal::OpenedModals::default(),
            headless: false,
            pending_discoveries: Default::default(),
        }
    }
}

// Aggregate test modules sitting under `effect/tests/` directory
#[cfg(test)]
mod tests {
    mod card_selection_reroll_and_slots;
    mod random_effects_deterministic;
    mod run_effect_integration;
}
