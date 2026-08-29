use crate::game_state::{GameState, flow::GameFlow};
use namui::*;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, State)]
#[serde(tag = "type", content = "payload")]
pub enum HeadedPlayerCommand {
    Reroll {
        selected_slot_indices: Vec<usize>,
    },
    PurchaseShopItem {
        slot_index: usize,
    },
    UseInventoryItem {
        item_index: usize,
    },
    StartSelectingTower,
    SelectTower {
        selected_slot_indices: Vec<usize>,
    },
    PlaceTower {
        hand_slot_index: usize,
        left: usize,
        top: usize,
    },
    RemoveTower {
        tower_id: u64,
    },
    StartDefense,
    SelectTreasure {
        option_index: usize,
    },
    ConfirmCardServiceSelection {
        selected_card_ids: Vec<Vec<u64>>,
    },
}

pub use td_core::{CommandError, PlayerCommand, RecordedPlayerCommand};

impl From<&HeadedPlayerCommand> for PlayerCommand {
    fn from(command: &HeadedPlayerCommand) -> Self {
        match command {
            HeadedPlayerCommand::Reroll {
                selected_slot_indices,
            } => Self::Reroll {
                selected_slot_indices: selected_slot_indices.clone(),
            },
            HeadedPlayerCommand::PurchaseShopItem { slot_index } => Self::PurchaseShopItem {
                slot_index: *slot_index,
            },
            HeadedPlayerCommand::UseInventoryItem { item_index } => Self::UseInventoryItem {
                item_index: *item_index,
            },
            HeadedPlayerCommand::StartSelectingTower => Self::StartSelectingTower,
            HeadedPlayerCommand::SelectTower {
                selected_slot_indices,
            } => Self::SelectTower {
                selected_slot_indices: selected_slot_indices.clone(),
            },
            HeadedPlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => Self::PlaceTower {
                hand_slot_index: *hand_slot_index,
                left: *left,
                top: *top,
            },
            HeadedPlayerCommand::RemoveTower { tower_id } => Self::RemoveTower {
                tower_id: *tower_id,
            },
            HeadedPlayerCommand::StartDefense => Self::StartDefense,
            HeadedPlayerCommand::SelectTreasure { option_index } => Self::SelectTreasure {
                option_index: *option_index,
            },
            HeadedPlayerCommand::ConfirmCardServiceSelection { selected_card_ids } => {
                Self::ConfirmCardServiceSelection {
                    selected_card_ids: selected_card_ids.clone(),
                }
            }
        }
    }
}

impl From<PlayerCommand> for HeadedPlayerCommand {
    fn from(command: PlayerCommand) -> Self {
        match command {
            PlayerCommand::Reroll {
                selected_slot_indices,
            } => Self::Reroll {
                selected_slot_indices,
            },
            PlayerCommand::PurchaseShopItem { slot_index } => Self::PurchaseShopItem { slot_index },
            PlayerCommand::UseInventoryItem { item_index } => Self::UseInventoryItem { item_index },
            PlayerCommand::StartSelectingTower => Self::StartSelectingTower,
            PlayerCommand::SelectTower {
                selected_slot_indices,
            } => Self::SelectTower {
                selected_slot_indices,
            },
            PlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => Self::PlaceTower {
                hand_slot_index,
                left,
                top,
            },
            PlayerCommand::RemoveTower { tower_id } => Self::RemoveTower { tower_id },
            PlayerCommand::StartDefense => Self::StartDefense,
            PlayerCommand::SelectTreasure { option_index } => Self::SelectTreasure { option_index },
            PlayerCommand::ConfirmCardServiceSelection { selected_card_ids } => {
                Self::ConfirmCardServiceSelection { selected_card_ids }
            }
        }
    }
}

impl GameState {
    pub(crate) fn apply_player_command(
        &mut self,
        command: HeadedPlayerCommand,
    ) -> Result<(), CommandError> {
        self.apply_player_command_at(command, crate::PresentationInstant::capture())
    }

    pub(crate) fn apply_player_command_at(
        &mut self,
        command: HeadedPlayerCommand,
        presentation_instant: crate::PresentationInstant,
    ) -> Result<(), CommandError> {
        let player_command = PlayerCommand::from(&command);
        let before = self.raw_core.state().clone();
        let before_hp = before.hp_raw();
        let reroll_health_cost = before.stage_modifiers().reroll_health_cost;
        let purchase_presentation =
            if let HeadedPlayerCommand::PurchaseShopItem { slot_index } = &command {
                let slot_id = self.shop_slot_id(*slot_index)?;
                let flow = self.presentation_flow_snapshot();
                let slot = match &flow {
                    GameFlow::Shopping(flow) => flow
                        .shop
                        .get_slot_by_id(slot_id)
                        .ok_or(CommandError::InvalidIndex)?
                        .slot
                        .clone(),
                    _ => return Err(CommandError::InvalidFlow),
                };
                Some((slot_id, slot))
            } else {
                None
            };
        let inventory_item = if let HeadedPlayerCommand::UseInventoryItem { item_index } = &command
        {
            Some(
                self.presentation_items_snapshot()
                    .get(*item_index)
                    .cloned()
                    .ok_or(CommandError::InvalidIndex)?,
            )
        } else {
            None
        };

        let receipt = self.raw_core.apply(player_command)?;
        let after = self.raw_core.state().clone();
        let placed_tower_id = if matches!(&command, HeadedPlayerCommand::PlaceTower { .. }) {
            after
                .towers()
                .iter()
                .filter_map(|tower| tower.id)
                .find(|tower_id| {
                    !before
                        .towers()
                        .iter()
                        .any(|previous| previous.id == Some(*tower_id))
                })
        } else {
            None
        };
        self.restore_raw_core_projection_at(after.clone(), presentation_instant, false)?;
        self.raw_core.extend_events(receipt.events);

        match command {
            HeadedPlayerCommand::Reroll { .. } => {
                let actual_damage = before_hp.saturating_sub(after.hp_raw()).max(0);
                let rerolled = after
                    .progress()
                    .rerolled_count
                    .saturating_sub(before.progress().rerolled_count);
                crate::game_state::presentation_effect::apply_card_reroll(
                    self,
                    rerolled,
                    crate::Damage::from_usize(reroll_health_cost),
                    crate::Damage::from_raw(actual_damage),
                );
            }
            HeadedPlayerCommand::PurchaseShopItem { .. } => {
                let (slot_id, slot) =
                    purchase_presentation.expect("purchase presentation context must exist");
                let cost = before.progress().gold.saturating_sub(after.progress().gold);
                crate::game_state::presentation_effect::apply_shop_purchase(
                    self, slot_id, slot, cost,
                );
            }
            HeadedPlayerCommand::UseInventoryItem { .. } => {
                let item = inventory_item.expect("inventory presentation context must exist");
                crate::game_state::presentation_effect::apply_inventory_item(self, &item);
            }
            HeadedPlayerCommand::PlaceTower { .. } => {
                let tower_id = placed_tower_id.ok_or(CommandError::Rejected)?;
                let tower = self
                    .presentation_tower(crate::TowerId::from_raw(tower_id))
                    .ok_or(CommandError::Rejected)?;
                crate::game_state::presentation_effect::apply_place_tower(self, &tower);
            }
            HeadedPlayerCommand::RemoveTower { tower_id } => {
                crate::game_state::presentation_effect::apply_remove_tower(
                    self,
                    crate::TowerId::from_raw(tower_id),
                );
            }
            HeadedPlayerCommand::ConfirmCardServiceSelection { .. } => {
                self.set_user_modal(None);
            }
            HeadedPlayerCommand::StartSelectingTower
            | HeadedPlayerCommand::SelectTower { .. }
            | HeadedPlayerCommand::StartDefense
            | HeadedPlayerCommand::SelectTreasure { .. } => {}
        }
        Ok(())
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn apply_player_command_legacy(
        &mut self,
        command: HeadedPlayerCommand,
    ) -> Result<(), CommandError> {
        match &command {
            HeadedPlayerCommand::Reroll {
                selected_slot_indices,
            } => {
                if !matches!(self.raw_core.flow(), td_core::GameFlowState::SelectingTower) {
                    return Err(CommandError::InvalidFlow);
                }
                let before_hp = crate::Health::from_raw(self.raw_core.hp_raw());
                let mut raw = self.raw_core.state().clone();
                let health_cost = raw.stage_modifiers().reroll_health_cost;
                let rerolled = raw.reroll_cards(selected_slot_indices)?;
                raw.trigger_card_reroll_upgrades();
                let actual_damage = before_hp.raw().saturating_sub(raw.hp_raw()).max(0);
                self.restore_raw_core_projection(raw)?;
                crate::game_state::presentation_effect::apply_card_reroll(
                    self,
                    rerolled,
                    crate::Damage::from_usize(health_cost),
                    crate::Damage::from_raw(actual_damage),
                );
            }
            HeadedPlayerCommand::PurchaseShopItem { slot_index } => {
                let slot_id = self.shop_slot_id(*slot_index)?;
                let flow = self.presentation_flow_snapshot();
                let slot = match &flow {
                    GameFlow::Shopping(flow) => flow
                        .shop
                        .get_slot_by_id(slot_id)
                        .ok_or(CommandError::InvalidIndex)?
                        .slot
                        .clone(),
                    _ => return Err(CommandError::InvalidFlow),
                };
                let raw_slot_index = match self.raw_core.flow() {
                    td_core::GameFlowState::Shopping(raw_shop) => raw_shop
                        .slots
                        .iter()
                        .position(|raw_slot| raw_slot.id == slot_id.raw())
                        .ok_or(CommandError::InvalidIndex)?,
                    _ => return Err(CommandError::InvalidFlow),
                };
                let mut raw = self.raw_core.state().clone();
                let purchase = raw.purchase_shop_item(raw_slot_index)?;
                match &purchase.slot {
                    td_core::ShopSlotState::Item { item, .. } => {
                        raw.grant_inventory_item(item.clone())?;
                    }
                    td_core::ShopSlotState::Upgrade { upgrade, .. } => {
                        let recovery = raw.acquire_upgrade(upgrade.clone())?.recovery;
                        raw.apply_upgrade_recovery(recovery);
                    }
                    td_core::ShopSlotState::CardService { kind, .. } => {
                        raw.begin_card_service_selection_raw(*kind)?;
                    }
                }
                let cost = purchase.cost;
                self.restore_raw_core_projection(raw)?;
                crate::game_state::presentation_effect::apply_shop_purchase(
                    self, slot_id, slot, cost,
                );
            }
            HeadedPlayerCommand::UseInventoryItem { item_index } => {
                let item = self
                    .presentation_items_snapshot()
                    .get(*item_index)
                    .cloned()
                    .ok_or(CommandError::InvalidIndex)?;
                let mut raw = self.raw_core.state().clone();
                raw.use_inventory_item(*item_index)?;
                self.restore_raw_core_projection(raw)?;
                crate::game_state::presentation_effect::apply_inventory_item(self, &item);
            }
            HeadedPlayerCommand::StartSelectingTower => {
                if !matches!(self.raw_core.flow(), td_core::GameFlowState::Shopping(_)) {
                    return Err(CommandError::InvalidFlow);
                }
                let mut raw = self.raw_core.state().clone();
                if !raw.start_selecting_tower() {
                    return Err(CommandError::InvalidFlow);
                }
                self.restore_raw_core_projection(raw)?;
            }
            HeadedPlayerCommand::SelectTower {
                selected_slot_indices,
            } => {
                if !matches!(self.raw_core.flow(), td_core::GameFlowState::SelectingTower) {
                    return Err(CommandError::InvalidFlow);
                }
                let mut raw = self.raw_core.state().clone();
                raw.select_tower(selected_slot_indices)?;
                self.restore_raw_core_projection(raw)?;
            }
            HeadedPlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => {
                if !matches!(self.raw_core.flow(), td_core::GameFlowState::PlacingTower) {
                    return Err(CommandError::InvalidFlow);
                }
                let mut raw = self.raw_core.state().clone();
                let output = raw.place_tower(*hand_slot_index, *left, *top)?;
                let tower_id = output.tower.id.ok_or(CommandError::Rejected)?;
                raw.trigger_tower_placed_upgrades(
                    tower_id,
                    td_core::rank_is_face(output.tower.template.rank),
                    &output.tower.template,
                );
                raw.refresh_tower_damage_multipliers();
                self.restore_raw_core_projection(raw)?;
                let tower = self
                    .presentation_tower(crate::TowerId::from_raw(tower_id))
                    .ok_or(CommandError::Rejected)?;
                crate::game_state::presentation_effect::apply_place_tower(self, &tower);
            }
            HeadedPlayerCommand::RemoveTower { tower_id } => {
                let tower_id = crate::TowerId::from_raw(*tower_id);
                if self
                    .raw_core
                    .towers()
                    .iter()
                    .all(|tower| tower.id != Some(tower_id.raw()))
                {
                    return Err(CommandError::UnknownTower);
                }
                let mut raw = self.raw_core.state().clone();
                let removed = raw
                    .remove_tower(tower_id.raw())
                    .ok_or(CommandError::UnknownTower)?;
                raw.trigger_tower_removed_upgrades(removed.rerolled_count);
                self.restore_raw_core_projection(raw)?;
                crate::game_state::presentation_effect::apply_remove_tower(self, tower_id);
            }
            HeadedPlayerCommand::StartDefense => {
                if !matches!(self.raw_core.flow(), td_core::GameFlowState::PlacingTower) {
                    return Err(CommandError::InvalidFlow);
                }
                let mut raw = self.raw_core.state().clone();
                if !raw.start_defense() {
                    return Err(CommandError::InvalidFlow);
                }
                self.restore_raw_core_projection(raw)?;
            }
            HeadedPlayerCommand::SelectTreasure { option_index } => {
                let mut raw = self.raw_core.state().clone();
                raw.select_treasure(*option_index)?;
                self.restore_raw_core_projection(raw)?;
            }
            HeadedPlayerCommand::ConfirmCardServiceSelection { selected_card_ids } => {
                let selected_card_ids = selected_card_ids
                    .iter()
                    .map(|card_ids| {
                        card_ids
                            .iter()
                            .copied()
                            .map(|card_id| {
                                usize::try_from(card_id).map_err(|_| CommandError::InvalidIndex)
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let mut raw = self.raw_core.state().clone();
                raw.apply_card_service_selection_mutation(&selected_card_ids)?;
                self.restore_raw_core_projection(raw)?;
                self.set_user_modal(None);
            }
        }

        let recorded_command = PlayerCommand::from(&command);
        let receipt = self.raw_core.record_compatibility_command(recorded_command);
        self.raw_core.extend_events(receipt.events);
        self.restore_raw_core_projection(self.raw_core.state().clone())?;
        Ok(())
    }

    fn shop_slot_id(&self, slot_index: usize) -> Result<crate::shop::ShopSlotId, CommandError> {
        let flow = self.presentation_flow_snapshot();
        let crate::game_state::flow::GameFlow::Shopping(flow) = &flow else {
            return Err(CommandError::InvalidFlow);
        };
        flow.shop
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .nth(slot_index)
            .map(|slot| slot.id)
            .ok_or(CommandError::InvalidIndex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::flow::GameFlow;
    use crate::game_state::item::RubberConeItem;
    use crate::shop::ShopSlot;

    fn test_state() -> GameState {
        let mut game_state = crate::game_state::create_game_state_with_seed(7);
        game_state.headless = true;
        game_state
    }

    fn record_compatibility_command(state: &mut td_core::CoreState, command: PlayerCommand) {
        let mut session =
            td_core::CoreSession::from_state(state.clone()).expect("test state must be valid");
        session.record_compatibility_command(command);
        *state = session.into_state();
    }

    #[test]
    fn accepted_commands_are_recorded_after_application() {
        let mut game_state = test_state();
        let initial_item_count = game_state.items.len();

        if let GameFlow::Shopping(flow) = &mut game_state.flow {
            flow.shop.push(ShopSlot::Item {
                item: RubberConeItem::standard().into_item(),
                cost: 0,
            });
        } else {
            panic!("expected shopping flow");
        }
        let last_slot_index = match &game_state.flow {
            GameFlow::Shopping(flow) => flow.shop.slots.len() - 1,
            _ => unreachable!(),
        };
        game_state.sync_raw_core_from_projection();
        game_state
            .apply_player_command(HeadedPlayerCommand::PurchaseShopItem {
                slot_index: last_slot_index,
            })
            .unwrap();
        game_state
            .apply_player_command(HeadedPlayerCommand::StartSelectingTower)
            .unwrap();
        game_state
            .apply_player_command(HeadedPlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            })
            .unwrap();
        game_state
            .apply_player_command(HeadedPlayerCommand::UseInventoryItem { item_index: 0 })
            .unwrap();

        assert_eq!(game_state.player_commands.len(), 4);
        assert_eq!(game_state.player_commands[0].sequence, 0);
        assert_eq!(game_state.player_commands[3].sequence, 3);
        assert_eq!(game_state.replay_checkpoints.len(), 4);
        assert_eq!(game_state.replay_checkpoints[3].sequence, 3);
        assert_eq!(
            game_state.replay_checkpoints[3].completed_sim_tick,
            game_state.sim_tick().ticks()
        );
        assert_eq!(
            game_state.replay_checkpoints[3].state_hash,
            game_state.authoritative_hash()
        );
        assert_eq!(game_state.items.len(), initial_item_count);
    }

    #[test]
    fn normal_headed_path_records_a_core_player_command() {
        let mut game_state = test_state();

        game_state
            .apply_player_command(HeadedPlayerCommand::StartSelectingTower)
            .expect("headed command should be accepted from the shopping flow");

        assert_eq!(
            game_state.player_commands[0].command,
            PlayerCommand::StartSelectingTower
        );
    }

    #[test]
    fn rejected_command_does_not_mutate_or_record_state() {
        let mut game_state = test_state();
        let initial_command_count = game_state.player_commands.len();
        let initial_flow = game_state.flow.clone();

        let result = game_state.apply_player_command(HeadedPlayerCommand::StartDefense);

        assert_eq!(result, Err(CommandError::InvalidFlow));
        assert_eq!(game_state.player_commands.len(), initial_command_count);
        assert!(game_state.replay_checkpoints.is_empty());
        assert!(matches!(game_state.flow, GameFlow::Shopping(_)));
        assert!(matches!(initial_flow, GameFlow::Shopping(_)));
    }

    #[test]
    fn headed_and_core_command_schemas_match() {
        let headed = HeadedPlayerCommand::PlaceTower {
            hand_slot_index: 2,
            left: 11,
            top: 17,
        };
        let core = td_core::PlayerCommand::PlaceTower {
            hand_slot_index: 2,
            left: 11,
            top: 17,
        };

        assert_eq!(
            serde_json::to_string(&headed).unwrap(),
            serde_json::to_string(&core).unwrap()
        );
    }

    #[test]
    fn headed_place_tower_runs_raw_placement_upgrades() {
        let mut game_state = test_state();
        game_state.sync_raw_core_from_projection();
        game_state
            .raw_core
            .edit_snapshot(|parts| {
                parts.upgrades.upgrades.push(
                    crate::game_state::upgrade::UpgradeWithId::new(
                        crate::game_state::upgrade::MirrorUpgrade::into_upgrade(),
                    )
                    .to_core_state(),
                )
            })
            .expect("test upgrade edit must preserve a valid snapshot");

        game_state
            .apply_player_command(HeadedPlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        game_state
            .apply_player_command(HeadedPlayerCommand::SelectTower {
                selected_slot_indices: Vec::new(),
            })
            .expect("tower selection should succeed");
        game_state
            .apply_player_command(HeadedPlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            })
            .expect("tower placement should succeed");

        assert_eq!(game_state.raw_core.towers().len(), 1);
        assert_eq!(game_state.raw_core.hand().slots.len(), 1);
        assert!(
            game_state
                .raw_core
                .upgrades()
                .upgrades
                .iter()
                .any(|upgrade| {
                    crate::game_state::upgrade::UpgradeWithId::from_core_state(upgrade.clone())
                        .is_some_and(|upgrade| {
                            matches!(
                                upgrade.upgrade,
                                crate::game_state::upgrade::Upgrade::Mirror(
                                    crate::game_state::upgrade::MirrorUpgrade { pending: false }
                                )
                            )
                        })
                })
        );
    }

    #[test]
    fn headed_place_tower_assigns_name_tag_to_placed_tower() {
        let mut game_state = test_state();
        game_state.sync_raw_core_from_projection();
        game_state
            .raw_core
            .edit_snapshot(|parts| {
                parts.upgrades.upgrades.push(
                    crate::game_state::upgrade::UpgradeWithId::new(
                        crate::game_state::upgrade::NameTagUpgrade::into_upgrade(
                            crate::FixedRatio::from_integer(2),
                        ),
                    )
                    .to_core_state(),
                )
            })
            .expect("test upgrade edit must preserve a valid snapshot");

        game_state
            .apply_player_command(HeadedPlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        game_state
            .apply_player_command(HeadedPlayerCommand::SelectTower {
                selected_slot_indices: Vec::new(),
            })
            .expect("tower selection should succeed");
        game_state
            .apply_player_command(HeadedPlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            })
            .expect("tower placement should succeed");

        let tower_id = game_state.raw_core.towers()[0]
            .id
            .expect("tower should have an ID");
        assert_eq!(
            game_state.raw_core.upgrades().upgrades[0]
                .optional_ids
                .first()
                .copied()
                .flatten(),
            Some(tower_id)
        );
        assert!(game_state.raw_core.towers()[0].damage_multiplier_raw > 1_000_000);
    }

    #[test]
    fn headed_place_tower_applies_camera_reward_for_face_tower() {
        let mut game_state = test_state();
        game_state.sync_raw_core_from_projection();
        game_state
            .raw_core
            .edit_snapshot(|parts| {
                parts.upgrades.upgrades.push(
                    crate::game_state::upgrade::UpgradeWithId::new(
                        crate::game_state::upgrade::CameraUpgrade::into_upgrade(),
                    )
                    .to_core_state(),
                );
                parts.hand.slots = vec![td_core::HandSlotState {
                    id: 1,
                    item: td_core::HandItemState::Card(td_core::CardState {
                        id: 1,
                        suit: 0,
                        rank: 11,
                        polish_pct_raw: 0,
                        engraving: None,
                    }),
                    selected: false,
                }];
            })
            .expect("test hand edit must preserve a valid snapshot");
        let initial_gold = game_state.raw_core.progress().gold;

        game_state
            .apply_player_command(HeadedPlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        game_state
            .apply_player_command(HeadedPlayerCommand::SelectTower {
                selected_slot_indices: Vec::new(),
            })
            .expect("tower selection should succeed");
        game_state
            .apply_player_command(HeadedPlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            })
            .expect("tower placement should succeed");

        assert_eq!(game_state.raw_core.progress().gold, initial_gold + 50);
    }

    #[test]
    fn headed_placement_matches_raw_core_for_extra_slot_and_invalid_position() {
        let mut game_state = test_state();
        game_state.sync_raw_core_from_projection();
        game_state
            .raw_core
            .edit_snapshot(|parts| {
                parts
                    .stage_modifiers
                    .extra_tower_cards
                    .push(td_core::StageModifierTowerCardState {
                        kind: 1,
                        suit: Some(0),
                        rank: Some(12),
                    })
            })
            .expect("test modifier edit must preserve a valid snapshot");
        let mut raw = game_state.raw_core.state().clone();

        game_state
            .apply_player_command(HeadedPlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        assert!(raw.start_selecting_tower());
        record_compatibility_command(&mut raw, PlayerCommand::StartSelectingTower);
        assert_eq!(game_state.raw_core.state(), &raw);

        game_state
            .apply_player_command(HeadedPlayerCommand::SelectTower {
                selected_slot_indices: Vec::new(),
            })
            .expect("tower selection should succeed");
        raw.select_tower(&[])
            .expect("tower selection should succeed");
        record_compatibility_command(
            &mut raw,
            PlayerCommand::SelectTower {
                selected_slot_indices: Vec::new(),
            },
        );
        assert_eq!(game_state.raw_core.state(), &raw);
        assert_eq!(game_state.raw_core.hand().slots.len(), 2);
        assert!(game_state.raw_core.hand().slots[0].selected);
        assert!(!game_state.raw_core.hand().slots[1].selected);
        assert!(
            game_state
                .raw_core
                .stage_modifiers()
                .extra_tower_cards
                .is_empty()
        );

        let invalid_hash = td_core::authoritative_hash(&raw);
        assert_eq!(
            game_state.apply_player_command(HeadedPlayerCommand::PlaceTower {
                hand_slot_index: 1,
                left: 5,
                top: 0,
            }),
            Err(CommandError::InvalidPlacement)
        );
        assert_eq!(
            raw.place_tower(1, 5, 0),
            Err(CommandError::InvalidPlacement)
        );
        assert_eq!(td_core::authoritative_hash(&raw), invalid_hash);
        assert_eq!(game_state.raw_core.state(), &raw);

        game_state
            .apply_player_command(HeadedPlayerCommand::PlaceTower {
                hand_slot_index: 1,
                left: 0,
                top: 0,
            })
            .expect("second tower slot should be placeable");
        let output = raw
            .place_tower(1, 0, 0)
            .expect("second tower slot should be placeable");
        raw.trigger_tower_placed_upgrades(
            output.tower.id.expect("placed tower should have an ID"),
            td_core::rank_is_face(output.tower.template.rank),
            &output.tower.template,
        );
        raw.refresh_tower_damage_multipliers();
        record_compatibility_command(
            &mut raw,
            PlayerCommand::PlaceTower {
                hand_slot_index: 1,
                left: 0,
                top: 0,
            },
        );
        assert_eq!(game_state.raw_core.state(), &raw);
        assert_eq!(game_state.raw_core.towers().len(), 1);
        assert_eq!(game_state.raw_core.hand().slots.len(), 1);
        assert_eq!(game_state.raw_core.hand().slots[0].id, 1);
        assert!(game_state.raw_core.hand().slots[0].selected);
    }

    #[test]
    fn card_service_confirmation_payload_round_trips_between_command_schemas() {
        let headed = HeadedPlayerCommand::ConfirmCardServiceSelection {
            selected_card_ids: vec![vec![3, 11], vec![29]],
        };
        let core = PlayerCommand::from(&headed);
        let decoded: PlayerCommand =
            serde_json::from_str(&serde_json::to_string(&core).unwrap()).unwrap();

        assert_eq!(decoded, core);
        assert_eq!(
            serde_json::to_string(&headed).unwrap(),
            serde_json::to_string(&core).unwrap()
        );
    }
}
