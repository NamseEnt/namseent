use crate::config::GameConfig;
use td_core::{CommandError, PlayerCommand, SimTick};

pub use td_core::CoreEvent;

pub use td_core::{CommandOutput, CommandReceipt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health(i64);

impl Health {
    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }

    pub const fn from_integer(value: i64) -> Self {
        Self::from_raw(value.saturating_mul(1_000))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shield(i64);

impl Shield {
    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < 0 { 0 } else { raw })
    }

    pub const fn raw(self) -> i64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ClearRate(i64);

impl ClearRate {
    pub fn from_raw(raw: i64) -> Self {
        Self(raw.clamp(0, td_core::RATIO_SCALE))
    }

    pub fn as_percent_f32(self) -> f32 {
        self.0 as f32 * 100.0 / td_core::RATIO_SCALE as f32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefenseEndTransition {
    GameOver,
    TreasureSelection,
    StartStage { stage: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefenseEndOutput {
    pub perfect_clear: bool,
    pub gold: usize,
    pub item_count: usize,
    pub card_count: usize,
    pub transition: DefenseEndTransition,
}

/// Simulator-facing result of one fully recorded fixed simulation tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedTickOutput {
    pub sim_tick: SimTick,
    pub events: Vec<CoreEvent>,
    pub defense_end: Option<DefenseEndOutput>,
    pub event_count: u64,
    pub event_digest: String,
}

#[deprecated(note = "use RecordedTickOutput")]
pub type StepOutput = RecordedTickOutput;

/// Simulator-facing result of one unrecorded fixed simulation tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TickTransition {
    pub sim_tick: SimTick,
    pub defense_end: Option<DefenseEndOutput>,
}

#[deprecated(note = "use RecordedTickOutput")]
pub type CoreTickOutput = RecordedTickOutput;

#[deprecated(note = "use TickTransition")]
pub type FastTickOutput = TickTransition;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RewardMetricsSnapshot {
    pub total_gold_earned: usize,
    pub total_escaped_hp: f32,
    pub total_player_damage: f32,
    pub total_tower_damage: f32,
    pub stage_damage: Vec<(usize, f32)>,
}

pub struct GameCore {
    session: td_core::CoreSession,
    deferred_card_service_selection: Option<td_core::CardServiceSelectionState>,
}

impl GameCore {
    fn build_deferred_card_service_selection(
        service_kind: td_core::CardServiceKind,
    ) -> Option<td_core::CardServiceSelectionState> {
        td_core::CardServiceSelectionState::new(service_kind)
    }

    fn request_card_service_selection(&mut self, service_kind: td_core::CardServiceKind) {
        self.deferred_card_service_selection =
            Self::build_deferred_card_service_selection(service_kind);
    }

    #[cfg(all(feature = "simulator", test))]
    pub(crate) fn set_hp_for_test(&mut self, hp: Health) {
        self.session
            .edit_snapshot(|parts| parts.hp_raw = hp.raw())
            .expect("test HP edit must preserve a valid snapshot");
    }

    #[cfg(all(feature = "simulator", test))]
    pub(crate) fn set_left_dice_for_test(&mut self, left_dice: usize) {
        self.session
            .edit_snapshot(|parts| parts.progress.left_dice = left_dice)
            .expect("test dice edit must preserve a valid snapshot");
    }

    #[cfg(all(feature = "simulator", test))]
    pub(crate) fn add_card_service_shop_slot_for_test(&mut self) -> usize {
        let mut slot_count = None;
        self.session
            .edit_snapshot(|parts| {
                let td_core::GameFlowState::Shopping(shop) = &mut parts.flow else {
                    return;
                };
                let slot_id = shop.slots.iter().map(|slot| slot.id).max().unwrap_or(0) + 1;
                shop.slots.push(td_core::ShopSlotDataState {
                    id: slot_id,
                    slot: td_core::ShopSlotState::CardService { kind: 7, cost: 0 },
                    purchased: false,
                });
                slot_count = Some(shop.slots.len() - 1);
            })
            .expect("test shop edit must preserve a valid snapshot");
        let Some(slot_count) = slot_count else {
            panic!("expected initial shopping flow");
        };
        slot_count
    }

    #[cfg(all(feature = "simulator", test))]
    pub(crate) fn shop_slot_id_for_test(&self, slot_index: usize) -> usize {
        let td_core::GameFlowState::Shopping(shop) = self.session.flow() else {
            panic!("expected initial shopping flow");
        };
        shop.slots[slot_index].id
    }

    pub(crate) fn replay(&self) -> td_core::CoreReplay {
        self.session.replay()
    }

    pub fn new(config: GameConfig, seed: u64) -> Self {
        Self::from_core_config(config.to_core_state(), seed).expect("GameCore config must be valid")
    }

    pub fn from_core_config(config: td_core::GameConfigState, seed: u64) -> Result<Self, String> {
        if GameConfig::from_core_state(config.clone()).is_none() {
            return Err("invalid core game config".to_string());
        }
        let session = td_core::CoreSession::new(config, seed);
        Ok(Self {
            session,
            deferred_card_service_selection: None,
        })
    }

    /// Applies one command and returns the authoritative core receipt.
    ///
    /// The receipt is produced by [`td_core::CoreSession`] and includes the
    /// command sequence, completed simulation tick, post-command state hash,
    /// and event metadata. Use this method when low-level simulator callers
    /// need replay or determinism metadata.
    pub fn apply_with_receipt(
        &mut self,
        command: PlayerCommand,
    ) -> Result<CommandReceipt, CommandError> {
        let requested_service_kind =
            if let PlayerCommand::PurchaseShopItem { slot_index } = &command {
                match self.session.flow() {
                    td_core::GameFlowState::Shopping(shop) => {
                        shop.slots
                            .get(*slot_index)
                            .and_then(|slot| match slot.slot {
                                td_core::ShopSlotState::CardService { kind, .. } => {
                                    td_core::CardServiceKind::from_raw(kind)
                                }
                                _ => None,
                            })
                    }
                    _ => None,
                }
            } else {
                None
            };
        let receipt = self.session.apply(command)?;
        if let Some(service_kind) = requested_service_kind {
            self.request_card_service_selection(service_kind);
        }
        if matches!(
            receipt.command,
            PlayerCommand::ConfirmCardServiceSelection { .. }
        ) {
            self.deferred_card_service_selection = None;
        }
        Ok(receipt)
    }

    /// Applies one command and returns the legacy simulator output.
    ///
    /// This compatibility API delegates to [`Self::apply_with_receipt`].
    /// Callers that need command sequence, state hash, or event digest should
    /// use the receipt-returning method instead.
    pub fn apply(&mut self, command: PlayerCommand) -> Result<CommandOutput, CommandError> {
        let receipt = self.apply_with_receipt(command)?;
        Ok(CommandOutput::Accepted {
            events: receipt.events,
        })
    }

    pub fn authoritative_hash(&self) -> String {
        self.session.authoritative_hash()
    }

    #[cfg(test)]
    pub(crate) fn core_state_snapshot(&self) -> td_core::CoreState {
        self.session.raw_state().clone_without_events()
    }

    pub(crate) fn raw_state(&self) -> &td_core::CoreState {
        self.session.raw_state()
    }

    pub(crate) fn observation(
        &self,
        environment_version: u32,
        action_schema_version: u32,
        map_width: usize,
        map_height: usize,
    ) -> td_core::Observation {
        self.session.observation(
            environment_version,
            action_schema_version,
            map_width,
            map_height,
        )
    }

    #[cfg(test)]
    pub(crate) fn restore_core_state_snapshot(&mut self, snapshot: td_core::CoreState) -> bool {
        if let Some(kind) = snapshot.pending_card_service_kind()
            && td_core::CardServiceSelectionState::new_raw(kind).is_none()
        {
            return false;
        }
        if !snapshot.validate_snapshot() {
            return false;
        }
        self.session = td_core::CoreSession::from_state(snapshot)
            .expect("validated snapshot must construct a core session");
        self.deferred_card_service_selection = None;
        if let Some(kind) = self.session.raw_state().pending_card_service_kind_typed() {
            self.deferred_card_service_selection =
                Self::build_deferred_card_service_selection(kind);
        }
        true
    }

    #[cfg(test)]
    pub fn flow(&self) -> td_core::GameFlowState {
        self.session.flow().clone()
    }

    pub fn stage(&self) -> usize {
        self.session.progress().stage
    }

    pub fn hp(&self) -> Health {
        Health::from_raw(self.session.hp_raw())
    }

    pub fn shield(&self) -> Shield {
        Shield::from_raw(self.session.shield_raw())
    }

    pub fn gold(&self) -> usize {
        self.session.progress().gold
    }

    pub fn sim_tick(&self) -> SimTick {
        SimTick::from_ticks(self.session.sim_tick().ticks())
    }

    pub fn config(&self) -> GameConfig {
        GameConfig::from_core_state(self.session.config().clone())
            .expect("GameCore config must be restorable")
    }

    pub fn stage_modifiers(&self) -> td_core::StageModifiersState {
        self.session.raw_state().stage_modifiers().clone()
    }

    pub(crate) fn clear_rate(&self) -> ClearRate {
        ClearRate::from_raw(self.session.clear_rate_raw())
    }

    pub(crate) fn reward_metrics(&self) -> RewardMetricsSnapshot {
        let metrics = self.session.metrics();
        RewardMetricsSnapshot {
            total_gold_earned: metrics.total_gold_earned,
            total_escaped_hp: metrics.total_escaped_hp_raw as f32 / 1_000.0,
            total_player_damage: metrics.total_player_damage_raw as f32 / 1_000.0,
            total_tower_damage: metrics
                .tower_damage_stats
                .iter()
                .map(|stats| stats.total_damage_raw as f32 / 1_000.0)
                .sum(),
            stage_damage: metrics
                .stage_damage
                .iter()
                .map(|(stage, damage)| (*stage, *damage as f32 / 1_000.0))
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn left_dice(&self) -> usize {
        self.session.raw_state().progress().left_dice
    }

    #[cfg(all(feature = "simulator", test))]
    pub(crate) fn deck(&self) -> td_core::DeckState {
        self.session.raw_state().deck().clone()
    }

    #[cfg(all(feature = "simulator", test))]
    pub(crate) fn can_purchase_shop_slot(&self, slot_id: usize) -> bool {
        let td_core::GameFlowState::Shopping(shop) = self.session.flow() else {
            return false;
        };
        let Some(slot_index) = shop.slots.iter().position(|slot| slot.id == slot_id) else {
            return false;
        };
        self.session.raw_state().can_purchase_shop_slot(slot_index)
    }

    pub(crate) fn apply_card_service_selection(
        &mut self,
        selected_card_ids: Vec<Vec<usize>>,
    ) -> Result<CommandOutput, CommandError> {
        let Some(selection) = self.deferred_card_service_selection.clone() else {
            return Err(CommandError::InvalidFlow);
        };
        if selection.service_kind() != self.session.raw_state().pending_card_service_kind_typed() {
            return Err(CommandError::InvalidSelection);
        }
        let selected_card_ids = selected_card_ids
            .into_iter()
            .map(|card_ids| card_ids.into_iter().map(|card_id| card_id as u64).collect())
            .collect();
        self.apply(PlayerCommand::ConfirmCardServiceSelection { selected_card_ids })
    }

    pub(crate) fn take_card_service_selection(
        &mut self,
    ) -> Option<td_core::CardServiceSelectionState> {
        self.deferred_card_service_selection.clone()
    }

    /// Advances exactly one fixed simulation tick and records replay metadata.
    pub fn advance_tick(&mut self) -> RecordedTickOutput {
        let output = self.session.advance_tick();
        RecordedTickOutput {
            sim_tick: SimTick::from_ticks(output.sim_tick.ticks()),
            events: output.events,
            defense_end: output.defense_end.map(|output| DefenseEndOutput {
                perfect_clear: output.perfect_clear,
                gold: output.gold,
                item_count: output.item_count,
                card_count: output.card_count,
                transition: match output.transition {
                    td_core::DefenseEndTransitionState::GameOver => DefenseEndTransition::GameOver,
                    td_core::DefenseEndTransitionState::TreasureSelection => {
                        DefenseEndTransition::TreasureSelection
                    }
                    td_core::DefenseEndTransitionState::StartStage { stage } => {
                        DefenseEndTransition::StartStage { stage }
                    }
                },
            }),
            event_count: output.event_count,
            event_digest: output.event_digest,
        }
    }

    /// Advances one authoritative simulation tick without allocating event output
    /// or computing replay metadata. Use this for explicit bulk statistics;
    /// use [`Self::advance_tick`] for replayable or policy-traced execution.
    pub fn advance_tick_unrecorded(&mut self) -> TickTransition {
        let output = self.session.advance_tick_unrecorded();
        TickTransition {
            sim_tick: SimTick::from_ticks(output.sim_tick.ticks()),
            defense_end: output.defense_end.map(|output| DefenseEndOutput {
                perfect_clear: output.perfect_clear,
                gold: output.gold,
                item_count: output.item_count,
                card_count: output.card_count,
                transition: match output.transition {
                    td_core::DefenseEndTransitionState::GameOver => DefenseEndTransition::GameOver,
                    td_core::DefenseEndTransitionState::TreasureSelection => {
                        DefenseEndTransition::TreasureSelection
                    }
                    td_core::DefenseEndTransitionState::StartStage { stage } => {
                        DefenseEndTransition::StartStage { stage }
                    }
                },
            }),
        }
    }

    /// Compatibility alias for the previous recorded tick API.
    #[deprecated(note = "use advance_tick")]
    pub fn step(&mut self) -> RecordedTickOutput {
        self.advance_tick()
    }

    /// Compatibility alias for the previous unrecorded tick API.
    #[deprecated(note = "use advance_tick_unrecorded")]
    pub fn step_fast(&mut self) -> TickTransition {
        self.advance_tick_unrecorded()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mutate_raw_state(core: &mut GameCore, mutate: impl FnOnce(&mut td_core::CoreSnapshotParts)) {
        let mut parts = core.session.snapshot_parts();
        mutate(&mut parts);
        core.session = td_core::CoreSession::from_state(
            td_core::CoreState::from_snapshot_parts(parts)
                .expect("test snapshot mutation must preserve a valid state"),
        )
        .expect("test snapshot mutation must preserve a valid session");
    }

    fn mutate_session<R>(
        session: &mut td_core::CoreSession,
        mutate: impl FnOnce(&mut td_core::CoreState) -> R,
    ) -> R {
        let mut state = session.raw_state().clone();
        let result = mutate(&mut state);
        *session = td_core::CoreSession::from_state(state)
            .expect("test state mutation must preserve a valid session");
        result
    }

    fn inspect_raw_state<R>(core: &GameCore, inspect: impl FnOnce(&td_core::CoreState) -> R) -> R {
        inspect(core.session.raw_state())
    }

    fn move_first_monster_to_end(core: &mut GameCore) {
        mutate_raw_state(core, |parts| {
            let monster = parts
                .monsters
                .first_mut()
                .expect("first monster should have spawned");
            monster.move_on_route.route_index = monster
                .move_on_route
                .route
                .world_coords
                .len()
                .saturating_sub(1);
            monster.move_on_route.map_coord = *monster
                .move_on_route
                .route
                .world_coords
                .last()
                .expect("monster route should have an endpoint");
        });
    }

    #[test]
    fn core_session_snapshot_round_trips_authoritative_state() {
        let config = GameConfig::default_config();
        let mut session = td_core::CoreSession::new(config.to_core_state(), 7);
        let before_hash = session.authoritative_hash();
        session.advance_tick();
        let snapshot = session
            .snapshot()
            .expect("initial core snapshot should validate");

        let restored =
            td_core::CoreSession::from_snapshot(snapshot).expect("snapshot should restore");

        assert_eq!(restored.authoritative_hash(), session.authoritative_hash());
        assert_ne!(before_hash, restored.authoritative_hash());
        assert_eq!(restored.sim_tick().ticks(), 1);
        let replay = session.replay();
        replay
            .validate()
            .expect("core replay contract should validate");
        let replay_json = serde_json::to_string(&replay).expect("core replay should serialize");
        let decoded: td_core::CoreReplay =
            serde_json::from_str(&replay_json).expect("core replay should deserialize");
        assert_eq!(decoded, replay);
    }

    #[test]
    fn core_receipt_records_sequence_tick_hash_and_command_events_atomically() {
        let config = GameConfig::default_config();
        let mut session = td_core::CoreSession::new(config.to_core_state(), 7);
        assert!(mutate_session(&mut session, |state| state.start_selecting_tower()));

        let receipt = session.record_compatibility_command(PlayerCommand::StartSelectingTower);

        assert_eq!(receipt.sequence, 0);
        assert_eq!(receipt.completed_sim_tick, 0);
        assert_eq!(receipt.command, PlayerCommand::StartSelectingTower);
        assert_eq!(
            receipt.state_hash,
            td_core::authoritative_hash(session.state())
        );
        assert_eq!(session.state().player_commands().len(), 1);
        assert!(session.drain_events().next().is_none());
    }

    #[test]
    fn core_tick_output_contains_sim_tick_events_and_post_tick_hash() {
        let config = GameConfig::default_config();
        let mut session = td_core::CoreSession::new(config.to_core_state(), 7);

        let output = session.advance_tick();

        assert_eq!(output.sim_tick.ticks(), 1);
        assert_eq!(output.state_hash, session.authoritative_hash());
        assert_eq!(session.sim_tick().ticks(), 1);
    }

    #[test]
    fn apply_uses_the_existing_command_validation_and_hash_path() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let initial_hash = core.authoritative_hash();

        assert_eq!(
            core.apply(PlayerCommand::StartDefense),
            Err(CommandError::InvalidFlow)
        );
        assert_eq!(
            core.apply(PlayerCommand::RemoveTower { tower_id: 999 }),
            Err(CommandError::UnknownTower)
        );
        assert_eq!(core.authoritative_hash(), initial_hash);
    }

    #[test]
    fn apply_returns_an_accepted_output_for_a_valid_command() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);

        assert_eq!(
            core.apply(PlayerCommand::StartSelectingTower),
            Ok(CommandOutput::Accepted { events: Vec::new() })
        );
    }

    #[test]
    fn apply_with_receipt_matches_direct_core_session_metadata() {
        let config = GameConfig::default_config();
        let mut game_core = GameCore::new(config.clone(), 0x51E5510);
        let mut session = td_core::CoreSession::new(config.to_core_state(), 0x51E5510);

        for (expected_sequence, command) in [
            (0, PlayerCommand::StartSelectingTower),
            (
                1,
                PlayerCommand::Reroll {
                    selected_slot_indices: Vec::new(),
                },
            ),
        ] {
            let game_core_receipt = game_core
                .apply_with_receipt(command.clone())
                .expect("GameCore command should be accepted");
            let session_receipt = session
                .apply(command)
                .expect("CoreSession command should be accepted");

            assert_eq!(game_core_receipt, session_receipt);
            assert_eq!(game_core_receipt.sequence, expected_sequence);
            assert_eq!(
                game_core_receipt.completed_sim_tick,
                session.sim_tick().ticks()
            );
            assert_eq!(game_core_receipt.state_hash, session.authoritative_hash());
            assert_eq!(
                game_core_receipt.event_count,
                game_core_receipt.events.len() as u64
            );
            assert_eq!(
                game_core_receipt.event_digest,
                td_core::event_digest(&game_core_receipt.events)
            );
            assert_eq!(game_core.authoritative_hash(), session.authoritative_hash());
            assert_eq!(game_core.raw_state(), session.raw_state());
        }

        let adapter_tick = game_core.advance_tick();
        let session_tick = session.advance_tick();
        assert_eq!(adapter_tick.sim_tick.ticks(), session_tick.sim_tick.ticks());
        assert_eq!(adapter_tick.events, session_tick.events);
        assert_eq!(adapter_tick.event_count, session_tick.event_count);
        assert_eq!(adapter_tick.event_digest, session_tick.event_digest);
        assert_eq!(game_core.authoritative_hash(), session.authoritative_hash());
        assert_eq!(game_core.replay(), session.replay());
    }

    #[test]
    fn raw_snapshot_validation_accepts_valid_state_and_rejects_corruption() {
        let core = GameCore::new(GameConfig::default_config(), 7);
        assert!(core.session.validate_snapshot());

        let invalid_route = serde_json::from_value::<td_core::CoreState>({
            let mut value = serde_json::to_value(core.session.raw_state()).expect("state JSON");
            value["route"]["map_coords"] = serde_json::json!([]);
            value
        })
        .expect("route fixture");
        assert!(!invalid_route.validate_snapshot());

        let invalid_pending = core.session.clone();
        let invalid_pending = serde_json::from_value::<td_core::CoreState>({
            let mut value = serde_json::to_value(&invalid_pending).expect("state JSON");
            value["pending_card_service_kind"] = serde_json::json!(99);
            value
        })
        .expect("pending fixture");
        assert!(!invalid_pending.validate_snapshot());
    }

    #[test]
    fn two_core_sessions_have_matching_command_tick_event_and_replay_results() {
        let mut first = GameCore::new(GameConfig::default_config(), 0xC0DE);
        let mut second = GameCore::new(GameConfig::default_config(), 0xC0DE);
        let commands = [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            },
            PlayerCommand::UseInventoryItem { item_index: 0 },
            PlayerCommand::SelectTower {
                selected_slot_indices: Vec::new(),
            },
            PlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            PlayerCommand::StartDefense,
        ];

        assert_eq!(first.core_state_snapshot(), second.core_state_snapshot());
        assert_eq!(first.authoritative_hash(), second.authoritative_hash());

        for command in commands {
            let first_result = first.apply(command.clone());
            let second_result = second.apply(command.clone());
            assert_eq!(
                first_result.as_ref().map(|_| ()).map_err(Clone::clone),
                second_result.as_ref().map(|_| ()).map_err(Clone::clone)
            );
            assert_eq!(
                match first_result {
                    Ok(CommandOutput::Accepted { events }) => events,
                    Err(_) => Vec::new(),
                },
                match second_result {
                    Ok(CommandOutput::Accepted { events }) => events,
                    Err(_) => Vec::new(),
                },
            );
            assert_eq!(first.core_state_snapshot(), second.core_state_snapshot());
            assert_eq!(first.authoritative_hash(), second.authoritative_hash());
        }

        for _ in 0..120 {
            assert_eq!(first.advance_tick(), second.advance_tick());
            assert_eq!(first.core_state_snapshot(), second.core_state_snapshot());
            assert_eq!(first.authoritative_hash(), second.authoritative_hash());
        }

        assert_eq!(first.replay(), second.replay());
    }

    #[test]
    fn game_core_delegates_commands_and_ticks_to_core_session() {
        let config = GameConfig::default_config();
        let mut adapter = GameCore::new(config.clone(), 0x51E5510);
        let mut session = td_core::CoreSession::new(config.to_core_state(), 0x51E5510);

        for command in [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            },
        ] {
            let adapter_result = adapter.apply(command.clone());
            let session_result = session.apply(command);
            assert_eq!(
                adapter_result.as_ref().map(|_| ()).map_err(Clone::clone),
                session_result.as_ref().map(|_| ()).map_err(Clone::clone)
            );
            assert_eq!(adapter.authoritative_hash(), session.authoritative_hash());
            assert_eq!(adapter.raw_state(), session.raw_state());
        }

        for _ in 0..8 {
            let adapter_tick = adapter.advance_tick();
            let session_tick = session.advance_tick();
            assert_eq!(adapter_tick.sim_tick, session_tick.sim_tick);
            assert_eq!(adapter_tick.events, session_tick.events);
            assert_eq!(adapter_tick.event_count, session_tick.event_count);
            assert_eq!(adapter_tick.event_digest, session_tick.event_digest);
            assert_eq!(adapter.authoritative_hash(), session.authoritative_hash());
            assert_eq!(adapter.raw_state(), session.raw_state());
        }
    }

    #[test]
    fn start_selecting_tower_updates_raw_flow_and_replay() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);

        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");

        assert_eq!(core.session.flow(), &td_core::GameFlowState::SelectingTower);
        assert_eq!(core.session.progress().player_command_sequence, 1);
        assert!(matches!(
            core.session.player_commands(),
            [td_core::RecordedPlayerCommand {
                command: PlayerCommand::StartSelectingTower,
                ..
            }]
        ));
    }

    #[test]
    fn select_tower_updates_raw_flow_without_headless_adapter() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");

        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");

        assert!(matches!(
            core.session.flow(),
            td_core::GameFlowState::PlacingTower
        ));
        assert_eq!(core.session.hand().slots.len(), 1);
        assert!(matches!(
            core.session.hand().slots[0].item,
            td_core::HandItemState::Tower(_)
        ));
        assert_eq!(core.session.progress().player_command_sequence, 2);
    }

    #[test]
    fn direct_core_commands_preserve_flow_and_replay_progression() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);

        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        assert!(matches!(core.flow(), td_core::GameFlowState::PlacingTower));
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");

        core.apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        assert!(matches!(core.flow(), td_core::GameFlowState::Defense(_)));
        assert_eq!(core.session.monster_spawn().monster_queue.len(), 5);
        assert_eq!(
            core.session.monster_spawn().next_spawn_tick,
            Some(core.session.sim_tick().ticks())
        );
        assert_eq!(
            core.session.flow(),
            &td_core::GameFlowState::Defense(td_core::DefenseFlowState {
                start_total_hp_raw: 338_285,
                processed_hp_raw: 0,
                took_damage: false,
            })
        );
        assert_eq!(
            core.session.monster_spawn().monster_queue[0].max_hp_raw,
            67_657
        );
        assert_eq!(
            core.session.monster_spawn().monster_queue[0]
                .move_on_route
                .velocity_raw,
            5 * td_core::WORLD_UNITS_PER_TILE
        );
        #[cfg(feature = "simulator")]
        {
            let replay = core.replay();
            assert_eq!(replay.checkpoints.len(), 4);
            assert_eq!(replay.commands.len(), 4);
            assert_eq!(
                replay
                    .checkpoints
                    .iter()
                    .map(|checkpoint| checkpoint.sequence)
                    .collect::<Vec<_>>(),
                vec![0, 1, 2, 3]
            );
            assert!(
                replay
                    .checkpoints
                    .iter()
                    .all(|checkpoint| !checkpoint.state_hash.is_empty())
            );
        }
    }

    #[test]
    fn step_spawns_from_raw_queue_before_raw_combat() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");
        let start_output = core
            .apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        assert!(matches!(
            start_output,
            CommandOutput::Accepted { ref events }
                if events.iter().any(|event| matches!(
                    event,
                    CoreEvent::DefenseStarted { stage: 1 }
                ))
        ));

        let output = core.advance_tick();

        assert_eq!(output.sim_tick, SimTick::from_ticks(1));
        assert_eq!(core.session.monsters().len(), 1);
        assert_eq!(core.session.monster_spawn().monster_queue.len(), 4);
        assert_eq!(core.session.monster_spawn().next_spawn_tick, Some(134));
        assert!(output.events.iter().any(|event| matches!(
            event,
            CoreEvent::MonsterSpawned {
                monster_id: _,
                monster_kind: 0,
                position: _
            }
        )));
    }

    #[test]
    fn step_resolves_raw_timed_attack_before_raw_tower_shooting() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");
        core.apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        core.advance_tick();

        let monster = core
            .session
            .monsters()
            .first()
            .expect("first monster should have spawned");
        let monster_id = monster.id;
        let hp_before = monster.hp_raw;
        mutate_raw_state(&mut core, |parts| {
            parts.next_entity_id = td_core::EntityIdAllocator::from_next_id(10_000);
            parts.towers.clear();
            parts.in_flight_attacks.push(td_core::InFlightAttackState {
                id: 9_999,
                damage_raw: 1_000,
                source_tower: None,
                kind: td_core::InFlightAttackKindState::Timed(td_core::TimedAttackState {
                    target_monster_id: monster_id,
                    execute_at: 2,
                }),
                on_hit_splashes: vec![],
            });
        });

        core.advance_tick();

        let monster = core
            .session
            .monsters()
            .iter()
            .find(|monster| monster.id == monster_id)
            .expect("monster should survive the test hit");
        assert_eq!(monster.hp_raw, hp_before - 1_000);
        assert!(
            core.session
                .in_flight_attacks()
                .iter()
                .all(|attack| attack.id != 9_999)
        );
    }

    #[test]
    fn raw_damage_updates_defense_progress_by_applied_hp() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");
        core.apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        core.advance_tick();

        let monster = core
            .session
            .monsters()
            .first()
            .expect("first monster should have spawned");
        let monster_id = monster.id;
        let hp_before = monster.hp_raw;
        let target_xy = [
            monster.move_on_route.map_coord[0],
            monster.move_on_route.map_coord[1],
        ];

        mutate_session(&mut core.session, |state| {
            state.apply_damage_hits(
                vec![td_core::DamageHit {
                    target_index: 0,
                    damage_raw: 1_000,
                    at_xy: target_xy,
                    source_index: 0,
                    splashes: Vec::new(),
                }],
                &[None],
            );
        });
        assert_eq!(
            core.session.flow(),
            &td_core::GameFlowState::Defense(td_core::DefenseFlowState {
                start_total_hp_raw: 338_285,
                processed_hp_raw: 1_000,
                took_damage: false,
            })
        );

        let deaths = mutate_session(&mut core.session, |state| {
            state.apply_damage_hits(
                vec![td_core::DamageHit {
                    target_index: 0,
                    damage_raw: hp_before,
                    at_xy: target_xy,
                    source_index: 0,
                    splashes: Vec::new(),
                }],
                &[None],
            )
        });

        assert_eq!(deaths.len(), 1);
        assert_eq!(deaths[0].0.monster.id, monster_id);
        assert_eq!(core.session.monsters().len(), 0);
        assert_eq!(
            core.session.flow(),
            &td_core::GameFlowState::Defense(td_core::DefenseFlowState {
                start_total_hp_raw: 338_285,
                processed_hp_raw: hp_before,
                took_damage: false,
            })
        );
    }

    #[test]
    fn step_resolves_base_damage_from_raw_monster_escape() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");
        core.apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        core.advance_tick();

        move_first_monster_to_end(&mut core);
        mutate_raw_state(&mut core, |parts| {
            parts.hp_raw = 60_000;
            parts.shield_raw = 0;
        });
        core.advance_tick();

        assert_eq!(core.session.hp_raw(), 59_000);
        assert_eq!(core.session.metrics().total_escaped_hp_raw, 67_657);
        assert_eq!(core.session.metrics().total_player_damage_raw, 1_000);
        assert_eq!(core.session.monsters().len(), 0);
        assert!(matches!(core.flow(), td_core::GameFlowState::Defense(_)));
    }

    #[test]
    fn raw_base_damage_transitions_to_defeat_when_hp_reaches_zero() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");
        core.apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        core.advance_tick();

        move_first_monster_to_end(&mut core);
        mutate_raw_state(&mut core, |parts| parts.hp_raw = 1);

        let output = core.advance_tick();

        assert!(matches!(core.flow(), td_core::GameFlowState::Result { .. }));
        assert!(
            output
                .events
                .iter()
                .any(|event| matches!(event, CoreEvent::GameFinished { victory: false }))
        );
        assert_eq!(
            output
                .events
                .iter()
                .filter(|event| matches!(event, CoreEvent::GameFinished { .. }))
                .count(),
            1
        );
    }

    #[test]
    fn raw_base_damage_syncs_escape_state_when_damage_is_reduced_to_zero() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");
        core.apply(PlayerCommand::StartDefense)
            .expect("start defense should be accepted");
        core.advance_tick();

        move_first_monster_to_end(&mut core);
        mutate_raw_state(&mut core, |parts| {
            parts.stage_modifiers.damage_reduction_multipliers_raw = vec![0];
        });

        core.advance_tick();

        assert_eq!(core.session.monsters().len(), 0);
        assert_eq!(core.session.metrics().total_escaped_hp_raw, 67_657);
        assert_eq!(core.session.hp_raw(), 60_000);
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn reroll_runs_core_mutation_and_records_replay_progression() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);

        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        mutate_raw_state(&mut core, |state| {
            state.upgrades.upgrades.push(td_core::UpgradeEntryState {
                id: 0,
                kind: 34,
                scalar_values: Vec::new(),
                ratio_values_raw: Vec::new(),
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            });
        });
        core.session
            .edit_snapshot(|parts| {
                parts.progress.rerolled_count = 3;
                parts.progress.left_dice = 1;
            })
            .expect("test reroll edit must preserve a valid snapshot");
        let before_hash = core.authoritative_hash();
        core.apply(PlayerCommand::Reroll {
            selected_slot_indices: vec![0],
        })
        .expect("reroll should be accepted");

        assert_ne!(core.authoritative_hash(), before_hash);
        assert_eq!(core.left_dice(), 1);
        assert_eq!(core.replay().checkpoints.len(), 2);
    }

    #[test]
    fn reroll_with_a_die_still_applies_health_cost_and_records_damage() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        mutate_raw_state(&mut core, |state| {
            state.stage_modifiers.reroll_health_cost = 3;
            state.hp_raw = 100_000;
            state.progress.left_dice = 1;
        });

        core.apply(PlayerCommand::Reroll {
            selected_slot_indices: vec![0],
        })
        .expect("reroll should be accepted");

        assert_eq!(core.hp().raw(), 97_000);
        assert_eq!(core.shield().raw(), 0);
        assert_eq!(
            inspect_raw_state(&core, |state| state.progress().left_dice),
            0
        );
        assert_eq!(
            inspect_raw_state(&core, |state| state.metrics().total_player_damage_raw),
            3_000
        );
    }

    #[test]
    fn reroll_health_cost_is_absorbed_by_shield_before_hp() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        mutate_raw_state(&mut core, |state| {
            state.stage_modifiers.reroll_health_cost = 3;
            state.hp_raw = 100_000;
            state.shield_raw = 2_000;
            state.progress.left_dice = 1;
        });

        core.apply(PlayerCommand::Reroll {
            selected_slot_indices: vec![0],
        })
        .expect("reroll should be accepted");

        assert_eq!(core.hp().raw(), 99_000);
        assert_eq!(core.shield().raw(), 0);
        assert_eq!(
            inspect_raw_state(&core, |state| state.metrics().total_player_damage_raw),
            1_000
        );
    }

    #[test]
    fn reroll_without_a_die_is_rejected_when_health_would_reach_one() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        mutate_raw_state(&mut core, |state| {
            state.stage_modifiers.reroll_health_cost = 3;
            state.hp_raw = 4_000;
            state.progress.left_dice = 0;
        });
        let before_hash = core.authoritative_hash();

        assert_eq!(
            core.apply(PlayerCommand::Reroll {
                selected_slot_indices: vec![0],
            }),
            Err(CommandError::InvalidSelection)
        );
        assert_eq!(core.authoritative_hash(), before_hash);
    }

    #[test]
    fn command_error_round_trips_through_json() {
        let error = CommandError::InvalidSelection;

        let json = serde_json::to_string(&error).expect("command errors are serializable");
        let decoded: CommandError =
            serde_json::from_str(&json).expect("command errors are deserializable");

        assert_eq!(decoded, error);
    }

    #[test]
    fn core_events_round_trip_through_json() {
        let events = vec![
            CoreEvent::DamageApplied {
                target_id: 3,
                amount: 12,
                position: [4, 5],
            },
            CoreEvent::MonsterDefeated {
                monster_id: 3,
                position: [4, 5],
                monster_kind: 0,
                reward: 0,
                rotation_milliradians: 0,
            },
            CoreEvent::TowerAttack {
                tower_id: 9,
                target_id: 3,
                attack_kind: 0,
                attack_ids: vec![12],
                projectile_attack_ids: Vec::new(),
            },
            CoreEvent::CardServiceSelectionRequested {
                service_kind: "eraser".to_string(),
                step_counts: vec![1],
            },
        ];

        let json = serde_json::to_string(&events).expect("core events are serializable");
        let decoded: Vec<CoreEvent> =
            serde_json::from_str(&json).expect("core events are deserializable");

        assert_eq!(decoded, events);
    }

    #[test]
    fn raw_event_queue_is_not_part_of_authoritative_snapshot_or_hash() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let before_hash = core.authoritative_hash();
        mutate_session(&mut core.session, |state| {
            state.extend_events([CoreEvent::DamageApplied {
                target_id: 3,
                amount: 12,
                position: [4, 5],
            }]);
        });
        assert_eq!(core.authoritative_hash(), before_hash);

        let mut snapshot = core.core_state_snapshot();
        assert!(snapshot.drain_events().next().is_none());
    }

    #[test]
    fn raw_render_snapshot_is_authoritative_and_excludes_presentation_fields() {
        let core = GameCore::new(GameConfig::default_config(), 7);
        let snapshot = core.raw_state().render_snapshot();

        assert_eq!(snapshot.sim_tick, core.raw_state().sim_tick());
        assert!(snapshot.monsters.is_empty());
        assert!(snapshot.spatial_attacks.is_empty());
        assert!(snapshot.towers.is_empty());
    }

    #[test]
    fn raw_render_snapshot_exposes_tower_range_metadata() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: Vec::new(),
        })
        .expect("tower selection should succeed");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should succeed");

        mutate_raw_state(&mut core, |parts| {
            parts.towers[0].attack_range_radius_raw = 12_345;
            parts.towers[0]
                .on_attack_splashes
                .push(td_core::DamageSplash {
                    radius_raw: 67_890,
                    damage_pct_raw: 300_000,
                });
        });

        let tower = core
            .raw_state()
            .render_snapshot()
            .towers
            .into_iter()
            .next()
            .expect("placed tower should be rendered");
        assert_eq!(tower.attack_range_radius_raw, 12_345);
        assert_eq!(tower.on_attack_splash_radii_raw, vec![67_890]);
    }

    #[test]
    fn step_advances_exactly_one_sim_tick() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let initial_tick = core.sim_tick();

        let output = core.advance_tick();

        assert_eq!(output.sim_tick, initial_tick + td_core::SimTickSpan::ONE);
        assert_eq!(core.sim_tick(), output.sim_tick);
        assert_eq!(output.defense_end, None);
    }

    #[test]
    fn core_state_snapshot_tracks_authoritative_progress() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.advance_tick();

        let snapshot = core.core_state_snapshot();

        assert_eq!(snapshot.progress().player_command_sequence, 1);
        assert_eq!(snapshot.progress().stage, core.stage());
        assert_eq!(snapshot.progress().gold, core.gold());
        assert_eq!(snapshot.sim_tick().ticks(), core.sim_tick().ticks());
        assert_eq!(snapshot.hp_raw(), core.hp().raw());
        assert_eq!(snapshot.shield_raw(), core.shield().raw());
    }

    #[test]
    fn core_state_snapshot_round_trips_through_json() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![0],
        })
        .expect("tower selection should be accepted");
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should be accepted");

        let snapshot = core.core_state_snapshot();
        let encoded = serde_json::to_string(&snapshot).expect("core state should serialize");
        let decoded: td_core::CoreState =
            serde_json::from_str(&encoded).expect("core state should deserialize");
        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn core_state_snapshot_restores_authoritative_hash() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");
        let snapshot = core.core_state_snapshot();
        let expected_hash = core.authoritative_hash();

        core.advance_tick();
        assert_ne!(core.authoritative_hash(), expected_hash);
        assert!(core.restore_core_state_snapshot(snapshot));

        assert_eq!(core.authoritative_hash(), expected_hash);
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn core_state_snapshot_restores_pending_card_service_selection() {
        let mut original = GameCore::new(GameConfig::default_config(), 7);
        let slot_index = original.add_card_service_shop_slot_for_test();
        original
            .apply(PlayerCommand::PurchaseShopItem { slot_index })
            .expect("card service purchase should be accepted");
        let snapshot = original.core_state_snapshot();
        let card_id = original.deck().all_cards[0].id;

        let mut restored = GameCore::new(GameConfig::default_config(), 7);
        assert!(restored.restore_core_state_snapshot(snapshot));
        assert!(restored.take_card_service_selection().is_some());

        restored
            .apply_card_service_selection(vec![vec![card_id]])
            .expect("restored card service selection should be accepted");
        original
            .apply_card_service_selection(vec![vec![card_id]])
            .expect("original card service selection should be accepted");

        assert_eq!(restored.authoritative_hash(), original.authoritative_hash());
    }

    #[test]
    fn step_is_deterministic_for_the_same_seed() {
        let mut left = GameCore::new(GameConfig::default_config(), 7);
        let mut right = GameCore::new(GameConfig::default_config(), 7);

        assert_eq!(left.advance_tick(), right.advance_tick());
        assert_eq!(left.authoritative_hash(), right.authoritative_hash());
    }

    #[test]
    fn fast_step_matches_full_step_and_does_not_add_replay_metadata() {
        let config = GameConfig::default_config();
        let mut full = GameCore::new(config.clone(), 7);
        let mut fast = GameCore::new(config, 7);

        let command = PlayerCommand::StartSelectingTower;
        assert_eq!(full.apply(command.clone()), fast.apply(command));
        let checkpoint_count = fast.replay().checkpoints.len();

        for _ in 0..8 {
            mutate_session(&mut full.session, |state| {
                state.extend_events([CoreEvent::DefenseStarted { stage: 1 }]);
            });
            mutate_session(&mut fast.session, |state| {
                state.extend_events([CoreEvent::DefenseStarted { stage: 1 }]);
            });
            let full_output = full.advance_tick();
            let fast_output = fast.advance_tick_unrecorded();

            assert_eq!(full_output.sim_tick, fast_output.sim_tick);
            assert_eq!(full_output.defense_end, fast_output.defense_end);
            assert_eq!(full_output.events.len(), 1);
            assert_eq!(full_output.event_count, 1);
            assert_eq!(
                full_output.event_digest,
                td_core::event_digest(&full_output.events)
            );
            assert!(fast.session.drain_events().next().is_none());
            assert_eq!(full.authoritative_hash(), fast.authoritative_hash());
            assert_eq!(full.core_state_snapshot(), fast.core_state_snapshot());
            assert_eq!(fast.replay().checkpoints.len(), checkpoint_count);
        }
    }

    #[test]
    #[ignore = "manual release profiling diagnostic"]
    fn profile_fast_tick_event_allocation_cost() {
        const TICKS: usize = 512;

        fn active_core() -> GameCore {
            let mut core = GameCore::new(GameConfig::default_config(), 7);
            core.apply(PlayerCommand::StartSelectingTower)
                .expect("tower selection should start");
            core.apply(PlayerCommand::SelectTower {
                selected_slot_indices: vec![],
            })
            .expect("tower selection should succeed");
            core.apply(PlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            })
            .expect("tower placement should succeed");
            core.apply(PlayerCommand::StartDefense)
                .expect("defense should start");
            core
        }

        let mut full = active_core();
        let mut fast = active_core();
        let full_started = std::time::Instant::now();
        let mut full_event_count = 0u64;
        for _ in 0..TICKS {
            let output = full.advance_tick();
            full_event_count += output.event_count;
            std::hint::black_box(output);
        }
        let full_elapsed = full_started.elapsed();

        let fast_started = std::time::Instant::now();
        for _ in 0..TICKS {
            let output = fast.advance_tick_unrecorded();
            std::hint::black_box(output);
        }
        let fast_elapsed = fast_started.elapsed();

        assert_eq!(full.authoritative_hash(), fast.authoritative_hash());
        println!(
            "fast_tick_event_profile ticks={TICKS} full_events={full_event_count} \
             full_ns_per_tick={:.0} fast_ns_per_tick={:.0} fast_pct_of_full={:.1}",
            full_elapsed.as_nanos() as f64 / TICKS as f64,
            fast_elapsed.as_nanos() as f64 / TICKS as f64,
            fast_elapsed.as_secs_f64() / full_elapsed.as_secs_f64() * 100.0
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn shop_legality_reads_core_state_without_ui_status() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let slot_index = core.add_card_service_shop_slot_for_test();
        let slot_id = core.shop_slot_id_for_test(slot_index);

        assert!(core.can_purchase_shop_slot(slot_id));
        let output = core
            .apply(PlayerCommand::PurchaseShopItem { slot_index })
            .expect("card service purchase should be accepted");
        let selection = core.take_card_service_selection();
        assert!(selection.is_some());
        assert!(matches!(
        output,
        CommandOutput::Accepted { ref events }
            if events.iter().any(|event| matches!(
                event,
                CoreEvent::CardServiceSelectionRequested {
                    service_kind,
                    step_counts,
                } if service_kind == "eraser" && step_counts == &vec![1]
            ))
            ));
        assert!(!core.can_purchase_shop_slot(slot_id));
        assert_eq!(
            core.core_state_snapshot().pending_card_service_kind(),
            Some(7)
        );
        let card_id = core.deck().all_cards[0].id;
        core.apply_card_service_selection(vec![vec![card_id]])
            .expect("card service selection should be accepted");
        assert_eq!(core.core_state_snapshot().pending_card_service_kind(), None);
        assert!(matches!(
            core.replay().commands.last(),
            Some(command)
                if matches!(
                    &command.command,
                    PlayerCommand::ConfirmCardServiceSelection {
                        selected_card_ids
                    } if selected_card_ids == &vec![vec![card_id as u64]]
                )
        ));
    }

    #[test]
    fn raw_shop_purchase_validates_cost_and_free_shop_modifier() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let (slot_index, cost) = match core.session.flow() {
            td_core::GameFlowState::Shopping(shop) => shop
                .slots
                .iter()
                .enumerate()
                .find_map(|(index, slot)| {
                    (!matches!(&slot.slot, td_core::ShopSlotState::CardService { .. }))
                        .then(|| {
                            let cost = match &slot.slot {
                                td_core::ShopSlotState::Item { cost, .. }
                                | td_core::ShopSlotState::Upgrade { cost, .. } => *cost,
                                td_core::ShopSlotState::CardService { .. } => unreachable!(),
                            };
                            (cost > 0).then_some((index, cost))
                        })
                        .flatten()
                })
                .expect("initial shop should contain an item or upgrade"),
            _ => panic!("expected initial shopping flow"),
        };

        core.session
            .edit_snapshot(|parts| parts.progress.gold = cost.saturating_sub(1))
            .expect("test gold edit must preserve a valid snapshot");
        assert!(!core.session.can_purchase_shop_slot(slot_index));
        assert_eq!(
            mutate_session(&mut core.session, |state| {
                state.purchase_shop_item(slot_index)
            }),
            Err(CommandError::Rejected)
        );

        core.session
            .edit_snapshot(|parts| {
                parts.stage_modifiers.free_shop_this_stage = true;
                parts.progress.gold = 0;
            })
            .expect("test shop edit must preserve a valid snapshot");
        assert!(core.session.can_purchase_shop_slot(slot_index));
        let purchase = mutate_session(&mut core.session, |state| {
            state
                .purchase_shop_item(slot_index)
                .expect("free shop purchase should be accepted")
        });
        assert_eq!(purchase.cost, 0);
        assert_eq!(core.session.progress().gold, 0);
        assert_eq!(core.session.metrics().total_gold_spent, 0);
    }

    #[test]
    fn raw_shop_purchase_checks_card_service_card_requirements() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let mut slot_index = None;
        core.session
            .edit_snapshot(|parts| {
                let td_core::GameFlowState::Shopping(shop) = &mut parts.flow else {
                    return;
                };
                let slot_id = shop.slots.iter().map(|slot| slot.id).max().unwrap_or(0) + 1;
                shop.slots.push(td_core::ShopSlotDataState {
                    id: slot_id,
                    slot: td_core::ShopSlotState::CardService { kind: 8, cost: 0 },
                    purchased: false,
                });
                slot_index = Some(shop.slots.len() - 1);
                for card in &mut parts.deck.all_cards {
                    card.engraving = Some(0);
                }
            })
            .expect("test card service edit must preserve a valid snapshot");
        let slot_index = slot_index.expect("expected initial shopping flow");
        assert!(!core.session.can_purchase_shop_slot(slot_index));

        core.session
            .edit_snapshot(|parts| parts.deck.all_cards[0].engraving = None)
            .expect("test card edit must preserve a valid snapshot");
        assert!(core.session.can_purchase_shop_slot(slot_index));
        mutate_session(&mut core.session, |state| {
            state
                .purchase_shop_item(slot_index)
                .expect("card service purchase should be accepted");
        });
    }

    #[test]
    fn shop_purchase_runs_item_and_spent_upgrades_in_core() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.session
            .edit_snapshot(|parts| {
                parts.progress.gold = 200;
                parts.progress.left_dice = 0;
            })
            .expect("test shop edit must preserve a valid snapshot");
        mutate_raw_state(&mut core, |state| {
            state.upgrades.upgrades.extend([
                td_core::UpgradeEntryState {
                    id: 0,
                    kind: 21,
                    scalar_values: Vec::new(),
                    ratio_values_raw: Vec::new(),
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                },
                td_core::UpgradeEntryState {
                    id: 1,
                    kind: 12,
                    scalar_values: vec![0],
                    ratio_values_raw: Vec::new(),
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                },
            ]);
        });

        let slot_index = inspect_raw_state(&core, |state| {
            let td_core::GameFlowState::Shopping(shop) = state.flow() else {
                panic!("expected initial shopping flow");
            };
            shop.slots.len()
        });
        mutate_raw_state(&mut core, |state| {
            let td_core::GameFlowState::Shopping(shop) = &mut state.flow else {
                panic!("expected initial shopping flow");
            };
            let slot_id = shop.slots.iter().map(|slot| slot.id).max().unwrap_or(0) + 1;
            shop.slots.push(td_core::ShopSlotDataState {
                id: slot_id,
                slot: td_core::ShopSlotState::Item {
                    item: td_core::ItemEntryState {
                        id: 0,
                        kind: 7,
                        scalar_values: vec![1],
                        signed_values: Vec::new(),
                    },
                    cost: 100,
                },
                purchased: false,
            });
        });

        core.apply(PlayerCommand::PurchaseShopItem { slot_index })
            .expect("item purchase should be accepted");

        assert_eq!(core.left_dice(), 1);
        assert_eq!(core.gold(), 100);
        assert!(
            core.session
                .items()
                .iter()
                .any(|item| item.kind == 7 && item.scalar_values == vec![1])
        );
        assert!(core.session.items().iter().any(|item| item.kind == 7));
        assert!(core.session.upgrades().upgrades.iter().any(|upgrade| {
            upgrade.upgrade_kind() == Ok(td_core::UpgradeKind::Crock)
                && upgrade.scalar_values == vec![1]
        }));
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn inventory_item_use_applies_effect_in_core() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("start selecting tower should be accepted");

        let item_count = core.session.items().len();
        let initial_left_dice = core.left_dice();
        core.apply(PlayerCommand::UseInventoryItem { item_index: 0 })
            .expect("item use should be accepted");

        assert_eq!(core.session.items().len(), item_count - 1);
        assert_eq!(core.left_dice(), initial_left_dice + 1);
        assert!(core.session.progress().item_used);
    }

    #[test]
    fn raw_inventory_food_use_updates_health_and_shield() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let max_hp = core.session.max_hp_raw();
        core.session
            .edit_snapshot(|parts| {
                parts.hp_raw = max_hp.saturating_sub(10_000);
                parts.items[0] = td_core::ItemEntryState {
                    id: 1,
                    kind: 0,
                    scalar_values: Vec::new(),
                    signed_values: vec![7_000, 5_000],
                };
            })
            .expect("test food edit must preserve a valid snapshot");

        core.apply(PlayerCommand::UseInventoryItem { item_index: 0 })
            .expect("food item use should be accepted");

        assert_eq!(core.session.hp_raw(), max_hp.saturating_sub(3_000));
        assert_eq!(core.session.shield_raw(), 5_000);
        assert_eq!(core.session.items().len(), 2);
    }

    #[test]
    fn raw_inventory_item_use_rejects_invalid_flow_and_disabled_stage() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.session
            .edit_snapshot(|parts| {
                parts.flow = td_core::GameFlowState::Defense(td_core::DefenseFlowState {
                    start_total_hp_raw: 1,
                    processed_hp_raw: 0,
                    took_damage: false,
                });
            })
            .expect("test flow edit must preserve a valid snapshot");
        assert_eq!(
            core.apply(PlayerCommand::UseInventoryItem { item_index: 0 }),
            Err(CommandError::Rejected)
        );

        core.session
            .edit_snapshot(|parts| {
                parts.items[0] = td_core::ItemEntryState {
                    id: 1,
                    kind: 0,
                    scalar_values: Vec::new(),
                    signed_values: vec![1_000, 1_000],
                };
                parts.stage_modifiers.disable_item_use = true;
            })
            .expect("test item edit must preserve a valid snapshot");
        assert_eq!(
            core.apply(PlayerCommand::UseInventoryItem { item_index: 0 }),
            Err(CommandError::Rejected)
        );
        assert_eq!(core.session.items().len(), 3);
    }

    #[test]
    fn raw_rubber_cone_item_use_queues_tower_cards() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.session
            .edit_snapshot(|parts| {
                parts.items[0] = td_core::ItemEntryState {
                    id: 1,
                    kind: 9,
                    scalar_values: vec![2],
                    signed_values: Vec::new(),
                };
            })
            .expect("test item edit must preserve a valid snapshot");

        core.apply(PlayerCommand::UseInventoryItem { item_index: 0 })
            .expect("rubber cone item use should be accepted");

        assert_eq!(core.session.stage_modifiers().extra_tower_cards.len(), 2);
        assert!(
            core.session
                .stage_modifiers()
                .extra_tower_cards
                .iter()
                .all(|card| card.kind == 0 && card.suit.is_none() && card.rank.is_none())
        );
    }

    #[test]
    fn treasure_selection_acquires_upgrade_in_core() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        mutate_session(&mut core.session, |state| {
            state.start_treasure_selection();
        });
        let initial_upgrade_count =
            inspect_raw_state(&core, |state| state.upgrades().upgrades.len());

        core.apply(PlayerCommand::SelectTreasure { option_index: 0 })
            .expect("treasure selection should be accepted");

        assert!(matches!(core.flow(), td_core::GameFlowState::Shopping(_)));
        assert_eq!(
            inspect_raw_state(&core, |state| state.upgrades().upgrades.len()),
            initial_upgrade_count + 1
        );
    }

    #[test]
    fn raw_upgrade_acquisition_applies_health_recovery_and_max_health() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let initial_max_hp = core.session.max_hp_raw();
        core.session
            .edit_snapshot(|parts| parts.hp_raw = initial_max_hp.saturating_sub(10_000))
            .expect("test HP edit must preserve a valid snapshot");

        let acquire = mutate_session(&mut core.session, |state| {
            state
                .acquire_upgrade(td_core::UpgradeEntryState {
                    id: 0,
                    kind: 2,
                    scalar_values: Vec::new(),
                    ratio_values_raw: Vec::new(),
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                })
                .expect("test upgrade kind must be valid")
        });
        mutate_session(&mut core.session, |state| {
            state.apply_upgrade_recovery(acquire.recovery);
        });

        assert_eq!(core.session.max_hp_raw(), initial_max_hp + 6_000);
        assert_eq!(core.session.hp_raw(), initial_max_hp + 6_000);
        assert_eq!(core.session.upgrades().upgrades.len(), 1);
    }

    #[test]
    fn raw_upgrade_acquisition_updates_existing_shop_costs() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        let initial_costs = match core.session.flow() {
            td_core::GameFlowState::Shopping(shop) => shop
                .slots
                .iter()
                .map(|slot| match &slot.slot {
                    td_core::ShopSlotState::Item { cost, .. }
                    | td_core::ShopSlotState::Upgrade { cost, .. }
                    | td_core::ShopSlotState::CardService { cost, .. } => *cost,
                })
                .collect::<Vec<_>>(),
            _ => panic!("expected initial shopping flow"),
        };

        mutate_session(&mut core.session, |state| {
            state
                .acquire_upgrade(td_core::UpgradeEntryState {
                    id: 0,
                    kind: 6,
                    scalar_values: vec![5],
                    ratio_values_raw: Vec::new(),
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                })
                .expect("test upgrade kind must be valid");
        });

        let updated_costs = match core.session.flow() {
            td_core::GameFlowState::Shopping(shop) => shop
                .slots
                .iter()
                .map(|slot| match &slot.slot {
                    td_core::ShopSlotState::Item { cost, .. }
                    | td_core::ShopSlotState::Upgrade { cost, .. }
                    | td_core::ShopSlotState::CardService { cost, .. } => *cost,
                })
                .collect::<Vec<_>>(),
            _ => panic!("expected initial shopping flow"),
        };
        assert_eq!(updated_costs.len(), initial_costs.len());
        assert!(
            updated_costs
                .iter()
                .zip(initial_costs)
                .all(|(updated, initial)| *updated == initial.saturating_sub(5))
        );
    }

    #[test]
    fn tower_placement_camera_reward_runs_in_core() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        mutate_raw_state(&mut core, |state| {
            state.upgrades.upgrades.push(td_core::UpgradeEntryState {
                id: 0,
                kind: 29,
                scalar_values: Vec::new(),
                ratio_values_raw: Vec::new(),
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            });
        });
        let initial_gold = core.gold();

        mutate_session(&mut core.session, |state| {
            state.trigger_tower_placed_upgrades(
                1,
                true,
                &td_core::TowerTemplateState {
                    kind: 1,
                    rerolled_count: 0,
                    shoot_interval: 1,
                    default_attack_range_radius_raw: 1,
                    default_damage_raw: 1,
                    suit: Some(0),
                    rank: Some(10),
                    skill_templates: Vec::new(),
                    default_status_effects: Vec::new(),
                    used_cards: Vec::new(),
                },
            );
        });

        assert_eq!(core.gold(), initial_gold + 50);
    }

    #[test]
    fn tower_removal_demolition_hammer_reward_runs_in_core() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.session
            .edit_snapshot(|parts| parts.progress.left_dice = 0)
            .expect("test dice edit must preserve a valid snapshot");
        mutate_raw_state(&mut core, |state| {
            state.upgrades.upgrades.push(td_core::UpgradeEntryState {
                id: 0,
                kind: 17,
                scalar_values: Vec::new(),
                ratio_values_raw: Vec::new(),
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            });
        });

        mutate_session(&mut core.session, |state| {
            state.trigger_tower_removed_upgrades(3);
        });

        assert_eq!(core.left_dice(), 3);
    }

    #[test]
    fn mirror_tower_placement_duplication_runs_in_core() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        mutate_raw_state(&mut core, |state| {
            state.upgrades.upgrades.push(td_core::UpgradeEntryState {
                id: 0,
                kind: 23,
                scalar_values: Vec::new(),
                ratio_values_raw: Vec::new(),
                bool_values: vec![true],
                optional_ids: Vec::new(),
            });
        });
        let template = td_core::TowerTemplateState {
            kind: 1,
            rerolled_count: 0,
            shoot_interval: 1,
            default_attack_range_radius_raw: 1,
            default_damage_raw: 1,
            suit: Some(0),
            rank: Some(10),
            skill_templates: Vec::new(),
            default_status_effects: Vec::new(),
            used_cards: Vec::new(),
        };
        let initial_hand_len = inspect_raw_state(&core, |state| state.hand().slots.len());

        mutate_session(&mut core.session, |state| {
            state.trigger_tower_placed_upgrades(1, false, &template);
        });

        assert_eq!(
            inspect_raw_state(&core, |state| state.hand().slots.len()),
            initial_hand_len + 1
        );
        assert_eq!(core.session.upgrades().upgrades[0].bool_values, vec![false]);
    }

    #[test]
    fn select_and_place_tower_use_raw_core_without_legacy_adapter() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![],
        })
        .expect("tower selection should succeed");
        assert!(inspect_raw_state(&core, |state| {
            matches!(state.flow(), td_core::GameFlowState::PlacingTower)
        }));
        let tower_count = inspect_raw_state(&core, |state| state.towers().len());
        core.apply(PlayerCommand::PlaceTower {
            hand_slot_index: 0,
            left: 0,
            top: 0,
        })
        .expect("tower placement should succeed");
        assert_eq!(
            inspect_raw_state(&core, |state| state.towers().len()),
            tower_count + 1
        );
        assert_eq!(
            inspect_raw_state(&core, |state| state.towers()[0].left_top),
            [0, 0]
        );
    }

    #[test]
    fn raw_tower_placement_rejects_travel_point_and_out_of_bounds() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![],
        })
        .expect("tower selection should succeed");
        let hash = core.authoritative_hash();
        assert_eq!(
            core.apply(PlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 5,
                top: 0,
            }),
            Err(CommandError::InvalidPlacement)
        );
        assert_eq!(core.authoritative_hash(), hash);
    }

    #[test]
    fn raw_select_tower_consumes_extra_tower_cards_into_placement_hand() {
        let mut core = GameCore::new(GameConfig::default_config(), 7);
        mutate_raw_state(&mut core, |state| {
            state
                .stage_modifiers
                .extra_tower_cards
                .push(td_core::StageModifierTowerCardState {
                    kind: 1,
                    suit: Some(0),
                    rank: Some(12),
                });
        });
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("tower selection should start");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![],
        })
        .expect("tower selection should succeed");

        assert_eq!(
            inspect_raw_state(&core, |state| state.hand().slots.len()),
            2
        );
        assert!(inspect_raw_state(&core, |state| {
            state.stage_modifiers().extra_tower_cards.is_empty()
        }));
    }
}
