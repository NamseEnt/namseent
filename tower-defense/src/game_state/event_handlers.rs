use super::*;
use crate::game_state::camera::ShakeIntensity;
use crate::game_state::effect_event::GameEffectEvent;
use crate::game_state::upgrade::{
    UpgradeBehavior, UpgradeTriggerEvent, UpgradeTriggerResult, UpgradeUpdateFlags,
};
use crate::{
    game_state::{
        effect::run_effect, item, play_history::HistoryEventType, tower::Tower, upgrade::Upgrade,
    },
    shop::ShopSlot,
    sound::{self},
};
use rand::Rng;

const DAMAGE_SOUND_DELAY_MIN_MS: i64 = 10;
const DAMAGE_SOUND_DELAY_MAX_MS: i64 = 50;

enum GameStateAction<'a> {
    StageStart {
        stage: usize,
    },
    Upgrade(Upgrade, Option<usize>),
    PlaceTower(Box<Tower>),
    PurchaseShopItem(crate::shop::ShopSlotId),
    UseItem(&'a item::Item),
    TakeDamage(f32),
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
}

impl GameState {
    fn apply_game_state_action(&mut self, action: GameStateAction<'_>) {
        match action {
            GameStateAction::StageStart { stage } => self.do_stage_start(stage),
            GameStateAction::Upgrade(upgrade, cost) => {
                self.handle_upgrade_trigger(UpgradeTriggerEvent::UpgradeAcquired { upgrade });
                self.record_event(HistoryEventType::UpgradeAcquired { upgrade, cost });
            }
            GameStateAction::PlaceTower(tower) => self.do_place_tower(*tower),
            GameStateAction::PurchaseShopItem(slot_id) => self.do_purchase_shop_item(slot_id),
            GameStateAction::UseItem(item) => self.do_use_item(item),
            GameStateAction::TakeDamage(damage) => self.do_take_damage(damage),
            GameStateAction::StageEnd {
                perfect_clear,
                gold,
                item_count,
            } => self.do_stage_end(perfect_clear, gold, item_count),
        }
    }

    pub(crate) fn apply_stage_start(&mut self, stage: usize) {
        self.apply_game_state_action(GameStateAction::StageStart { stage });
    }

    pub(crate) fn apply_stage_end(&mut self, perfect_clear: bool, gold: usize, item_count: usize) {
        self.apply_game_state_action(GameStateAction::StageEnd {
            perfect_clear,
            gold,
            item_count,
        });
    }

    fn do_stage_start(&mut self, stage: usize) {
        self.handle_upgrade_trigger(UpgradeTriggerEvent::StageStart { stage });
    }

    fn do_stage_end(&mut self, perfect_clear: bool, gold: usize, item_count: usize) {
        if perfect_clear {
            self.record_event(HistoryEventType::StagePerfectClear { stage: self.stage });
            self.metrics.current_consecutive_perfect_clears += 1;
            self.metrics.max_consecutive_perfect_clears = self
                .metrics
                .max_consecutive_perfect_clears
                .max(self.metrics.current_consecutive_perfect_clears);
        } else {
            self.metrics.current_consecutive_perfect_clears = 0;
        }

        self.handle_upgrade_trigger(UpgradeTriggerEvent::StageEnd {
            perfect_clear,
            gold,
            item_count,
        });
    }

    pub(crate) fn record_game_start(&mut self) {
        self.record_event(HistoryEventType::GameStart);
    }

    pub(crate) fn record_stage_start(&mut self) {
        self.record_event(HistoryEventType::StageStart {
            stage: self.stage,
            boss: crate::game_state::is_boss_stage(self.stage),
        });
    }

    pub(crate) fn record_game_over(&mut self) {
        self.record_event(HistoryEventType::GameOver);
    }

    pub fn earn_gold(&mut self, gold: usize) {
        self.gold += gold;
        self.metrics.total_gold_earned += gold;
        if gold > 0 {
            self.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_coin_sounds(),
                    sound::SoundGroup::Ui,
                    sound::VolumePreset::High,
                    sound::SpatialMode::NonSpatial,
                ),
            ));
        }
    }
    /// WARNING: `gold` must be less than or equal to self.gold
    pub fn spend_gold(&mut self, gold: usize) {
        self.gold -= gold;
        self.metrics.total_gold_spent += gold;
        if gold > 0 {
            self.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_coin_sounds(),
                    sound::SoundGroup::Ui,
                    sound::VolumePreset::High,
                    sound::SpatialMode::NonSpatial,
                ),
            ));
        }
    }

    pub fn refresh_tower_upgrade_caches(&mut self) {
        let upgrade_bonuses = self.upgrade_state.tower_upgrade_damage_bonuses(self);
        let revision = self.upgrade_state.revision;
        for tower in self.towers.iter_mut() {
            tower.refresh_cached_upgrade_damage(revision, &upgrade_bonuses);
        }
    }

    pub(crate) fn refresh_tower_upgrade_caches_if_dirty(&mut self, flags: UpgradeUpdateFlags) {
        if flags.contains(UpgradeUpdateFlags::TOWER_STATS) {
            self.refresh_tower_upgrade_caches();
        }
    }

    pub(crate) fn refresh_upgrade_trigger_side_effects(&mut self, flags: UpgradeUpdateFlags) {
        if flags.requires_revision() {
            self.upgrade_state.revision = self.upgrade_state.revision.wrapping_add(1);
        }

        self.refresh_tower_upgrade_caches_if_dirty(flags);

        if flags.contains(UpgradeUpdateFlags::RESOURCE) {
            crate::shop::refresh_shop(self);
        }

        if flags.contains(UpgradeUpdateFlags::PLAYER_STATS) {
            self.hp = self.hp.min(self.max_hp());
        }

        if flags.contains(UpgradeUpdateFlags::CARD_OPTIONS) {
            // CARD_OPTIONS is reserved for future card selection / option refresh logic.
        }
    }

    fn handle_upgrade_trigger(&mut self, event: UpgradeTriggerEvent<'_>) {
        let result = match event {
            UpgradeTriggerEvent::UpgradeAcquired { upgrade } => {
                self.upgrade_state.upgrade(upgrade);
                let flags = {
                    let upgrade_ptr =
                        self.upgrade_state
                            .upgrades
                            .last_mut()
                            .expect("upgrade just added") as *mut Upgrade;
                    unsafe { (*upgrade_ptr).on_upgrade_acquired_mut(self) }
                };
                UpgradeTriggerResult::Flags(flags)
            }
            UpgradeTriggerEvent::TowerPlaced { tower } => {
                let game_state_ptr: *const GameState = self;
                let result = self.upgrade_state.handle_upgrade_trigger(
                    unsafe { &*game_state_ptr },
                    UpgradeTriggerEvent::TowerPlaced { tower },
                );
                let extra_flags = {
                    let upgrade_state_ptr: *mut UpgradeState = &mut self.upgrade_state;
                    unsafe { (*upgrade_state_ptr).on_tower_placed_mut(self, tower) }
                };
                match result {
                    UpgradeTriggerResult::TowerPlaced(placement_result, flags) => {
                        UpgradeTriggerResult::TowerPlaced(placement_result, flags | extra_flags)
                    }
                    _ => unreachable!(),
                }
            }
            event => {
                let game_state_ptr: *const GameState = self;
                unsafe {
                    self.upgrade_state
                        .handle_upgrade_trigger(&*game_state_ptr, event)
                }
            }
        };

        let flags = match result {
            UpgradeTriggerResult::Flags(flags) => flags,
            UpgradeTriggerResult::StageStart(effects, flags) => {
                self.left_dice = self.max_dice_chance() + effects.extra_dice;
                if let Some(speed_multiplier) = effects.enemy_speed_multiplier {
                    self.stage_modifiers
                        .apply_enemy_speed_multiplier(speed_multiplier);
                }
                self.stage_modifiers
                    .set_free_shop_this_stage(effects.free_shop_this_stage);
                flags
            }
            UpgradeTriggerResult::TowerPlaced(placement_result, flags) => {
                if placement_result.gold_earn > 0 {
                    self.earn_gold(placement_result.gold_earn);
                }
                flags
            }
            UpgradeTriggerResult::TowerPlacement(gold) => {
                if gold > 0 {
                    self.earn_gold(gold);
                }
                UpgradeUpdateFlags::NONE
            }
            UpgradeTriggerResult::StageEnd(bonus_gold, flags) => {
                if bonus_gold > 0 {
                    self.earn_gold(bonus_gold);
                }
                flags
            }
        };
        self.refresh_upgrade_trigger_side_effects(flags);
    }

    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.apply_game_state_action(GameStateAction::Upgrade(upgrade, None));
    }

    pub fn place_tower(&mut self, tower: Tower) {
        self.apply_game_state_action(GameStateAction::PlaceTower(Box::new(tower)));
    }

    fn do_place_tower(&mut self, mut tower: Tower) {
        let rank = tower.rank;
        let suit = tower.suit;
        let hand = tower.kind;
        let left_top = tower.left_top;

        self.handle_upgrade_trigger(UpgradeTriggerEvent::TowerPlacement {
            tower_template: tower.template_mut(),
            left_dice: self.left_dice,
        });
        tower.refresh_status_effects_from_template();
        tower.refresh_cached_upgrade_damage(
            self.upgrade_state.revision,
            &self.upgrade_state.tower_upgrade_damage_bonuses(self),
        );

        let tower_count_before = self.towers.iter().count();
        self.towers.place_tower(tower.clone());
        self.route = calculate_routes(&self.towers.coords(), &TRAVEL_POINTS, MAP_SIZE).unwrap();

        self.record_event(HistoryEventType::TowerPlaced {
            tower_kind: hand,
            rank,
            suit,
            left_top,
        });

        let tower_placed = self.towers.iter().count() > tower_count_before;
        if tower_placed {
            self.handle_upgrade_trigger(UpgradeTriggerEvent::TowerPlaced { tower: &tower });

            self.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_luggage_drop(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::High,
                    sound::SpatialMode::NonSpatial,
                ),
            ));
        }
    }

    pub fn remove_tower(&mut self, tower_id: usize) -> bool {
        let tower_count_before = self.towers.iter().count();
        let removed_tower_left_top = self
            .towers
            .iter()
            .find(|tower| tower.id() == tower_id)
            .map(|tower| tower.left_top);
        let tower_center_xy =
            self.towers
                .iter()
                .find(|tower| tower.id() == tower_id)
                .map(|tower| {
                    let center = tower.center_xy_f32();
                    (center.x, center.y)
                });
        self.towers.remove_tower(tower_id);
        let tower_removed = self.towers.iter().count() < tower_count_before;
        if tower_removed {
            self.route = calculate_routes(&self.towers.coords(), &TRAVEL_POINTS, MAP_SIZE)
                .expect("route should exist after removing a tower");
            self.handle_upgrade_trigger(UpgradeTriggerEvent::TowerRemoved);
            if let Some(left_top) = removed_tower_left_top {
                self.record_event(HistoryEventType::TowerRemoved { left_top });
            }
            if let Some(center_xy) = tower_center_xy {
                self.effect_events
                    .push(GameEffectEvent::SpawnTowerRemoveDustBurst(
                        center_xy,
                        self.now(),
                    ));
            }
        }

        tower_removed
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.apply_game_state_action(GameStateAction::TakeDamage(damage));
    }

    pub fn purchase_shop_item(&mut self, slot_id: crate::shop::ShopSlotId) {
        self.apply_game_state_action(GameStateAction::PurchaseShopItem(slot_id));
    }

    fn do_take_damage(&mut self, damage: f32) {
        let mut actual_damage = damage;

        // Camera shake based on damage
        let intensity = match actual_damage {
            d if d < 10.0 => ShakeIntensity::Light,
            d if d < 25.0 => ShakeIntensity::Medium,
            _ => ShakeIntensity::Heavy,
        };
        self.camera.shake(intensity);
        self.on_player_damaged(intensity);

        // Shield absorption
        if self.shield > 0.0 {
            let absorbed = damage.min(self.shield);
            actual_damage -= absorbed;
            self.shield -= absorbed;
        }

        // Apply damage
        self.hp -= actual_damage;
        if let GameFlow::Defense(defense_flow) = &mut self.flow {
            defense_flow.took_damage = true;
        }

        // Record event
        if damage > 0.0 {
            let repeat_count = match damage {
                d if d < 10.0 => 1,
                d if d < 25.0 => 2,
                d if d < 50.0 => 3,
                _ => 4,
            };

            let mut rng = rand::thread_rng();
            let mut accumulated_delay_ms = 0i64;

            for index in 0..repeat_count {
                self.effect_events.push(GameEffectEvent::PlaySoundDelayed(
                    sound::EmitSoundParams::one_shot(
                        sound::random_pickaxe(),
                        sound::SoundGroup::Sfx,
                        sound::VolumePreset::High,
                        sound::SpatialMode::NonSpatial,
                    ),
                    Duration::from_millis(accumulated_delay_ms),
                ));

                if index + 1 < repeat_count {
                    accumulated_delay_ms +=
                        rng.gen_range(DAMAGE_SOUND_DELAY_MIN_MS..=DAMAGE_SOUND_DELAY_MAX_MS);
                }
            }

            self.record_event(HistoryEventType::DamageTaken {
                amount: actual_damage,
            });
        }

        // Check game over
        if self.hp <= 0.0 {
            self.effect_events.push(GameEffectEvent::PlaySound(
                sound::EmitSoundParams::one_shot(
                    sound::random_fail(),
                    sound::SoundGroup::Sfx,
                    sound::VolumePreset::High,
                    sound::SpatialMode::NonSpatial,
                ),
            ));
            self.goto_result();
        }
    }

    fn do_purchase_shop_item(&mut self, slot_id: crate::shop::ShopSlotId) {
        let shop = match &mut self.flow {
            GameFlow::SelectingTower(flow) => &mut flow.shop,
            _ => return,
        };

        let Some(slot_data) = shop.get_slot_by_id_mut(slot_id) else {
            return;
        };

        if slot_data.purchased {
            return;
        }

        match &slot_data.slot {
            ShopSlot::Item { item, cost } => {
                if self.gold < *cost {
                    return;
                }

                if self
                    .stage_modifiers
                    .is_item_and_upgrade_purchases_disabled()
                {
                    return;
                }

                let item_clone = item.clone();
                let cost_value = *cost;

                slot_data.purchased = true;
                slot_data.start_exit_animation(Instant::now());
                self.items.push(item_clone.clone());
                self.handle_upgrade_trigger(UpgradeTriggerEvent::ItemBought);
                self.record_event(HistoryEventType::ItemPurchased {
                    item: item_clone,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
            ShopSlot::Upgrade { upgrade, cost } => {
                if self.gold < *cost {
                    return;
                }

                if self
                    .stage_modifiers
                    .is_item_and_upgrade_purchases_disabled()
                {
                    return;
                }

                let upgrade_value = *upgrade;
                let cost_value = *cost;

                slot_data.purchased = true;
                slot_data.start_exit_animation(Instant::now());
                self.apply_game_state_action(GameStateAction::Upgrade(
                    upgrade_value,
                    Some(cost_value),
                ));
                self.spend_gold(cost_value);
            }
        }
    }

    pub fn use_item(&mut self, item: &item::Item) {
        self.apply_game_state_action(GameStateAction::UseItem(item));
    }

    fn do_use_item(&mut self, item: &item::Item) {
        // 아이템 사용 불가 효과 체크
        if self.stage_modifiers.is_item_use_disabled() {
            return; // 아이템 사용 불가 상태에서는 아무것도 하지 않음
        }

        self.item_used = true;
        run_effect(self, &item.effect);
        self.record_event(HistoryEventType::ItemUsed { item: item.clone() });
    }

    pub fn can_purchase_shop_item(&self, slot_id: crate::shop::ShopSlotId) -> bool {
        let shop = match &self.flow {
            GameFlow::SelectingTower(flow) => &flow.shop,
            _ => return false,
        };

        let Some(slot_data) = shop.get_slot_by_id(slot_id) else {
            return false;
        };

        if slot_data.purchased {
            return false;
        }

        match &slot_data.slot {
            ShopSlot::Item { cost, .. } | ShopSlot::Upgrade { cost, .. } => {
                self.gold >= *cost
                    && !self
                        .stage_modifiers
                        .is_item_and_upgrade_purchases_disabled()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};
    use crate::game_state::flow::GameFlow;
    use crate::game_state::tower::{Tower, TowerKind, TowerTemplate};
    use namui::Instant;

    #[test]
    fn resource_flag_refreshes_shop_when_selecting_tower() {
        let mut game_state = create_initial_game_state();
        game_state
            .upgrade_state
            .upgrade(Upgrade::Camera(crate::game_state::upgrade::CameraUpgrade));

        let old_ids: Vec<_> = match &game_state.flow {
            GameFlow::SelectingTower(flow) => flow.shop.slots.iter().map(|slot| slot.id).collect(),
            _ => panic!("expected selecting tower flow"),
        };

        let tower_template = TowerTemplate::new(TowerKind::Barricade, Suit::Spades, Rank::Jack);
        let tower = Tower::new(&tower_template, crate::MapCoord::new(0, 0), Instant::now());

        game_state.handle_upgrade_trigger(
            crate::game_state::upgrade::UpgradeTriggerEvent::TowerPlaced { tower: &tower },
        );

        let new_ids: Vec<_> = match &game_state.flow {
            GameFlow::SelectingTower(flow) => flow.shop.slots.iter().map(|slot| slot.id).collect(),
            _ => panic!("expected selecting tower flow"),
        };

        assert!(new_ids.len() >= old_ids.len());
        assert!(new_ids.iter().any(|id| !old_ids.contains(id)));
        assert_eq!(game_state.gold, game_state.config.player.starting_gold + 50);
    }

    #[test]
    fn player_stats_flag_clamps_hp_to_max() {
        let mut game_state = create_initial_game_state();
        game_state.hp = game_state.max_hp() + 10.0;
        game_state.refresh_upgrade_trigger_side_effects(
            crate::game_state::upgrade::UpgradeUpdateFlags::PLAYER_STATS,
        );
        assert_eq!(game_state.hp, game_state.max_hp());
    }

    #[test]
    fn revision_required_flag_increments_upgrade_revision() {
        let mut game_state = create_initial_game_state();
        let before = game_state.upgrade_state.revision;

        game_state.refresh_upgrade_trigger_side_effects(
            crate::game_state::upgrade::UpgradeUpdateFlags::REVISION_REQUIRED,
        );

        assert_eq!(game_state.upgrade_state.revision, before + 1);
    }

    #[test]
    fn no_flag_does_not_increment_upgrade_revision() {
        let mut game_state = create_initial_game_state();
        let before = game_state.upgrade_state.revision;

        game_state.refresh_upgrade_trigger_side_effects(
            crate::game_state::upgrade::UpgradeUpdateFlags::NONE,
        );

        assert_eq!(game_state.upgrade_state.revision, before);
    }
}
