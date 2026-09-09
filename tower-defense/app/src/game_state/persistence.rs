#![allow(dead_code)]

use crate::game_state::GameState;
use crate::game_state::presentation_metadata::PresentationMetadataStore;
use crate::game_state::raw_core::HeadedRawCoreState;
use namui::*;
#[cfg(test)]
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

pub(crate) const CURRENT_SCHEMA_VERSION: u16 = 5;
const STORAGE_KEY: &str = "tower-defense-game-state";
const STORAGE_MAGIC: &[u8; 4] = b"TDGS";

#[derive(Clone, State)]
pub(crate) struct PersistedGameState {
    pub(crate) schema_version: u16,
    pub(crate) raw_core: HeadedRawCoreState,
    pub(crate) presentation_metadata: PresentationMetadataStore,
    pub(crate) presentation_hand: crate::hand::Hand<crate::hand::HandItem>,
    pub(crate) presentation_flow: crate::game_state::flow::GameFlow,
    pub(crate) locale: crate::l10n::Locale,
}

pub(crate) enum LoadedGameState {
    Current(Box<PersistedGameState>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PersistenceError {
    UnsupportedSchemaVersion { expected: u16, actual: u16 },
    InvalidEncoding,
    TrailingBytes,
    InvalidRawCore,
    InvalidEntitySnapshots,
}

pub(crate) fn capture(game_state: &GameState) -> PersistedGameState {
    PersistedGameState {
        schema_version: CURRENT_SCHEMA_VERSION,
        raw_core: game_state.raw_core.clone(),
        presentation_metadata: game_state.presentation_metadata.clone(),
        presentation_hand: game_state.presentation_hand.clone(),
        presentation_flow: game_state.presentation_flow.clone(),
        locale: game_state.locale,
    }
}

pub(crate) fn encode(game_state: &GameState) -> Vec<u8> {
    let payload =
        namui::bincode::encode_to_vec(capture(game_state), namui::bincode::config::standard())
            .expect("persisted game state encoding");
    let mut bytes = Vec::with_capacity(STORAGE_MAGIC.len() + payload.len());
    bytes.extend_from_slice(STORAGE_MAGIC);
    bytes.extend_from_slice(&payload);
    bytes
}

pub(crate) fn decode(bytes: &[u8]) -> Result<PersistedGameState, PersistenceError> {
    if !bytes.starts_with(STORAGE_MAGIC) {
        return Err(PersistenceError::InvalidEncoding);
    }
    let payload = &bytes[STORAGE_MAGIC.len()..];
    let (schema_version, _) =
        namui::bincode::decode_from_slice::<u16, _>(payload, namui::bincode::config::standard())
            .map_err(|_| PersistenceError::InvalidEncoding)?;
    match schema_version {
        CURRENT_SCHEMA_VERSION => {
            let (persisted, consumed) = namui::bincode::decode_from_slice::<PersistedGameState, _>(
                payload,
                namui::bincode::config::standard(),
            )
            .map_err(|_| PersistenceError::InvalidEncoding)?;
            if consumed != payload.len() {
                return Err(PersistenceError::TrailingBytes);
            }
            checked_persisted_state(persisted)
        }
        actual => Err(PersistenceError::UnsupportedSchemaVersion {
            expected: CURRENT_SCHEMA_VERSION,
            actual,
        }),
    }
}

fn checked_persisted_state(
    persisted: PersistedGameState,
) -> Result<PersistedGameState, PersistenceError> {
    if validate_entity_snapshots(persisted.raw_core.state()) {
        Ok(persisted)
    } else {
        Err(PersistenceError::InvalidEntitySnapshots)
    }
}

fn validate_entity_snapshots(state: &td_core::CoreState) -> bool {
    if !state.validate_snapshot() {
        return false;
    }
    if state.items().iter().any(|item| {
        item.id() == 0
            || crate::game_state::item::ItemWithId::from_core_state(item.clone()).is_none()
    }) {
        return false;
    }
    if crate::game_state::upgrade::UpgradeState::from_core_state(state.upgrades().clone()).is_none()
    {
        return false;
    }
    let next_entity_id = state.next_entity_id().next_id();
    if next_entity_id == 0 {
        return false;
    }
    let mut ids = HashSet::new();
    let mut insert_id = |id: u64| id != 0 && id < next_entity_id && ids.insert(id);

    state
        .monster_spawn()
        .monster_queue
        .iter()
        .all(|monster| insert_id(monster.id))
        && state.monsters().iter().all(|monster| insert_id(monster.id))
        && state
            .towers()
            .iter()
            .all(|tower| tower.id.is_some_and(&mut insert_id))
        && state
            .in_flight_attacks()
            .iter()
            .all(|attack| insert_id(attack.id))
        && {
            let mut item_ids = HashSet::new();
            state
                .items()
                .iter()
                .all(|item| item.id() != 0 && item_ids.insert(item.id()))
        }
}

pub(crate) async fn load_async() -> Result<Option<LoadedGameState>, PersistenceError> {
    let Some(bytes) = namui::system::kv_store::get(STORAGE_KEY).await else {
        return Ok(None);
    };
    decode_loaded_bytes(&bytes).map(Some)
}

fn decode_loaded_bytes(bytes: &[u8]) -> Result<LoadedGameState, PersistenceError> {
    if bytes.starts_with(STORAGE_MAGIC) {
        return decode(bytes).map(|persisted| LoadedGameState::Current(Box::new(persisted)));
    }
    Err(PersistenceError::InvalidEncoding)
}

#[derive(Default)]
struct SaveState {
    last_saved: Option<Vec<u8>>,
    pending: Option<Vec<u8>>,
    write_in_flight: bool,
}

fn save_state() -> &'static Mutex<SaveState> {
    static SAVE_STATE: OnceLock<Mutex<SaveState>> = OnceLock::new();
    SAVE_STATE.get_or_init(|| Mutex::new(SaveState::default()))
}

pub(crate) fn save(game_state: &GameState) {
    let bytes = encode(game_state);
    let next = {
        let mut state = save_state().lock().expect("game state save mutex poisoned");
        if state.last_saved.as_ref() == Some(&bytes) || state.pending.as_ref() == Some(&bytes) {
            return;
        }
        state.pending = Some(bytes);
        if state.write_in_flight {
            None
        } else {
            state.write_in_flight = true;
            state.pending.take()
        }
    };
    if let Some(bytes) = next {
        spawn_save(bytes);
    }
}

fn spawn_save(bytes: Vec<u8>) {
    spawn(async move {
        namui::system::kv_store::put(STORAGE_KEY, Some(&bytes)).await;
        let next = {
            let mut state = save_state().lock().expect("game state save mutex poisoned");
            let next = state.pending.take().filter(|next| next != &bytes);
            state.last_saved = Some(bytes);
            if next.is_none() {
                state.write_in_flight = false;
            }
            next
        };
        if let Some(bytes) = next {
            spawn_save(bytes);
        }
    });
}

pub(crate) fn load_into(game_state: &mut GameState, bytes: &[u8]) -> Result<(), PersistenceError> {
    restore(game_state, decode(bytes)?)
}

pub(crate) fn restore(
    game_state: &mut GameState,
    persisted: PersistedGameState,
) -> Result<(), PersistenceError> {
    if persisted.schema_version != CURRENT_SCHEMA_VERSION {
        return Err(PersistenceError::UnsupportedSchemaVersion {
            expected: CURRENT_SCHEMA_VERSION,
            actual: persisted.schema_version,
        });
    }
    if !validate_entity_snapshots(persisted.raw_core.state()) {
        return Err(PersistenceError::InvalidEntitySnapshots);
    }

    game_state
        .restore_raw_core_projection_at(
            persisted.raw_core.state().clone(),
            crate::PresentationInstant::zero(),
            true,
        )
        .map_err(|_| PersistenceError::InvalidRawCore)?;
    // Rebuild active presentation entries from authoritative Core state.
    // Persisted exit/spring lifetimes belong to the previous session and are
    // intentionally not resumed after restore.
    game_state.presentation_metadata = persisted.presentation_metadata;
    let sim_tick = game_state.sim_tick();
    game_state
        .presentation_metadata
        .rehydrate_runtime_state(sim_tick);
    game_state.locale = persisted.locale;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persisted_state_codec_round_trips_raw_hash_and_metadata() {
        let mut original = crate::game_state::create_game_state_with_seed(0x5EED);
        original.set_locale(crate::l10n::Locale::ENGLISH);
        original.presentation_metadata.towers.push(
            crate::game_state::presentation_metadata::TowerPresentationCache {
                id: crate::TowerId::from_raw(11),
                animation_kind: crate::game_state::tower::AnimationKind::Attack,
                y_ratio_offset: 0.25,
                animation: Default::default(),
                royal_straight_flush_visual: None,
            },
        );
        let expected_hash = original.authoritative_hash();
        let bytes = encode(&original);
        let fixture_digest = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert_eq!(
            fixture_digest,
            "e90d85915d9a9d37f9315668be09cd0f4cc25210a8235c26677364a7a7c9a88f"
        );
        let decoded = decode(&bytes).expect("persisted game state decoding");
        let mut restored_game_state = crate::game_state::create_game_state_with_seed(0xA11CE);

        load_into(&mut restored_game_state, &bytes).expect("persisted game state restore");

        assert_eq!(encode(&restored_game_state), bytes);
        assert_eq!(restored_game_state.authoritative_hash(), expected_hash);
        assert_eq!(decoded.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(restored_game_state.locale(), crate::l10n::Locale::ENGLISH);
        assert_eq!(
            restored_game_state.presentation_metadata.monsters.len(),
            original.presentation_metadata.monsters.len()
        );
        assert_eq!(
            restored_game_state.presentation_metadata.projectiles.len(),
            original.presentation_metadata.projectiles.len()
        );
        assert_eq!(
            restored_game_state.presentation_metadata.towers.len(),
            original.presentation_metadata.towers.len()
        );
        assert_eq!(
            restored_game_state.presentation_metadata.towers[0].id,
            crate::TowerId::from_raw(11)
        );
        assert!(matches!(
            restored_game_state.presentation_metadata.towers[0].animation_kind,
            crate::game_state::tower::AnimationKind::Attack
        ));
        assert_eq!(
            restored_game_state.presentation_metadata.towers[0].y_ratio_offset,
            0.25
        );
    }

    #[test]
    fn restore_rejects_unsupported_schema_before_mutating_state() {
        let original = crate::game_state::create_game_state_with_seed(0xBAD);
        let mut persisted = capture(&original);
        persisted.schema_version += 1;
        let mut target = crate::game_state::create_game_state_with_seed(0x7A6E7);
        let expected_hash = target.authoritative_hash();

        let result = restore(&mut target, persisted);

        assert_eq!(
            result,
            Err(PersistenceError::UnsupportedSchemaVersion {
                expected: CURRENT_SCHEMA_VERSION,
                actual: CURRENT_SCHEMA_VERSION + 1,
            })
        );
        assert_eq!(target.authoritative_hash(), expected_hash);
    }

    #[test]
    fn decode_rejects_invalid_and_trailing_bytes() {
        assert!(matches!(
            decode(&[]),
            Err(PersistenceError::InvalidEncoding)
        ));

        let original = crate::game_state::create_game_state_with_seed(0xC0DE);
        let mut bytes = encode(&original);
        bytes.push(0);

        assert!(matches!(
            decode(&bytes),
            Err(PersistenceError::TrailingBytes)
        ));
    }

    #[test]
    fn duplicate_entity_snapshots_are_rejected_without_mutating_target() {
        let mut original = crate::game_state::create_game_state_with_seed(0xD001);
        original.apply_compatibility_action(crate::game_state::CompatibilityAction::StartDefense);
        original.step_raw_simulation();

        let mut persisted = capture(&original);
        let mut raw_json =
            serde_json::to_value(persisted.raw_core.state()).expect("raw state JSON");
        let duplicate = raw_json["monsters"][0].clone();
        raw_json["monsters"]
            .as_array_mut()
            .expect("monster array")
            .push(duplicate);
        persisted.raw_core =
            HeadedRawCoreState::new(serde_json::from_value(raw_json).expect("duplicate fixture"));
        let payload =
            namui::bincode::encode_to_vec(persisted.clone(), namui::bincode::config::standard())
                .expect("duplicate persisted state encoding");
        let mut bytes = STORAGE_MAGIC.to_vec();
        bytes.extend_from_slice(&payload);
        let mut target = crate::game_state::create_game_state_with_seed(0xD002);
        let expected_hash = target.authoritative_hash();

        assert!(matches!(
            decode(&bytes),
            Err(PersistenceError::InvalidEntitySnapshots)
        ));
        assert_eq!(
            restore(&mut target, persisted),
            Err(PersistenceError::InvalidEntitySnapshots)
        );
        assert_eq!(target.authoritative_hash(), expected_hash);
    }

    #[test]
    fn invalid_allocator_bounds_and_duplicate_items_are_rejected() {
        let original = crate::game_state::create_game_state_with_seed(0xD003);
        let mut target = crate::game_state::create_game_state_with_seed(0xD004);
        let expected_hash = target.authoritative_hash();

        let mut invalid_allocator = capture(&original);
        let mut invalid_allocator_json =
            serde_json::to_value(invalid_allocator.raw_core.state()).expect("raw state JSON");
        invalid_allocator_json["next_entity_id"] = serde_json::json!({ "next": 0 });
        invalid_allocator.raw_core = HeadedRawCoreState::new(
            serde_json::from_value(invalid_allocator_json).expect("invalid allocator fixture"),
        );
        assert_eq!(
            restore(&mut target, invalid_allocator),
            Err(PersistenceError::InvalidEntitySnapshots)
        );
        assert_eq!(target.authoritative_hash(), expected_hash);

        let mut duplicate_items = capture(&original);
        let duplicate_item_id = duplicate_items.raw_core.state().items()[0].id();
        let mut duplicate_items_json =
            serde_json::to_value(duplicate_items.raw_core.state()).expect("raw state JSON");
        duplicate_items_json["items"][1]["id"] = serde_json::json!(duplicate_item_id);
        duplicate_items.raw_core = HeadedRawCoreState::new(
            serde_json::from_value(duplicate_items_json).expect("duplicate item fixture"),
        );
        assert_eq!(
            restore(&mut target, duplicate_items),
            Err(PersistenceError::InvalidEntitySnapshots)
        );
        assert_eq!(target.authoritative_hash(), expected_hash);
    }

    #[test]
    fn transient_headed_state_is_excluded_from_persistence_bytes() {
        let mut game_state = crate::game_state::create_game_state_with_seed(0x7A7A);
        let baseline = encode(&game_state);
        game_state
            .pending_history_events
            .push(crate::game_state::play_history::HistoryEvent {
                stage: game_state.stage,
                timestamp: game_state.sim_tick(),
                event_type: crate::game_state::play_history::HistoryEventType::GameStart,
            });
        game_state
            .pending_card_service_notifications
            .push(crate::game_state::card_notification::CardServiceNotification::new());
        game_state
            .pending_presentation_events
            .push(crate::game_state::PresentationEvent::SaveDebugSnapshot);
        game_state
            .pending_discoveries
            .items
            .push("transient-test-discovery".to_string());

        assert_eq!(encode(&game_state), baseline);
    }
}
