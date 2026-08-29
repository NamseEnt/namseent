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
    let mut rng = game_state.rng.next_rng(
        crate::deterministic_rng::domain::EFFECT_PAYLOAD,
        &[game_state.stage as u64],
    );
    run_effect_with_rng(game_state, effect, &mut rng);
}

/// 테스트 및 결정적(Deterministic) 실행을 위해 RNG를 주입할 수 있는 버전.
/// 기존 `run_effect` 는 thread_rng() 를 사용하며, 이 함수는 재사용 가능한 코어 로직을 담는다.
pub fn run_effect_with_rng<R: rand::Rng>(game_state: &mut GameState, effect: &Effect, rng: &mut R) {
    match effect {
        Effect::Heal { amount } => {
            game_state.hp = game_state
                .hp
                .saturating_add(*amount)
                .min(game_state.max_hp());
        }
        Effect::Shield { amount } => {
            game_state.shield = game_state.shield.saturating_add(*amount);
        }
        Effect::EarnGold { amount } => {
            game_state.gold = game_state.gold.saturating_add(*amount);
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
            game_state.user_status_effects.push(status_effect);
        }
        Effect::LoseHealth { amount } => {
            game_state.hp = game_state
                .hp
                .saturating_sub(*amount)
                .max(Health::from_integer(1));
        }
        Effect::LoseGold { amount } => {
            if game_state.gold >= *amount {
                game_state.gold -= *amount;
            } else {
                let remaining = *amount - game_state.gold;
                game_state.gold = 0;
                let health_penalty = Health::from_usize(remaining)
                    .scaled_by(FixedRatio::from_raw(100_000))
                    .max(Health::from_integer(1));
                game_state.hp = game_state
                    .hp
                    .saturating_sub(health_penalty)
                    .max(Health::from_integer(1));
            }
        }
        Effect::GrantUpgrade { rarity: _ } => {
            let upgrade = crate::game_state::upgrade::generate_boss_reward_upgrade(game_state);
            game_state.action(crate::game_state::GameStateAction::Upgrade(upgrade, None));
        }
        Effect::GrantItem { rarity } => {
            if let Some(item) =
                crate::game_state::item::generation::generate_item_of_rarity_with_rng(*rarity, rng)
            {
                game_state.action(crate::game_state::GameStateAction::GrantItem(item));
            }
        }
        Effect::IncreaseAllTowersDamage { multiplier } => {
            game_state
                .stage_modifiers
                .apply_damage_multiplier(*multiplier);
        }
        Effect::DecreaseAllTowersDamage { multiplier } => {
            game_state
                .stage_modifiers
                .apply_damage_multiplier(*multiplier);
        }
        Effect::IncreaseIncomingDamage { multiplier } => {
            game_state
                .stage_modifiers
                .apply_incoming_damage_multiplier(*multiplier);
        }
        Effect::DecreaseIncomingDamage { multiplier } => {
            game_state
                .stage_modifiers
                .apply_damage_reduction_multiplier(*multiplier);
        }
        Effect::IncreaseGoldGain { multiplier } => {
            game_state
                .stage_modifiers
                .apply_gold_gain_multiplier(*multiplier);
        }
        Effect::DecreaseGoldGainPercent {
            reduction_percentage,
        } => {
            game_state
                .stage_modifiers
                .apply_gold_gain_multiplier(reduction_percentage.one_minus());
        }
        Effect::DisableItemAndUpgradePurchases => {
            game_state
                .stage_modifiers
                .disable_item_and_upgrade_purchases();
        }
        Effect::DisableItemUse => {
            game_state.stage_modifiers.disable_item_use();
        }
        Effect::DecreaseMaxHandSlots { penalty } => {
            game_state
                .stage_modifiers
                .apply_max_hand_slots_penalty(*penalty);
        }
        Effect::IncreaseMaxHandSlots { bonus } => {
            game_state
                .stage_modifiers
                .apply_max_hand_slots_bonus(*bonus);
        }
        Effect::IncreaseMaxRerolls { bonus } => {
            game_state.stage_modifiers.apply_max_rerolls_bonus(*bonus);
        }
        Effect::DecreaseMaxRerolls { penalty } => {
            game_state
                .stage_modifiers
                .apply_max_rerolls_penalty(*penalty);
        }
        Effect::IncreaseEnemyHealthPercent { percentage } => {
            let multiplier = percentage_to_multiplier(*percentage);
            game_state
                .stage_modifiers
                .apply_enemy_health_multiplier(multiplier);
        }
        Effect::DecreaseEnemyHealthPercent { percentage } => {
            let multiplier = FixedRatio::ONE.decreased_by_percent(*percentage);
            game_state
                .stage_modifiers
                .apply_enemy_health_multiplier(multiplier);
        }
        Effect::IncreaseEnemySpeed { multiplier } => {
            game_state
                .stage_modifiers
                .apply_enemy_speed_multiplier(*multiplier);
        }
        Effect::DecreaseEnemySpeed { multiplier } => {
            game_state
                .stage_modifiers
                .apply_enemy_speed_multiplier(*multiplier);
        }
        Effect::RankTowerDisable { rank } => {
            game_state.stage_modifiers.disable_rank(*rank);
        }
        Effect::SuitTowerDisable { suit } => {
            game_state.stage_modifiers.disable_suit(*suit);
        }
        Effect::GainShield {
            min_amount,
            max_amount,
        } => {
            let shield_amount =
                Shield::from_raw(rng.gen_range(min_amount.raw()..=max_amount.raw()));
            game_state.shield = game_state.shield.saturating_add(shield_amount);
        }
        Effect::HealHealth {
            min_amount,
            max_amount,
        } => {
            let heal_amount = Health::from_raw(rng.gen_range(min_amount.raw()..=max_amount.raw()));
            game_state.hp = game_state
                .hp
                .saturating_add(heal_amount)
                .min(game_state.max_hp());
        }
        Effect::GainGold {
            min_amount,
            max_amount,
        } => {
            let gold_amount = rng.gen_range(*min_amount..=*max_amount);
            game_state.gold += gold_amount;
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
        let decorations = crate::game_state::background::generate_decorations();
        let config = std::sync::Arc::new(crate::config::GameConfig::default_config());
        GameState {
            monsters: Default::default(),
            towers: Default::default(),
            camera: crate::game_state::camera::Camera::new(),
            route: crate::game_state::calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
            backgrounds: crate::game_state::generate_backgrounds(),
            decorations,
            effect_events: crate::game_state::EffectEventQueue::default(),
            upgrade_state: Default::default(),
            flow: GameFlow::Initializing,
            hand: Hand::new(std::iter::empty::<HandItem>()),
            stage: 1,
            left_dice: config.player.base_dice_chance,
            monster_spawn_state: MonsterSpawnState::idle(),
            in_flight_attacks: Default::default(),
            items: vec![],
            gold: 0,
            cursor_preview: Default::default(),
            hp: crate::Health::from_integer(100),
            shield: crate::Shield::ZERO,
            user_status_effects: Default::default(),
            left_quest_board_refresh_chance: 0,
            item_used: false,
            next_entity_id: crate::game_state::entity_id::EntityIdAllocator::default(),
            sim_tick: crate::SimTick::ZERO,
            sim_scheduler: crate::game_state::tick::scheduler::FixedTickScheduler::default(),
            sim_scheduler_report:
                crate::game_state::tick::scheduler::ScheduleReport::default(),
            deck: Deck::new(),
            fast_forward_multiplier: Default::default(),
            rerolled_count: 0,
            metrics: crate::game_state::GameMetrics {
                total_gold_earned: 0,
                total_gold_spent: 0,
                current_consecutive_perfect_clears: 0,
                max_consecutive_perfect_clears: 0,
                tower_damage_stats: vec![],
                total_rerolled_count: 0,
                total_escaped_hp: crate::Health::ZERO,
                total_player_damage: crate::Health::ZERO,
                stage_damage: Vec::new(),
            },
            locale: crate::l10n::Locale::KOREAN,
            play_history: crate::game_state::play_history::PlayHistory::new(),
            player_command_sequence: 0,
            player_commands: Vec::new(),
            replay_checkpoints: Vec::new(),
            card_service_notifications:
                crate::game_state::card_notification::CardServiceNotificationState::default(),
            opened_modals: crate::game_state::modal::OpenedModals::default(),
            stage_modifiers: StageModifiers::new(),
            ui_state: crate::game_state::UIState::new(),
            status_effect_particle_generator:
                crate::game_state::status_effect_particle_generator::StatusEffectParticleGenerator::new(
                    crate::PresentationInstant::capture(),
                ),
            black_smoke_sources: Default::default(),
            base_animation_state:
                crate::game_state::BaseAnimationState::new(crate::SimTick::ZERO),
            config: config.clone(),

            rng: crate::game_state::rng::GameRngState::new(0),
            headless: false,
            #[cfg(feature = "simulator")]
            defer_card_service_selection: false,
            discovery: Default::default(),
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
