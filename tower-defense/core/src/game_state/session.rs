use super::{CoreState, RecordedTickOutput};
use crate::{
    CommandReceipt, GameConfigState, Observation, PlayerCommand, ReplayCheckpoint, SimTick,
};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CoreSnapshot {
    state: CoreState,
}

/// Explicit restore payload used by compatibility adapters that translate a
/// legacy/presentation projection into an authoritative snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreSnapshotParts {
    pub progress: super::CoreProgress,
    pub sim_tick: crate::SimTick,
    pub rng: crate::RngState,
    pub route: crate::RouteState,
    pub config: crate::GameConfigState,
    pub stage_modifiers: crate::StageModifiersState,
    pub upgrades: crate::UpgradeCollection,
    pub hand: crate::HandState,
    pub deck: crate::DeckState,
    pub items: crate::ItemCollection,
    pub monster_spawn: crate::MonsterSpawnState,
    pub in_flight_attacks: Vec<crate::InFlightAttackState>,
    pub user_status_effects: Vec<crate::UserStatusEffect>,
    pub next_entity_id: crate::EntityIdAllocator,
    pub metrics: super::GameMetrics,
    pub flow: crate::GameFlowState,
    pub hp_raw: i64,
    pub shield_raw: i64,
    pub monsters: Vec<crate::MonsterState>,
    pub towers: Vec<crate::TowerState>,
    pub player_commands: Vec<crate::RecordedPlayerCommand>,
    pub replay_checkpoints: Vec<ReplayCheckpoint>,
    pub pending_card_service_kind: Option<u8>,
    pub card_service_selection: Option<crate::CardServiceSelectionState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotValidationError {
    InvalidState,
}

impl CoreSnapshot {
    pub fn from_state(state: &CoreState) -> Result<Self, SnapshotValidationError> {
        if !state.validate_snapshot() {
            return Err(SnapshotValidationError::InvalidState);
        }
        Ok(state.to_snapshot())
    }

    pub fn as_state(&self) -> &CoreState {
        &self.state
    }

    pub fn into_state(self) -> CoreState {
        self.state
    }

    pub fn into_parts(self) -> CoreSnapshotParts {
        let state = self.state;
        CoreSnapshotParts {
            progress: state.progress,
            sim_tick: state.sim_tick,
            rng: state.rng,
            route: state.route,
            config: state.config,
            stage_modifiers: state.stage_modifiers,
            upgrades: state.upgrades,
            hand: state.hand,
            deck: state.deck,
            items: state.items,
            monster_spawn: state.monster_spawn,
            in_flight_attacks: state.in_flight_attacks,
            user_status_effects: state.user_status_effects,
            next_entity_id: state.next_entity_id,
            metrics: state.metrics,
            flow: state.flow,
            hp_raw: state.hp_raw,
            shield_raw: state.shield_raw,
            monsters: state.monsters,
            towers: state.towers,
            player_commands: state.player_commands,
            replay_checkpoints: state.replay_checkpoints,
            pending_card_service_kind: state.pending_card_service_kind,
            card_service_selection: state.card_service_selection,
        }
    }
}

impl CoreState {
    pub fn clone_without_events(&self) -> Self {
        let mut clone = self.clone();
        clone.events = Default::default();
        clone
    }

    pub fn snapshot_parts(&self) -> CoreSnapshotParts {
        self.to_snapshot().into_parts()
    }

    /// Applies a compatibility/snapshot edit through the restore validator.
    ///
    /// Runtime gameplay should use [`crate::CoreSession::apply`] instead. This
    /// narrow escape hatch exists for headed projection migration and tests that
    /// need to construct a deliberately altered snapshot.
    pub fn edit_snapshot(
        &mut self,
        edit: impl FnOnce(&mut CoreSnapshotParts),
    ) -> Result<(), SnapshotValidationError> {
        let mut parts = self.snapshot_parts();
        edit(&mut parts);
        *self = Self::from_snapshot_parts(parts)?;
        Ok(())
    }

    pub fn to_snapshot(&self) -> CoreSnapshot {
        CoreSnapshot {
            state: self.clone(),
        }
    }

    pub fn from_snapshot(snapshot: CoreSnapshot) -> Result<Self, SnapshotValidationError> {
        let mut state = snapshot.into_state();
        state.hand.migrate_slot_ids();
        if !state.validate_snapshot() {
            return Err(SnapshotValidationError::InvalidState);
        }
        Ok(state)
    }

    pub fn from_snapshot_parts(parts: CoreSnapshotParts) -> Result<Self, SnapshotValidationError> {
        let mut state = Self::from_snapshot_parts_unvalidated(parts)?;
        state.hand.migrate_slot_ids();
        if !state.validate_snapshot() {
            return Err(SnapshotValidationError::InvalidState);
        }
        Ok(state)
    }

    fn from_snapshot_parts_unvalidated(
        parts: CoreSnapshotParts,
    ) -> Result<Self, SnapshotValidationError> {
        Ok(Self {
            progress: parts.progress,
            sim_tick: parts.sim_tick,
            rng: parts.rng,
            route: parts.route,
            config: parts.config,
            stage_modifiers: parts.stage_modifiers,
            upgrades: parts.upgrades,
            hand: parts.hand,
            deck: parts.deck,
            items: parts.items,
            monster_spawn: parts.monster_spawn,
            in_flight_attacks: parts.in_flight_attacks,
            user_status_effects: parts.user_status_effects,
            next_entity_id: parts.next_entity_id,
            metrics: parts.metrics,
            flow: parts.flow,
            hp_raw: parts.hp_raw,
            shield_raw: parts.shield_raw,
            monsters: parts.monsters,
            towers: parts.towers,
            player_commands: parts.player_commands,
            replay_checkpoints: parts.replay_checkpoints,
            pending_card_service_kind: parts.pending_card_service_kind,
            card_service_selection: parts.card_service_selection,
            events: Default::default(),
        })
    }
}

pub struct CoreSession {
    state: CoreState,
}

// Keep read-only compatibility access for projections and snapshots while
// directing all session mutation through explicit APIs.
impl Deref for CoreSession {
    type Target = CoreState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl CoreSession {
    pub fn new(config: GameConfigState, seed: u64) -> Self {
        Self {
            state: CoreState::new_initial(config, seed),
        }
    }

    pub fn from_snapshot(snapshot: CoreSnapshot) -> Result<Self, SnapshotValidationError> {
        Ok(Self {
            state: CoreState::from_snapshot(snapshot)?,
        })
    }

    /// Creates a session around an already validated authoritative state.
    ///
    /// This is intended for headed adapters and persistence migration. New
    /// runtime code should prefer [`CoreSession::new`] or
    /// [`CoreSession::from_snapshot`].
    pub fn from_state(mut state: CoreState) -> Result<Self, SnapshotValidationError> {
        state.hand.migrate_slot_ids();
        if !state.validate_snapshot() {
            return Err(SnapshotValidationError::InvalidState);
        }
        Ok(Self { state })
    }

    /// Creates a session around a state that is validated by a later
    /// persistence or migration boundary.
    ///
    /// Runtime callers should use [`Self::from_state`] so invalid snapshots
    /// are rejected before execution.
    pub fn from_unvalidated_state(state: CoreState) -> Self {
        Self { state }
    }

    /// Returns the authoritative state for read-only projections.
    pub fn raw_state(&self) -> &CoreState {
        &self.state
    }

    /// Drains events that were produced by compatibility state edits.
    ///
    /// Normal runtime callers should consume events from [`CommandReceipt`] or
    /// [`RecordedTickOutput`]. This method exists for headed presentation
    /// bridges.
    pub fn drain_events(&mut self) -> std::vec::Drain<'_, crate::CoreEvent> {
        self.state.drain_events()
    }

    pub fn extend_events(&mut self, events: impl IntoIterator<Item = crate::CoreEvent>) {
        self.state.extend_events(events);
    }

    /// Consumes the session and returns its authoritative state.
    pub fn into_state(self) -> CoreState {
        self.state
    }

    pub fn snapshot(&self) -> Result<CoreSnapshot, SnapshotValidationError> {
        CoreSnapshot::from_state(&self.state)
    }

    /// Applies a compatibility or test snapshot edit through validation.
    ///
    /// Runtime gameplay must use [`Self::apply`] or [`Self::tick`]. This
    /// narrow escape hatch never exposes a mutable [`CoreState`] reference.
    pub fn edit_snapshot(
        &mut self,
        edit: impl FnOnce(&mut CoreSnapshotParts),
    ) -> Result<(), SnapshotValidationError> {
        self.state.edit_snapshot(edit)
    }

    pub fn state(&self) -> &CoreState {
        &self.state
    }

    pub fn authoritative_hash(&self) -> String {
        crate::authoritative_hash(&self.state)
    }

    pub fn sim_tick(&self) -> SimTick {
        self.state.sim_tick()
    }

    pub fn observation(
        &self,
        environment_version: u32,
        action_schema_version: u32,
        map_width: usize,
        map_height: usize,
    ) -> Observation {
        self.state.observation(
            environment_version,
            action_schema_version,
            map_width,
            map_height,
        )
    }

    /// Applies one authoritative player command.
    pub fn apply(&mut self, command: PlayerCommand) -> Result<CommandReceipt, crate::CommandError> {
        Self::apply_to(&mut self.state, command)
    }

    /// Records a command after an explicit compatibility-state transition.
    ///
    /// Runtime gameplay must use [`Self::apply`]. This API is reserved for
    /// adapters that perform a separately validated legacy or presentation
    /// transition and then need to preserve replay metadata.
    pub fn record_compatibility_command(&mut self, command: PlayerCommand) -> CommandReceipt {
        self.state.record_accepted_command(command)
    }

    pub(crate) fn apply_to(
        state: &mut CoreState,
        command: PlayerCommand,
    ) -> Result<CommandReceipt, crate::CommandError> {
        if let PlayerCommand::ConfirmCardServiceSelection { selected_card_ids } = &command {
            let selected_card_ids = selected_card_ids
                .iter()
                .map(|card_ids| {
                    card_ids
                        .iter()
                        .copied()
                        .map(|card_id| {
                            usize::try_from(card_id).map_err(|_| crate::CommandError::InvalidIndex)
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?;
            state.apply_card_service_selection_mutation(&selected_card_ids)?;
            return Ok(state.record_accepted_command(command));
        }

        if let PlayerCommand::UseInventoryItem { item_index } = &command {
            state.use_inventory_item(*item_index)?;
            return Ok(state.record_accepted_command(command));
        }

        if let PlayerCommand::PurchaseShopItem { slot_index } = &command {
            let mut next = state.clone();
            let purchase = next.purchase_shop_item(*slot_index)?;
            if let crate::ShopSlotState::Item { item, .. } = &purchase.slot {
                next.grant_inventory_item(item.clone())?;
            }
            if let crate::ShopSlotState::Upgrade { upgrade, .. } = &purchase.slot {
                let acquire = next.acquire_upgrade(upgrade.clone())?;
                next.apply_upgrade_recovery(acquire.recovery);
            }
            if let crate::ShopSlotState::CardService { kind, .. } = &purchase.slot {
                next.begin_card_service_selection_raw(*kind)?;
            }
            *state = next;
            return Ok(state.record_accepted_command(command));
        }

        match command.clone() {
            PlayerCommand::Reroll {
                selected_slot_indices,
            } => {
                let mut next = state.clone();
                next.reroll_cards(&selected_slot_indices)?;
                next.trigger_card_reroll_upgrades();
                *state = next;
            }
            PlayerCommand::StartSelectingTower => {
                if !state.start_selecting_tower() {
                    return Err(crate::CommandError::InvalidFlow);
                }
            }
            PlayerCommand::SelectTower {
                selected_slot_indices,
            } => {
                let mut next = state.clone();
                next.select_tower(&selected_slot_indices)?;
                *state = next;
            }
            PlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => {
                let mut next = state.clone();
                let output = next.place_tower(hand_slot_index, left, top)?;
                next.trigger_tower_placed_upgrades(
                    output.tower.id.ok_or(crate::CommandError::Rejected)?,
                    crate::rank_is_face(output.tower.template.rank),
                    &output.tower.template,
                );
                next.refresh_tower_damage_multipliers();
                *state = next;
            }
            PlayerCommand::RemoveTower { tower_id } => {
                let removed = state
                    .remove_tower(tower_id)
                    .ok_or(crate::CommandError::UnknownTower)?;
                state.trigger_tower_removed_upgrades(removed.rerolled_count);
            }
            PlayerCommand::SelectTreasure { option_index } => {
                state.select_treasure(option_index)?;
            }
            PlayerCommand::StartDefense => {
                if !state.start_defense() {
                    return Err(crate::CommandError::InvalidFlow);
                }
            }
            PlayerCommand::ConfirmCardServiceSelection { .. }
            | PlayerCommand::UseInventoryItem { .. }
            | PlayerCommand::PurchaseShopItem { .. } => unreachable!(),
        }
        Ok(state.record_accepted_command(command))
    }

    /// Advances exactly one fixed simulation tick and records replay metadata.
    pub fn advance_tick(&mut self) -> RecordedTickOutput {
        self.state.advance_tick()
    }

    /// Advances exactly one fixed simulation tick without recording tick
    /// events, event digests, or an authoritative hash.
    ///
    /// This is intended for explicit bulk statistics runs. It preserves the
    /// authoritative gameplay transition, but the discarded event metadata
    /// makes the run non-replayable at those tick boundaries. Accepted
    /// commands still use the normal full [`CommandReceipt`] path.
    pub fn advance_tick_unrecorded(&mut self) -> crate::TickTransition {
        self.state.advance_tick_unrecorded()
    }

    /// Advances one fixed tick for headed presentation.
    ///
    /// This preserves authoritative events for presentation bridges while
    /// omitting replay-only state hashes and event digests.
    pub fn advance_tick_with_events(&mut self) -> crate::TickEventsOutput {
        self.state.advance_tick_with_events()
    }

    /// Compatibility alias for callers that use the simulator terminology.
    #[deprecated(note = "use advance_tick")]
    pub fn step(&mut self) -> RecordedTickOutput {
        self.advance_tick()
    }

    /// Compatibility alias for the old recorded tick name.
    #[deprecated(note = "use advance_tick")]
    pub fn tick(&mut self) -> RecordedTickOutput {
        self.advance_tick()
    }

    /// Compatibility alias for the explicit bulk-statistics tick path.
    #[deprecated(note = "use advance_tick_unrecorded")]
    pub fn step_fast(&mut self) -> crate::TickTransition {
        self.advance_tick_unrecorded()
    }

    /// Compatibility alias for the old unrecorded tick name.
    #[deprecated(note = "use advance_tick_unrecorded")]
    pub fn tick_fast(&mut self) -> crate::TickTransition {
        self.advance_tick_unrecorded()
    }

    /// Compatibility alias for headed presentation tick callers.
    #[deprecated(note = "use advance_tick_with_events")]
    pub fn step_presentation(&mut self) -> crate::TickEventsOutput {
        self.advance_tick_with_events()
    }

    /// Compatibility alias for the old headed presentation tick name.
    #[deprecated(note = "use advance_tick_with_events")]
    pub fn tick_presentation(&mut self) -> crate::TickEventsOutput {
        self.advance_tick_with_events()
    }

    pub fn replay_checkpoints(&self) -> &[ReplayCheckpoint] {
        self.state.replay_checkpoints()
    }

    pub fn replay(&self) -> crate::CoreReplay {
        crate::CoreReplay::from_state(&self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> crate::GameConfigState {
        crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 5,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: (0..=10)
                    .map(|kind| crate::TowerConfigEntryState {
                        kind,
                        damage_raw: 1_000,
                        range_raw: 1_000_000,
                        cooldown_ms: 1_000,
                    })
                    .collect(),
            },
            monsters: crate::MonsterConfigState {
                stats: vec![crate::MonsterConfigEntryState {
                    kind: 0,
                    base_hp_raw: 1_000,
                    velocity_mul_raw: crate::RATIO_SCALE,
                    damage_raw: 100,
                    reward: 1,
                }],
                stage_waves: vec![crate::StageWaveState {
                    stage: 1,
                    entries: vec![crate::StageWaveEntryState { kind: 0, count: 1 }],
                }],
            },
        }
    }

    #[test]
    fn snapshot_serde_is_compatible_with_raw_core_state_serde() {
        let state = CoreState::new_initial(test_config(), 7);
        let snapshot = CoreSnapshot::from_state(&state).expect("initial state is valid");
        let state_json = serde_json::to_string(&state).expect("state serialization");
        let snapshot_json = serde_json::to_string(&snapshot).expect("snapshot serialization");

        assert_eq!(snapshot_json, state_json);
        let decoded: CoreSnapshot =
            serde_json::from_str(&state_json).expect("snapshot compatibility decode");
        assert_eq!(decoded.as_state(), &state);
    }

    #[test]
    fn legacy_snapshot_without_hand_allocator_gets_a_migration_default() {
        let state = CoreState::new_initial(test_config(), 7);
        let mut value = serde_json::to_value(&state).expect("state serialization");
        value
            .get_mut("hand")
            .and_then(serde_json::Value::as_object_mut)
            .expect("hand object")
            .remove("next_hand_slot_id");
        let snapshot: CoreSnapshot =
            serde_json::from_value(value).expect("legacy snapshot deserialization");
        let restored = CoreState::from_snapshot(snapshot).expect("legacy snapshot should migrate");

        assert_eq!(restored.hand().next_hand_slot_id, 6);
    }

    #[test]
    fn snapshot_edit_is_validated_before_replacement() {
        let mut state = CoreState::new_initial(test_config(), 7);
        let hash = crate::authoritative_hash(&state);
        let result = state.edit_snapshot(|parts| {
            parts.pending_card_service_kind = Some(u8::MAX);
        });

        assert_eq!(result, Err(SnapshotValidationError::InvalidState));
        assert_eq!(crate::authoritative_hash(&state), hash);
    }

    #[test]
    fn public_snapshot_parts_restore_preserves_authoritative_state() {
        let state = CoreState::new_initial(test_config(), 7);
        let snapshot = CoreSnapshot::from_state(&state).expect("initial state is valid");
        let restored = CoreState::from_snapshot_parts(snapshot.into_parts())
            .expect("public snapshot parts should restore");

        assert_eq!(restored, state);
        assert_eq!(
            crate::authoritative_hash(&restored),
            crate::authoritative_hash(&state)
        );
    }

    #[test]
    fn accepted_command_receipt_and_snapshot_preserve_session_boundary() {
        let mut session = CoreSession::new(test_config(), 7);
        let receipt = session
            .apply(crate::PlayerCommand::StartSelectingTower)
            .expect("initial tower selection command should be accepted");

        assert_eq!(receipt.sequence, 0);
        assert_eq!(receipt.completed_sim_tick, session.sim_tick().ticks());
        assert_eq!(receipt.state_hash, session.authoritative_hash());
        assert_eq!(receipt.event_count, receipt.events.len() as u64);
        assert_eq!(receipt.event_digest, crate::event_digest(&receipt.events));
        let checkpoint = session
            .replay_checkpoints()
            .last()
            .expect("accepted command checkpoint");
        assert_eq!(checkpoint.event_count, receipt.event_count);
        assert_eq!(checkpoint.event_digest, receipt.event_digest);

        let snapshot = session
            .snapshot()
            .expect("accepted command should leave a valid snapshot");
        let restored =
            CoreSession::from_snapshot(snapshot).expect("snapshot should restore after command");
        assert_eq!(restored.authoritative_hash(), session.authoritative_hash());
        assert_eq!(restored.state(), session.state());

        let tick = session.advance_tick();
        assert_eq!(tick.event_count, tick.events.len() as u64);
        assert_eq!(tick.event_digest, crate::event_digest(&tick.events));
    }

    #[test]
    fn fast_ticks_preserve_authoritative_state_without_tick_metadata() {
        let config = test_config();
        let mut full = CoreSession::new(config.clone(), 7);
        let mut fast = CoreSession::new(config, 7);

        let command = crate::PlayerCommand::StartSelectingTower;
        let full_receipt = full
            .apply(command.clone())
            .expect("command should be accepted");
        let fast_receipt = fast.apply(command).expect("command should be accepted");
        assert_eq!(full_receipt, fast_receipt);
        let checkpoint_count = fast.replay_checkpoints().len();

        for _ in 0..8 {
            full.state
                .extend_events([crate::CoreEvent::DefenseStarted { stage: 1 }]);
            fast.state
                .extend_events([crate::CoreEvent::DefenseStarted { stage: 1 }]);
            let full_output = full.advance_tick();
            let fast_output = fast.advance_tick_unrecorded();

            assert_eq!(full_output.sim_tick, fast_output.sim_tick);
            assert_eq!(full_output.defense_end, fast_output.defense_end);
            assert_eq!(full_output.events.len(), 1);
            assert_eq!(full_output.event_count, 1);
            assert_eq!(
                full_output.event_digest,
                crate::event_digest(&full_output.events)
            );
            assert!(fast.drain_events().next().is_none());
            assert_eq!(full.authoritative_hash(), fast.authoritative_hash());
            assert_eq!(
                full.state().clone_without_events(),
                fast.state().clone_without_events()
            );
            assert_eq!(fast.replay_checkpoints().len(), checkpoint_count);
        }
    }

    #[test]
    fn presentation_ticks_preserve_events_without_replay_metadata() {
        let config = test_config();
        let mut full = CoreSession::new(config.clone(), 7);
        let mut presentation = CoreSession::new(config, 7);

        let command = crate::PlayerCommand::StartSelectingTower;
        full.apply(command.clone())
            .expect("command should be accepted");
        presentation
            .apply(command)
            .expect("command should be accepted");
        let checkpoint_count = presentation.replay_checkpoints().len();

        for _ in 0..8 {
            let event = crate::CoreEvent::DefenseStarted { stage: 1 };
            full.state.extend_events([event.clone()]);
            presentation.state.extend_events([event.clone()]);

            let full_output = full.advance_tick();
            let presentation_output = presentation.advance_tick_with_events();

            assert_eq!(presentation_output.sim_tick, full_output.sim_tick);
            assert_eq!(presentation_output.defense_end, full_output.defense_end);
            assert_eq!(presentation_output.events, full_output.events);
            assert_eq!(full.authoritative_hash(), presentation.authoritative_hash());
            assert_eq!(
                full.state().clone_without_events(),
                presentation.state().clone_without_events()
            );
            assert_eq!(presentation.replay_checkpoints().len(), checkpoint_count);
        }
    }
}
