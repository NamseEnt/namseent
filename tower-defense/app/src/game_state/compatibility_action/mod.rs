#[cfg(test)]
mod card_reroll;
pub(crate) mod earn_gold;
pub(crate) mod game_over;
pub(crate) mod game_start;
pub(crate) mod place_tower;
pub(crate) mod purchase_shop_item;
pub(crate) mod remove_tower;
mod spend_gold;
#[cfg(test)]
mod stage_end;
pub(crate) mod start_stage;
pub(crate) mod take_damage;
pub(crate) mod upgrade;
#[cfg(test)]
mod use_card_service;
pub(crate) mod use_item;

#[cfg(test)]
use crate::Damage;
#[cfg(test)]
use crate::TowerId;
#[cfg(any(test, feature = "debug-tools"))]
use crate::game_state::hand::HandSlotId;
#[cfg(any(test, feature = "debug-tools"))]
use crate::game_state::item;
#[cfg(any(test, feature = "debug-tools"))]
use crate::game_state::tower::Tower;
use crate::game_state::{
    GameState,
    card::{Rank, Suit},
    upgrade::Upgrade,
    user_status_effect::UserStatusEffect,
};
use crate::{FixedRatio, Health, Shield};

#[derive(Clone, Copy)]
pub(crate) enum StageMultiplierKind {
    Damage,
    DamageReduction,
    IncomingDamage,
    GoldGain,
    EnemyHealth,
    EnemySpeed,
}

#[derive(Clone, Copy)]
pub(crate) enum StageModifierMutation {
    DisableItemAndUpgradePurchases,
    DisableItemUse,
    AddMaxHandSlotsBonus(usize),
    AddMaxHandSlotsPenalty(usize),
    AddMaxRerollsBonus(usize),
    AddMaxRerollsPenalty(usize),
    DisableRank(Rank),
    DisableSuit(Suit),
}

/// App-only legacy effect and test/debug mutation; never a gameplay command.
pub(crate) enum CompatibilityAction {
    GameStart,
    #[cfg(test)]
    StartStage {
        stage: usize,
    },
    EarnGold(usize),
    Heal(Health),
    LoseHealth(Health),
    LoseGold(usize),
    AddStageMultiplier(StageMultiplierKind, FixedRatio),
    ApplyStageModifier(StageModifierMutation),
    #[cfg(test)]
    CardReroll,
    GainShield(Shield),
    Upgrade(Upgrade, Option<usize>),
    #[cfg(any(test, feature = "debug-tools"))]
    PlaceTower(Box<Tower>, Option<HandSlotId>),
    #[cfg(test)]
    RemoveTower(TowerId),
    #[cfg(test)]
    MonsterDeath,
    #[cfg(test)]
    PurchaseShopItem(crate::shop::ShopSlotId),
    #[cfg(any(test, feature = "debug-tools"))]
    #[allow(dead_code)]
    GrantItem(item::Item),
    // Apply a user status effect to the player. maybe useful for treasure in near future.
    #[allow(dead_code)]
    ApplyUserStatusEffect(UserStatusEffect),
    #[cfg(test)]
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
    #[cfg(any(test, feature = "debug-tools"))]
    StartPlacingTower(crate::game_state::tower::TowerTemplate),
    #[cfg(any(test, feature = "debug-tools"))]
    StartDefense,
    #[cfg(test)]
    StartTreasureSelection,
    #[cfg(test)]
    UseCardService {
        card_service: crate::game_state::card_service::CardService,
        locale: crate::l10n::Locale,
    },
}

impl GameState {
    pub(crate) fn apply_compatibility_action(&mut self, action: CompatibilityAction) -> bool {
        #[cfg(any(test, feature = "debug-tools"))]
        self.sync_raw_core_from_projection();
        self.apply_compatibility_action_inner(action)
    }

    fn apply_compatibility_action_inner(&mut self, action: CompatibilityAction) -> bool {
        match action {
            CompatibilityAction::GameStart => {
                self.discover_inventory_items();
                game_start::record_history_event(self);
                true
            }
            #[cfg(test)]
            CompatibilityAction::StartStage { stage } => {
                let mut raw = self.raw_core.state().clone();
                raw.start_stage(stage);
                self.restore_raw_core_projection(raw)
                    .expect("raw stage start must be restorable in headed adapter");
                self.consume_core_events_now();
                true
            }
            CompatibilityAction::EarnGold(amount) => {
                let mut raw = self.raw_core.state().clone();
                raw.earn_gold(amount);
                self.restore_raw_core_projection(raw)
                    .expect("raw gold gain must be restorable in headed adapter");
                earn_gold::play_earn_sound(self, amount);
                true
            }
            CompatibilityAction::Heal(amount) => {
                let mut raw = self.raw_core.state().clone();
                let max_hp = raw.max_hp_raw();
                raw.edit_snapshot(|parts| {
                    parts.hp_raw = parts.hp_raw.saturating_add(amount.raw()).min(max_hp);
                })
                .expect("raw heal must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw heal must be restorable in headed adapter");
                true
            }
            CompatibilityAction::LoseHealth(amount) => {
                let mut raw = self.raw_core.state().clone();
                raw.edit_snapshot(|parts| {
                    parts.hp_raw = parts.hp_raw.saturating_sub(amount.raw()).max(1_000);
                })
                .expect("raw health loss must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw health loss must be restorable in headed adapter");
                true
            }
            CompatibilityAction::LoseGold(amount) => {
                let mut raw = self.raw_core.state().clone();
                raw.edit_snapshot(|parts| {
                    if parts.progress.gold >= amount {
                        parts.progress.gold -= amount;
                    } else {
                        let remaining = amount - parts.progress.gold;
                        parts.progress.gold = 0;
                        let health_penalty_raw = remaining.min(i64::MAX as usize) as i64 * 100;
                        parts.hp_raw = parts.hp_raw.saturating_sub(health_penalty_raw).max(1_000);
                    }
                })
                .expect("raw gold loss must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw gold loss must be restorable in headed adapter");
                true
            }
            CompatibilityAction::AddStageMultiplier(kind, multiplier) => {
                let mut raw = self.raw_core.state().clone();
                raw.edit_snapshot(|parts| {
                    let multipliers = match kind {
                        StageMultiplierKind::Damage => {
                            &mut parts.stage_modifiers.damage_multipliers_raw
                        }
                        StageMultiplierKind::DamageReduction => {
                            &mut parts.stage_modifiers.damage_reduction_multipliers_raw
                        }
                        StageMultiplierKind::IncomingDamage => {
                            &mut parts.stage_modifiers.incoming_damage_multipliers_raw
                        }
                        StageMultiplierKind::GoldGain => {
                            &mut parts.stage_modifiers.gold_gain_multipliers_raw
                        }
                        StageMultiplierKind::EnemyHealth => {
                            &mut parts.stage_modifiers.enemy_health_multipliers_raw
                        }
                        StageMultiplierKind::EnemySpeed => {
                            &mut parts.stage_modifiers.enemy_speed_multipliers_raw
                        }
                    };
                    multipliers.push(multiplier.raw());
                })
                .expect("raw stage multiplier must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw stage multiplier must be restorable in headed adapter");
                true
            }
            CompatibilityAction::ApplyStageModifier(mutation) => {
                let mut raw = self.raw_core.state().clone();
                raw.edit_snapshot(|parts| match mutation {
                    StageModifierMutation::DisableItemAndUpgradePurchases => {
                        parts.stage_modifiers.disable_item_and_upgrade_purchases = true;
                    }
                    StageModifierMutation::DisableItemUse => {
                        parts.stage_modifiers.disable_item_use = true;
                    }
                    StageModifierMutation::AddMaxHandSlotsBonus(amount) => {
                        parts.stage_modifiers.card_selection_hand_max_slots_bonus = parts
                            .stage_modifiers
                            .card_selection_hand_max_slots_bonus
                            .saturating_add(amount);
                    }
                    StageModifierMutation::AddMaxHandSlotsPenalty(amount) => {
                        parts.stage_modifiers.card_selection_hand_max_slots_penalty = parts
                            .stage_modifiers
                            .card_selection_hand_max_slots_penalty
                            .saturating_add(amount);
                    }
                    StageModifierMutation::AddMaxRerollsBonus(amount) => {
                        parts.stage_modifiers.max_dice_rerolls_bonus = parts
                            .stage_modifiers
                            .max_dice_rerolls_bonus
                            .saturating_add(amount);
                    }
                    StageModifierMutation::AddMaxRerollsPenalty(amount) => {
                        parts.stage_modifiers.max_dice_rerolls_penalty = parts
                            .stage_modifiers
                            .max_dice_rerolls_penalty
                            .saturating_add(amount);
                    }
                    StageModifierMutation::DisableRank(rank) => {
                        let rank = rank.ordinal() as u8;
                        if !parts.stage_modifiers.disabled_ranks.contains(&rank) {
                            parts.stage_modifiers.disabled_ranks.push(rank);
                        }
                    }
                    StageModifierMutation::DisableSuit(suit) => {
                        let suit = match suit {
                            Suit::Spades => 0,
                            Suit::Hearts => 1,
                            Suit::Diamonds => 2,
                            Suit::Clubs => 3,
                        };
                        if !parts.stage_modifiers.disabled_suits.contains(&suit) {
                            parts.stage_modifiers.disabled_suits.push(suit);
                        }
                    }
                })
                .expect("raw stage modifier must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw stage modifier must be restorable in headed adapter");
                true
            }
            CompatibilityAction::GainShield(amount) => {
                let mut raw = self.raw_core.state().clone();
                raw.edit_snapshot(|parts| {
                    parts.shield_raw = parts.shield_raw.saturating_add(amount.raw()).max(0);
                })
                .expect("raw shield gain must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw shield gain must be restorable in headed adapter");
                true
            }
            #[cfg(test)]
            CompatibilityAction::CardReroll => {
                let mut raw = self.raw_core.state().clone();
                let health_cost = raw.stage_modifiers().reroll_health_cost;
                let before_hp = raw.hp_raw();
                if ((raw.progress().left_dice > 0)
                    || raw.hp_raw().saturating_sub(health_cost as i64 * 1_000) > 1_000)
                    && let Ok(rerolled) = raw.reroll_cards(&[])
                {
                    raw.trigger_card_reroll_upgrades();
                    let actual_damage = before_hp.saturating_sub(raw.hp_raw()).max(0);
                    self.restore_raw_core_projection(raw)
                        .expect("raw card reroll must be restorable in headed adapter");
                    crate::game_state::presentation_effect::apply_card_reroll(
                        self,
                        rerolled,
                        Damage::from_usize(health_cost),
                        Damage::from_raw(actual_damage),
                    );
                }
                true
            }
            CompatibilityAction::Upgrade(upgrade, cost) => {
                let mut raw = self.raw_core.state().clone();
                let raw_upgrade =
                    crate::game_state::upgrade::UpgradeWithId::new(upgrade).to_core_state();
                let recovery = raw
                    .acquire_upgrade(raw_upgrade)
                    .expect("legacy upgrade payload must contain a valid kind")
                    .recovery;
                raw.apply_upgrade_recovery(recovery);
                self.restore_raw_core_projection(raw)
                    .expect("raw upgrade acquisition must be restorable in headed adapter");
                crate::game_state::presentation_effect::apply_upgrade(self, upgrade, cost);
                true
            }
            #[cfg(test)]
            CompatibilityAction::MonsterDeath => {
                let mut raw = self.raw_core.state().clone();
                raw.trigger_monster_death_upgrades();
                self.restore_raw_core_projection(raw)
                    .expect("raw monster-death trigger must be restorable in headed adapter");
                true
            }
            #[cfg(any(test, feature = "debug-tools"))]
            CompatibilityAction::PlaceTower(tower, placing_tower_slot_id) => {
                let hand_slot_index = placing_tower_slot_id.and_then(|slot_id| {
                    self.presentation_hand_snapshot()
                        .active_slot_ids()
                        .iter()
                        .position(|candidate| *candidate == slot_id)
                });
                let mut raw = self.raw_core.state().clone();
                let Ok(output) = raw.place_tower_with_template(
                    tower.template.to_core_state(),
                    hand_slot_index,
                    tower.left_top.x,
                    tower.left_top.y,
                ) else {
                    return true;
                };
                let tower_id = output.tower.id.expect("placed tower has an ID");
                raw.trigger_tower_placed_upgrades(
                    tower_id,
                    td_core::rank_is_face(output.tower.template.rank),
                    &output.tower.template,
                );
                raw.refresh_tower_damage_multipliers();
                self.restore_raw_core_projection(raw)
                    .expect("raw tower placement must be restorable in headed adapter");
                let tower = self
                    .presentation_projection()
                    .towers
                    .iter()
                    .find(|placed| placed.id().raw() == tower_id)
                    .cloned()
                    .expect("placed tower must be present after raw projection");
                crate::game_state::presentation_effect::apply_place_tower(self, &tower);
                true
            }
            #[cfg(test)]
            CompatibilityAction::RemoveTower(tower_id) => {
                let mut raw = self.raw_core.state().clone();
                let Some(removed_tower) = raw.remove_tower(tower_id.raw()) else {
                    return true;
                };
                raw.trigger_tower_removed_upgrades(removed_tower.rerolled_count);
                self.restore_raw_core_projection(raw)
                    .expect("raw tower removal must be restorable in headed adapter");
                crate::game_state::presentation_effect::apply_remove_tower(self, tower_id);
                true
            }
            #[cfg(test)]
            CompatibilityAction::PurchaseShopItem(slot_id) => {
                purchase_shop_item::try_purchase(self, slot_id)
            }
            #[cfg(any(test, feature = "debug-tools"))]
            CompatibilityAction::GrantItem(item) => {
                self.grant_core_item(
                    crate::game_state::item::ItemWithId::new(item).to_core_state(),
                )
                .expect("raw item grant must be restorable in headed adapter");
                true
            }
            CompatibilityAction::ApplyUserStatusEffect(status_effect) => {
                let mut raw = self.raw_core.state().clone();
                raw.edit_snapshot(|parts| {
                    parts
                        .user_status_effects
                        .push(status_effect.to_core_status_effect());
                })
                .expect("raw user status effect must preserve a valid snapshot");
                self.restore_raw_core_projection(raw)
                    .expect("raw user status effect must be restorable in headed adapter");
                true
            }
            #[cfg(test)]
            CompatibilityAction::StageEnd {
                perfect_clear,
                gold,
                item_count,
            } => {
                stage_end::update_clear_metrics(self, perfect_clear);
                stage_end::trigger_upgrades(self, perfect_clear, gold, item_count);
                true
            }
            #[cfg(any(test, feature = "debug-tools"))]
            CompatibilityAction::StartPlacingTower(tower_template) => {
                let mut raw = self.raw_core.state().clone();
                raw.start_placing_tower_with_template(tower_template.to_core_state());
                self.restore_raw_core_projection(raw)
                    .expect("raw tower placement start must be restorable in headed adapter");
                true
            }
            #[cfg(any(test, feature = "debug-tools"))]
            CompatibilityAction::StartDefense => {
                let mut raw = self.raw_core.state().clone();
                if raw.start_defense() {
                    self.restore_raw_core_projection(raw)
                        .expect("raw defense start must be restorable in headed adapter");
                    self.consume_core_events_now();
                } else {
                    raw.force_start_defense();
                    self.restore_raw_core_projection(raw)
                        .expect("compatibility defense start must be restorable in headed adapter");
                    self.consume_core_events_now();
                }
                true
            }
            #[cfg(test)]
            CompatibilityAction::StartTreasureSelection => {
                let mut raw = self.raw_core.state().clone();
                raw.start_treasure_selection();
                self.restore_raw_core_projection(raw)
                    .expect("raw treasure selection must be restorable in headed adapter");
                self.discover_treasure_options();
                true
            }
            #[cfg(test)]
            CompatibilityAction::UseCardService {
                card_service,
                locale,
            } => {
                use_card_service::use_card_service(self, card_service.clone(), locale);
                self.discover_card_service(&card_service);
                true
            }
        }
    }
}
