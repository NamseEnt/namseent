use super::*;
use crate::game_state::camera::ShakeIntensity;
use crate::game_state::effect_event::GameEffectEvent;
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

impl GameState {
    pub fn record_game_start(&mut self) {
        self.record_event(HistoryEventType::GameStart);
    }

    pub fn record_stage_start(&mut self) {
        self.record_event(HistoryEventType::StageStart { stage: self.stage });
    }

    pub fn record_game_over(&mut self) {
        self.record_event(HistoryEventType::GameOver);
    }

    pub fn earn_gold(&mut self, gold: usize) {
        self.gold += gold;
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

    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.upgrade_state.upgrade(upgrade);
        self.record_event(HistoryEventType::UpgradeSelected { upgrade });
    }

    pub fn place_tower(&mut self, tower: Tower) {
        let rank = tower.rank;
        let suit = tower.suit;
        let hand = tower.kind;
        let left_top = tower.left_top;
        let tower_count_before = self.towers.iter().count();

        self.towers.place_tower(tower);
        self.route = calculate_routes(&self.towers.coords(), &TRAVEL_POINTS, MAP_SIZE).unwrap();

        self.record_event(HistoryEventType::TowerPlaced {
            tower_kind: hand,
            rank,
            suit,
            left_top,
        });

        let tower_placed = self.towers.iter().count() > tower_count_before;
        if tower_placed {
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

    pub fn purchase_shop_item(&mut self, slot_id: crate::shop::ShopSlotId) {
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
                self.upgrade_state.upgrade(upgrade_value);
                self.record_event(HistoryEventType::UpgradePurchased {
                    upgrade: upgrade_value,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
        }
    }

    pub fn use_item(&mut self, item: &item::Item) {
        // 아이템 사용 불가 효과 체크
        if self.stage_modifiers.is_item_use_disabled() {
            return; // 아이템 사용 불가 상태에서는 아무것도 하지 않음
        }

        self.item_used = true;
        run_effect(self, &item.effect);
        self.record_event(HistoryEventType::ItemUsed {
            item_effect: item.effect.clone(),
        });
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
