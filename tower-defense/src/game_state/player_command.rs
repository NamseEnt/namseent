use crate::MapCoord;
use crate::flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template;
use crate::game_state::{GameState, GameStateAction, flow::GameFlow, tower::Tower};
use crate::hand::{HandItem, HandSlotId};
use namui::*;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, State)]
#[serde(tag = "type", content = "payload")]
pub(crate) enum PlayerCommand {
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
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, State)]
pub(crate) struct RecordedPlayerCommand {
    pub(crate) sequence: u64,
    pub(crate) completed_sim_tick: u64,
    pub(crate) command: PlayerCommand,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PlayerCommandError {
    InvalidFlow,
    InvalidIndex,
    InvalidSelection,
    InvalidPlacement,
    UnknownTower,
    Rejected,
}

impl GameState {
    pub(crate) fn apply_player_command(
        &mut self,
        command: PlayerCommand,
    ) -> Result<(), PlayerCommandError> {
        match &command {
            PlayerCommand::Reroll {
                selected_slot_indices,
            } => {
                if !matches!(self.flow, GameFlow::SelectingTower(_)) {
                    return Err(PlayerCommandError::InvalidFlow);
                }
                let slot_ids = self.active_slot_ids(selected_slot_indices)?;
                let health_cost = self.stage_modifiers.get_reroll_health_cost();
                if slot_ids.is_empty() && self.hand.is_empty()
                    || self.left_dice == 0
                        && self
                            .hp
                            .saturating_sub(crate::Health::from_usize(health_cost))
                            <= crate::Health::from_integer(1)
                {
                    return Err(PlayerCommandError::InvalidSelection);
                }
                self.cards_at_indices(selected_slot_indices)?;
                for slot_id in self.hand.active_slot_ids() {
                    self.hand.deselect_slot(slot_id);
                }
                for slot_id in slot_ids {
                    self.hand.select_slot(slot_id);
                }
                self.action(GameStateAction::CardReroll);
            }
            PlayerCommand::PurchaseShopItem { slot_index } => {
                let slot_id = self.shop_slot_id(*slot_index)?;
                if !self.action(GameStateAction::PurchaseShopItem(slot_id)) {
                    return Err(PlayerCommandError::Rejected);
                }
            }
            PlayerCommand::UseInventoryItem { item_index } => {
                let item = self
                    .items
                    .get(*item_index)
                    .ok_or(PlayerCommandError::InvalidIndex)?;
                if item.can_use(self).is_err() {
                    return Err(PlayerCommandError::Rejected);
                }
                let item_id = item.id;
                let item_count = self.items.len();
                self.action(GameStateAction::UseInventoryItem(item_id));
                if self.items.len() == item_count {
                    return Err(PlayerCommandError::Rejected);
                }
            }
            PlayerCommand::StartSelectingTower => {
                if !matches!(self.flow, GameFlow::Shopping(_)) {
                    return Err(PlayerCommandError::InvalidFlow);
                }
                self.action(GameStateAction::StartSelectingTower);
            }
            PlayerCommand::SelectTower {
                selected_slot_indices,
            } => {
                if !matches!(self.flow, GameFlow::SelectingTower(_)) {
                    return Err(PlayerCommandError::InvalidFlow);
                }
                let cards = self.cards_at_indices(selected_slot_indices)?;
                if cards.is_empty() {
                    return Err(PlayerCommandError::InvalidSelection);
                }
                let template = get_highest_tower_template(
                    &cards,
                    &self.upgrade_state,
                    self.rerolled_count,
                    &self.config,
                );
                self.action(GameStateAction::StartPlacingTower(template));
            }
            PlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => {
                if !matches!(self.flow, GameFlow::PlacingTower) {
                    return Err(PlayerCommandError::InvalidFlow);
                }
                let slot_id = self
                    .hand
                    .active_slot_id_by_index(*hand_slot_index)
                    .ok_or(PlayerCommandError::InvalidIndex)?;
                let template = self
                    .hand
                    .get_item(slot_id)
                    .and_then(HandItem::as_tower)
                    .cloned()
                    .ok_or(PlayerCommandError::InvalidSelection)?;
                let tower = Tower::new(&template, MapCoord::new(*left, *top), self.sim_tick());
                let previous_count = self.towers.iter().count();
                self.action(GameStateAction::PlaceTower(Box::new(tower), Some(slot_id)));
                if self.towers.iter().count() == previous_count {
                    return Err(PlayerCommandError::InvalidPlacement);
                }
            }
            PlayerCommand::RemoveTower { tower_id } => {
                let tower_id = crate::TowerId::from_raw(*tower_id);
                if self.towers.iter().all(|tower| tower.id() != tower_id) {
                    return Err(PlayerCommandError::UnknownTower);
                }
                self.action(GameStateAction::RemoveTower(tower_id));
            }
            PlayerCommand::StartDefense => {
                if !matches!(self.flow, GameFlow::PlacingTower) {
                    return Err(PlayerCommandError::InvalidFlow);
                }
                self.action(GameStateAction::StartDefense);
            }
            PlayerCommand::SelectTreasure { option_index } => {
                let GameFlow::TreasureSelection(flow) = &self.flow else {
                    return Err(PlayerCommandError::InvalidFlow);
                };
                let upgrade = flow
                    .options
                    .get(*option_index)
                    .cloned()
                    .ok_or(PlayerCommandError::InvalidIndex)?;
                self.action(GameStateAction::Upgrade(upgrade, None));
                self.action(GameStateAction::StartStage { stage: self.stage });
            }
        }

        let sequence = self.player_command_sequence;
        self.player_command_sequence = sequence.wrapping_add(1);
        self.player_commands.push(RecordedPlayerCommand {
            sequence,
            completed_sim_tick: self.sim_tick().ticks(),
            command,
        });
        self.replay_checkpoints
            .push(crate::game_state::replay::ReplayCheckpoint {
                sequence,
                completed_sim_tick: self.sim_tick().ticks(),
                state_hash: self.authoritative_hash(),
            });
        Ok(())
    }

    fn active_slot_ids(
        &self,
        selected_slot_indices: &[usize],
    ) -> Result<Vec<HandSlotId>, PlayerCommandError> {
        let slot_indices = if selected_slot_indices.is_empty() {
            (0..self.hand.active_slot_ids().len()).collect()
        } else {
            selected_slot_indices.to_vec()
        };
        slot_indices
            .iter()
            .map(|index| {
                self.hand
                    .active_slot_ids()
                    .get(*index)
                    .copied()
                    .ok_or(PlayerCommandError::InvalidIndex)
            })
            .collect()
    }

    fn cards_at_indices(
        &self,
        selected_slot_indices: &[usize],
    ) -> Result<Vec<crate::card::Card>, PlayerCommandError> {
        let slot_indices = if selected_slot_indices.is_empty() {
            (0..self.hand.active_slot_ids().len()).collect()
        } else {
            selected_slot_indices.to_vec()
        };
        slot_indices
            .iter()
            .map(|index| {
                let slot_id = self
                    .hand
                    .active_slot_ids()
                    .get(*index)
                    .copied()
                    .ok_or(PlayerCommandError::InvalidIndex)?;
                self.hand
                    .get_item(slot_id)
                    .and_then(HandItem::as_card)
                    .copied()
                    .ok_or(PlayerCommandError::InvalidSelection)
            })
            .collect()
    }

    fn shop_slot_id(
        &self,
        slot_index: usize,
    ) -> Result<crate::shop::ShopSlotId, PlayerCommandError> {
        let GameFlow::Shopping(flow) = &self.flow else {
            return Err(PlayerCommandError::InvalidFlow);
        };
        flow.shop
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .nth(slot_index)
            .map(|slot| slot.id)
            .ok_or(PlayerCommandError::InvalidIndex)
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
        game_state
            .apply_player_command(PlayerCommand::PurchaseShopItem {
                slot_index: last_slot_index,
            })
            .unwrap();
        game_state
            .apply_player_command(PlayerCommand::StartSelectingTower)
            .unwrap();
        game_state
            .apply_player_command(PlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            })
            .unwrap();
        game_state
            .apply_player_command(PlayerCommand::UseInventoryItem { item_index: 0 })
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
    fn rejected_command_does_not_mutate_or_record_state() {
        let mut game_state = test_state();
        let initial_command_count = game_state.player_commands.len();
        let initial_flow = game_state.flow.clone();

        let result = game_state.apply_player_command(PlayerCommand::StartDefense);

        assert_eq!(result, Err(PlayerCommandError::InvalidFlow));
        assert_eq!(game_state.player_commands.len(), initial_command_count);
        assert!(game_state.replay_checkpoints.is_empty());
        assert!(matches!(game_state.flow, GameFlow::Shopping(_)));
        assert!(matches!(initial_flow, GameFlow::Shopping(_)));
    }
}
