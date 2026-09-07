pub mod attack;
pub mod background;
mod base;
mod camera;
pub mod can_place_tower;
pub(crate) mod compatibility_action;
pub mod cursor_preview;
#[cfg(feature = "debug-tools")]
mod debug_tools;
pub mod difficulty;
pub mod effect;
mod entity_id;
pub mod fast_forward;
pub mod field_particle;
pub mod flow;
pub mod item;
#[allow(unused)]
mod map_decoration_atlas;
pub mod modal;
pub mod monster;
pub(crate) mod monster_spawn;
mod placed_towers;
pub(crate) mod presentation_deck;
pub(crate) mod presentation_effect;
pub mod presentation_event;
pub(crate) mod presentation_inventory;
pub(crate) mod presentation_metadata;
pub(crate) mod presentation_reconciler;
pub(crate) mod presentation_transition;
pub(crate) mod presentation_upgrade;
pub(crate) use compatibility_action::CompatibilityAction;
pub use player_command::HeadedPlayerCommand;
pub use player_command::{CommandError, PlayerCommand, RecordedPlayerCommand};
pub(crate) use td_core::RngState as GameRngState;
pub mod card_notification;
pub mod card_service;
pub(crate) mod core_event_bridge;
pub(crate) mod discovery;
pub(crate) mod persistence;
pub(crate) mod play_history;
pub(crate) mod player_command;
pub mod poker_action;
pub(crate) mod presentation_projection;
pub mod projectile;
pub(crate) mod raw_core;
mod render;
pub(crate) mod render_snapshot;
#[allow(dead_code)]
pub mod replay;
pub(crate) mod shop_purchase;
pub mod stage_modifiers;
mod status_effect_particle_generator;
pub(crate) mod tick;
pub mod tower;
mod tower_info_popup;
pub(crate) mod tower_selection;
mod ui_state;
pub mod upgrade;
mod user_status_effect;

use crate::card::{Deck, Rank, Suit};
use crate::config::GameConfig;
use crate::game_state::stage_modifiers::StageModifiers;
use crate::hand::{Hand, HandItem};
use crate::route::*;
use crate::sound;
use crate::*;
pub use base::*;
pub(crate) use camera::Camera;
pub(crate) use entity_id::EntityIdAllocator;
pub use entity_id::{AttackId, EntityId, MonsterId, TowerId};
use flow::GameFlow;
pub use modal::UserModal;
pub use monster::*;
use monster_spawn::*;
use namui::bincode::{Decode, Encode};
use namui::*;
pub(crate) use placed_towers::PlacedTowers;
pub use presentation_event::*;
use projectile::*;
use rand::Rng;
pub use render::*;
pub(crate) use status_effect_particle_generator::StatusEffectParticleGenerator;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use tower::*;
pub use ui_state::UIState;
use upgrade::UpgradeState;

/// The size of a tile in pixels, with zoom level 1.0.
pub const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
pub const MAP_SIZE: Wh<BlockUnit> = Wh::new(36, 36);
pub const MAP_OUTSIDE_MARGIN_TILES: f32 = 4.0;

pub const TRAVEL_POINTS: [MapCoord; 7] = [
    MapCoord::new(5, 0),
    MapCoord::new(5, 17),
    MapCoord::new(31, 17),
    MapCoord::new(31, 5),
    MapCoord::new(18, 5),
    MapCoord::new(18, 31),
    MapCoord::new(35, 31),
];

const PROJECTILE_WHOOSH_INTERVAL_MIN_SECS: f32 = 0.5;
const PROJECTILE_WHOOSH_INTERVAL_MAX_SECS: f32 = 0.75;

pub(crate) static PROJECTILE_TRAIL_SOUND_IDS: LazyLock<
    Mutex<HashMap<AttackId, (ProjectileTrail, sound::SoundId)>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Default, Clone)]
pub(crate) struct ProjectileTrailEffectState {
    pub trail_distance_remainder: f32,
    pub whoosh_cooldown_secs: f32,
}

pub(crate) static PROJECTILE_TRAIL_EFFECT_STATE: LazyLock<
    Mutex<HashMap<AttackId, ProjectileTrailEffectState>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

impl namui::bincode::Encode for PresentationEventQueue {
    fn encode<__E: namui::bincode::enc::Encoder>(
        &self,
        _encoder: &mut __E,
    ) -> Result<(), namui::bincode::error::EncodeError> {
        Ok(())
    }
}

impl namui::bincode::Decode<()> for PresentationEventQueue {
    fn decode<__D: namui::bincode::de::Decoder<Context = ()>>(
        _decoder: &mut __D,
    ) -> Result<Self, namui::bincode::error::DecodeError> {
        Ok(Self::default())
    }
}

impl namui::Serialize for PresentationEventQueue {
    fn serialize(&self, _buf: &mut Vec<u8>) {}

    fn serialize_without_name(&self, _buf: &mut Vec<u8>) {}
}

impl namui::Deserialize for PresentationEventQueue {
    fn deserialize(_buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        Ok(Self::default())
    }

    fn deserialize_without_name(_buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        Ok(Self::default())
    }
}

#[derive(Debug, Clone, State)]
pub struct TowerDamageStats {
    pub tower_id: TowerId,
    pub tower_kind: TowerKind,
    pub rank: Option<Rank>,
    pub suit: Option<Suit>,
    pub total_damage: Damage,
}

#[derive(Debug, Clone, State)]
pub struct GameMetrics {
    pub total_gold_earned: usize,
    pub total_gold_spent: usize,
    pub current_consecutive_perfect_clears: usize,
    pub max_consecutive_perfect_clears: usize,
    pub tower_damage_stats: Vec<TowerDamageStats>,
    pub total_rerolled_count: usize,
    pub total_escaped_hp: Health,
    pub total_player_damage: Health,
    pub stage_damage: Vec<(usize, Health)>,
}

impl GameMetrics {
    pub(crate) fn to_core_metrics(&self) -> td_core::GameMetrics {
        td_core::GameMetrics {
            total_gold_earned: self.total_gold_earned,
            total_gold_spent: self.total_gold_spent,
            current_consecutive_perfect_clears: self.current_consecutive_perfect_clears,
            max_consecutive_perfect_clears: self.max_consecutive_perfect_clears,
            tower_damage_stats: self
                .tower_damage_stats
                .iter()
                .map(|stats| td_core::TowerDamageStats {
                    tower_id: stats.tower_id.raw(),
                    tower_kind: stats.tower_kind as u8,
                    rank: stats.rank.map(|rank| rank as u8),
                    suit: stats.suit.map(|suit| suit as u8),
                    total_damage_raw: stats.total_damage.raw(),
                })
                .collect(),
            total_rerolled_count: self.total_rerolled_count,
            total_escaped_hp_raw: self.total_escaped_hp.raw(),
            total_player_damage_raw: self.total_player_damage.raw(),
            stage_damage: self
                .stage_damage
                .iter()
                .map(|(stage, damage)| (*stage, damage.raw()))
                .collect(),
        }
    }

    pub(crate) fn from_core_metrics(metrics: td_core::GameMetrics) -> Self {
        Self {
            total_gold_earned: metrics.total_gold_earned,
            total_gold_spent: metrics.total_gold_spent,
            current_consecutive_perfect_clears: metrics.current_consecutive_perfect_clears,
            max_consecutive_perfect_clears: metrics.max_consecutive_perfect_clears,
            tower_damage_stats: metrics
                .tower_damage_stats
                .into_iter()
                .map(|stats| TowerDamageStats {
                    tower_id: TowerId::from_raw(stats.tower_id),
                    tower_kind: tower_kind_from_raw(stats.tower_kind),
                    rank: stats.rank.map(rank_from_raw),
                    suit: stats.suit.map(suit_from_raw),
                    total_damage: Damage::from_raw(stats.total_damage_raw),
                })
                .collect(),
            total_rerolled_count: metrics.total_rerolled_count,
            total_escaped_hp: Health::from_raw(metrics.total_escaped_hp_raw),
            total_player_damage: Health::from_raw(metrics.total_player_damage_raw),
            stage_damage: metrics
                .stage_damage
                .into_iter()
                .map(|(stage, damage)| (stage, Health::from_raw(damage)))
                .collect(),
        }
    }
}

fn tower_kind_from_raw(value: u8) -> TowerKind {
    match value {
        0 => TowerKind::RubberCone,
        1 => TowerKind::High,
        2 => TowerKind::OnePair,
        3 => TowerKind::TwoPair,
        4 => TowerKind::ThreeOfAKind,
        5 => TowerKind::Straight,
        6 => TowerKind::Flush,
        7 => TowerKind::FullHouse,
        8 => TowerKind::FourOfAKind,
        9 => TowerKind::StraightFlush,
        10 => TowerKind::RoyalFlush,
        _ => panic!("invalid tower kind raw value: {value}"),
    }
}

fn rank_from_raw(value: u8) -> Rank {
    match value {
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::Jack,
        10 => Rank::Queen,
        11 => Rank::King,
        12 => Rank::Ace,
        _ => panic!("invalid rank raw value: {value}"),
    }
}

fn suit_from_raw(value: u8) -> Suit {
    match value {
        0 => Suit::Spades,
        1 => Suit::Hearts,
        2 => Suit::Diamonds,
        3 => Suit::Clubs,
        _ => panic!("invalid suit raw value: {value}"),
    }
}

#[derive(State)]
pub struct GameState {
    #[cfg(any(test, feature = "debug-tools"))]
    pub(crate) presentation_projection: presentation_projection::LegacyProjectionCodec,
    pub(crate) raw_core: raw_core::HeadedRawCoreState,
    pub(crate) presentation_metadata: presentation_metadata::PresentationMetadataStore,
    pub(crate) monster_animation_runtime: Vec<MonsterAnimationRuntime>,
    pub(crate) presentation_hand: Hand<HandItem>,
    pub(crate) presentation_flow: GameFlow,
    pub(crate) presentation_inventory: presentation_inventory::PresentationInventory,
    pub(crate) presentation_upgrades: presentation_upgrade::PresentationUpgradeList,
    pub(crate) presentation_deck: presentation_deck::PresentationDeckZones,
    locale: crate::l10n::Locale,
    pending_history_events: Vec<play_history::HistoryEvent>,
    pending_card_service_notifications: Vec<card_notification::CardServiceNotification>,
    pending_modals: modal::OpenedModals,
    pending_presentation_events: PresentationEventQueue,
    pub(crate) pending_discoveries: discovery::DiscoveryState,

    // headless mode for simulator (no UI side-effects like modals, tooltips, notifications)
    pub(crate) headless: bool,
}

#[derive(Clone, State)]
pub(crate) struct MonsterAnimationRuntime {
    pub(crate) id: MonsterId,
    pub(crate) rotation_velocity: f32,
    pub(crate) y_offset_velocity: f32,
    pub(crate) next_descending_left: bool,
}

fn encode_replay_checkpoints<__E: namui::bincode::enc::Encoder>(
    checkpoints: &[replay::ReplayCheckpoint],
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    checkpoints.len().encode(encoder)?;
    for checkpoint in checkpoints {
        checkpoint.sequence.encode(encoder)?;
        checkpoint.completed_sim_tick.encode(encoder)?;
        checkpoint.state_hash.encode(encoder)?;
    }
    Ok(())
}

fn encode_recorded_player_commands<__E: namui::bincode::enc::Encoder>(
    commands: &[RecordedPlayerCommand],
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    commands.len().encode(encoder)?;
    for recorded in commands {
        recorded.sequence.encode(encoder)?;
        recorded.completed_sim_tick.encode(encoder)?;
        let headed = HeadedPlayerCommand::from(recorded.command.clone());
        headed.encode(encoder)?;
    }
    Ok(())
}

fn decode_recorded_player_commands<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Vec<RecordedPlayerCommand>, namui::bincode::error::DecodeError> {
    let len = usize::decode(decoder)?;
    let mut commands = Vec::with_capacity(len);
    for _ in 0..len {
        let sequence = u64::decode(decoder)?;
        let completed_sim_tick = u64::decode(decoder)?;
        let headed = HeadedPlayerCommand::decode(decoder)?;
        commands.push(RecordedPlayerCommand {
            sequence,
            completed_sim_tick,
            command: PlayerCommand::from(&headed),
        });
    }
    Ok(commands)
}

fn serialize_recorded_player_commands(commands: &[RecordedPlayerCommand], buf: &mut Vec<u8>) {
    commands.len().serialize(buf);
    for recorded in commands {
        recorded.sequence.serialize(buf);
        recorded.completed_sim_tick.serialize(buf);
        let headed = HeadedPlayerCommand::from(recorded.command.clone());
        headed.serialize(buf);
    }
}

fn serialize_recorded_player_commands_without_name(
    commands: &[RecordedPlayerCommand],
    buf: &mut Vec<u8>,
) {
    commands.len().serialize_without_name(buf);
    for recorded in commands {
        recorded.sequence.serialize_without_name(buf);
        recorded.completed_sim_tick.serialize_without_name(buf);
        let headed = HeadedPlayerCommand::from(recorded.command.clone());
        headed.serialize_without_name(buf);
    }
}

fn deserialize_recorded_player_commands(
    buf: &mut &[u8],
) -> Result<Vec<RecordedPlayerCommand>, namui::DeserializeError> {
    let len = usize::deserialize(buf)?;
    let mut commands = Vec::with_capacity(len);
    for _ in 0..len {
        let sequence = u64::deserialize(buf)?;
        let completed_sim_tick = u64::deserialize(buf)?;
        let headed = HeadedPlayerCommand::deserialize(buf)?;
        commands.push(RecordedPlayerCommand {
            sequence,
            completed_sim_tick,
            command: PlayerCommand::from(&headed),
        });
    }
    Ok(commands)
}

fn deserialize_recorded_player_commands_without_name(
    buf: &mut &[u8],
) -> Result<Vec<RecordedPlayerCommand>, namui::DeserializeError> {
    let len = usize::deserialize_without_name(buf)?;
    let mut commands = Vec::with_capacity(len);
    for _ in 0..len {
        let sequence = u64::deserialize_without_name(buf)?;
        let completed_sim_tick = u64::deserialize_without_name(buf)?;
        let headed = HeadedPlayerCommand::deserialize_without_name(buf)?;
        commands.push(RecordedPlayerCommand {
            sequence,
            completed_sim_tick,
            command: PlayerCommand::from(&headed),
        });
    }
    Ok(commands)
}

fn decode_replay_checkpoints<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Vec<replay::ReplayCheckpoint>, namui::bincode::error::DecodeError> {
    let len = usize::decode(decoder)?;
    let mut checkpoints = Vec::with_capacity(len);
    for _ in 0..len {
        checkpoints.push(replay::ReplayCheckpoint {
            sequence: u64::decode(decoder)?,
            completed_sim_tick: u64::decode(decoder)?,
            state_hash: String::decode(decoder)?,
            event_count: 0,
            event_digest: String::new(),
        });
    }
    Ok(checkpoints)
}

fn serialize_replay_checkpoints(checkpoints: &[replay::ReplayCheckpoint], buf: &mut Vec<u8>) {
    checkpoints.len().serialize(buf);
    for checkpoint in checkpoints {
        checkpoint.sequence.serialize(buf);
        checkpoint.completed_sim_tick.serialize(buf);
        checkpoint.state_hash.serialize(buf);
    }
}

fn serialize_replay_checkpoints_without_name(
    checkpoints: &[replay::ReplayCheckpoint],
    buf: &mut Vec<u8>,
) {
    checkpoints.len().serialize_without_name(buf);
    for checkpoint in checkpoints {
        checkpoint.sequence.serialize_without_name(buf);
        checkpoint.completed_sim_tick.serialize_without_name(buf);
        checkpoint.state_hash.serialize_without_name(buf);
    }
}

fn deserialize_replay_checkpoints(
    buf: &mut &[u8],
) -> Result<Vec<replay::ReplayCheckpoint>, namui::DeserializeError> {
    let len = usize::deserialize(buf)?;
    let mut checkpoints = Vec::with_capacity(len);
    for _ in 0..len {
        checkpoints.push(replay::ReplayCheckpoint {
            sequence: u64::deserialize(buf)?,
            completed_sim_tick: u64::deserialize(buf)?,
            state_hash: String::deserialize(buf)?,
            event_count: 0,
            event_digest: String::new(),
        });
    }
    Ok(checkpoints)
}

fn deserialize_replay_checkpoints_without_name(
    buf: &mut &[u8],
) -> Result<Vec<replay::ReplayCheckpoint>, namui::DeserializeError> {
    let len = usize::deserialize_without_name(buf)?;
    let mut checkpoints = Vec::with_capacity(len);
    for _ in 0..len {
        checkpoints.push(replay::ReplayCheckpoint {
            sequence: u64::deserialize_without_name(buf)?,
            completed_sim_tick: u64::deserialize_without_name(buf)?,
            state_hash: String::deserialize_without_name(buf)?,
            event_count: 0,
            event_digest: String::new(),
        });
    }
    Ok(checkpoints)
}

fn encode_entity_id_allocator<__E: namui::bincode::enc::Encoder>(
    allocator: EntityIdAllocator,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    allocator.next_id().encode(encoder)
}

fn decode_entity_id_allocator<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<EntityIdAllocator, namui::bincode::error::DecodeError> {
    Ok(EntityIdAllocator::from_next_id(u64::decode(decoder)?))
}

fn serialize_entity_id_allocator(allocator: EntityIdAllocator, buf: &mut Vec<u8>) {
    allocator.next_id().serialize(buf);
}

fn serialize_entity_id_allocator_without_name(allocator: EntityIdAllocator, buf: &mut Vec<u8>) {
    allocator.next_id().serialize_without_name(buf);
}

fn deserialize_entity_id_allocator(
    buf: &mut &[u8],
) -> Result<EntityIdAllocator, namui::DeserializeError> {
    Ok(EntityIdAllocator::from_next_id(u64::deserialize(buf)?))
}

fn deserialize_entity_id_allocator_without_name(
    buf: &mut &[u8],
) -> Result<EntityIdAllocator, namui::DeserializeError> {
    Ok(EntityIdAllocator::from_next_id(
        u64::deserialize_without_name(buf)?,
    ))
}

fn encode_user_status_effects<__E: namui::bincode::enc::Encoder>(
    effects: &[td_core::UserStatusEffect],
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    effects.len().encode(encoder)?;
    for effect in effects {
        match effect.kind {
            td_core::UserStatusEffectKind::DamageReduction {
                damage_multiply_raw,
            } => {
                0u8.encode(encoder)?;
                damage_multiply_raw.encode(encoder)?;
            }
        }
        effect.end_at.encode(encoder)?;
    }
    Ok(())
}

fn decode_user_status_effects<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Vec<td_core::UserStatusEffect>, namui::bincode::error::DecodeError> {
    let len = usize::decode(decoder)?;
    let mut effects = Vec::with_capacity(len);
    for _ in 0..len {
        let kind = match u8::decode(decoder)? {
            0 => td_core::UserStatusEffectKind::DamageReduction {
                damage_multiply_raw: i64::decode(decoder)?,
            },
            value => panic!("invalid user status effect kind: {value}"),
        };
        effects.push(td_core::UserStatusEffect {
            kind,
            end_at: u64::decode(decoder)?,
        });
    }
    Ok(effects)
}

fn serialize_user_status_effects(effects: &[td_core::UserStatusEffect], buf: &mut Vec<u8>) {
    effects.len().serialize(buf);
    for effect in effects {
        match effect.kind {
            td_core::UserStatusEffectKind::DamageReduction {
                damage_multiply_raw,
            } => {
                0u8.serialize(buf);
                damage_multiply_raw.serialize(buf);
            }
        }
        effect.end_at.serialize(buf);
    }
}

fn serialize_user_status_effects_without_name(
    effects: &[td_core::UserStatusEffect],
    buf: &mut Vec<u8>,
) {
    effects.len().serialize_without_name(buf);
    for effect in effects {
        match effect.kind {
            td_core::UserStatusEffectKind::DamageReduction {
                damage_multiply_raw,
            } => {
                0u8.serialize_without_name(buf);
                damage_multiply_raw.serialize_without_name(buf);
            }
        }
        effect.end_at.serialize_without_name(buf);
    }
}

fn deserialize_user_status_effects(
    buf: &mut &[u8],
) -> Result<Vec<td_core::UserStatusEffect>, namui::DeserializeError> {
    let len = usize::deserialize(buf)?;
    let mut effects = Vec::with_capacity(len);
    for _ in 0..len {
        let kind = match u8::deserialize(buf)? {
            0 => td_core::UserStatusEffectKind::DamageReduction {
                damage_multiply_raw: i64::deserialize(buf)?,
            },
            value => panic!("invalid user status effect kind: {value}"),
        };
        effects.push(td_core::UserStatusEffect {
            kind,
            end_at: u64::deserialize(buf)?,
        });
    }
    Ok(effects)
}

fn deserialize_user_status_effects_without_name(
    buf: &mut &[u8],
) -> Result<Vec<td_core::UserStatusEffect>, namui::DeserializeError> {
    let len = usize::deserialize_without_name(buf)?;
    let mut effects = Vec::with_capacity(len);
    for _ in 0..len {
        let kind = match u8::deserialize_without_name(buf)? {
            0 => td_core::UserStatusEffectKind::DamageReduction {
                damage_multiply_raw: i64::deserialize_without_name(buf)?,
            },
            value => panic!("invalid user status effect kind: {value}"),
        };
        effects.push(td_core::UserStatusEffect {
            kind,
            end_at: u64::deserialize_without_name(buf)?,
        });
    }
    Ok(effects)
}

fn encode_route<__E: namui::bincode::enc::Encoder>(
    route: &Route,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    let state = route.to_core_state();
    state.map_coords.encode(encoder)?;
    state.world_coords.encode(encoder)?;
    state.segment_lengths.encode(encoder)?;
    state.cumulative_lengths.encode(encoder)
}

fn encode_stage_modifiers<__E: namui::bincode::enc::Encoder>(
    modifiers: &StageModifiers,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    let normalized = StageModifiers::from_core_state(modifiers.to_core_state())
        .expect("valid stage modifiers state");
    normalized.encode(encoder)
}

fn decode_stage_modifiers<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<StageModifiers, namui::bincode::error::DecodeError> {
    let decoded = StageModifiers::decode(decoder)?;
    StageModifiers::from_core_state(decoded.to_core_state()).ok_or(
        namui::bincode::error::DecodeError::OtherString(
            "invalid stage modifiers state".to_string(),
        ),
    )
}

fn serialize_stage_modifiers(modifiers: &StageModifiers, buf: &mut Vec<u8>) {
    let normalized = StageModifiers::from_core_state(modifiers.to_core_state())
        .expect("valid stage modifiers state");
    normalized.serialize(buf);
}

fn serialize_stage_modifiers_without_name(modifiers: &StageModifiers, buf: &mut Vec<u8>) {
    let normalized = StageModifiers::from_core_state(modifiers.to_core_state())
        .expect("valid stage modifiers state");
    normalized.serialize_without_name(buf);
}

fn deserialize_stage_modifiers(buf: &mut &[u8]) -> Result<StageModifiers, namui::DeserializeError> {
    let decoded = StageModifiers::deserialize(buf)?;
    StageModifiers::from_core_state(decoded.to_core_state()).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid stage modifiers state".to_string(),
            actual: "invalid stage modifiers state".to_string(),
        },
    )
}

fn deserialize_stage_modifiers_without_name(
    buf: &mut &[u8],
) -> Result<StageModifiers, namui::DeserializeError> {
    let decoded = StageModifiers::deserialize_without_name(buf)?;
    StageModifiers::from_core_state(decoded.to_core_state()).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid stage modifiers state".to_string(),
            actual: "invalid stage modifiers state".to_string(),
        },
    )
}

fn encode_deck<__E: namui::bincode::enc::Encoder>(
    deck: &Deck,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    let normalized = Deck::from_core_state(deck.to_core_state()).expect("valid deck state");
    normalized.encode(encoder)
}

fn decode_deck<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Deck, namui::bincode::error::DecodeError> {
    let decoded = Deck::decode(decoder)?;
    Deck::from_core_state(decoded.to_core_state()).ok_or(
        namui::bincode::error::DecodeError::OtherString("invalid deck state".to_string()),
    )
}

fn serialize_deck(deck: &Deck, buf: &mut Vec<u8>) {
    let normalized = Deck::from_core_state(deck.to_core_state()).expect("valid deck state");
    normalized.serialize(buf);
}

fn serialize_deck_without_name(deck: &Deck, buf: &mut Vec<u8>) {
    let normalized = Deck::from_core_state(deck.to_core_state()).expect("valid deck state");
    normalized.serialize_without_name(buf);
}

fn deserialize_deck(buf: &mut &[u8]) -> Result<Deck, namui::DeserializeError> {
    let decoded = Deck::deserialize(buf)?;
    Deck::from_core_state(decoded.to_core_state()).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid deck state".to_string(),
            actual: "invalid deck state".to_string(),
        },
    )
}

fn deserialize_deck_without_name(buf: &mut &[u8]) -> Result<Deck, namui::DeserializeError> {
    let decoded = Deck::deserialize_without_name(buf)?;
    Deck::from_core_state(decoded.to_core_state()).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid deck state".to_string(),
            actual: "invalid deck state".to_string(),
        },
    )
}

fn hand_for_legacy_serialization(hand: &Hand<HandItem>) -> Hand<HandItem> {
    Hand::from_core_state(hand.to_core_state(), Some(hand)).expect("valid hand state")
}

fn encode_hand<__E: namui::bincode::enc::Encoder>(
    hand: &Hand<HandItem>,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    hand_for_legacy_serialization(hand).encode(encoder)
}

fn decode_hand<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Hand<HandItem>, namui::bincode::error::DecodeError> {
    let decoded = Hand::<HandItem>::decode(decoder)?;
    Hand::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::bincode::error::DecodeError::OtherString("invalid hand state".to_string()),
    )
}

fn serialize_hand(hand: &Hand<HandItem>, buf: &mut Vec<u8>) {
    hand_for_legacy_serialization(hand).serialize(buf);
}

fn serialize_hand_without_name(hand: &Hand<HandItem>, buf: &mut Vec<u8>) {
    hand_for_legacy_serialization(hand).serialize_without_name(buf);
}

fn deserialize_hand(buf: &mut &[u8]) -> Result<Hand<HandItem>, namui::DeserializeError> {
    let decoded = Hand::<HandItem>::deserialize(buf)?;
    Hand::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid hand state".to_string(),
            actual: "invalid hand state".to_string(),
        },
    )
}

fn deserialize_hand_without_name(
    buf: &mut &[u8],
) -> Result<Hand<HandItem>, namui::DeserializeError> {
    let decoded = Hand::<HandItem>::deserialize_without_name(buf)?;
    Hand::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid hand state".to_string(),
            actual: "invalid hand state".to_string(),
        },
    )
}

fn items_for_legacy_serialization(
    items: &[crate::game_state::item::ItemWithId],
) -> Vec<crate::game_state::item::ItemWithId> {
    items
        .iter()
        .map(crate::game_state::item::ItemWithId::to_core_state)
        .map(|state| {
            crate::game_state::item::ItemWithId::from_core_state(state).expect("valid item state")
        })
        .collect()
}

fn normalize_decoded_items(
    items: Vec<crate::game_state::item::ItemWithId>,
) -> Option<Vec<crate::game_state::item::ItemWithId>> {
    let items = items
        .into_iter()
        .map(|item| crate::game_state::item::ItemWithId::from_core_state(item.to_core_state()))
        .collect::<Option<Vec<_>>>()?;
    let mut ids = Vec::with_capacity(items.len());
    for item in &items {
        if ids.contains(&item.id) {
            return None;
        }
        ids.push(item.id);
    }
    Some(items)
}

fn monster_spawn_for_legacy_serialization(state: &MonsterSpawnState) -> MonsterSpawnState {
    MonsterSpawnState::from_core_state(state.to_core_state(), Some(state))
        .expect("valid monster spawn state")
}

fn encode_monster_spawn_state<__E: namui::bincode::enc::Encoder>(
    state: &MonsterSpawnState,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    monster_spawn_for_legacy_serialization(state).encode(encoder)
}

fn decode_monster_spawn_state<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<MonsterSpawnState, namui::bincode::error::DecodeError> {
    let decoded = MonsterSpawnState::decode(decoder)?;
    MonsterSpawnState::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::bincode::error::DecodeError::OtherString("invalid monster spawn state".to_string()),
    )
}

fn serialize_monster_spawn_state(state: &MonsterSpawnState, buf: &mut Vec<u8>) {
    monster_spawn_for_legacy_serialization(state).serialize(buf);
}

fn serialize_monster_spawn_state_without_name(state: &MonsterSpawnState, buf: &mut Vec<u8>) {
    monster_spawn_for_legacy_serialization(state).serialize_without_name(buf);
}

fn deserialize_monster_spawn_state(
    buf: &mut &[u8],
) -> Result<MonsterSpawnState, namui::DeserializeError> {
    let decoded = MonsterSpawnState::deserialize(buf)?;
    MonsterSpawnState::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid monster spawn state".to_string(),
            actual: "invalid monster spawn state".to_string(),
        },
    )
}

fn deserialize_monster_spawn_state_without_name(
    buf: &mut &[u8],
) -> Result<MonsterSpawnState, namui::DeserializeError> {
    let decoded = MonsterSpawnState::deserialize_without_name(buf)?;
    MonsterSpawnState::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid monster spawn state".to_string(),
            actual: "invalid monster spawn state".to_string(),
        },
    )
}

fn flow_for_legacy_serialization(flow: &GameFlow) -> GameFlow {
    GameFlow::from_core_state(flow.to_core_state(), Some(flow)).expect("valid game flow state")
}

fn encode_flow<__E: namui::bincode::enc::Encoder>(
    flow: &GameFlow,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    flow_for_legacy_serialization(flow).encode(encoder)
}

fn decode_flow<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<GameFlow, namui::bincode::error::DecodeError> {
    let decoded = GameFlow::decode(decoder)?;
    GameFlow::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::bincode::error::DecodeError::OtherString("invalid game flow state".to_string()),
    )
}

fn serialize_flow(flow: &GameFlow, buf: &mut Vec<u8>) {
    flow_for_legacy_serialization(flow).serialize(buf);
}

fn serialize_flow_without_name(flow: &GameFlow, buf: &mut Vec<u8>) {
    flow_for_legacy_serialization(flow).serialize_without_name(buf);
}

fn deserialize_flow(buf: &mut &[u8]) -> Result<GameFlow, namui::DeserializeError> {
    let decoded = GameFlow::deserialize(buf)?;
    GameFlow::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid game flow state".to_string(),
            actual: "invalid game flow state".to_string(),
        },
    )
}

fn deserialize_flow_without_name(buf: &mut &[u8]) -> Result<GameFlow, namui::DeserializeError> {
    let decoded = GameFlow::deserialize_without_name(buf)?;
    GameFlow::from_core_state(decoded.to_core_state(), Some(&decoded)).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid game flow state".to_string(),
            actual: "invalid game flow state".to_string(),
        },
    )
}

fn encode_upgrade_state<__E: namui::bincode::enc::Encoder>(
    state: &UpgradeState,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    let normalized = UpgradeState::from_core_state(state.to_core_state())
        .expect("valid upgrade collection state");
    normalized.encode(encoder)
}

fn decode_upgrade_state<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<UpgradeState, namui::bincode::error::DecodeError> {
    let decoded = UpgradeState::decode(decoder)?;
    UpgradeState::from_core_state(decoded.to_core_state()).ok_or(
        namui::bincode::error::DecodeError::OtherString(
            "invalid upgrade collection state".to_string(),
        ),
    )
}

fn serialize_upgrade_state(state: &UpgradeState, buf: &mut Vec<u8>) {
    let normalized = UpgradeState::from_core_state(state.to_core_state())
        .expect("valid upgrade collection state");
    normalized.serialize(buf);
}

fn serialize_upgrade_state_without_name(state: &UpgradeState, buf: &mut Vec<u8>) {
    let normalized = UpgradeState::from_core_state(state.to_core_state())
        .expect("valid upgrade collection state");
    normalized.serialize_without_name(buf);
}

fn deserialize_upgrade_state(buf: &mut &[u8]) -> Result<UpgradeState, namui::DeserializeError> {
    let decoded = UpgradeState::deserialize(buf)?;
    UpgradeState::from_core_state(decoded.to_core_state()).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid upgrade collection state".to_string(),
            actual: "invalid upgrade collection state".to_string(),
        },
    )
}

fn deserialize_upgrade_state_without_name(
    buf: &mut &[u8],
) -> Result<UpgradeState, namui::DeserializeError> {
    let decoded = UpgradeState::deserialize_without_name(buf)?;
    UpgradeState::from_core_state(decoded.to_core_state()).ok_or(
        namui::DeserializeError::InvalidEnumVariant {
            expected: "valid upgrade collection state".to_string(),
            actual: "invalid upgrade collection state".to_string(),
        },
    )
}

fn decode_route<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Arc<Route>, namui::bincode::error::DecodeError> {
    let state = td_core::RouteState {
        map_coords: Vec::<[usize; 2]>::decode(decoder)?,
        world_coords: Vec::<[i64; 2]>::decode(decoder)?,
        segment_lengths: Vec::<i64>::decode(decoder)?,
        cumulative_lengths: Vec::<i64>::decode(decoder)?,
    };
    Ok(Arc::new(
        Route::from_core_state(state).expect("serialized route should be valid"),
    ))
}

fn serialize_route(route: &Route, buf: &mut Vec<u8>) {
    let state = route.to_core_state();
    state.map_coords.serialize(buf);
    state.world_coords.serialize(buf);
    state.segment_lengths.serialize(buf);
    state.cumulative_lengths.serialize(buf);
}

fn serialize_route_without_name(route: &Route, buf: &mut Vec<u8>) {
    let state = route.to_core_state();
    state.map_coords.serialize_without_name(buf);
    state.world_coords.serialize_without_name(buf);
    state.segment_lengths.serialize_without_name(buf);
    state.cumulative_lengths.serialize_without_name(buf);
}

fn deserialize_route(buf: &mut &[u8]) -> Result<Arc<Route>, namui::DeserializeError> {
    let state = td_core::RouteState {
        map_coords: Vec::<[usize; 2]>::deserialize(buf)?,
        world_coords: Vec::<[i64; 2]>::deserialize(buf)?,
        segment_lengths: Vec::<i64>::deserialize(buf)?,
        cumulative_lengths: Vec::<i64>::deserialize(buf)?,
    };
    Ok(Arc::new(
        Route::from_core_state(state).expect("serialized route should be valid"),
    ))
}

fn deserialize_route_without_name(buf: &mut &[u8]) -> Result<Arc<Route>, namui::DeserializeError> {
    let state = td_core::RouteState {
        map_coords: Vec::<[usize; 2]>::deserialize_without_name(buf)?,
        world_coords: Vec::<[i64; 2]>::deserialize_without_name(buf)?,
        segment_lengths: Vec::<i64>::deserialize_without_name(buf)?,
        cumulative_lengths: Vec::<i64>::deserialize_without_name(buf)?,
    };
    Ok(Arc::new(
        Route::from_core_state(state).expect("serialized route should be valid"),
    ))
}

fn encode_game_metrics<__E: namui::bincode::enc::Encoder>(
    metrics: &GameMetrics,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    metrics.total_gold_earned.encode(encoder)?;
    metrics.total_gold_spent.encode(encoder)?;
    metrics.current_consecutive_perfect_clears.encode(encoder)?;
    metrics.max_consecutive_perfect_clears.encode(encoder)?;
    metrics.tower_damage_stats.encode(encoder)?;
    metrics.total_rerolled_count.encode(encoder)?;
    metrics.total_escaped_hp.encode(encoder)?;
    metrics.total_player_damage.encode(encoder)?;
    metrics.stage_damage.encode(encoder)
}

fn decode_game_metrics<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<GameMetrics, namui::bincode::error::DecodeError> {
    Ok(GameMetrics {
        total_gold_earned: usize::decode(decoder)?,
        total_gold_spent: usize::decode(decoder)?,
        current_consecutive_perfect_clears: usize::decode(decoder)?,
        max_consecutive_perfect_clears: usize::decode(decoder)?,
        tower_damage_stats: Vec::<TowerDamageStats>::decode(decoder)?,
        total_rerolled_count: usize::decode(decoder)?,
        total_escaped_hp: Health::decode(decoder)?,
        total_player_damage: Health::decode(decoder)?,
        stage_damage: Vec::<(usize, Health)>::decode(decoder)?,
    })
}

fn serialize_game_metrics(metrics: &GameMetrics, buf: &mut Vec<u8>) {
    metrics.total_gold_earned.serialize(buf);
    metrics.total_gold_spent.serialize(buf);
    metrics.current_consecutive_perfect_clears.serialize(buf);
    metrics.max_consecutive_perfect_clears.serialize(buf);
    metrics.tower_damage_stats.serialize(buf);
    metrics.total_rerolled_count.serialize(buf);
    metrics.total_escaped_hp.serialize(buf);
    metrics.total_player_damage.serialize(buf);
    metrics.stage_damage.serialize(buf);
}

fn serialize_game_metrics_without_name(metrics: &GameMetrics, buf: &mut Vec<u8>) {
    metrics.total_gold_earned.serialize_without_name(buf);
    metrics.total_gold_spent.serialize_without_name(buf);
    metrics
        .current_consecutive_perfect_clears
        .serialize_without_name(buf);
    metrics
        .max_consecutive_perfect_clears
        .serialize_without_name(buf);
    metrics.tower_damage_stats.serialize_without_name(buf);
    metrics.total_rerolled_count.serialize_without_name(buf);
    metrics.total_escaped_hp.serialize_without_name(buf);
    metrics.total_player_damage.serialize_without_name(buf);
    metrics.stage_damage.serialize_without_name(buf);
}

fn deserialize_game_metrics(buf: &mut &[u8]) -> Result<GameMetrics, namui::DeserializeError> {
    Ok(GameMetrics {
        total_gold_earned: usize::deserialize(buf)?,
        total_gold_spent: usize::deserialize(buf)?,
        current_consecutive_perfect_clears: usize::deserialize(buf)?,
        max_consecutive_perfect_clears: usize::deserialize(buf)?,
        tower_damage_stats: Vec::<TowerDamageStats>::deserialize(buf)?,
        total_rerolled_count: usize::deserialize(buf)?,
        total_escaped_hp: Health::deserialize(buf)?,
        total_player_damage: Health::deserialize(buf)?,
        stage_damage: Vec::<(usize, Health)>::deserialize(buf)?,
    })
}

fn deserialize_game_metrics_without_name(
    buf: &mut &[u8],
) -> Result<GameMetrics, namui::DeserializeError> {
    Ok(GameMetrics {
        total_gold_earned: usize::deserialize_without_name(buf)?,
        total_gold_spent: usize::deserialize_without_name(buf)?,
        current_consecutive_perfect_clears: usize::deserialize_without_name(buf)?,
        max_consecutive_perfect_clears: usize::deserialize_without_name(buf)?,
        tower_damage_stats: Vec::<TowerDamageStats>::deserialize_without_name(buf)?,
        total_rerolled_count: usize::deserialize_without_name(buf)?,
        total_escaped_hp: Health::deserialize_without_name(buf)?,
        total_player_damage: Health::deserialize_without_name(buf)?,
        stage_damage: Vec::<(usize, Health)>::deserialize_without_name(buf)?,
    })
}

fn encode_rng_state<__E: namui::bincode::enc::Encoder>(
    rng: &GameRngState,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    rng.seed.encode(encoder)?;
    rng.shop.generation_sequence.encode(encoder)?;
    rng.shop.config.category_bag_size.encode(encoder)?;
    rng.shop.config.category_weights.encode(encoder)?;
    rng.shop.config.rarity_bag_size.encode(encoder)?;
    rng.shop.config.item_rarity_weights.encode(encoder)?;
    rng.shop
        .config
        .card_service_rarity_weights
        .encode(encoder)?;
    rng.shop.config.upgrade_rarity_weights.encode(encoder)?;
    encode_rng_bag(&rng.shop.category_bag, encoder)?;
    encode_rng_bags(&rng.shop.rarity_bags, encoder)?;
    encode_rng_content_bags(&rng.shop.content_bags, encoder)?;
    rng.domain_sequences.encode(encoder)
}

fn encode_rng_bag<__E: namui::bincode::enc::Encoder>(
    bag: &td_core::BagState,
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    bag.entries.encode(encoder)?;
    bag.cursor.encode(encoder)?;
    bag.cycle.encode(encoder)
}

fn decode_rng_state<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<GameRngState, namui::bincode::error::DecodeError> {
    Ok(GameRngState {
        seed: u64::decode(decoder)?,
        shop: td_core::ShopBagState {
            generation_sequence: u64::decode(decoder)?,
            config: td_core::ShopGenerationConfig {
                category_bag_size: usize::decode(decoder)?,
                category_weights: Vec::<u32>::decode(decoder)?,
                rarity_bag_size: usize::decode(decoder)?,
                item_rarity_weights: Vec::<u32>::decode(decoder)?,
                card_service_rarity_weights: Vec::<u32>::decode(decoder)?,
                upgrade_rarity_weights: Vec::<u32>::decode(decoder)?,
            },
            category_bag: decode_rng_bag(decoder)?,
            rarity_bags: decode_rng_bags(decoder)?,
            content_bags: decode_rng_content_bags(decoder)?,
        },
        domain_sequences: std::collections::BTreeMap::<u64, u64>::decode(decoder)?,
    })
}

fn decode_rng_bag<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<td_core::BagState, namui::bincode::error::DecodeError> {
    Ok(td_core::BagState {
        entries: Vec::<u8>::decode(decoder)?,
        cursor: usize::decode(decoder)?,
        cycle: u64::decode(decoder)?,
    })
}

fn encode_rng_bags<__E: namui::bincode::enc::Encoder>(
    bags: &[td_core::BagState],
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    bags.len().encode(encoder)?;
    for bag in bags {
        encode_rng_bag(bag, encoder)?;
    }
    Ok(())
}

fn encode_rng_content_bags<__E: namui::bincode::enc::Encoder>(
    bags: &[td_core::ContentBagState],
    encoder: &mut __E,
) -> Result<(), namui::bincode::error::EncodeError> {
    bags.len().encode(encoder)?;
    for bag in bags {
        bag.entries.encode(encoder)?;
        bag.cursor.encode(encoder)?;
        bag.cycle.encode(encoder)?;
    }
    Ok(())
}

fn decode_rng_bags<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Vec<td_core::BagState>, namui::bincode::error::DecodeError> {
    let len = usize::decode(decoder)?;
    (0..len).map(|_| decode_rng_bag(decoder)).collect()
}

fn decode_rng_content_bags<__D: namui::bincode::de::Decoder<Context = ()>>(
    decoder: &mut __D,
) -> Result<Vec<td_core::ContentBagState>, namui::bincode::error::DecodeError> {
    let len = usize::decode(decoder)?;
    (0..len)
        .map(|_| {
            Ok(td_core::ContentBagState {
                entries: Vec::<String>::decode(decoder)?,
                cursor: usize::decode(decoder)?,
                cycle: u64::decode(decoder)?,
            })
        })
        .collect()
}

fn serialize_rng_state(rng: &GameRngState, buf: &mut Vec<u8>) {
    rng.seed.serialize(buf);
    rng.shop.generation_sequence.serialize(buf);
    rng.shop.config.category_bag_size.serialize(buf);
    rng.shop.config.category_weights.serialize(buf);
    rng.shop.config.rarity_bag_size.serialize(buf);
    rng.shop.config.item_rarity_weights.serialize(buf);
    rng.shop.config.card_service_rarity_weights.serialize(buf);
    rng.shop.config.upgrade_rarity_weights.serialize(buf);
    serialize_rng_bag(&rng.shop.category_bag, buf);
    serialize_rng_bags(&rng.shop.rarity_bags, buf);
    serialize_rng_content_bags(&rng.shop.content_bags, buf);
    rng.domain_sequences.serialize(buf);
}

fn serialize_rng_bag(bag: &td_core::BagState, buf: &mut Vec<u8>) {
    bag.entries.serialize(buf);
    bag.cursor.serialize(buf);
    bag.cycle.serialize(buf);
}

fn serialize_rng_bags(bags: &[td_core::BagState], buf: &mut Vec<u8>) {
    bags.len().serialize(buf);
    for bag in bags {
        serialize_rng_bag(bag, buf);
    }
}

fn serialize_rng_content_bags(bags: &[td_core::ContentBagState], buf: &mut Vec<u8>) {
    bags.len().serialize(buf);
    for bag in bags {
        bag.entries.serialize(buf);
        bag.cursor.serialize(buf);
        bag.cycle.serialize(buf);
    }
}

fn serialize_rng_state_without_name(rng: &GameRngState, buf: &mut Vec<u8>) {
    rng.seed.serialize_without_name(buf);
    rng.shop.generation_sequence.serialize_without_name(buf);
    rng.shop
        .config
        .category_bag_size
        .serialize_without_name(buf);
    rng.shop.config.category_weights.serialize_without_name(buf);
    rng.shop.config.rarity_bag_size.serialize_without_name(buf);
    rng.shop
        .config
        .item_rarity_weights
        .serialize_without_name(buf);
    rng.shop
        .config
        .card_service_rarity_weights
        .serialize_without_name(buf);
    rng.shop
        .config
        .upgrade_rarity_weights
        .serialize_without_name(buf);
    serialize_rng_bag_without_name(&rng.shop.category_bag, buf);
    serialize_rng_bags_without_name(&rng.shop.rarity_bags, buf);
    serialize_rng_content_bags_without_name(&rng.shop.content_bags, buf);
    rng.domain_sequences.serialize_without_name(buf);
}

fn serialize_rng_bag_without_name(bag: &td_core::BagState, buf: &mut Vec<u8>) {
    bag.entries.serialize_without_name(buf);
    bag.cursor.serialize_without_name(buf);
    bag.cycle.serialize_without_name(buf);
}

fn serialize_rng_bags_without_name(bags: &[td_core::BagState], buf: &mut Vec<u8>) {
    bags.len().serialize_without_name(buf);
    for bag in bags {
        serialize_rng_bag_without_name(bag, buf);
    }
}

fn serialize_rng_content_bags_without_name(bags: &[td_core::ContentBagState], buf: &mut Vec<u8>) {
    bags.len().serialize_without_name(buf);
    for bag in bags {
        bag.entries.serialize_without_name(buf);
        bag.cursor.serialize_without_name(buf);
        bag.cycle.serialize_without_name(buf);
    }
}

fn deserialize_rng_state(buf: &mut &[u8]) -> Result<GameRngState, namui::DeserializeError> {
    Ok(GameRngState {
        seed: u64::deserialize(buf)?,
        shop: td_core::ShopBagState {
            generation_sequence: u64::deserialize(buf)?,
            config: td_core::ShopGenerationConfig {
                category_bag_size: usize::deserialize(buf)?,
                category_weights: Vec::<u32>::deserialize(buf)?,
                rarity_bag_size: usize::deserialize(buf)?,
                item_rarity_weights: Vec::<u32>::deserialize(buf)?,
                card_service_rarity_weights: Vec::<u32>::deserialize(buf)?,
                upgrade_rarity_weights: Vec::<u32>::deserialize(buf)?,
            },
            category_bag: deserialize_rng_bag(buf)?,
            rarity_bags: deserialize_rng_bags(buf)?,
            content_bags: deserialize_rng_content_bags(buf)?,
        },
        domain_sequences: std::collections::BTreeMap::<u64, u64>::deserialize(buf)?,
    })
}

fn deserialize_rng_bag(buf: &mut &[u8]) -> Result<td_core::BagState, namui::DeserializeError> {
    Ok(td_core::BagState {
        entries: Vec::<u8>::deserialize(buf)?,
        cursor: usize::deserialize(buf)?,
        cycle: u64::deserialize(buf)?,
    })
}

fn deserialize_rng_bags(
    buf: &mut &[u8],
) -> Result<Vec<td_core::BagState>, namui::DeserializeError> {
    let len = usize::deserialize(buf)?;
    (0..len).map(|_| deserialize_rng_bag(buf)).collect()
}

fn deserialize_rng_content_bags(
    buf: &mut &[u8],
) -> Result<Vec<td_core::ContentBagState>, namui::DeserializeError> {
    let len = usize::deserialize(buf)?;
    (0..len)
        .map(|_| {
            Ok(td_core::ContentBagState {
                entries: Vec::<String>::deserialize(buf)?,
                cursor: usize::deserialize(buf)?,
                cycle: u64::deserialize(buf)?,
            })
        })
        .collect()
}

fn deserialize_rng_state_without_name(
    buf: &mut &[u8],
) -> Result<GameRngState, namui::DeserializeError> {
    Ok(GameRngState {
        seed: u64::deserialize_without_name(buf)?,
        shop: td_core::ShopBagState {
            generation_sequence: u64::deserialize_without_name(buf)?,
            config: td_core::ShopGenerationConfig {
                category_bag_size: usize::deserialize_without_name(buf)?,
                category_weights: Vec::<u32>::deserialize_without_name(buf)?,
                rarity_bag_size: usize::deserialize_without_name(buf)?,
                item_rarity_weights: Vec::<u32>::deserialize_without_name(buf)?,
                card_service_rarity_weights: Vec::<u32>::deserialize_without_name(buf)?,
                upgrade_rarity_weights: Vec::<u32>::deserialize_without_name(buf)?,
            },
            category_bag: deserialize_rng_bag_without_name(buf)?,
            rarity_bags: deserialize_rng_bags_without_name(buf)?,
            content_bags: deserialize_rng_content_bags_without_name(buf)?,
        },
        domain_sequences: std::collections::BTreeMap::<u64, u64>::deserialize_without_name(buf)?,
    })
}

fn deserialize_rng_bag_without_name(
    buf: &mut &[u8],
) -> Result<td_core::BagState, namui::DeserializeError> {
    Ok(td_core::BagState {
        entries: Vec::<u8>::deserialize_without_name(buf)?,
        cursor: usize::deserialize_without_name(buf)?,
        cycle: u64::deserialize_without_name(buf)?,
    })
}

fn deserialize_rng_bags_without_name(
    buf: &mut &[u8],
) -> Result<Vec<td_core::BagState>, namui::DeserializeError> {
    let len = usize::deserialize_without_name(buf)?;
    (0..len)
        .map(|_| deserialize_rng_bag_without_name(buf))
        .collect()
}

fn deserialize_rng_content_bags_without_name(
    buf: &mut &[u8],
) -> Result<Vec<td_core::ContentBagState>, namui::DeserializeError> {
    let len = usize::deserialize_without_name(buf)?;
    (0..len)
        .map(|_| {
            Ok(td_core::ContentBagState {
                entries: Vec::<String>::deserialize_without_name(buf)?,
                cursor: usize::deserialize_without_name(buf)?,
                cycle: u64::deserialize_without_name(buf)?,
            })
        })
        .collect()
}

pub(crate) struct PendingActionEffects {
    pub(crate) history_events: Vec<play_history::HistoryEvent>,
    pub(crate) card_service_notifications: Vec<card_notification::CardServiceNotification>,
    pub(crate) presentation_events: PresentationEventQueue,
    pub(crate) discoveries: discovery::DiscoveryState,
}

impl namui::bincode::Encode for presentation_projection::LegacyProjectionCodec {
    fn encode<__E: namui::bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), namui::bincode::error::EncodeError> {
        self.sim_tick.ticks().encode(encoder)?;
        encode_rng_state(&self.rng, encoder)?;
        encode_route(&self.route, encoder)?;
        self.config_for_legacy_serialization().encode(encoder)?;
        encode_stage_modifiers(&self.stage_modifiers, encoder)?;
        encode_upgrade_state(&self.upgrade_state, encoder)?;
        encode_hand(&self.hand, encoder)?;
        encode_deck(&self.deck, encoder)?;
        items_for_legacy_serialization(&self.items).encode(encoder)?;
        encode_monster_spawn_state(&self.monster_spawn_state, encoder)?;
        self.attacks_for_legacy_serialization().encode(encoder)?;
        encode_user_status_effects(&self.user_status_effects, encoder)?;
        encode_entity_id_allocator(self.next_entity_id, encoder)?;
        encode_game_metrics(
            &GameMetrics::from_core_metrics(self.metrics.clone()),
            encoder,
        )?;
        encode_flow(&self.flow, encoder)?;
        self.progress.stage.encode(encoder)?;
        self.progress.gold.encode(encoder)?;
        self.hp.encode(encoder)?;
        self.shield.encode(encoder)?;
        self.progress.left_dice.encode(encoder)?;
        self.progress.rerolled_count.encode(encoder)?;
        self.progress
            .left_quest_board_refresh_chance
            .encode(encoder)?;
        self.progress.item_used.encode(encoder)?;
        self.monsters_for_legacy_serialization().encode(encoder)?;
        self.towers_for_legacy_serialization().encode(encoder)?;
        self.progress.player_command_sequence.encode(encoder)?;
        encode_recorded_player_commands(&self.player_commands, encoder)?;
        encode_replay_checkpoints(&self.replay_checkpoints, encoder)?;
        self.pending_card_service_kind.encode(encoder)
    }
}

impl namui::bincode::Decode<()> for presentation_projection::LegacyProjectionCodec {
    fn decode<__D: namui::bincode::de::Decoder<Context = ()>>(
        decoder: &mut __D,
    ) -> Result<Self, namui::bincode::error::DecodeError> {
        let sim_tick = SimTick::from_ticks(u64::decode(decoder)?);
        let rng = decode_rng_state(decoder)?;
        let route = decode_route(decoder)?;
        let config = Arc::new(
            GameConfig::from_core_state(Arc::<GameConfig>::decode(decoder)?.to_core_state())
                .ok_or(namui::bincode::error::DecodeError::OtherString(
                    "invalid game config state".to_string(),
                ))?,
        );
        let stage_modifiers = decode_stage_modifiers(decoder)?;
        let upgrade_state = decode_upgrade_state(decoder)?;
        let hand = decode_hand(decoder)?;
        let deck = decode_deck(decoder)?;
        let items = normalize_decoded_items(Vec::<item::ItemWithId>::decode(decoder)?).ok_or(
            namui::bincode::error::DecodeError::OtherString(
                "invalid item collection state".to_string(),
            ),
        )?;
        let monster_spawn_state = decode_monster_spawn_state(decoder)?;
        let in_flight_attacks = Vec::<attack::InFlightAttack>::decode(decoder)?;
        let user_status_effects = decode_user_status_effects(decoder)?;
        let next_entity_id = decode_entity_id_allocator(decoder)?;
        let metrics = decode_game_metrics(decoder)?;
        let flow = decode_flow(decoder)?;
        let stage = usize::decode(decoder)?;
        let gold = usize::decode(decoder)?;
        let hp = Health::decode(decoder)?;
        let shield = Shield::from_raw(i64::decode(decoder)?);
        let left_dice = usize::decode(decoder)?;
        let rerolled_count = usize::decode(decoder)?;
        let left_quest_board_refresh_chance = usize::decode(decoder)?;
        let item_used = bool::decode(decoder)?;
        let monsters = Vec::<Monster>::decode(decoder)?;
        let towers = PlacedTowers::decode(decoder)?;
        let player_command_sequence = u64::decode(decoder)?;
        let player_commands = decode_recorded_player_commands(decoder)?;
        let replay_checkpoints = decode_replay_checkpoints(decoder)?;
        let pending_card_service_kind = Option::<u8>::decode(decoder)?;
        let mut state = Self::new(
            sim_tick,
            rng,
            route,
            config,
            stage_modifiers,
            upgrade_state,
            hand,
            deck,
            items,
            monster_spawn_state,
            in_flight_attacks,
            user_status_effects,
            next_entity_id,
            metrics.to_core_metrics(),
            flow,
            stage,
            gold,
            hp,
            shield,
            left_dice,
            rerolled_count,
            left_quest_board_refresh_chance,
            item_used,
            monsters,
            towers,
            player_command_sequence,
            player_commands,
            replay_checkpoints,
        );
        state.pending_card_service_kind = pending_card_service_kind;
        if !state.normalize_decoded_entity_snapshots()
            || !state.normalize_decoded_attack_snapshots()
        {
            return Err(namui::bincode::error::DecodeError::OtherString(
                "invalid entity snapshot collection".to_string(),
            ));
        }
        Ok(state)
    }
}

impl namui::Serialize for presentation_projection::LegacyProjectionCodec {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.sim_tick.ticks().serialize(buf);
        serialize_rng_state(&self.rng, buf);
        serialize_route(&self.route, buf);
        self.config_for_legacy_serialization().serialize(buf);
        serialize_stage_modifiers(&self.stage_modifiers, buf);
        serialize_upgrade_state(&self.upgrade_state, buf);
        serialize_hand(&self.hand, buf);
        serialize_deck(&self.deck, buf);
        items_for_legacy_serialization(&self.items).serialize(buf);
        serialize_monster_spawn_state(&self.monster_spawn_state, buf);
        self.attacks_for_legacy_serialization().serialize(buf);
        serialize_user_status_effects(&self.user_status_effects, buf);
        serialize_entity_id_allocator(self.next_entity_id, buf);
        serialize_game_metrics(&GameMetrics::from_core_metrics(self.metrics.clone()), buf);
        serialize_flow(&self.flow, buf);
        self.progress.stage.serialize(buf);
        self.progress.gold.serialize(buf);
        self.hp.serialize(buf);
        self.shield.serialize(buf);
        self.progress.left_dice.serialize(buf);
        self.progress.rerolled_count.serialize(buf);
        self.progress.left_quest_board_refresh_chance.serialize(buf);
        self.progress.item_used.serialize(buf);
        self.monsters_for_legacy_serialization().serialize(buf);
        self.towers_for_legacy_serialization().serialize(buf);
        self.progress.player_command_sequence.serialize(buf);
        serialize_recorded_player_commands(&self.player_commands, buf);
        serialize_replay_checkpoints(&self.replay_checkpoints, buf);
        self.pending_card_service_kind.serialize(buf);
    }

    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        self.sim_tick.ticks().serialize_without_name(buf);
        serialize_rng_state_without_name(&self.rng, buf);
        serialize_route_without_name(&self.route, buf);
        self.config_for_legacy_serialization()
            .serialize_without_name(buf);
        serialize_stage_modifiers_without_name(&self.stage_modifiers, buf);
        serialize_upgrade_state_without_name(&self.upgrade_state, buf);
        serialize_hand_without_name(&self.hand, buf);
        serialize_deck_without_name(&self.deck, buf);
        items_for_legacy_serialization(&self.items).serialize_without_name(buf);
        serialize_monster_spawn_state_without_name(&self.monster_spawn_state, buf);
        self.attacks_for_legacy_serialization()
            .serialize_without_name(buf);
        serialize_user_status_effects_without_name(&self.user_status_effects, buf);
        serialize_entity_id_allocator_without_name(self.next_entity_id, buf);
        serialize_game_metrics_without_name(
            &GameMetrics::from_core_metrics(self.metrics.clone()),
            buf,
        );
        serialize_flow_without_name(&self.flow, buf);
        self.progress.stage.serialize_without_name(buf);
        self.progress.gold.serialize_without_name(buf);
        self.hp.serialize_without_name(buf);
        self.shield.serialize_without_name(buf);
        self.progress.left_dice.serialize_without_name(buf);
        self.progress.rerolled_count.serialize_without_name(buf);
        self.progress
            .left_quest_board_refresh_chance
            .serialize_without_name(buf);
        self.progress.item_used.serialize_without_name(buf);
        self.monsters_for_legacy_serialization()
            .serialize_without_name(buf);
        self.towers_for_legacy_serialization()
            .serialize_without_name(buf);
        self.progress
            .player_command_sequence
            .serialize_without_name(buf);
        serialize_recorded_player_commands_without_name(&self.player_commands, buf);
        serialize_replay_checkpoints_without_name(&self.replay_checkpoints, buf);
        self.pending_card_service_kind.serialize_without_name(buf);
    }
}

impl namui::Deserialize for presentation_projection::LegacyProjectionCodec {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        let mut state = Self::new(
            SimTick::from_ticks(u64::deserialize(buf)?),
            deserialize_rng_state(buf)?,
            deserialize_route(buf)?,
            Arc::new(
                GameConfig::from_core_state(Arc::<GameConfig>::deserialize(buf)?.to_core_state())
                    .ok_or(namui::DeserializeError::InvalidEnumVariant {
                    expected: "valid game config state".to_string(),
                    actual: "invalid game config state".to_string(),
                })?,
            ),
            deserialize_stage_modifiers(buf)?,
            deserialize_upgrade_state(buf)?,
            deserialize_hand(buf)?,
            deserialize_deck(buf)?,
            normalize_decoded_items(Vec::<item::ItemWithId>::deserialize(buf)?).ok_or(
                namui::DeserializeError::InvalidEnumVariant {
                    expected: "valid item collection state".to_string(),
                    actual: "invalid item collection state".to_string(),
                },
            )?,
            deserialize_monster_spawn_state(buf)?,
            Vec::<attack::InFlightAttack>::deserialize(buf)?,
            deserialize_user_status_effects(buf)?,
            deserialize_entity_id_allocator(buf)?,
            deserialize_game_metrics(buf)?.to_core_metrics(),
            deserialize_flow(buf)?,
            usize::deserialize(buf)?,
            usize::deserialize(buf)?,
            Health::deserialize(buf)?,
            Shield::from_raw(i64::deserialize(buf)?),
            usize::deserialize(buf)?,
            usize::deserialize(buf)?,
            usize::deserialize(buf)?,
            bool::deserialize(buf)?,
            Vec::<Monster>::deserialize(buf)?,
            PlacedTowers::deserialize(buf)?,
            u64::deserialize(buf)?,
            deserialize_recorded_player_commands(buf)?,
            deserialize_replay_checkpoints(buf)?,
        );
        let pending_card_service_kind = Option::<u8>::deserialize(buf)?;
        state.pending_card_service_kind = pending_card_service_kind;
        if !state.normalize_decoded_entity_snapshots()
            || !state.normalize_decoded_attack_snapshots()
        {
            return Err(namui::DeserializeError::InvalidEnumVariant {
                expected: "valid entity snapshot collection".to_string(),
                actual: "invalid entity snapshot collection".to_string(),
            });
        }
        Ok(state)
    }

    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        let mut state = Self::new(
            SimTick::from_ticks(u64::deserialize_without_name(buf)?),
            deserialize_rng_state_without_name(buf)?,
            deserialize_route_without_name(buf)?,
            Arc::new(
                GameConfig::from_core_state(
                    Arc::<GameConfig>::deserialize_without_name(buf)?.to_core_state(),
                )
                .ok_or(namui::DeserializeError::InvalidEnumVariant {
                    expected: "valid game config state".to_string(),
                    actual: "invalid game config state".to_string(),
                })?,
            ),
            deserialize_stage_modifiers_without_name(buf)?,
            deserialize_upgrade_state_without_name(buf)?,
            deserialize_hand_without_name(buf)?,
            deserialize_deck_without_name(buf)?,
            normalize_decoded_items(Vec::<item::ItemWithId>::deserialize_without_name(buf)?)
                .ok_or(namui::DeserializeError::InvalidEnumVariant {
                    expected: "valid item collection state".to_string(),
                    actual: "invalid item collection state".to_string(),
                })?,
            deserialize_monster_spawn_state_without_name(buf)?,
            Vec::<attack::InFlightAttack>::deserialize_without_name(buf)?,
            deserialize_user_status_effects_without_name(buf)?,
            deserialize_entity_id_allocator_without_name(buf)?,
            deserialize_game_metrics_without_name(buf)?.to_core_metrics(),
            deserialize_flow_without_name(buf)?,
            usize::deserialize_without_name(buf)?,
            usize::deserialize_without_name(buf)?,
            Health::deserialize_without_name(buf)?,
            Shield::from_raw(i64::deserialize_without_name(buf)?),
            usize::deserialize_without_name(buf)?,
            usize::deserialize_without_name(buf)?,
            usize::deserialize_without_name(buf)?,
            bool::deserialize_without_name(buf)?,
            Vec::<Monster>::deserialize_without_name(buf)?,
            PlacedTowers::deserialize_without_name(buf)?,
            u64::deserialize_without_name(buf)?,
            deserialize_recorded_player_commands_without_name(buf)?,
            deserialize_replay_checkpoints_without_name(buf)?,
        );
        let pending_card_service_kind = Option::<u8>::deserialize_without_name(buf)?;
        state.pending_card_service_kind = pending_card_service_kind;
        if !state.normalize_decoded_entity_snapshots() {
            return Err(namui::DeserializeError::InvalidEnumVariant {
                expected: "valid entity snapshot collection".to_string(),
                actual: "invalid entity snapshot collection".to_string(),
            });
        }
        Ok(state)
    }
}

impl GameState {
    pub(crate) fn raw_render_snapshot(&self) -> td_core::RenderSnapshot {
        self.raw_core_state().render_snapshot()
    }

    pub(crate) fn raw_core_state(&self) -> &td_core::CoreState {
        self.raw_core.state()
    }

    #[cfg(test)]
    pub(crate) fn shield_amount(&self) -> Shield {
        Shield::from_raw(self.raw_core.shield_raw())
    }

    #[cfg(any(test, feature = "debug-tools"))]
    pub(crate) fn presentation_projection(
        &self,
    ) -> &presentation_projection::LegacyProjectionCodec {
        &self.presentation_projection
    }

    #[cfg(any(test, feature = "debug-tools"))]
    pub(crate) fn presentation_projection_mut(
        &mut self,
    ) -> &mut presentation_projection::LegacyProjectionCodec {
        &mut self.presentation_projection
    }

    pub(crate) fn presentation_hand_snapshot(&self) -> Hand<HandItem> {
        self.presentation_hand.clone()
    }

    pub(crate) fn presentation_flow_snapshot(&self) -> GameFlow {
        GameFlow::from_core_state(self.raw_core.flow().clone(), Some(&self.presentation_flow))
            .expect("raw flow must be restorable for presentation")
    }

    pub(crate) fn presentation_items_snapshot(&self) -> Vec<item::ItemWithId> {
        self.raw_core
            .items()
            .iter()
            .map(|entry| {
                let mut item = item::ItemWithId::from_core_state(entry.clone())
                    .expect("raw item must be restorable for presentation");
                item.id = crate::game_state::item::ItemId(entry.id());
                item
            })
            .collect()
    }

    pub(crate) fn presentation_inventory_snapshot(
        &self,
    ) -> Vec<presentation_inventory::PresentationInventoryEntry> {
        self.presentation_inventory.entries.clone()
    }

    pub(crate) fn presentation_upgrade_entries_snapshot(
        &self,
    ) -> Vec<presentation_upgrade::PresentationUpgradeEntry> {
        self.presentation_upgrades.entries.clone()
    }

    pub(crate) fn presentation_route_snapshot(&self) -> Arc<Route> {
        Arc::new(
            Route::from_core_state(self.raw_core.route().clone())
                .expect("raw route must be restorable for presentation"),
        )
    }

    pub(crate) fn presentation_upgrade_state_snapshot(&self) -> UpgradeState {
        UpgradeState::from_core_state(self.raw_core.upgrades().clone())
            .expect("raw upgrades must be restorable for presentation")
    }

    pub(crate) fn presentation_deck_snapshot(&self) -> Deck {
        Deck::from_core_state(self.raw_core.deck().clone())
            .expect("raw deck must be restorable for presentation")
    }

    pub(crate) fn presentation_deck_zone_snapshot(
        &self,
        zone: usize,
    ) -> Vec<presentation_deck::PresentationDeckCard> {
        self.presentation_deck.snapshot(zone)
    }

    pub(crate) fn presentation_tower(
        &self,
        tower_id: crate::TowerId,
    ) -> Option<crate::game_state::tower::Tower> {
        self.raw_core
            .towers()
            .iter()
            .find(|tower| tower.id == Some(tower_id.raw()))
            .cloned()
            .and_then(|tower| {
                crate::game_state::tower::Tower::from_core_state(
                    tower,
                    SimTick::from_ticks(self.raw_core.sim_tick().ticks()),
                )
            })
    }

    pub(crate) fn presentation_flow_mut(&mut self) -> &mut GameFlow {
        &mut self.presentation_flow
    }

    pub(crate) fn presentation_hand_mut(&mut self) -> &mut Hand<HandItem> {
        &mut self.presentation_hand
    }

    fn refresh_monster_animation_runtime(&mut self) {
        let active_ids = self
            .raw_core
            .monsters()
            .iter()
            .map(|monster| MonsterId::from_raw(monster.id))
            .collect::<std::collections::HashSet<_>>();
        self.monster_animation_runtime
            .retain(|runtime| active_ids.contains(&runtime.id));
        for id in active_ids {
            if !self
                .monster_animation_runtime
                .iter()
                .any(|runtime| runtime.id == id)
            {
                self.monster_animation_runtime
                    .push(MonsterAnimationRuntime {
                        id,
                        rotation_velocity: 0.0,
                        y_offset_velocity: 0.0,
                        next_descending_left: false,
                    });
            }
        }
    }

    pub(crate) fn presentation_in_flight_attacks(
        &self,
    ) -> Vec<crate::game_state::attack::InFlightAttack> {
        self.raw_core
            .in_flight_attacks()
            .iter()
            .map(|attack| {
                let presentation = self
                    .presentation_metadata
                    .projectile(crate::AttackId::from_raw(attack.id))
                    .map(|projectile| {
                        (
                            projectile.projectile_kind,
                            projectile.trail,
                            projectile.hit_effect,
                        )
                    })
                    .or_else(|| {
                        matches!(&attack.kind, td_core::InFlightAttackKindState::Spatial(_))
                            .then_some((
                                crate::game_state::projectile::ProjectileKind::Trash01,
                                crate::game_state::projectile::ProjectileTrail::None,
                                crate::game_state::attack::ProjectileHitEffect::TrashBounce,
                            ))
                    });
                crate::game_state::attack::InFlightAttack::from_core_state(
                    attack.clone(),
                    presentation,
                )
                .expect("raw attack must be restorable for presentation")
            })
            .collect()
    }

    pub(crate) fn deselect_placing_tower_slot(&mut self, hand_slot_index: usize) -> bool {
        let mut raw = self.raw_core.state().clone();
        if !matches!(raw.flow(), td_core::GameFlowState::PlacingTower) {
            return false;
        }
        let mut updated = false;
        let result = raw.edit_snapshot(|parts| {
            let Some(slot) = parts.hand.slots.get_mut(hand_slot_index) else {
                return;
            };
            if !matches!(slot.item, td_core::HandItemState::Tower(_)) {
                return;
            }
            slot.selected = false;
            updated = true;
        });
        if result.is_err() || !updated {
            return false;
        }
        self.restore_raw_core_projection(raw).is_ok()
    }

    pub(crate) fn toggle_selecting_tower_card(
        &mut self,
        hand_slot_id: crate::hand::HandSlotId,
    ) -> Option<bool> {
        let hand_slot_index = self
            .presentation_hand_snapshot()
            .active_slot_ids()
            .iter()
            .position(|candidate| *candidate == hand_slot_id)?;
        let mut raw = self.raw_core.state().clone();
        if !matches!(raw.flow(), td_core::GameFlowState::SelectingTower) {
            return None;
        }
        let mut selected = None;
        raw.edit_snapshot(|parts| {
            let Some(slot) = parts.hand.slots.get_mut(hand_slot_index) else {
                return;
            };
            if !matches!(slot.item, td_core::HandItemState::Card(_)) {
                return;
            }
            slot.selected = !slot.selected;
            selected = Some(slot.selected);
        })
        .ok()?;
        let selected = selected?;
        self.restore_raw_core_projection(raw).ok()?;
        Some(selected)
    }

    pub(crate) fn select_placing_tower_slot(
        &mut self,
        hand_slot_id: crate::hand::HandSlotId,
    ) -> bool {
        let Some(hand_slot_index) = self
            .presentation_hand_snapshot()
            .active_slot_ids()
            .iter()
            .position(|candidate| *candidate == hand_slot_id)
        else {
            return false;
        };
        let mut raw = self.raw_core.state().clone();
        if !matches!(raw.flow(), td_core::GameFlowState::PlacingTower)
            || raw.hand().slots.iter().any(|slot| slot.selected)
        {
            return false;
        }
        let mut selected = false;
        if raw
            .edit_snapshot(|parts| {
                let Some(slot) = parts.hand.slots.get_mut(hand_slot_index) else {
                    return;
                };
                if !matches!(slot.item, td_core::HandItemState::Tower(_)) {
                    return;
                }
                slot.selected = true;
                selected = true;
            })
            .is_err()
            || !selected
        {
            return false;
        }
        self.restore_raw_core_projection(raw).is_ok()
    }

    #[cfg(test)]
    pub(crate) fn authoritative_core_state(&self) -> td_core::CoreState {
        self.raw_core.state().clone()
    }

    #[cfg(test)]
    pub(crate) fn authoritative_hash(&self) -> String {
        td_core::authoritative_hash(self.raw_core_state())
    }

    #[cfg(any(test, feature = "debug-tools"))]
    #[allow(dead_code)]
    pub(crate) fn step_raw_simulation(&mut self) {
        self.step_raw_simulation_at(PresentationInstant::capture());
    }

    pub(crate) fn step_raw_simulation_at(&mut self, presentation_instant: PresentationInstant) {
        self.raw_core.advance_tick_with_events();
        let events = self.raw_core.drain_events().collect::<Vec<_>>();
        self.raw_core.extend_events(events);
        #[cfg(any(test, feature = "debug-tools"))]
        {
            let presentation_source = self.presentation_projection.clone();
            self.presentation_projection =
                crate::game_state::presentation_projection::LegacyProjectionCodec::from_td_core_state(
                    self.raw_core.state().clone(),
                    Some(&presentation_source),
                )
                .expect("raw simulation step must be restorable in compatibility fixture");
        }
        let raw = self.raw_core.state().clone();
        self.presentation_metadata.refresh_from_core(&raw);
        self.refresh_monster_animation_runtime();
        if !self.headless {
            self.presentation_hand = Hand::from_core_state_at(
                raw.hand().clone(),
                Some(&self.presentation_hand),
                presentation_instant,
            )
            .expect("raw simulation hand must be restorable for presentation");
            self.presentation_flow =
                GameFlow::from_core_state(raw.flow().clone(), Some(&self.presentation_flow))
                    .expect("raw simulation flow must be restorable for presentation");
        }
        self.reconcile_presentation(presentation_instant, false);
    }

    pub(crate) fn sync_raw_core_from_projection(&mut self) {
        #[cfg(any(test, feature = "debug-tools"))]
        {
            let mut events = self.raw_core.drain_events().collect::<Vec<_>>();
            events.extend(self.presentation_projection_mut().drain_events());
            self.raw_core = raw_core::HeadedRawCoreState::new(
                self.presentation_projection().to_td_core_state(),
            );
            self.raw_core.extend_events(events);
            self.presentation_hand = Hand::from_core_state(self.raw_core.hand().clone(), None)
                .expect("legacy projection hand must produce a valid core hand");
            self.presentation_flow = GameFlow::from_core_state(self.raw_core.flow().clone(), None)
                .expect("legacy projection flow must produce a valid core flow");
            self.reconcile_presentation(PresentationInstant::zero(), true);
            let raw = self.raw_core.state().clone();
            self.presentation_metadata.refresh_from_core(&raw);
            self.refresh_monster_animation_runtime();
            #[cfg(any(test, feature = "debug-tools"))]
            for attack in &self.presentation_projection.in_flight_attacks {
                if let crate::game_state::attack::InFlightAttackKind::Spatial(spatial) =
                    &attack.kind
                {
                    self.presentation_metadata.insert_projectile(
                        attack.id,
                        spatial.projectile_kind,
                        spatial.trail,
                        spatial.hit_effect,
                    );
                }
            }
        }
    }

    pub(crate) fn restore_raw_core_projection(
        &mut self,
        raw: td_core::CoreState,
    ) -> Result<(), CommandError> {
        self.restore_raw_core_projection_at(raw, PresentationInstant::capture(), false)
    }

    pub(crate) fn restore_raw_core_projection_at(
        &mut self,
        raw: td_core::CoreState,
        presentation_instant: PresentationInstant,
        restore: bool,
    ) -> Result<(), CommandError> {
        self.raw_core = raw_core::HeadedRawCoreState::new(raw);
        let mut presentation_metadata = std::mem::take(&mut self.presentation_metadata);
        presentation_metadata.refresh_from_core(self.raw_core.state());
        self.presentation_metadata = presentation_metadata;
        self.refresh_monster_animation_runtime();
        self.presentation_hand = Hand::from_core_state_at(
            self.raw_core.hand().clone(),
            (!restore).then_some(&self.presentation_hand),
            presentation_instant,
        )
        .ok_or(CommandError::Rejected)?;
        self.presentation_flow = GameFlow::from_core_state(
            self.raw_core.flow().clone(),
            (!restore).then_some(&self.presentation_flow),
        )
        .ok_or(CommandError::Rejected)?;
        self.reconcile_presentation(presentation_instant, restore);
        #[cfg(any(test, feature = "debug-tools"))]
        {
            let presentation_source = self.presentation_projection.clone();
            self.presentation_projection =
                crate::game_state::presentation_projection::LegacyProjectionCodec::from_td_core_state(
                    self.raw_core.state().clone(),
                    Some(&presentation_source),
                )
                .ok_or(CommandError::Rejected)?;
        }
        Ok(())
    }

    fn reconcile_presentation(&mut self, presentation_instant: PresentationInstant, restore: bool) {
        if self.headless {
            return;
        }
        let items = self.presentation_items_snapshot();
        let upgrades = self.presentation_upgrade_state_snapshot();
        let deck = self.presentation_deck_snapshot();
        presentation_reconciler::PresentationReconciler::reconcile(
            presentation_reconciler::PresentationReconcileInput {
                inventory: &mut self.presentation_inventory,
                upgrades: &mut self.presentation_upgrades,
                deck_zones: &mut self.presentation_deck,
                items: &items,
                upgrade_state: &upgrades,
                deck: &deck,
                presentation_instant,
                restore,
            },
        );
    }

    pub(crate) fn reseed_presentation_collections(&mut self) {
        self.reconcile_presentation(PresentationInstant::zero(), true);
    }

    pub(crate) fn locale(&self) -> crate::l10n::Locale {
        self.locale
    }

    pub(crate) fn grant_core_item(&mut self, item: td_core::ItemEntry) -> Option<item::ItemWithId> {
        let mut raw = self.raw_core.state().clone();
        raw.grant_inventory_item(item).ok()?;
        let granted = raw.items().entries().last()?.clone();
        self.restore_raw_core_projection(raw).ok()?;
        let item = item::ItemWithId::from_core_state(granted)?;
        self.discover_item(&item.item);
        Some(item)
    }

    pub(crate) fn set_locale(&mut self, locale: crate::l10n::Locale) {
        self.locale = locale;
    }

    pub(crate) fn take_pending_action_effects(&mut self) -> PendingActionEffects {
        PendingActionEffects {
            history_events: std::mem::take(&mut self.pending_history_events),
            card_service_notifications: std::mem::take(
                &mut self.pending_card_service_notifications,
            ),
            presentation_events: std::mem::take(&mut self.pending_presentation_events),
            discoveries: std::mem::take(&mut self.pending_discoveries),
        }
    }

    pub(crate) fn take_pending_history_events(&mut self) -> Vec<play_history::HistoryEvent> {
        std::mem::take(&mut self.pending_history_events)
    }

    pub(crate) fn push_presentation_event(&mut self, event: PresentationEvent) {
        self.pending_presentation_events.push(event);
    }

    pub(crate) fn pending_presentation_events_mut(&mut self) -> &mut PresentationEventQueue {
        &mut self.pending_presentation_events
    }

    pub(crate) fn clear_presentation_events(&mut self) {
        self.pending_presentation_events.clear();
    }

    pub(crate) fn drain_core_events(
        &mut self,
    ) -> Vec<crate::game_state::presentation_projection::CoreEvent> {
        self.raw_core.drain_events().collect()
    }

    #[cfg(any(test, feature = "debug-tools"))]
    pub(crate) fn consume_core_events_now(&mut self) {
        let events = self.drain_core_events();
        if self.headless {
            core_event_bridge::consume_headless(events);
        } else {
            core_event_bridge::consume_headed(self, events, PresentationInstant::capture());
        }
    }

    #[cfg(test)]
    pub(crate) fn allocate_entity_id(&mut self) -> EntityId {
        let mut allocated = None;
        self.raw_core
            .edit_snapshot(|parts| allocated = Some(parts.next_entity_id.allocate_raw()))
            .expect("entity allocation must preserve a valid snapshot");
        EntityId::from_raw(allocated.expect("entity allocation result"))
    }

    #[cfg(test)]
    pub(crate) fn allocate_tower_id(&mut self) -> TowerId {
        TowerId::from_entity_id(self.allocate_entity_id())
    }

    pub fn max_shop_slot(&self) -> usize {
        td_core::max_shop_slot_count(self.raw_core_state())
    }

    pub fn max_hp(&self) -> Health {
        Health::from_raw(self.raw_core_state().max_hp_raw())
    }

    pub fn max_dice_chance(&self) -> usize {
        self.raw_core_state().max_dice_chance()
    }

    pub fn generate_rarity(&self) -> crate::rarity::Rarity {
        crate::rarity::Rarity::Common
    }

    /// Returns whether the shop panel is allowed to be opened based on current flow.
    pub fn can_open_shop_panel(&self) -> bool {
        matches!(
            self.raw_core_state().flow(),
            td_core::GameFlowState::Shopping(_)
        )
    }
    pub fn sim_tick(&self) -> SimTick {
        SimTick::from_ticks(self.raw_core.sim_tick().ticks())
    }

    pub fn is_headless(&self) -> bool {
        self.headless
    }

    pub(crate) fn set_user_modal(&mut self, modal: Option<modal::UserModal>) {
        if self.headless {
            self.pending_modals.user = modal;
        } else {
            set_modal(modal);
        }
    }

    #[cfg(test)]
    pub(crate) fn set_card_service_selection(
        &mut self,
        selection: crate::game_state::modal::deck::CardSelectionState,
    ) {
        self.set_user_modal(Some(modal::UserModal::Deck(modal::deck::DeckModal {
            deck_kind: modal::deck::DeckKind::Deck,
            selection: Some(selection),
        })));
    }

    pub(crate) fn open_card_service_selection_from_core_event(
        &mut self,
        service_kind: &str,
        step_counts: &[usize],
    ) {
        let Some(modal) =
            core_event_bridge::card_service_selection(self, service_kind, step_counts)
        else {
            return;
        };
        self.set_user_modal(Some(modal));
    }

    pub(crate) fn metrics(&self) -> GameMetrics {
        GameMetrics::from_core_metrics(self.raw_core.metrics().clone())
    }

    pub(crate) fn replay(&self) -> td_core::CoreReplay {
        td_core::CoreReplay::from_state(self.raw_core_state())
    }

    pub fn export_core_replay(&self) -> td_core::CoreReplay {
        self.replay()
    }

    pub fn flush_presentation_events(&mut self, presentation_instant: PresentationInstant) {
        let presentation_now = presentation_instant.as_namui();
        let mut active_trail_sound_projectiles = std::collections::HashSet::new();
        let mut active_projectile_sound_ids = PROJECTILE_TRAIL_SOUND_IDS.lock().unwrap();

        let presentation_events = self.pending_presentation_events.drain().collect::<Vec<_>>();
        for event in presentation_events {
            match event {
                PresentationEvent::AnimateBase(_) => {}
                PresentationEvent::ShakeCamera { .. } => {}
                PresentationEvent::SpawnRoyalStraightFlushVisual { .. } => {}
                PresentationEvent::SpawnParticle(request) => match request {
                    ParticleSpawnRequest::DamageText { position, damage } => {
                        field_particle::DAMAGE_TEXTS.spawn(
                            field_particle::DamageTextParticle::new(
                                MapCoordF32::new(position[0], position[1]),
                                damage,
                                presentation_now,
                            ),
                        );
                    }
                    ParticleSpawnRequest::TrashBounce {
                        projectile_kind,
                        start_xy,
                        end_xy,
                    } => {
                        for particle in field_particle::emitter::create_bounce_particles(
                            projectile_kind,
                            (start_xy[0], start_xy[1]),
                            (end_xy[0], end_xy[1]),
                            presentation_now,
                        ) {
                            field_particle::TRASHES.spawn(particle);
                        }
                    }
                    ParticleSpawnRequest::MonsterSoul {
                        position,
                        rotation_radians,
                    } => {
                        let pixel_position = TILE_PX_SIZE.to_xy()
                            * WorldCoord::new(position[0], position[1]).as_map_coord_f32();
                        field_particle::MONSTER_SOULS.spawn(
                            field_particle::MonsterSoulParticle::new(
                                pixel_position,
                                presentation_now,
                                rotation_radians.rad(),
                            ),
                        );
                    }
                    ParticleSpawnRequest::MonsterCorpse {
                        position,
                        rotation_radians,
                        monster_kind,
                    } => {
                        let pixel_position = TILE_PX_SIZE.to_xy()
                            * WorldCoord::new(position[0], position[1]).as_map_coord_f32();
                        field_particle::MONSTER_CORPSES.spawn(
                            field_particle::MonsterCorpseParticle::new(
                                pixel_position,
                                presentation_now,
                                rotation_radians.rad(),
                                monster_kind,
                                monster::monster_wh(monster_kind),
                            ),
                        );
                    }
                },
                PresentationEvent::PlaySoundCue {
                    cue,
                    position,
                    volume,
                    max_duration_ms,
                } => {
                    let asset = match cue {
                        SoundCue::KnifeSlash => sound::deterministic_knife_slash(),
                        SoundCue::Coin => sound::random_coin_sounds(),
                        SoundCue::LuggageDrop => sound::random_luggage_drop(),
                        SoundCue::StartDefenseFanfare => sound::random_trumpet_fanfares(),
                        SoundCue::Pickaxe => sound::random_pickaxe(),
                        SoundCue::Fail => sound::random_fail(),
                        SoundCue::MonsterFootstep => sound::random_cloth_footstep(),
                        SoundCue::PaperCrumpling => sound::random_paper_crumpling(),
                        SoundCue::RedLaserShot => sound::deterministic_red_laser_shot(),
                        SoundCue::Whoop => sound::deterministic_whoop(),
                        SoundCue::Wind => sound::deterministic_wind(),
                        SoundCue::Flamethrower => sound::deterministic_flamethrower(),
                        SoundCue::SmokeBomb => sound::deterministic_smoke_bomb(),
                    };
                    let volume_preset = match volume {
                        SoundVolume::Minimum => sound::VolumePreset::Minimum,
                        SoundVolume::Low => sound::VolumePreset::Low,
                        SoundVolume::High => sound::VolumePreset::High,
                    };
                    let (group, spatial) = match (cue, position) {
                        (SoundCue::KnifeSlash, Some(position)) => (
                            sound::SoundGroup::Sfx,
                            sound::SpatialMode::Spatial {
                                position: MapCoordF32::new(position[0], position[1]),
                            },
                        ),
                        (SoundCue::Coin, None) => {
                            (sound::SoundGroup::Ui, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::LuggageDrop, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::StartDefenseFanfare, None) => {
                            (sound::SoundGroup::Ui, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::Fail, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::MonsterFootstep, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::PaperCrumpling, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::RedLaserShot, Some(position))
                        | (SoundCue::Whoop, Some(position))
                        | (SoundCue::Wind, Some(position))
                        | (SoundCue::Flamethrower, Some(position))
                        | (SoundCue::SmokeBomb, Some(position)) => (
                            sound::SoundGroup::Sfx,
                            sound::SpatialMode::Spatial {
                                position: MapCoordF32::new(position[0], position[1]),
                            },
                        ),
                        _ => unreachable!("sound cue position mismatch"),
                    };
                    let params =
                        sound::EmitSoundParams::one_shot(asset, group, volume_preset, spatial);
                    let params = match max_duration_ms {
                        Some(max_duration_ms) => {
                            params.with_max_duration(Duration::from_millis(max_duration_ms))
                        }
                        None => params,
                    };
                    crate::sound::emit_sound(params);
                }
                PresentationEvent::PlaySoundCueDelayed {
                    cue,
                    position,
                    volume,
                    delay_ms,
                } => {
                    let asset = match cue {
                        SoundCue::KnifeSlash => sound::deterministic_knife_slash(),
                        SoundCue::Coin => sound::random_coin_sounds(),
                        SoundCue::LuggageDrop => sound::random_luggage_drop(),
                        SoundCue::StartDefenseFanfare => sound::random_trumpet_fanfares(),
                        SoundCue::Pickaxe => sound::random_pickaxe(),
                        SoundCue::Fail => sound::random_fail(),
                        SoundCue::MonsterFootstep => sound::random_cloth_footstep(),
                        SoundCue::PaperCrumpling => sound::random_paper_crumpling(),
                        SoundCue::RedLaserShot => sound::deterministic_red_laser_shot(),
                        SoundCue::Whoop => sound::deterministic_whoop(),
                        SoundCue::Wind => sound::deterministic_wind(),
                        SoundCue::Flamethrower => sound::deterministic_flamethrower(),
                        SoundCue::SmokeBomb => sound::deterministic_smoke_bomb(),
                    };
                    let volume_preset = match volume {
                        SoundVolume::Minimum => sound::VolumePreset::Minimum,
                        SoundVolume::Low => sound::VolumePreset::Low,
                        SoundVolume::High => sound::VolumePreset::High,
                    };
                    let (group, spatial) = match (cue, position) {
                        (SoundCue::KnifeSlash, Some(position)) => (
                            sound::SoundGroup::Sfx,
                            sound::SpatialMode::Spatial {
                                position: MapCoordF32::new(position[0], position[1]),
                            },
                        ),
                        (SoundCue::Coin, None) => {
                            (sound::SoundGroup::Ui, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::LuggageDrop, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::StartDefenseFanfare, None) => {
                            (sound::SoundGroup::Ui, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::Pickaxe, None) | (SoundCue::Fail, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::MonsterFootstep, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::PaperCrumpling, None) => {
                            (sound::SoundGroup::Sfx, sound::SpatialMode::NonSpatial)
                        }
                        (SoundCue::RedLaserShot, Some(position))
                        | (SoundCue::Whoop, Some(position))
                        | (SoundCue::Wind, Some(position))
                        | (SoundCue::Flamethrower, Some(position))
                        | (SoundCue::SmokeBomb, Some(position)) => (
                            sound::SoundGroup::Sfx,
                            sound::SpatialMode::Spatial {
                                position: MapCoordF32::new(position[0], position[1]),
                            },
                        ),
                        _ => unreachable!("sound cue position mismatch"),
                    };
                    crate::sound::emit_sound_after_at(
                        sound::EmitSoundParams::one_shot(asset, group, volume_preset, spatial),
                        Duration::from_millis(delay_ms),
                        presentation_instant,
                    );
                }
                PresentationEvent::PlayCardDrawSounds { card_count } => {
                    sound::play_card_draw_sounds(card_count);
                }
                PresentationEvent::SaveDebugSnapshot =>
                {
                    #[cfg(feature = "debug-tools")]
                    if !self.is_headless() {
                        crate::game_state::debug_tools::state_snapshot::save_snapshot_from_state(
                            self,
                        );
                    }
                }
                PresentationEvent::SpawnProjectileTrail {
                    trail,
                    start_xy,
                    end_xy,
                    count,
                } => {
                    let start_xy = MapCoordF32::new(start_xy[0], start_xy[1]);
                    let end_xy = MapCoordF32::new(end_xy[0], end_xy[1]);
                    match trail {
                        ProjectileTrail::Burning => {
                            field_particle::emitter::spawn_burning_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::Sparkle => {
                            field_particle::emitter::spawn_sparkle_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::WindCurve => {
                            field_particle::emitter::spawn_wind_curve_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::Heart => {
                            field_particle::emitter::spawn_heart_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::LightningSparkle => {
                            field_particle::emitter::spawn_lightning_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                            field_particle::emitter::spawn_sparkle_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::None => {}
                    }
                }
                PresentationEvent::SpawnProjectileHitEffect(hit_effect, impact_xy) => {
                    let impact_xy = MapCoordF32::new(impact_xy[0], impact_xy[1]);
                    use crate::game_state::attack::ProjectileHitEffect;
                    match hit_effect {
                        ProjectileHitEffect::CardBurst => {
                            field_particle::emitter::spawn_card_burst(impact_xy, presentation_now);
                        }
                        ProjectileHitEffect::SparkleBurst => {
                            field_particle::emitter::spawn_sparkle_burst(
                                impact_xy,
                                presentation_now,
                            );
                        }
                        ProjectileHitEffect::HeartBurst => {
                            field_particle::emitter::spawn_heart_burst(impact_xy, presentation_now);
                        }
                        ProjectileHitEffect::TrashBounce => {
                            // Trash bounce is handled as direct projectile activity elsewhere.
                        }
                    }
                }
                PresentationEvent::SpawnLaserBeam(start_xy, end_xy) => {
                    field_particle::emitter::spawn_laser_beam(start_xy, end_xy, presentation_now);
                }
                PresentationEvent::SpawnTowerRemoveDustBurst(center_xy) => {
                    field_particle::emitter::spawn_tower_remove_dust_burst(
                        center_xy,
                        presentation_now,
                    );
                }
                PresentationEvent::SyncProjectileTrailState {
                    projectile_id,
                    trail,
                    start_xy,
                    end_xy,
                    moved_distance,
                    dt_secs,
                } => {
                    let start_xy = MapCoordF32::new(start_xy[0], start_xy[1]);
                    let end_xy = MapCoordF32::new(end_xy[0], end_xy[1]);
                    active_trail_sound_projectiles.insert(projectile_id);
                    let mut effect_states = PROJECTILE_TRAIL_EFFECT_STATE.lock().unwrap();
                    let state = effect_states.entry(projectile_id).or_default();

                    state.trail_distance_remainder += moved_distance;
                    let spawn_distance = match trail {
                        ProjectileTrail::None => None,
                        ProjectileTrail::Burning => {
                            Some(field_particle::emitter::BURNING_TRAIL_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::Sparkle => {
                            Some(field_particle::emitter::SPARKLE_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::WindCurve => {
                            Some(field_particle::emitter::WIND_CURVE_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::Heart => {
                            Some(field_particle::emitter::HEART_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::LightningSparkle => {
                            Some(field_particle::emitter::LIGHTNING_TRAIL_SPAWN_DISTANCE)
                        }
                    };

                    if let Some(spawn_distance) = spawn_distance {
                        let spawn_count =
                            (state.trail_distance_remainder / spawn_distance).floor() as usize;
                        if spawn_count > 0 {
                            state.trail_distance_remainder -= spawn_count as f32 * spawn_distance;
                            match trail {
                                ProjectileTrail::Burning => {
                                    field_particle::emitter::spawn_burning_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::Sparkle => {
                                    field_particle::emitter::spawn_sparkle_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::WindCurve => {
                                    field_particle::emitter::spawn_wind_curve_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::Heart => {
                                    field_particle::emitter::spawn_heart_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::LightningSparkle => {
                                    field_particle::emitter::spawn_lightning_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                    field_particle::emitter::spawn_sparkle_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::None => {}
                            }
                        }
                    }

                    state.whoosh_cooldown_secs -= dt_secs;
                    if state.whoosh_cooldown_secs <= 0.0 {
                        crate::sound::emit_sound(sound::EmitSoundParams::one_shot(
                            sound::random_whoosh(),
                            sound::SoundGroup::Sfx,
                            sound::VolumePreset::Minimum,
                            sound::SpatialMode::Spatial { position: end_xy },
                        ));
                        state.whoosh_cooldown_secs = rand::thread_rng().gen_range(
                            PROJECTILE_WHOOSH_INTERVAL_MIN_SECS
                                ..=PROJECTILE_WHOOSH_INTERVAL_MAX_SECS,
                        );
                    }

                    let existing_entry = active_projectile_sound_ids.get_mut(&projectile_id);
                    match trail {
                        ProjectileTrail::Burning => {
                            let sound_id = match existing_entry {
                                Some((existing_trail, sound_id))
                                    if *existing_trail == ProjectileTrail::Burning =>
                                {
                                    crate::sound::update_sound_position(*sound_id, end_xy);
                                    *sound_id
                                }
                                Some((existing_trail, sound_id)) => {
                                    crate::sound::stop_sound(*sound_id);
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_crackling_fire(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let new_sound_id = crate::sound::emit_sound(params);
                                    *existing_trail = ProjectileTrail::Burning;
                                    *sound_id = new_sound_id;
                                    new_sound_id
                                }
                                None => {
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_crackling_fire(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let sound_id = crate::sound::emit_sound(params);
                                    active_projectile_sound_ids.insert(
                                        projectile_id,
                                        (ProjectileTrail::Burning, sound_id),
                                    );
                                    sound_id
                                }
                            };
                            let _ = sound_id;
                        }
                        ProjectileTrail::Sparkle => {
                            let sound_id = match existing_entry {
                                Some((existing_trail, sound_id))
                                    if *existing_trail == ProjectileTrail::Sparkle =>
                                {
                                    crate::sound::update_sound_position(*sound_id, end_xy);
                                    *sound_id
                                }
                                Some((existing_trail, sound_id)) => {
                                    crate::sound::stop_sound(*sound_id);
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_shining_ringing(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let new_sound_id = crate::sound::emit_sound(params);
                                    *existing_trail = ProjectileTrail::Sparkle;
                                    *sound_id = new_sound_id;
                                    new_sound_id
                                }
                                None => {
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_shining_ringing(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let sound_id = crate::sound::emit_sound(params);
                                    active_projectile_sound_ids.insert(
                                        projectile_id,
                                        (ProjectileTrail::Sparkle, sound_id),
                                    );
                                    sound_id
                                }
                            };
                            let _ = sound_id;
                        }
                        ProjectileTrail::WindCurve => {
                            let sound_id = match existing_entry {
                                Some((existing_trail, sound_id))
                                    if *existing_trail == ProjectileTrail::WindCurve =>
                                {
                                    crate::sound::update_sound_position(*sound_id, end_xy);
                                    *sound_id
                                }
                                Some((existing_trail, sound_id)) => {
                                    crate::sound::stop_sound(*sound_id);
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_wind(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let new_sound_id = crate::sound::emit_sound(params);
                                    *existing_trail = ProjectileTrail::WindCurve;
                                    *sound_id = new_sound_id;
                                    new_sound_id
                                }
                                None => {
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_wind(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let sound_id = crate::sound::emit_sound(params);
                                    active_projectile_sound_ids.insert(
                                        projectile_id,
                                        (ProjectileTrail::WindCurve, sound_id),
                                    );
                                    sound_id
                                }
                            };
                            let _ = sound_id;
                        }
                        ProjectileTrail::Heart
                        | ProjectileTrail::LightningSparkle
                        | ProjectileTrail::None => {
                            if let Some((_, sound_id)) =
                                active_projectile_sound_ids.remove(&projectile_id)
                            {
                                crate::sound::stop_sound(sound_id);
                            }
                        }
                    }
                }
            }
        }

        let stale_keys: Vec<AttackId> = active_projectile_sound_ids
            .keys()
            .filter(|key| !active_trail_sound_projectiles.contains(key))
            .cloned()
            .collect();
        for stale_key in stale_keys {
            if let Some((_, sound_id)) = active_projectile_sound_ids.remove(&stale_key) {
                crate::sound::stop_sound(sound_id);
            }
        }
    }
}

#[derive(Clone, Copy, State)]
pub struct FloorTile {
    pub coord: MapCoord,
}
impl Component for &FloorTile {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(simple_rect(
            TILE_PX_SIZE,
            palette::OUTLINE,
            1.px(),
            Color::TRANSPARENT,
        ));
    }
}

static GAME_STATE_ATOM: Atom<crate::headed_game::HeadedGame> = Atom::uninitialized();

fn create_initial_game_state() -> GameState {
    create_game_state_with_seed(rand::thread_rng().r#gen())
}

pub fn create_game_state_with_seed(seed: u64) -> GameState {
    create_game_state_with_config(Arc::new(GameConfig::default_config()), seed)
}

pub(crate) fn create_game_state_with_config(config: Arc<GameConfig>, seed: u64) -> GameState {
    let raw_core = td_core::CoreState::new_initial(config.to_core_state(), seed);
    let mut game_state = GameState {
        raw_core: raw_core::HeadedRawCoreState::new(raw_core.clone()),
        #[cfg(any(test, feature = "debug-tools"))]
        presentation_projection:
            presentation_projection::LegacyProjectionCodec::from_td_core_state(
                raw_core.clone(),
                None,
            )
            .expect("initial raw core must be restorable in compatibility fixture"),
        presentation_metadata: Default::default(),
        monster_animation_runtime: Vec::new(),
        presentation_hand: Hand::from_core_state(raw_core.hand().clone(), None)
            .expect("initial raw hand must be restorable"),
        presentation_flow: GameFlow::from_core_state(raw_core.flow().clone(), None)
            .expect("initial raw flow must be restorable"),
        presentation_inventory: Default::default(),
        presentation_upgrades: Default::default(),
        presentation_deck: Default::default(),
        locale: crate::l10n::Locale::KOREAN,
        pending_history_events: Vec::new(),
        pending_card_service_notifications: Vec::new(),
        pending_modals: modal::OpenedModals::default(),
        pending_presentation_events: PresentationEventQueue::default(),
        pending_discoveries: Default::default(),
        headless: false,
    };

    game_state.apply_compatibility_action(CompatibilityAction::GameStart);
    game_state.reseed_presentation_collections();
    game_state
}

pub(crate) fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, crate::headed_game::HeadedGame> {
    ctx.init_atom(&GAME_STATE_ATOM, || {
        crate::headed_game::HeadedGame::new(create_initial_game_state())
    })
    .0
}

pub(crate) fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, crate::headed_game::HeadedGame> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(move |headed_game| {
        headed_game.state.set_locale(headed_game.locale);
        f(&mut headed_game.state);
        #[cfg(any(test, feature = "debug-tools"))]
        headed_game.state.sync_raw_core_from_projection();
        headed_game.flush_pending_action_effects();
        headed_game.persist_discoveries_if_dirty();
    });
}

pub(crate) fn mutate_headed_game(
    f: impl FnOnce(&mut crate::headed_game::HeadedGame) + Send + Sync + 'static,
) {
    GAME_STATE_ATOM.mutate(move |headed_game| {
        headed_game.state.set_locale(headed_game.locale);
        f(headed_game);
        headed_game.flush_pending_action_effects();
        headed_game.persist_discoveries_if_dirty();
    });
}

pub(crate) fn dispatch_player_command(command: PlayerCommand) {
    let presentation_instant = PresentationInstant::capture();
    mutate_headed_game(move |game_state| {
        if game_state
            .apply_player_command_at(command.into(), presentation_instant)
            .is_ok()
        {
            game_state.consume_core_events(presentation_instant);
        }
    });
}

pub(crate) fn dispatch_use_inventory_item(item_id: crate::game_state::item::ItemId) {
    let presentation_instant = PresentationInstant::capture();
    mutate_headed_game(move |game_state| {
        let Some(item_index) = game_state
            .state()
            .presentation_inventory
            .active_item_index(item_id)
        else {
            return;
        };
        if game_state
            .apply_player_command_at(
                PlayerCommand::UseInventoryItem { item_index }.into(),
                presentation_instant,
            )
            .is_ok()
        {
            game_state.consume_core_events(presentation_instant);
        }
    });
}

pub(crate) fn dispatch_shop_purchase(slot_id: crate::shop::ShopSlotId) {
    let presentation_instant = PresentationInstant::capture();
    mutate_headed_game(move |game_state| {
        let flow = game_state.state().presentation_flow_snapshot();
        let Some(slot_index) = (match &flow {
            GameFlow::Shopping(flow) => flow
                .shop
                .slots
                .iter()
                .filter(|slot| slot.exit_animation.is_none())
                .position(|slot| slot.id == slot_id),
            _ => None,
        }) else {
            return;
        };
        if game_state
            .apply_player_command_at(
                PlayerCommand::PurchaseShopItem { slot_index }.into(),
                presentation_instant,
            )
            .is_ok()
        {
            game_state.consume_core_events(presentation_instant);
        }
    });
}

pub(crate) fn dispatch_remove_tower(tower_id: TowerId) {
    let presentation_instant = PresentationInstant::capture();
    mutate_headed_game(move |game_state| {
        let tower_removed = game_state
            .apply_player_command_at(
                PlayerCommand::RemoveTower {
                    tower_id: tower_id.raw(),
                }
                .into(),
                presentation_instant,
            )
            .is_ok();
        if tower_removed {
            game_state.push_presentation_event(PresentationEvent::PlaySoundCue {
                cue: SoundCue::PaperCrumpling,
                position: None,
                volume: SoundVolume::High,
                max_duration_ms: None,
            });
        }
    });
}

pub fn set_modal(modal: Option<UserModal>) {
    mutate_headed_game(|game_state| {
        game_state.opened_modals_mut().user = modal;
    });
}

pub fn set_overlay_modal(modal: Option<modal::SystemModal>) {
    mutate_headed_game(|game_state| {
        game_state.opened_modals_mut().system = modal;
    });
}

pub fn restart_game() {
    mutate_headed_game(|headed_game| {
        let previous_discoveries = headed_game.discovery.clone();
        headed_game.state = create_initial_game_state();
        headed_game.play_history = crate::game_state::play_history::PlayHistory::new();
        headed_game
            .state
            .preserve_discoveries_from(&previous_discoveries);
    });
}

impl GameState {
    /// Create a deep-ish clone of the current state for debug snapshotting.
    /// Particle systems are cleared and opened modal is dropped to avoid UI leakage.
    pub fn clone_for_debug(&self) -> GameState {
        let raw_core = self.raw_core.clone();
        GameState {
            #[cfg(any(test, feature = "debug-tools"))]
            presentation_projection:
                presentation_projection::LegacyProjectionCodec::from_td_core_state(
                    raw_core.state().clone(),
                    Some(self.presentation_projection()),
                )
                .expect("raw debug snapshot must be restorable in compatibility fixture"),
            raw_core,
            presentation_metadata: self.presentation_metadata.clone(),
            monster_animation_runtime: self.monster_animation_runtime.clone(),
            presentation_hand: self.presentation_hand.clone(),
            presentation_flow: self.presentation_flow.clone(),
            presentation_inventory: self.presentation_inventory.clone(),
            presentation_upgrades: self.presentation_upgrades.clone(),
            presentation_deck: self.presentation_deck.clone(),
            locale: self.locale,
            pending_history_events: self.pending_history_events.clone(),
            pending_modals: modal::OpenedModals::default(),
            pending_presentation_events: self.pending_presentation_events.clone(),
            pending_card_service_notifications: self.pending_card_service_notifications.clone(),
            headless: self.headless,
            pending_discoveries: self.pending_discoveries.clone(),
        }
    }

    /// 현재 스테이지의 클리어율을 계산합니다.
    /// 각 스테이지는 2% (100/50), 스테이지 내에서는 누적 처리 체력 / 총 체력으로 계산합니다.
    /// 처리 체력은 피해와 기지 도달 시점에만 증가하므로 몬스터 회복으로 감소하지 않습니다.
    pub fn calculate_clear_rate(&self) -> ClearRate {
        ClearRate::from_ratio(FixedRatio::from_raw(self.raw_core.clear_rate_raw()))
    }

    /// 특정 스테이지의 총 몬스터 체력을 계산합니다.
    pub fn calculate_stage_total_hp(
        stage: usize,
        config: &GameConfig,
        stage_modifiers: &StageModifiers,
    ) -> Health {
        let health_multipliers = stage_modifiers.enemy_health_multipliers();
        let (template_queue, _) = monster_spawn::monster_template_queue_table(stage, config);
        template_queue
            .iter()
            .map(|t| t.max_hp.scaled_by_product(health_multipliers))
            .fold(Health::ZERO, Health::saturating_add)
    }
}

#[cfg(any(test, feature = "debug-tools"))]
impl std::ops::Deref for GameState {
    type Target = presentation_projection::LegacyProjectionCodec;

    fn deref(&self) -> &Self::Target {
        &self.presentation_projection
    }
}

#[cfg(any(test, feature = "debug-tools"))]
impl std::ops::DerefMut for GameState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.presentation_projection
    }
}

pub fn is_boss_stage(stage: usize) -> bool {
    stage.is_multiple_of(5) || (46..=49).contains(&stage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shopping_allows_shop_panel() {
        let gs = create_initial_game_state();
        assert!(gs.can_open_shop_panel());
    }

    #[test]
    fn boss_stage_logic_is_every_fifth_stage_with_final_45_to_50() {
        for stage in [5, 10, 15, 20, 25, 30, 35, 40, 45, 46, 47, 48, 49, 50] {
            assert!(is_boss_stage(stage), "expected stage {} to be boss", stage);
        }
        assert!(!is_boss_stage(51));
    }

    #[test]
    fn headed_read_projection_is_shared_by_hash_and_render_snapshot() {
        let game_state = create_game_state_with_seed(0x5eed);
        let projected = game_state.authoritative_core_state();

        assert_eq!(
            td_core::authoritative_hash(&projected),
            game_state.authoritative_hash()
        );
        assert_eq!(
            projected.render_snapshot(),
            game_state.raw_render_snapshot()
        );
    }

    #[test]
    fn headed_placing_tower_cancel_deselects_raw_hand_slot() {
        let mut game_state = create_game_state_with_seed(7);
        let mut raw = game_state.raw_core.state().clone();
        raw.edit_snapshot(|parts| {
            parts.flow = td_core::GameFlowState::PlacingTower;
            parts.hand.slots = vec![td_core::HandSlotState {
                id: 1,
                item: td_core::HandItemState::Tower(
                    crate::game_state::tower::TowerTemplate::new(
                        crate::game_state::tower::TowerKind::High,
                        crate::card::Suit::Spades,
                        crate::card::Rank::Ace,
                    )
                    .to_core_state(),
                ),
                selected: false,
            }];
        })
        .expect("placing tower fixture must be valid");
        game_state
            .restore_raw_core_projection(raw)
            .expect("placing tower fixture must be restorable");

        let hand_slot_id = game_state.hand.active_slot_ids()[0];
        assert!(game_state.select_placing_tower_slot(hand_slot_id));
        assert!(game_state.raw_core.hand().slots[0].selected);
        assert!(game_state.deselect_placing_tower_slot(0));
        assert!(!game_state.raw_core.hand().slots[0].selected);
        assert!(game_state.hand.selected_slot_ids().is_empty());
    }

    #[test]
    fn headed_selecting_tower_card_toggles_raw_hand_slot() {
        let mut game_state = create_game_state_with_seed(7);
        let mut raw = game_state.raw_core.state().clone();
        raw.edit_snapshot(|parts| {
            parts.flow = td_core::GameFlowState::SelectingTower;
            parts.hand.slots[0].selected = false;
        })
        .expect("selecting tower fixture must be valid");
        assert!(matches!(
            raw.hand().slots[0].item,
            td_core::HandItemState::Card(_)
        ));
        game_state
            .restore_raw_core_projection(raw)
            .expect("selecting tower fixture must be restorable");

        let hand_slot_id = game_state.hand.active_slot_ids()[0];
        assert_eq!(
            game_state.toggle_selecting_tower_card(hand_slot_id),
            Some(true)
        );
        assert!(game_state.raw_core.hand().slots[0].selected);
        assert_eq!(
            game_state.toggle_selecting_tower_card(hand_slot_id),
            Some(false)
        );
        assert!(!game_state.raw_core.hand().slots[0].selected);
    }

    #[test]
    fn authoritative_clone_replay_is_bit_exact() {
        let mut left = create_game_state_with_seed(0x5eed);
        let mut right = left.clone_for_debug();
        let apply_commands = |game_state: &mut GameState| {
            let mut raw = game_state.raw_core.state().clone();
            raw.apply_player_damage_raw(7_125);
            game_state
                .restore_raw_core_projection(raw)
                .expect("raw damage must be restorable in headed adapter");
            game_state.apply_compatibility_action(CompatibilityAction::GainShield(
                Shield::from_raw(2_500),
            ));
            let mut raw = game_state.raw_core.state().clone();
            raw.apply_player_damage_raw(3_250);
            game_state
                .restore_raw_core_projection(raw)
                .expect("raw damage must be restorable in headed adapter");
            game_state
                .apply_compatibility_action(CompatibilityAction::Heal(Health::from_raw(1_125)));
            crate::game_state::effect::run_effect(
                game_state,
                &crate::game_state::effect::Effect::IncreaseEnemyHealthPercent {
                    percentage: FixedRatio::from_integer(20),
                },
            );
            crate::game_state::effect::run_effect(
                game_state,
                &crate::game_state::effect::Effect::DecreaseIncomingDamage {
                    multiplier: FixedRatio::from_raw(750_001),
                },
            );
        };
        apply_commands(&mut left);
        apply_commands(&mut right);
        for _ in 0..120 {
            tick::advance_simulation_tick(&mut left);
            tick::advance_simulation_tick(&mut right);
        }

        assert_eq!(left.hp, right.hp);
        assert_eq!(left.shield, right.shield);
        assert_eq!(left.max_hp(), right.max_hp());
        assert_eq!(left.calculate_clear_rate(), right.calculate_clear_rate());
        assert_eq!(left.stage, right.stage);
        assert_eq!(left.left_dice, right.left_dice);
        assert_eq!(left.gold, right.gold);
        assert_eq!(left.sim_tick, right.sim_tick);
        assert_eq!(
            left.stage_modifiers.get_enemy_health_multiplier(),
            right.stage_modifiers.get_enemy_health_multiplier()
        );
        assert_eq!(
            left.stage_modifiers.get_damage_reduction_multiplier(),
            right.stage_modifiers.get_damage_reduction_multiplier()
        );
        assert_eq!(left.rng.seed, right.rng.seed);
        assert_eq!(
            left.rng.shop.generation_sequence,
            right.rng.shop.generation_sequence
        );
        assert_eq!(left.monsters.len(), right.monsters.len());
        for (left_monster, right_monster) in left.monsters.iter().zip(&right.monsters) {
            assert_eq!(left_monster.hp, right_monster.hp);
            assert_eq!(left_monster.max_hp, right_monster.max_hp);
            assert_eq!(left_monster.damage, right_monster.damage);
            assert_eq!(
                left_monster.stage_progress_counted,
                right_monster.stage_progress_counted
            );
        }
    }

    #[test]
    fn same_seed_repeats_authoritative_random_choices() {
        let mut left = create_game_state_with_seed(0xA11C_E123);
        let mut right = create_game_state_with_seed(0xA11C_E123);

        assert_eq!(left.deck.draw_pile(), right.deck.draw_pile());

        left.apply_compatibility_action(CompatibilityAction::CardReroll);
        right.apply_compatibility_action(CompatibilityAction::CardReroll);

        let active_cards = |game_state: &GameState| {
            game_state
                .hand
                .active_slot_ids()
                .into_iter()
                .filter_map(|slot_id| {
                    game_state
                        .hand
                        .get_item(slot_id)
                        .and_then(HandItem::as_card)
                        .copied()
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(active_cards(&left), active_cards(&right));

        let left_reward = upgrade::generate_boss_reward_upgrade(&mut left);
        let right_reward = upgrade::generate_boss_reward_upgrade(&mut right);
        assert_eq!(format!("{left_reward:?}"), format!("{right_reward:?}"));
    }

    #[test]
    fn game_metrics_codec_preserves_values_and_authoritative_hash() {
        let mut original = create_game_state_with_seed(0x00E7_A1C5);
        original.user_status_effects.push(
            user_status_effect::UserStatusEffect {
                kind: user_status_effect::UserStatusEffectKind::DamageReduction {
                    damage_multiply: FixedRatio::from_raw(750_000),
                },
                end_at: SimTick::from_ticks(90),
            }
            .to_core_status_effect(),
        );
        let expected_metrics = GameMetrics {
            total_gold_earned: 101,
            total_gold_spent: 37,
            current_consecutive_perfect_clears: 2,
            max_consecutive_perfect_clears: 5,
            tower_damage_stats: vec![TowerDamageStats {
                tower_id: TowerId::from_raw(19),
                tower_kind: crate::game_state::tower::TowerKind::High,
                rank: Some(Rank::Queen),
                suit: Some(Suit::Hearts),
                total_damage: Damage::from_raw(12_345),
            }],
            total_rerolled_count: 11,
            total_escaped_hp: Health::from_raw(2_500),
            total_player_damage: Health::from_raw(1_750),
            stage_damage: vec![(3, Health::from_raw(400)), (7, Health::from_raw(900))],
        };
        original.metrics = expected_metrics.to_core_metrics();
        let raw_metrics = expected_metrics.to_core_metrics();
        assert_eq!(
            GameMetrics::from_core_metrics(raw_metrics.clone()).to_core_metrics(),
            raw_metrics
        );
        let expected_hash = original.authoritative_hash();

        let bytes = namui::bincode::encode_to_vec(&original, namui::bincode::config::standard())
            .expect("GameState should serialize");
        let (restored, consumed): (GameState, usize) =
            namui::bincode::decode_from_slice(&bytes, namui::bincode::config::standard())
                .expect("GameState should deserialize");

        assert_eq!(consumed, bytes.len());
        let restored_metrics = GameMetrics::from_core_metrics(restored.metrics.clone());
        assert_eq!(
            restored_metrics.total_gold_earned,
            expected_metrics.total_gold_earned
        );
        assert_eq!(
            restored_metrics.total_gold_spent,
            expected_metrics.total_gold_spent
        );
        assert_eq!(
            restored_metrics.current_consecutive_perfect_clears,
            expected_metrics.current_consecutive_perfect_clears
        );
        assert_eq!(
            restored_metrics.max_consecutive_perfect_clears,
            expected_metrics.max_consecutive_perfect_clears
        );
        assert_eq!(restored_metrics.tower_damage_stats.len(), 1);
        assert_eq!(
            restored_metrics.tower_damage_stats[0].tower_id,
            TowerId::from_raw(19)
        );
        assert_eq!(
            restored_metrics.tower_damage_stats[0].total_damage,
            Damage::from_raw(12_345)
        );
        assert_eq!(
            restored_metrics.total_rerolled_count,
            expected_metrics.total_rerolled_count
        );
        assert_eq!(
            restored_metrics.total_escaped_hp,
            expected_metrics.total_escaped_hp
        );
        assert_eq!(
            restored_metrics.total_player_damage,
            expected_metrics.total_player_damage
        );
        assert_eq!(restored_metrics.stage_damage, expected_metrics.stage_damage);
        assert_eq!(restored.authoritative_hash(), expected_hash);
    }

    #[test]
    fn pending_card_service_identity_codec_preserves_hash() {
        let mut original = create_game_state_with_seed(0xCA7D);
        original.pending_card_service_kind = Some(7);
        let expected_hash = original.authoritative_hash();

        let bytes = namui::bincode::encode_to_vec(&original, namui::bincode::config::standard())
            .expect("GameState should serialize pending card service state");
        let (restored, consumed): (GameState, usize) =
            namui::bincode::decode_from_slice(&bytes, namui::bincode::config::standard())
                .expect("GameState should deserialize pending card service state");

        assert_eq!(consumed, bytes.len());
        assert_eq!(restored.pending_card_service_kind, Some(7));
        assert_eq!(
            restored.presentation_projection().to_td_core_state(),
            original.presentation_projection().to_td_core_state()
        );
        assert_eq!(restored.authoritative_hash(), expected_hash);
    }

    #[test]
    fn monster_collection_codec_fixture_preserves_snapshot_and_hash() {
        let mut original = create_game_state_with_seed(0xBEEF_2026);
        original.apply_compatibility_action(CompatibilityAction::StartDefense);
        original.step_raw_simulation();
        assert!(!original.monsters.is_empty());

        let expected_snapshots = original.presentation_projection().monster_snapshots();
        let expected_hash = original.authoritative_hash();
        let bytes = namui::bincode::encode_to_vec(&original, namui::bincode::config::standard())
            .expect("GameState should serialize with monsters");
        let (restored, consumed): (GameState, usize) =
            namui::bincode::decode_from_slice(&bytes, namui::bincode::config::standard())
                .expect("GameState should deserialize with monsters");

        assert_eq!(consumed, bytes.len());
        assert_eq!(
            restored.presentation_projection().monster_snapshots(),
            expected_snapshots
        );
        assert_eq!(restored.authoritative_hash(), expected_hash);
    }

    #[test]
    fn in_flight_attack_codec_fixture_preserves_raw_state_and_hash() {
        let mut original = create_game_state_with_seed(0xA771_AA77);
        let spatial = crate::game_state::attack::SpatialAttack::new_homing(
            WorldCoord::ZERO,
            crate::game_state::projectile::ProjectileTargetIndicator::from_id(MonsterId::from_raw(
                7,
            )),
            19,
            crate::game_state::projectile::ProjectileKind::Cards00,
            crate::game_state::projectile::ProjectileTrail::None,
            crate::game_state::attack::ProjectileHitEffect::CardBurst,
        );
        original
            .presentation_projection_mut()
            .in_flight_attacks
            .push(
                crate::game_state::attack::InFlightAttack::new_spatial(
                    AttackId::from_raw(77),
                    spatial,
                    Damage::from_integer(42),
                    None,
                )
                .with_on_hit_splashes(vec![crate::card::EngravingSplash {
                    radius: WorldDistance::from_tiles(2),
                    damage_pct: FixedRatio::from_raw(500_000),
                }]),
            );
        let expected_state =
            original.presentation_projection().in_flight_attacks[0].to_core_state();
        let expected_hash = original.authoritative_hash();
        let bytes = namui::bincode::encode_to_vec(&original, namui::bincode::config::standard())
            .expect("GameState should serialize with an in-flight attack");
        let (restored, consumed): (GameState, usize) =
            namui::bincode::decode_from_slice(&bytes, namui::bincode::config::standard())
                .expect("GameState should deserialize with an in-flight attack");

        assert_eq!(consumed, bytes.len());
        assert_eq!(
            restored.presentation_projection().in_flight_attacks.len(),
            1
        );
        assert_eq!(
            restored.presentation_projection().in_flight_attacks[0].to_core_state(),
            expected_state
        );
        assert_eq!(restored.authoritative_hash(), expected_hash);
    }

    #[test]
    fn entity_collection_snapshot_restore_preserves_legacy_collection_bytes() {
        let mut original = create_game_state_with_seed(0xC011_EC71);
        original.apply_compatibility_action(CompatibilityAction::StartDefense);
        original.step_raw_simulation();
        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        let mut tower = crate::game_state::tower::Tower::new(
            &template,
            MapCoord::new(2, 2),
            original.sim_tick(),
        );
        tower.assign_id(TowerId::from_raw(100));
        original.towers.place_tower(tower);
        original.sync_raw_core_from_projection();

        let cloned = original.clone_for_debug();
        let original_monster_bytes =
            namui::bincode::encode_to_vec(&original.monsters, namui::bincode::config::standard())
                .expect("monster collection encoding");
        let cloned_monster_bytes =
            namui::bincode::encode_to_vec(&cloned.monsters, namui::bincode::config::standard())
                .expect("restored monster collection encoding");
        let original_tower_bytes =
            namui::bincode::encode_to_vec(&original.towers, namui::bincode::config::standard())
                .expect("tower collection encoding");
        let cloned_tower_bytes =
            namui::bincode::encode_to_vec(&cloned.towers, namui::bincode::config::standard())
                .expect("restored tower collection encoding");

        assert_eq!(original_monster_bytes, cloned_monster_bytes);
        assert_eq!(original_tower_bytes, cloned_tower_bytes);
    }

    #[test]
    fn entity_ids_are_sequential_and_owned_by_game_state() {
        let mut game_state = create_game_state_with_seed(0x1D);
        game_state.apply_compatibility_action(CompatibilityAction::StartDefense);

        let queued_count = game_state.monster_spawn_state.monster_queue.len();
        assert!(queued_count > 0);
        assert_eq!(
            game_state.monster_spawn_state.monster_queue[0].id(),
            MonsterId::from_raw(1)
        );

        game_state.step_raw_simulation();
        assert_eq!(game_state.monsters[0].id(), MonsterId::from_raw(1));

        let mut expected_state = game_state.clone_for_debug();
        let tower_id = expected_state.allocate_tower_id();
        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        game_state.apply_compatibility_action(CompatibilityAction::PlaceTower(
            Box::new(crate::game_state::tower::Tower::new(
                &template,
                MapCoord::new(0, 0),
                SimTick::ZERO,
            )),
            None,
        ));

        assert_eq!(game_state.towers.iter().next().unwrap().id(), tower_id);

        let cloned = game_state.clone_for_debug();
        assert_eq!(cloned.next_entity_id, game_state.next_entity_id);
    }

    #[test]
    fn clear_rate_is_monotonic_and_bounded() {
        let mut game_state = create_game_state_with_seed(0xc1ea);
        let defense = flow::DefenseFlow::new(&game_state);
        game_state.flow = GameFlow::Defense(defense);
        game_state.sync_raw_core_from_projection();
        let mut previous = game_state.calculate_clear_rate();
        for processed in [1, 7, 19, 37, 61] {
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp = Health::from_integer(processed);
            }
            game_state.sync_raw_core_from_projection();
            let current = game_state.calculate_clear_rate();
            assert!(current >= previous);
            assert!(current <= ClearRate::FULL);
            previous = current;
        }
    }

    #[test]
    fn representative_stage_hp_matches_integer_migration_baseline() {
        let config = GameConfig::default_config();
        let modifiers = StageModifiers::new();
        for (stage, expected_raw) in [(1, 338_285), (25, 60_058_670), (50, 179_198_724_000)] {
            assert_eq!(
                GameState::calculate_stage_total_hp(stage, &config, &modifiers).raw(),
                expected_raw,
                "stage {stage} total HP changed"
            );
        }
    }
}
