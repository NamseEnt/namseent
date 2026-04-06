use crate::card::{Rank, Suit};
use crate::game_state::flow::GameFlow;
use crate::game_state::{
    GameState,
    stage_modifiers::StageModifiers,
    tower::{TowerKind, TowerTemplate},
    user_status_effect::{UserStatusEffect, UserStatusEffectKind},
};

use crate::hand::HandItem;
use crate::rarity::Rarity;
use namui::*;

#[derive(Clone, Debug, PartialEq, State)]
pub enum Effect {
    Heal {
        amount: f32,
    },
    Shield {
        amount: f32,
    },
    ExtraDice,
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
    IncreaseAllTowersDamage {
        multiplier: f32,
    },
    DecreaseAllTowersDamage {
        multiplier: f32,
    },
    IncreaseIncomingDamage {
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
        percentage: f32,
    },
    DecreaseEnemyHealthPercent {
        percentage: f32,
    },
    IncreaseEnemySpeed {
        multiplier: f32,
    },
    DecreaseEnemySpeed {
        multiplier: f32,
    },
    RankTowerDisable {
        rank: Rank,
    },
    SuitTowerDisable {
        suit: Suit,
    },
    AddTowerCardToPlacementHand {
        tower_kind: TowerKind,
        suit: Suit,
        rank: Rank,
        count: usize,
    },
    AddCardToHand {
        card: crate::card::Card,
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
        Effect::ExtraDice => {
            game_state.left_dice += 1;
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
        Effect::GrantUpgrade { rarity: _ } => {
            let upgrade = crate::game_state::upgrade::generate_treasure_upgrade(game_state);
            game_state.upgrade_state.upgrade(upgrade);
        }
        Effect::GrantItem { rarity: _ } => {
            let item = crate::game_state::item::generation::generate_item_with_rng(rng);
            game_state.items.push(item);
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
                .apply_gold_gain_multiplier(1.0 - *reduction_percentage);
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
            let multiplier = 1.0 + percentage / 100.0;
            game_state
                .stage_modifiers
                .apply_enemy_health_multiplier(multiplier);
        }
        Effect::DecreaseEnemyHealthPercent { percentage } => {
            let multiplier = 1.0 - percentage / 100.0;
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
        Effect::AddTowerCardToPlacementHand {
            tower_kind,
            suit,
            rank,
            count,
        } => {
            for _ in 0..*count {
                if matches!(game_state.flow, GameFlow::PlacingTower) {
                    game_state.hand.push(HandItem::Tower(TowerTemplate::new(
                        *tower_kind,
                        *suit,
                        *rank,
                    )));
                } else {
                    game_state
                        .stage_modifiers
                        .enqueue_extra_tower_card(*tower_kind, *suit, *rank);
                }
            }
        }
        Effect::AddCardToHand { card } => {
            game_state.hand.push(HandItem::Card(*card));
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
    pub fn name_text(&self) -> crate::l10n::effect::EffectText {
        crate::l10n::effect::EffectText::Name(self.clone())
    }

    pub fn description_text(&self) -> crate::l10n::effect::EffectText {
        crate::l10n::effect::EffectText::Description(self.clone())
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Effect::Heal { .. }
            | Effect::Shield { .. }
            | Effect::EarnGold { .. }
            | Effect::Lottery { .. }
            | Effect::DamageReduction { .. }
            | Effect::UserDamageReduction { .. }
            | Effect::GrantUpgrade { .. }
            | Effect::GrantItem { .. }
            | Effect::IncreaseAllTowersDamage { .. }
            | Effect::DecreaseIncomingDamage { .. }
            | Effect::IncreaseGoldGain { .. }
            | Effect::IncreaseMaxHandSlots { .. }
            | Effect::IncreaseMaxRerolls { .. }
            | Effect::DecreaseEnemyHealthPercent { .. }
            | Effect::DecreaseEnemySpeed { .. }
            | Effect::AddTowerCardToPlacementHand { .. }
            | Effect::AddCardToHand { .. }
            | Effect::GainShield { .. }
            | Effect::HealHealth { .. }
            | Effect::GainGold { .. } => true,
            Effect::ExtraDice
            | Effect::LoseHealth { .. }
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
                let multiplier = 1.0 + percentage / 100.0;
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
                modifiers.apply_gold_gain_multiplier(1.0 - *reduction_percentage);
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

    pub fn can_execute(&self, game_state: &GameState) -> Result<(), EffectExecutionError> {
        if game_state.stage_modifiers.is_item_use_disabled() {
            return Err(EffectExecutionError::ItemUseDisabled);
        }

        if self == &Effect::ExtraDice && !matches!(game_state.flow, GameFlow::SelectingTower(_)) {
            return Err(EffectExecutionError::InvalidFlow {
                required: "SelectingTower".to_string(),
            });
        }

        Ok(())
    }

    /// Effect 실행 불가능한 이유를 사용자가 읽을 수 있는 메시지로 변환
    pub fn execution_error_message<'a>(
        &self,
        error: &EffectExecutionError,
        text_manager: &crate::l10n::TextManager,
        builder: crate::theme::typography::TypographyBuilder<'a>,
    ) -> crate::theme::typography::TypographyBuilder<'a> {
        text_manager.effect_execution_error(error, builder)
    }
}

/// Effect 실행 불가능 사유
#[derive(Clone, Debug, PartialEq, State)]
pub enum EffectExecutionError {
    /// 아이템 사용이 비활성화됨
    ItemUseDisabled,
    /// 잘못된 게임 흐름 단계
    InvalidFlow { required: String },
}

// ============================= Test Helpers =============================
#[cfg(test)]
pub mod tests_support {
    use crate::game_state::stage_modifiers::StageModifiers;
    use crate::game_state::{
        GameState, MAP_SIZE, TRAVEL_POINTS, flow::GameFlow, monster_spawn::MonsterSpawnState,
    };
    use crate::hand::{Hand, HandItem};
    use namui::Instant;

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
            hand: Hand::new(std::iter::empty::<HandItem>()),
            stage: 1,
            left_dice: 1,
            monster_spawn_state: MonsterSpawnState::idle(),
            projectiles: Default::default(),
            delayed_hits: Default::default(),
            items: vec![],
            gold: 0,
            cursor_preview: Default::default(),
            hp: 100.0,
            shield: 0.0,
            user_status_effects: Default::default(),
            left_quest_board_refresh_chance: 0,
            item_used: false,
            deck: crate::card::Deck::new(0),
            game_now: Instant::now(),
            fast_forward_multiplier: Default::default(),
            rerolled_count: 0,
            locale: crate::l10n::Locale::KOREAN,
            play_history: crate::game_state::play_history::PlayHistory::new(),
            opened_modal: None,
            stage_modifiers: StageModifiers::new(),
            ui_state: crate::game_state::UIState::new(),
            status_effect_particle_generator:
                crate::game_state::status_effect_particle_generator::StatusEffectParticleGenerator::new(
                    Instant::now(),
                ),
            black_smoke_sources: Default::default(),

            hand_panel_forced_open: true,
            shop_panel_forced_open: true,
        }
    }
}

// Aggregate test modules sitting under `effect/tests/` directory
#[cfg(test)]
mod tests {
    mod card_selection_reroll_and_slots;
    mod effect_can_execute;
    mod random_effects_deterministic;
    mod run_effect_integration;
    mod shop_reroll;
}
