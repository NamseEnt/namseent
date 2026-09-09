pub mod catalog;
pub mod combat_number;
pub mod deterministic_rng;
mod events;
pub use catalog::*;
pub mod game_state;
pub mod rarity;
mod route;
pub mod time;
pub mod world;

pub use combat_number::{
    AMOUNT_SCALE, ClearRate, Damage, DamageDelta, FixedRatio, Health, HealthDelta, RatioProduct,
    Shield,
};
pub use deterministic_rng::{
    RNG_ALGORITHM_VERSION, derive_seed, domain, rng_for, shuffle, stable_key_hash, uniform_index,
};
pub use events::{CoreEvent, CoreEventQueue};
pub use game_state::CoreState;
pub use game_state::EntitySnapshots;
pub use game_state::card_service::{
    CardSelectionFilterState, CardServicePurchaseBlockReason, CardServiceSelectionState,
    CardServiceSelectionStepState, CardState, DeckState, purchase_block_reasons,
    purchase_block_reasons_raw, purchase_is_available, purchase_is_available_raw,
};
pub use game_state::command::{
    ACTION_WIRE_SCHEMA_VERSION, ActionKind, AgentAction, CommandError, CommandOutput,
    CommandReceipt, DecisionPoint, PlayerCommand, RecordedPlayerCommand,
};
pub use game_state::config::{
    GameConfig, GameConfigState, MonsterConfigEntryState, MonsterConfigState, PlayerConfigState,
    StageWaveEntryState, StageWaveState, TowerConfigEntryState, TowerConfigState,
};
pub use game_state::effect::{
    MonsterStatusEffect, MonsterStatusEffectKind, StageModifierTowerCardState,
    StageModifiersObservation, StageModifiersState, TowerStatusEffect, TowerStatusEffectEnd,
    TowerStatusEffectKind, UserStatusEffect, UserStatusEffectKind,
};
pub(crate) use game_state::effect::{
    adjust_incoming_damage, remove_expired_monster_statuses, remove_expired_tower_statuses,
    remove_expired_user_status_effects,
};
pub use game_state::entity_id::EntityIdAllocator;
pub use game_state::flow::{
    DefenseEndOutputState, DefenseEndTransitionState, DefenseFlowState, GameFlowState,
    ShopPurchaseOutput, ShopSlotDataState, ShopSlotState, ShopState,
};
pub use game_state::hand::{HandItemState, HandSlotState, HandState};
pub use game_state::item::codec::{
    decode_item_collection, decode_item_entry, encode_item_collection, encode_item_entry,
};
#[allow(deprecated)]
pub use game_state::item::{
    ITEM_KIND_COUNT, ItemCollection, ItemEntry, ItemUseEffect, ItemUseOutput,
    generate_item_of_rarity_with_rng, generated_item, generated_item_raw, item_rarity,
    item_rarity_raw,
};
pub use game_state::monster::{
    ActivatedMonsterSkill, MonsterDamageResult, MonsterDeathResult, MonsterEscapeResult,
    MonsterSkill, MonsterSkillKind, MonsterSkillTarget, MonsterSkillTemplate, MonsterState,
    RemovedMonster,
};
pub(crate) use game_state::monster::{
    activate_monster_skills, advance_monster_states, apply_monster_damage,
    apply_monster_skill_activations, remove_dead_monster, resolve_monster_escapes,
};
pub use game_state::monster_spawn::MonsterSpawnState;
pub use game_state::observation::{
    CardObservation, CardServiceObservation, DeckObservation, HandItemObservation, HandObservation,
    InventoryObservation, MonsterObservation, Observation, OwnedUpgradeObservation,
    RouteCoordObservation, ShopSlotObservation, TowerObservation, TowerTemplateObservation,
};
pub use game_state::replay::{
    AUTHORITATIVE_HASH_VERSION, CORE_CONFIG_DIGEST_VERSION, CORE_CONFIG_SCHEMA_VERSION,
    CORE_EVENT_DIGEST_DOMAIN, CORE_EVENT_DIGEST_VERSION, CORE_REPLAY_SCHEMA_VERSION,
    CORE_RNG_ALGORITHM_VERSION, CoreReplay, CoreReplayError, POLICY_TRACE_SCHEMA_VERSION,
    PolicyTrace, PolicyTraceStep, ReplayCheckpoint, ReplayDivergence, authoritative_hash,
    event_digest, event_metadata, first_divergence,
};
pub use game_state::reward::{RewardComponents, RewardConfig, StepInfo, StepOutcome, StepReason};
pub use game_state::rng::{
    BagState, ContentBagState, RngState, ShopBagState, ShopGenerationConfig,
};
pub use game_state::session::{
    CoreSession, CoreSnapshot, CoreSnapshotParts, SnapshotValidationError,
};
pub use game_state::shop::max_shop_slot_count;
pub use game_state::stage::{
    MAX_STAGE_COUNT, STAGES_PER_ACT, StageKind, act_for_stage, boss_stage_for_act, is_boss_stage,
    is_normal_stage, is_treasure_stage, stage_in_act, stage_kind, treasure_stage_for_act,
};
pub use game_state::tick::{
    AreaDamageEvent, AttackSourceState, DamageHit, DamageHitResult, DamageSplash,
    InFlightAttackKindState, InFlightAttackState, LaserAttackState, ResolvedAttack,
    SpatialAttackBehaviorState, SpatialAttackState, TimedAttackState,
};
pub(crate) use game_state::tick::{advance_in_flight_attacks_with_events, damage_hit_sort_key};
pub use game_state::tower::RemoveTowerOutput;
pub use game_state::tower::{
    ActivatedTowerSkill, TowerAttackOutput, TowerSkill, TowerSkillKind, TowerSkillTemplate,
    TowerState, TowerTemplateState, rank_is_face,
};
pub(crate) use game_state::tower::{
    activate_tower_skills, advance_tower_cooldowns, apply_tower_skill_activations,
    generate_tower_attacks,
};
#[cfg(test)]
pub(crate) use game_state::upgrade::TestUpgradeWireEntry as UpgradeWireEntry;
pub use game_state::upgrade::codec::{
    decode_upgrade_collection, decode_upgrade_entry, encode_upgrade_collection,
    encode_upgrade_entry,
};
pub use game_state::upgrade::{
    UpgradeAcquireOutput, UpgradeAcquireRecovery, UpgradeCacheState, UpgradeCollection,
    UpgradeEntry, UpgradeEntryIdentityState, generate_boss_reward_option, generated_upgrade,
    generated_upgrade_raw, upgrade_rarity, upgrade_rarity_raw,
};
#[allow(deprecated)]
pub use game_state::{
    CoreProgress, FastTickOutput, GameMetrics, PreCombatOutput, PresentationTickOutput,
    RecordedTickOutput, RenderMonsterSnapshot, RenderSnapshot, RenderSpatialAttackSnapshot,
    RenderTowerSnapshot, TickEventsOutput, TickOutput, TickTransition, TowerDamageStats,
};
use route::multiply_ratio_raw;
pub use route::{
    MoveOnRouteState, RouteState, advance_move_on_route, apply_ratio_product_raw, calculate_routes,
    find_shortest_route, move_on_route_index, move_on_route_is_finished,
    move_on_route_motion_revision, move_on_route_position, move_on_route_progress_raw,
    move_on_route_remainder, move_on_route_velocity_raw, reset_move_on_route,
};
pub use time::{RATIO_SCALE, RatioRaw, SIM_TICKS_PER_SECOND, SimTick, SimTickSpan};

pub use rarity::Rarity;
pub use world::{WorldAcceleration, WorldCoord, WorldDistance, WorldSpeed, WorldVec, integer_sqrt};

pub const OBSERVATION_SCHEMA_VERSION: u32 = 2;

pub const WORLD_UNITS_PER_TILE: i64 = world::WORLD_UNITS_PER_TILE;

/// Map size in tiles (36 x 36), shared with the headed map layout.
pub const MAP_SIZE: [usize; 2] = [36, 36];

/// Monster travel points through the map, shared with the headed route layout.
pub const TRAVEL_POINTS: [[usize; 2]; 7] = [
    [5, 0],
    [5, 17],
    [31, 17],
    [31, 5],
    [18, 5],
    [18, 31],
    [35, 31],
];

pub fn vector_length_raw(vector: [i64; 2]) -> i64 {
    world::WorldVec::new(vector[0], vector[1]).length().raw()
}

fn distance_squared(left: [i64; 2], right: [i64; 2]) -> i128 {
    let dx = i128::from(left[0]) - i128::from(right[0]);
    let dy = i128::from(left[1]) - i128::from(right[1]);
    dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
}

pub fn segment_hits_point(
    start: [i64; 2],
    end: [i64; 2],
    point: [i64; 2],
    radius_raw: i64,
) -> bool {
    world::segment_hits_point(
        world::WorldCoord::new(start[0], start[1]),
        world::WorldCoord::new(end[0], end[1]),
        world::WorldCoord::new(point[0], point[1]),
        world::WorldDistance::from_raw(radius_raw),
    )
}

fn scale_component(component: i64, distance: i64, denominator: i64) -> i64 {
    if denominator <= 0 {
        return 0;
    }
    let value = i128::from(component).saturating_mul(i128::from(distance));
    let denominator = i128::from(denominator);
    let scaled = if value >= 0 {
        value.saturating_add(denominator / 2) / denominator
    } else {
        value.saturating_sub(denominator / 2) / denominator
    };
    scaled.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

pub fn advance_direct_projectile(
    position: &mut [i64; 2],
    velocity: &mut [i64; 2],
    movement_remainder: &mut i64,
    target: [i64; 2],
    speed_raw: i64,
    ticks_per_second: u64,
) {
    if ticks_per_second == 0 {
        return;
    }
    let direction = [target[0] - position[0], target[1] - position[1]];
    let distance = integer_sqrt(distance_squared(direction, [0, 0]).max(0) as u128)
        .min(i64::MAX as u128) as i64;
    let numerator = i128::from(speed_raw.max(0)).saturating_add(i128::from(*movement_remainder));
    let ticks_per_second = i128::from(ticks_per_second);
    let step = (numerator / ticks_per_second).clamp(0, i128::from(i64::MAX)) as i64;
    *movement_remainder = (numerator % ticks_per_second) as i64;
    if distance == 0 || step >= distance {
        *position = target;
        *velocity = [0, 0];
        return;
    }
    *velocity = [
        scale_component(direction[0], speed_raw.max(0), distance),
        scale_component(direction[1], speed_raw.max(0), distance),
    ];
    position[0] = position[0].saturating_add(scale_component(direction[0], step, distance));
    position[1] = position[1].saturating_add(scale_component(direction[1], step, distance));
}

pub struct HomingProjectileParams {
    pub acceleration_raw: i64,
    pub turn_rate_raw: i64,
    pub max_speed_raw: i64,
    pub target: [i64; 2],
    pub ticks_per_second: u64,
    pub direct_switch_distance_raw: i64,
    pub direct_acceleration_multiplier_raw: i64,
}

pub fn advance_homing_projectile(
    position: &mut [i64; 2],
    velocity: &mut [i64; 2],
    acceleration_remainder: &mut i64,
    turn_remainder: &mut i64,
    movement_remainder: &mut i64,
    params: HomingProjectileParams,
) {
    if params.ticks_per_second == 0 {
        return;
    }
    let direction = [
        params.target[0] - position[0],
        params.target[1] - position[1],
    ];
    let distance = integer_sqrt(distance_squared(direction, [0, 0]).max(0) as u128)
        .min(i64::MAX as u128) as i64;
    if distance == 0 {
        *position = params.target;
        *velocity = [0, 0];
        return;
    }

    let direct_steering = distance <= params.direct_switch_distance_raw.max(0);
    let effective_acceleration = if direct_steering {
        multiply_ratio_raw(
            params.acceleration_raw,
            params.direct_acceleration_multiplier_raw,
        )
    } else {
        params.acceleration_raw
    };
    let ticks_per_second = i128::from(params.ticks_per_second);
    let full_acceleration =
        i128::from(effective_acceleration).saturating_add(i128::from(*acceleration_remainder));
    let acceleration_per_tick =
        (full_acceleration / ticks_per_second).clamp(0, i128::from(i64::MAX)) as i64;
    *acceleration_remainder = (full_acceleration % ticks_per_second) as i64;

    let current_speed = integer_sqrt(distance_squared(*velocity, [0, 0]).max(0) as u128)
        .min(i64::MAX as u128) as i64;
    let speed = current_speed
        .saturating_add(acceleration_per_tick)
        .min(params.max_speed_raw.max(0));
    let desired = [
        scale_component(direction[0], speed, distance),
        scale_component(direction[1], speed, distance),
    ];
    if direct_steering {
        *velocity = desired;
    } else {
        let turn_numerator =
            i128::from(params.turn_rate_raw).saturating_add(i128::from(*turn_remainder));
        let turn_per_tick =
            (turn_numerator / ticks_per_second).clamp(0, i128::from(RATIO_SCALE)) as i64;
        *turn_remainder = (turn_numerator % ticks_per_second) as i64;
        velocity[0] = velocity[0].saturating_add(scale_component(
            desired[0].saturating_sub(velocity[0]),
            turn_per_tick,
            RATIO_SCALE,
        ));
        velocity[1] = velocity[1].saturating_add(scale_component(
            desired[1].saturating_sub(velocity[1]),
            turn_per_tick,
            RATIO_SCALE,
        ));
    }

    let current_speed = integer_sqrt(distance_squared(*velocity, [0, 0]).max(0) as u128)
        .min(i64::MAX as u128) as i64;
    let numerator = i128::from(current_speed).saturating_add(i128::from(*movement_remainder));
    let step = (numerator / ticks_per_second).clamp(0, i128::from(i64::MAX)) as i64;
    *movement_remainder = (numerator % ticks_per_second) as i64;
    position[0] = position[0].saturating_add(scale_component(direction[0], step, distance));
    position[1] = position[1].saturating_add(scale_component(direction[1], step, distance));
}

fn splash_damage_raw(damage_raw: i64, splashes: &[DamageSplash], distance_squared: i128) -> i64 {
    let damage_pct_raw = splashes
        .iter()
        .filter(|splash| {
            distance_squared
                <= i128::from(splash.radius_raw.max(0))
                    .saturating_mul(i128::from(splash.radius_raw.max(0)))
        })
        .map(|splash| splash.damage_pct_raw)
        .sum::<i64>();
    if damage_pct_raw <= 0 {
        return 0;
    }
    multiply_ratio_raw(damage_raw, damage_pct_raw)
}

pub fn expand_on_hit_splashes(
    monster_centers: &[[i64; 2]],
    hits: Vec<DamageHit>,
) -> Vec<DamageHit> {
    if hits.iter().all(|hit| hit.splashes.is_empty()) {
        return hits;
    }

    let mut expanded = Vec::with_capacity(hits.len());
    for mut hit in hits {
        for (index, center) in monster_centers.iter().enumerate() {
            if index == hit.target_index {
                continue;
            }
            let damage_raw = splash_damage_raw(
                hit.damage_raw,
                &hit.splashes,
                distance_squared(*center, hit.at_xy),
            );
            if damage_raw <= 0 {
                continue;
            }
            expanded.push(DamageHit {
                target_index: index,
                damage_raw,
                at_xy: *center,
                source_index: hit.source_index,
                splashes: Vec::new(),
            });
        }
        hit.splashes.clear();
        expanded.push(hit);
    }
    expanded
}

pub fn expand_area_damage_events(
    monster_centers: &[[i64; 2]],
    events: Vec<AreaDamageEvent>,
) -> Vec<DamageHit> {
    let mut hits = Vec::new();
    for event in events {
        for (index, center) in monster_centers.iter().enumerate() {
            let damage_raw = splash_damage_raw(
                event.damage_raw,
                &event.splashes,
                distance_squared(*center, event.center_xy),
            );
            if damage_raw <= 0 {
                continue;
            }
            hits.push(DamageHit {
                target_index: index,
                damage_raw,
                at_xy: *center,
                source_index: event.source_index,
                splashes: Vec::new(),
            });
        }
    }
    hits
}

impl ActionKind {
    pub const COUNT: usize = 19;

    pub const fn index(self) -> usize {
        match self {
            Self::PurchaseShopItem => 0,
            Self::StartSelectingTower => 1,
            Self::BeginRerollSelection => 2,
            Self::BeginTowerSelection => 3,
            Self::SelectHandCard => 4,
            Self::DeselectHandCard => 5,
            Self::ConfirmCardSelection => 6,
            Self::CancelCardSelection => 7,
            Self::Reroll => 8,
            Self::SelectTower => 9,
            Self::PlaceTower => 10,
            Self::RemoveTower => 11,
            Self::StartDefense => 12,
            Self::SelectTreasure => 13,
            Self::SelectCardServiceCard => 14,
            Self::ConfirmCardServiceSelection => 15,
            Self::UseInventoryItem => 16,
            Self::DiscardTreasure => 17,
            Self::Continue => 18,
        }
    }

    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::PurchaseShopItem => "purchase_shop_item",
            Self::StartSelectingTower => "start_selecting_tower",
            Self::BeginRerollSelection => "begin_reroll_selection",
            Self::BeginTowerSelection => "begin_tower_selection",
            Self::SelectHandCard => "select_hand_card",
            Self::DeselectHandCard => "deselect_hand_card",
            Self::ConfirmCardSelection => "confirm_card_selection",
            Self::CancelCardSelection => "cancel_card_selection",
            Self::Reroll => "reroll",
            Self::SelectTower => "select_tower",
            Self::PlaceTower => "place_tower",
            Self::RemoveTower => "remove_tower",
            Self::StartDefense => "start_defense",
            Self::SelectTreasure => "select_treasure",
            Self::SelectCardServiceCard => "select_card_service_card",
            Self::ConfirmCardServiceSelection => "confirm_card_service_selection",
            Self::UseInventoryItem => "use_inventory_item",
            Self::DiscardTreasure => "discard_treasure",
            Self::Continue => "continue",
        }
    }
}

impl AgentAction {
    pub fn kind(&self) -> ActionKind {
        match self {
            Self::PurchaseShopItem { .. } => ActionKind::PurchaseShopItem,
            Self::StartSelectingTower => ActionKind::StartSelectingTower,
            Self::BeginRerollSelection => ActionKind::BeginRerollSelection,
            Self::BeginTowerSelection => ActionKind::BeginTowerSelection,
            Self::SelectHandCard { .. } => ActionKind::SelectHandCard,
            Self::DeselectHandCard { .. } => ActionKind::DeselectHandCard,
            Self::ConfirmCardSelection => ActionKind::ConfirmCardSelection,
            Self::CancelCardSelection => ActionKind::CancelCardSelection,
            Self::Reroll { .. } => ActionKind::Reroll,
            Self::SelectTower { .. } => ActionKind::SelectTower,
            Self::PlaceTower { .. } => ActionKind::PlaceTower,
            Self::RemoveTower { .. } => ActionKind::RemoveTower,
            Self::StartDefense => ActionKind::StartDefense,
            Self::SelectTreasure { .. } => ActionKind::SelectTreasure,
            Self::SelectCardServiceCard { .. } => ActionKind::SelectCardServiceCard,
            Self::ConfirmCardServiceSelection => ActionKind::ConfirmCardServiceSelection,
            Self::UseInventoryItem { .. } => ActionKind::UseInventoryItem,
            Self::DiscardTreasure { .. } => ActionKind::DiscardTreasure,
            Self::Continue => ActionKind::Continue,
        }
    }

    pub fn action_id(&self) -> String {
        match self {
            Self::PurchaseShopItem { slot_index } => format!("purchase_shop_item:{slot_index}"),
            Self::StartSelectingTower => "start_selecting_tower".to_string(),
            Self::BeginRerollSelection => "begin_reroll_selection".to_string(),
            Self::BeginTowerSelection => "begin_tower_selection".to_string(),
            Self::SelectHandCard { hand_slot_index } => {
                format!("select_hand_card:{hand_slot_index}")
            }
            Self::DeselectHandCard { hand_slot_index } => {
                format!("deselect_hand_card:{hand_slot_index}")
            }
            Self::ConfirmCardSelection => "confirm_card_selection".to_string(),
            Self::CancelCardSelection => "cancel_card_selection".to_string(),
            Self::Reroll {
                selected_slot_indices,
            } => format!("reroll:{}", indices_key(selected_slot_indices)),
            Self::SelectTower {
                selected_slot_indices,
            } => format!("select_tower:{}", indices_key(selected_slot_indices)),
            Self::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => format!("place_tower:{hand_slot_index}:{left}:{top}"),
            Self::RemoveTower { tower_id } => format!("remove_tower:{tower_id}"),
            Self::StartDefense => "start_defense".to_string(),
            Self::SelectTreasure { option_index } => format!("select_treasure:{option_index}"),
            Self::SelectCardServiceCard { card_index } => {
                format!("select_card_service_card:{card_index}")
            }
            Self::ConfirmCardServiceSelection => "confirm_card_service_selection".to_string(),
            Self::UseInventoryItem { item_index } => format!("use_inventory_item:{item_index}"),
            Self::DiscardTreasure { upgrade_id } => format!("discard_treasure:{upgrade_id}"),
            Self::Continue => "continue".to_string(),
        }
    }

    pub fn to_player_command(&self) -> Option<PlayerCommand> {
        match self {
            Self::PurchaseShopItem { slot_index } => Some(PlayerCommand::PurchaseShopItem {
                slot_index: *slot_index,
            }),
            Self::StartSelectingTower => Some(PlayerCommand::StartSelectingTower),
            Self::BeginRerollSelection
            | Self::BeginTowerSelection
            | Self::SelectHandCard { .. }
            | Self::DeselectHandCard { .. }
            | Self::ConfirmCardSelection
            | Self::CancelCardSelection => None,
            Self::Reroll {
                selected_slot_indices,
            } => Some(PlayerCommand::Reroll {
                selected_slot_indices: selected_slot_indices.clone(),
            }),
            Self::SelectTower {
                selected_slot_indices,
            } => Some(PlayerCommand::SelectTower {
                selected_slot_indices: selected_slot_indices.clone(),
            }),
            Self::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => Some(PlayerCommand::PlaceTower {
                hand_slot_index: *hand_slot_index,
                left: *left,
                top: *top,
            }),
            Self::RemoveTower { tower_id } => Some(PlayerCommand::RemoveTower {
                tower_id: *tower_id,
            }),
            Self::StartDefense => Some(PlayerCommand::StartDefense),
            Self::SelectTreasure { option_index } => Some(PlayerCommand::SelectTreasure {
                option_index: *option_index,
            }),
            Self::SelectCardServiceCard { .. }
            | Self::ConfirmCardServiceSelection
            | Self::Continue => None,
            Self::UseInventoryItem { item_index } => Some(PlayerCommand::UseInventoryItem {
                item_index: *item_index,
            }),
            Self::DiscardTreasure { upgrade_id } => Some(PlayerCommand::DiscardTreasure {
                upgrade_id: *upgrade_id,
            }),
        }
    }
}

fn indices_key(indices: &[usize]) -> String {
    indices
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            terminal_win: 1.0,
            terminal_loss: -1.0,
            escaped_hp_penalty_scale: 50.0,
            player_hp_loss_penalty_scale: 60.0,
            potential_weight: 0.0,
            potential_gamma: 0.99,
            damage_progress_weight: 0.0,
            no_progress_cycle_penalty: 0.0,
        }
    }
}

impl RewardConfig {
    pub fn validate(&self) -> Result<(), String> {
        let fields = [
            ("terminal_win", self.terminal_win),
            ("terminal_loss", self.terminal_loss),
            ("escaped_hp_penalty_scale", self.escaped_hp_penalty_scale),
            (
                "player_hp_loss_penalty_scale",
                self.player_hp_loss_penalty_scale,
            ),
            ("potential_weight", self.potential_weight),
            ("potential_gamma", self.potential_gamma),
            ("damage_progress_weight", self.damage_progress_weight),
            ("no_progress_cycle_penalty", self.no_progress_cycle_penalty),
        ];
        if let Some((name, value)) = fields.iter().find(|(_, value)| !value.is_finite()) {
            return Err(format!("reward field {name} must be finite, got {value}"));
        }
        if self.no_progress_cycle_penalty > 0.0 {
            return Err(format!(
                "no_progress_cycle_penalty must be non-positive, got {}",
                self.no_progress_cycle_penalty
            ));
        }
        Ok(())
    }

    pub fn validate_gamma(&self, rollout_gamma: f32) -> Result<(), String> {
        self.validate()?;
        if (self.potential_gamma - rollout_gamma).abs() > f32::EPSILON {
            return Err(format!(
                "reward potential gamma {} does not match rollout gamma {}",
                self.potential_gamma, rollout_gamma
            ));
        }
        Ok(())
    }

    pub fn validate_equal(&self, other: &Self) -> Result<(), String> {
        if self != other {
            return Err("PPO and rollout reward configurations differ".to_string());
        }
        Ok(())
    }
}

impl CoreEventQueue {
    pub fn push(&mut self, event: CoreEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, CoreEvent> {
        self.events.drain(..)
    }
}

#[cfg(test)]
mod tests {
    use super::game_state::monster::{apply_damage_hits, prepare_monster_death};
    use super::game_state::tick::{advance_in_flight_attacks_with_events, timed_attack_is_due};
    use super::{
        ActionKind, AgentAction, AreaDamageEvent, AttackSourceState, CommandError, CoreEvent,
        CoreEventQueue, DamageHit, DamageSplash, EntitySnapshots, GameConfigState, GameMetrics,
        HomingProjectileParams, InFlightAttackKindState, InFlightAttackState, LaserAttackState,
        MonsterSkill, MonsterSkillKind, MonsterSkillTarget, MonsterSkillTemplate, MonsterState,
        MonsterStatusEffect, MonsterStatusEffectKind, MoveOnRouteState, PlayerCommand, RngState,
        RouteState, SimTick, SimTickSpan, SpatialAttackBehaviorState, SpatialAttackState,
        TimedAttackState, TowerSkill, TowerSkillKind, TowerSkillTemplate, TowerState,
        TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind, TowerTemplateState,
        UserStatusEffect, UserStatusEffectKind, WORLD_UNITS_PER_TILE, activate_monster_skills,
        activate_tower_skills, adjust_incoming_damage, advance_direct_projectile,
        advance_homing_projectile, advance_monster_states, advance_move_on_route,
        advance_tower_cooldowns, apply_monster_damage, apply_monster_skill_activations,
        apply_ratio_product_raw, apply_tower_skill_activations, damage_hit_sort_key,
        expand_area_damage_events, expand_on_hit_splashes, remove_dead_monster,
        remove_expired_monster_statuses, remove_expired_tower_statuses,
        remove_expired_user_status_effects, resolve_monster_escapes, segment_hits_point,
    };

    #[test]
    fn ticks_saturate_and_subtract() {
        let tick = SimTick::from_ticks(4) + SimTickSpan::from_ticks(3);
        assert_eq!(tick.ticks(), 7);
        assert_eq!((tick - SimTick::from_ticks(10)).ticks(), 0);
    }

    #[test]
    fn ratio_product_rounds_once_after_sorting_factors() {
        assert_eq!(apply_ratio_product_raw(1, &[1_500_000, 1_500_000]), 2);
        assert_eq!(apply_ratio_product_raw(1, &[1_500_000, 1_000_000]), 2);
    }

    #[test]
    fn timed_attack_due_semantics_are_inclusive() {
        let attack = TimedAttackState {
            target_monster_id: 7,
            execute_at: 12,
        };
        assert!(!timed_attack_is_due(attack, 11));
        assert!(timed_attack_is_due(attack, 12));
        assert!(timed_attack_is_due(attack, 13));
    }

    #[test]
    fn laser_attack_state_round_trips_without_host_types() {
        let state = LaserAttackState {
            start_xy: [10, 20],
            end_xy: [30, 40],
            created_at: 12,
            target_monster_id: 7,
        };
        let encoded = serde_json::to_string(&state).unwrap();
        assert_eq!(
            serde_json::from_str::<LaserAttackState>(&encoded).unwrap(),
            state
        );
    }

    #[test]
    fn spatial_attack_state_round_trips_without_presentation_fields() {
        let state = SpatialAttackState {
            position: [10, 20],
            target_monster_id: 7,
            velocity: [30, 40],
            behavior: SpatialAttackBehaviorState::Homing {
                velocity: [30, 40],
                acceleration_raw: 1_000,
                turn_rate_raw: 2_000,
                max_speed_raw: 3_000,
                acceleration_remainder: 4,
                turn_remainder: 5,
            },
            movement_remainder: 6,
            stable_key: 8,
        };
        let encoded = serde_json::to_string(&state).unwrap();
        assert_eq!(
            serde_json::from_str::<SpatialAttackState>(&encoded).unwrap(),
            state
        );
        assert!(!encoded.contains("projectile_kind"));
        assert!(!encoded.contains("trail"));
        assert!(!encoded.contains("hit_effect"));
    }

    #[test]
    fn in_flight_attack_state_round_trips_as_one_core_composite() {
        let state = InFlightAttackState {
            id: 11,
            damage_raw: 42_000,
            source_tower: Some(AttackSourceState {
                tower_id: 9,
                tower_kind: 0,
                rank: Some(12),
                suit: Some(0),
            }),
            kind: InFlightAttackKindState::Spatial(SpatialAttackState {
                position: [10, 20],
                target_monster_id: 7,
                velocity: [30, 40],
                behavior: SpatialAttackBehaviorState::Direct,
                movement_remainder: 3,
                stable_key: 4,
            }),
            on_hit_splashes: vec![DamageSplash {
                radius_raw: 2_000_000,
                damage_pct_raw: 500_000,
            }],
        };
        let encoded = serde_json::to_string(&state).unwrap();
        assert_eq!(
            serde_json::from_str::<InFlightAttackState>(&encoded).unwrap(),
            state
        );
    }

    #[test]
    fn in_flight_attacks_advance_in_deterministic_kind_order() {
        let monster = MonsterState {
            id: 7,
            move_on_route: MoveOnRouteState {
                route: RouteState {
                    map_coords: vec![],
                    world_coords: vec![],
                    segment_lengths: vec![],
                    cumulative_lengths: vec![],
                },
                route_index: 0,
                route_progress_raw: 0,
                map_coord: [0, 0],
                velocity_raw: 0,
                movement_remainder: 0,
                motion_revision: 0,
            },
            kind: 0,
            hp_raw: 100,
            max_hp_raw: 100,
            stage_progress_counted: false,
            skills: vec![],
            status_effects: vec![],
            damage_raw: 0,
            reward: 0,
        };
        let attack = |id, kind| InFlightAttackState {
            id,
            damage_raw: 1,
            source_tower: None,
            kind,
            on_hit_splashes: vec![],
        };
        let mut attacks = vec![
            attack(
                3,
                InFlightAttackKindState::Spatial(SpatialAttackState {
                    position: [0, WORLD_UNITS_PER_TILE / 2],
                    target_monster_id: 7,
                    velocity: [0, WORLD_UNITS_PER_TILE * 60],
                    behavior: SpatialAttackBehaviorState::Direct,
                    movement_remainder: 0,
                    stable_key: 0,
                }),
            ),
            attack(
                1,
                InFlightAttackKindState::Laser(LaserAttackState {
                    start_xy: [0, 0],
                    end_xy: [0, 0],
                    created_at: 0,
                    target_monster_id: 7,
                }),
            ),
            attack(
                2,
                InFlightAttackKindState::Timed(TimedAttackState {
                    target_monster_id: 7,
                    execute_at: 4,
                }),
            ),
        ];

        let batches =
            advance_in_flight_attacks_with_events(&mut attacks, &[monster], 4, &mut Vec::new());

        assert!(attacks.is_empty());
        assert_eq!(
            batches
                .iter()
                .map(|batch| batch[0].attack.id)
                .collect::<Vec<_>>(),
            vec![2, 1, 3]
        );
        assert_eq!(batches[2][0].at_xy, [WORLD_UNITS_PER_TILE / 2; 2]);
    }

    #[test]
    fn core_events_are_serializable() {
        let event = CoreEvent::StageStarted {
            stage: 3,
            card_count: 5,
        };
        let encoded = serde_json::to_string(&event).unwrap();
        assert_eq!(serde_json::from_str::<CoreEvent>(&encoded).unwrap(), event);
    }

    #[test]
    fn monster_damage_applies_hp_cap_and_invincibility() {
        let mut monster = MonsterState {
            id: 1,
            move_on_route: MoveOnRouteState {
                route: RouteState {
                    map_coords: vec![[0, 0], [1, 0]],
                    world_coords: vec![[0, 0], [1_000_000, 0]],
                    segment_lengths: vec![1_000_000],
                    cumulative_lengths: vec![0, 1_000_000],
                },
                route_index: 0,
                route_progress_raw: 0,
                map_coord: [0, 0],
                velocity_raw: 0,
                movement_remainder: 0,
                motion_revision: 0,
            },
            kind: 0,
            hp_raw: 100,
            max_hp_raw: 100,
            stage_progress_counted: false,
            skills: vec![],
            status_effects: vec![],
            damage_raw: 0,
            reward: 0,
        };

        let hits = [
            DamageHit {
                target_index: 0,
                damage_raw: 40,
                at_xy: [0, 0],
                source_index: 0,
                splashes: vec![],
            },
            DamageHit {
                target_index: 0,
                damage_raw: 100,
                at_xy: [0, 0],
                source_index: 0,
                splashes: vec![],
            },
        ];
        let results = apply_damage_hits(std::slice::from_mut(&mut monster), &hits);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].target_index, 0);
        assert_eq!(results[0].damage.applied_damage_raw, 40);
        assert!(!results[0].damage.dead);
        assert_eq!(results[1].damage.applied_damage_raw, 60);
        assert!(results[1].damage.dead);

        monster.hp_raw = 100;
        monster.status_effects.push(MonsterStatusEffect {
            kind: MonsterStatusEffectKind::Invincible,
            end_at: 10,
        });
        assert_eq!(
            apply_monster_damage(&mut monster, 40),
            super::MonsterDamageResult {
                applied_damage_raw: 0,
                dead: false,
            }
        );
        assert_eq!(monster.hp_raw, 100);

        monster.hp_raw = 0;
        let death = prepare_monster_death(&mut monster).unwrap();
        assert_eq!(death.remaining_hp_raw, 0);
        assert!(death.should_count_stage_progress);
        assert!(
            !prepare_monster_death(&mut monster)
                .unwrap()
                .should_count_stage_progress
        );

        let mut monsters = vec![monster];
        let removed = remove_dead_monster(&mut monsters, 0).unwrap();
        assert_eq!(removed.monster.id, 1);
        assert!(!removed.death.should_count_stage_progress);
        assert!(monsters.is_empty());
    }

    #[test]
    fn splash_and_area_damage_use_raw_centers_and_stack_percentages() {
        let centers = vec![[0, 0], [1_000_000, 0], [9_000_000, 0]];
        let splashes = vec![
            DamageSplash {
                radius_raw: 2_000_000,
                damage_pct_raw: 300_000,
            },
            DamageSplash {
                radius_raw: 2_000_000,
                damage_pct_raw: 400_000,
            },
        ];
        let expanded = expand_on_hit_splashes(
            &centers,
            vec![DamageHit {
                target_index: 0,
                damage_raw: 100,
                at_xy: centers[0],
                source_index: 7,
                splashes,
            }],
        );

        assert_eq!(expanded.len(), 2);
        assert_eq!(expanded[0].target_index, 1);
        assert_eq!(expanded[0].damage_raw, 70);
        assert_eq!(expanded[0].source_index, 7);
        assert_eq!(expanded[1].target_index, 0);
        assert_eq!(expanded[1].damage_raw, 100);

        let area_hits = expand_area_damage_events(
            &centers,
            vec![AreaDamageEvent {
                center_xy: centers[0],
                damage_raw: 100,
                source_index: 3,
                splashes: vec![DamageSplash {
                    radius_raw: 2_000_000,
                    damage_pct_raw: 300_000,
                }],
            }],
        );
        assert_eq!(area_hits.len(), 2);
        assert_eq!(area_hits[0].target_index, 0);
        assert_eq!(area_hits[0].damage_raw, 30);
        assert_eq!(area_hits[1].target_index, 1);
        assert_eq!(area_hits[1].source_index, 3);
    }

    #[test]
    fn projectile_segment_collision_handles_interior_endpoint_and_miss() {
        assert!(segment_hits_point([0, 0], [1_000, 0], [500, 50], 50));
        assert!(segment_hits_point([0, 0], [1_000, 0], [1_050, 0], 50));
        assert!(!segment_hits_point([0, 0], [1_000, 0], [500, 51], 50));
        assert!(segment_hits_point([100, 100], [100, 100], [100, 100], 0));
    }

    #[test]
    fn direct_projectile_movement_preserves_remainder_and_reaches_target() {
        let mut position = [0, 0];
        let mut velocity = [0, -1_001];
        let mut remainder = 0;

        advance_direct_projectile(
            &mut position,
            &mut velocity,
            &mut remainder,
            [0, -10_000],
            1_001,
            60,
        );
        assert_eq!(position, [0, -16]);
        assert_eq!(velocity, [0, -1_001]);
        assert_eq!(remainder, 41);

        advance_direct_projectile(
            &mut position,
            &mut velocity,
            &mut remainder,
            [0, -10],
            60_000,
            60,
        );
        assert_eq!(position, [0, -10]);
        assert_eq!(velocity, [0, 0]);
    }

    #[test]
    fn homing_projectile_uses_direct_steering_inside_switch_distance() {
        let mut position = [0, 0];
        let mut velocity = [0, -24_000_000];
        let mut acceleration_remainder = 0;
        let mut turn_remainder = 0;
        let mut movement_remainder = 0;

        advance_homing_projectile(
            &mut position,
            &mut velocity,
            &mut acceleration_remainder,
            &mut turn_remainder,
            &mut movement_remainder,
            HomingProjectileParams {
                acceleration_raw: 1_024_000_000,
                turn_rate_raw: 2_000_000,
                max_speed_raw: 36_000_000,
                target: [1_000_000, 0],
                ticks_per_second: 60,
                direct_switch_distance_raw: 4_000_000,
                direct_acceleration_multiplier_raw: 100_000,
            },
        );

        assert!(velocity[0] > 0);
        assert_eq!(velocity[1], 0);
        assert!(position[0] > 0);
        assert_eq!(position[1], 0);
    }

    #[test]
    fn damage_hit_sort_key_is_id_first_and_invalid_indices_sort_last() {
        let monster_ids = vec![42, 7];

        assert_eq!(damage_hit_sort_key(&monster_ids, 1), (7, 1));
        assert_eq!(damage_hit_sort_key(&monster_ids, 0), (42, 0));
        assert_eq!(damage_hit_sort_key(&monster_ids, 3), (u64::MAX, 3));
    }

    #[test]
    fn player_command_schema_is_stable() {
        let command = PlayerCommand::PlaceTower {
            hand_slot_index: 2,
            left: 11,
            top: 17,
        };
        let encoded = serde_json::to_string(&command).unwrap();
        assert_eq!(
            encoded,
            r#"{"type":"PlaceTower","payload":{"hand_slot_index":2,"left":11,"top":17}}"#
        );
        assert_eq!(
            serde_json::from_str::<PlayerCommand>(&encoded).unwrap(),
            command
        );
    }

    #[test]
    fn command_errors_round_trip() {
        let error = CommandError::InvalidPlacement;
        let encoded = serde_json::to_string(&error).unwrap();
        assert_eq!(
            serde_json::from_str::<CommandError>(&encoded).unwrap(),
            error
        );
    }

    #[test]
    fn raw_composite_contracts_round_trip_without_host_dependencies() {
        let contracts = (
            GameMetrics {
                total_gold_earned: 7,
                total_gold_spent: 3,
                current_consecutive_perfect_clears: 1,
                max_consecutive_perfect_clears: 4,
                tower_damage_stats: vec![],
                total_rerolled_count: 2,
                total_escaped_hp_raw: 11,
                total_player_damage_raw: 13,
                stage_damage: vec![(2, 17)],
            },
            UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply_raw: 750_000,
                },
                end_at: 30,
            },
            MonsterStatusEffect {
                kind: super::MonsterStatusEffectKind::SpeedMul { mul_raw: 800_000 },
                end_at: 45,
            },
            TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageAdd { add_raw: 15 },
                end: TowerStatusEffectEnd::NeverEnd,
            },
            TowerSkill {
                last_used_at: 12,
                template: TowerSkillTemplate {
                    kind: TowerSkillKind::TopCardBonus {
                        rank: 10,
                        bonus_damage: 15,
                    },
                    cooldown: 60,
                    duration: 120,
                },
            },
        );
        let encoded = serde_json::to_string(&contracts).unwrap();
        assert_eq!(
            serde_json::from_str::<(
                GameMetrics,
                UserStatusEffect,
                MonsterStatusEffect,
                TowerStatusEffect,
                TowerSkill,
            )>(&encoded)
            .unwrap(),
            contracts
        );
    }

    #[test]
    fn monster_state_round_trips_without_presentation_fields() {
        let state = MonsterState {
            id: 7,
            move_on_route: MoveOnRouteState {
                route: RouteState {
                    map_coords: vec![[0, 0], [1, 0]],
                    world_coords: vec![[0, 0], [1_000, 0]],
                    segment_lengths: vec![1_000],
                    cumulative_lengths: vec![0, 1_000],
                },
                route_index: 0,
                route_progress_raw: 250,
                map_coord: [250, 0],
                velocity_raw: 60,
                movement_remainder: 3,
                motion_revision: 2,
            },
            kind: 50,
            hp_raw: 800,
            max_hp_raw: 1_000,
            stage_progress_counted: false,
            skills: vec![MonsterSkill {
                last_used_at: 12,
                template: MonsterSkillTemplate {
                    kind: MonsterSkillKind::SpeedMul { mul_raw: 750_000 },
                    target: MonsterSkillTarget::MySelf,
                    cooldown: 60,
                    duration: 120,
                },
            }],
            status_effects: vec![MonsterStatusEffect {
                kind: MonsterStatusEffectKind::Invincible,
                end_at: 90,
            }],
            damage_raw: 25,
            reward: 4,
        };
        let encoded = serde_json::to_string(&state).unwrap();
        assert_eq!(
            serde_json::from_str::<MonsterState>(&encoded).unwrap(),
            state
        );
    }

    #[test]
    fn entity_snapshots_round_trip_as_one_core_contract() {
        let snapshots = EntitySnapshots {
            monsters: vec![],
            towers: vec![],
        };
        let encoded = serde_json::to_string(&snapshots).unwrap();

        assert_eq!(
            serde_json::from_str::<EntitySnapshots>(&encoded).unwrap(),
            snapshots
        );
    }

    #[test]
    fn event_queue_drains_in_order() {
        let mut queue = CoreEventQueue::default();
        queue.push(CoreEvent::StageStarted {
            stage: 1,
            card_count: 0,
        });
        queue.push(CoreEvent::GameFinished { victory: true });
        let events: Vec<_> = queue.drain().collect();
        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[0],
            CoreEvent::StageStarted {
                stage: 1,
                card_count: 0
            }
        ));
        assert!(matches!(
            events[1],
            CoreEvent::GameFinished { victory: true }
        ));
    }

    #[test]
    fn agent_action_schema_and_command_mapping_are_stable() {
        let actions = [
            AgentAction::PurchaseShopItem { slot_index: 2 },
            AgentAction::StartSelectingTower,
            AgentAction::BeginRerollSelection,
            AgentAction::BeginTowerSelection,
            AgentAction::SelectHandCard { hand_slot_index: 1 },
            AgentAction::DeselectHandCard { hand_slot_index: 1 },
            AgentAction::ConfirmCardSelection,
            AgentAction::CancelCardSelection,
            AgentAction::Reroll {
                selected_slot_indices: vec![0, 2],
            },
            AgentAction::SelectTower {
                selected_slot_indices: vec![1],
            },
            AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 3,
                top: 4,
            },
            AgentAction::RemoveTower { tower_id: 7 },
            AgentAction::StartDefense,
            AgentAction::SelectTreasure { option_index: 1 },
            AgentAction::SelectCardServiceCard { card_index: 2 },
            AgentAction::ConfirmCardServiceSelection,
            AgentAction::UseInventoryItem { item_index: 0 },
            AgentAction::DiscardTreasure { upgrade_id: 11 },
            AgentAction::Continue,
        ];

        assert_eq!(actions.len(), ActionKind::COUNT);
        assert_eq!(
            actions
                .iter()
                .map(|action| action.kind().index())
                .collect::<Vec<_>>(),
            (0..ActionKind::COUNT).collect::<Vec<_>>()
        );
        assert_eq!(actions[0].action_id(), "purchase_shop_item:2");
        assert_eq!(actions[8].action_id(), "reroll:0,2");
        assert_eq!(
            actions[10].to_player_command(),
            Some(PlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 3,
                top: 4,
            })
        );
        assert_eq!(actions[2].to_player_command(), None);
    }

    #[test]
    fn rng_state_round_trips_without_host_dependencies() {
        let mut state = RngState::new(42);
        state.domain_sequences.insert(7, 3);
        state.shop.category_bag.entries = vec![2, 1, 0];
        state.shop.category_bag.cursor = 2;
        state.shop.content_bags[0].entries = vec!["alpha".to_string()];

        let encoded = serde_json::to_string(&state).unwrap();
        assert_eq!(serde_json::from_str::<RngState>(&encoded).unwrap().seed, 42);
        assert_eq!(
            serde_json::from_str::<RngState>(&encoded)
                .unwrap()
                .domain_sequences
                .get(&7),
            Some(&3)
        );
    }

    #[test]
    fn move_on_route_state_round_trips_without_host_dependencies() {
        let state = MoveOnRouteState {
            route: RouteState {
                map_coords: vec![[0, 0], [1, 0]],
                world_coords: vec![[0, 0], [1_000, 0]],
                segment_lengths: vec![1_000],
                cumulative_lengths: vec![0, 1_000],
            },
            route_index: 1,
            route_progress_raw: 1_000,
            map_coord: [1_000, 0],
            velocity_raw: 60,
            movement_remainder: 4,
            motion_revision: 2,
        };

        let encoded = serde_json::to_string(&state).unwrap();
        assert_eq!(
            serde_json::from_str::<MoveOnRouteState>(&encoded).unwrap(),
            state
        );
    }

    #[test]
    fn raw_route_movement_reaches_endpoint_and_preserves_remainder() {
        let mut state = MoveOnRouteState {
            route: RouteState {
                map_coords: vec![[0, 0], [1, 0]],
                world_coords: vec![[0, 0], [1_000_000, 0]],
                segment_lengths: vec![1_000_000],
                cumulative_lengths: vec![0, 1_000_000],
            },
            route_index: 0,
            route_progress_raw: 0,
            map_coord: [0, 0],
            velocity_raw: 0,
            movement_remainder: 0,
            motion_revision: 0,
        };

        advance_move_on_route(&mut state, 60_000_001);
        assert_eq!(state.route_index, 1);
        assert_eq!(state.map_coord, [1_000_000, 0]);
        assert_eq!(state.movement_remainder, 1);
    }

    #[test]
    fn game_config_state_round_trips_as_one_core_contract() {
        let state = GameConfigState {
            player: super::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 10,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 50,
                base_hand_slots: 5,
            },
            towers: super::TowerConfigState { entries: vec![] },
            monsters: super::MonsterConfigState {
                stats: vec![],
                stage_waves: vec![],
            },
        };
        let encoded = serde_json::to_string(&state).unwrap();

        assert_eq!(
            serde_json::from_str::<GameConfigState>(&encoded).unwrap(),
            state
        );
    }

    #[test]
    fn monster_movement_loop_applies_status_and_enemy_speed_in_core() {
        let mut monsters = vec![MonsterState {
            id: 1,
            move_on_route: MoveOnRouteState {
                route: RouteState {
                    map_coords: vec![[0, 0], [1, 0]],
                    world_coords: vec![[0, 0], [1_000, 0]],
                    segment_lengths: vec![1_000],
                    cumulative_lengths: vec![0, 1_000],
                },
                route_index: 0,
                route_progress_raw: 0,
                map_coord: [0, 0],
                velocity_raw: 60_000,
                movement_remainder: 0,
                motion_revision: 0,
            },
            kind: 0,
            hp_raw: 1_000,
            max_hp_raw: 1_000,
            stage_progress_counted: false,
            skills: vec![],
            status_effects: vec![MonsterStatusEffect {
                kind: MonsterStatusEffectKind::SpeedMul { mul_raw: 500_000 },
                end_at: 60,
            }],
            damage_raw: 1,
            reward: 1,
        }];

        advance_monster_states(&mut monsters, 1_000_000);

        assert_eq!(monsters[0].move_on_route.route_progress_raw, 500);
        assert_eq!(monsters[0].move_on_route.map_coord, [500, 0]);
        assert_eq!(monsters[0].move_on_route.motion_revision, 0);
    }

    #[test]
    fn monster_escape_resolution_removes_mobs_and_resets_bosses_in_core() {
        let route = RouteState {
            map_coords: vec![[0, 0], [1, 0]],
            world_coords: vec![[0, 0], [1_000, 0]],
            segment_lengths: vec![1_000],
            cumulative_lengths: vec![0, 1_000],
        };
        let movement = |route: RouteState| MoveOnRouteState {
            route,
            route_index: 1,
            route_progress_raw: 1_000,
            map_coord: [1_000, 0],
            velocity_raw: 60,
            movement_remainder: 0,
            motion_revision: 0,
        };
        let mut monsters = vec![
            MonsterState {
                id: 1,
                move_on_route: movement(route.clone()),
                kind: 0,
                hp_raw: 100,
                max_hp_raw: 100,
                stage_progress_counted: false,
                skills: vec![],
                status_effects: vec![],
                damage_raw: 5,
                reward: 1,
            },
            MonsterState {
                id: 2,
                move_on_route: movement(route),
                kind: 50,
                hp_raw: 200,
                max_hp_raw: 200,
                stage_progress_counted: false,
                skills: vec![],
                status_effects: vec![],
                damage_raw: 7,
                reward: 1,
            },
        ];

        let result = resolve_monster_escapes(&mut monsters);

        assert_eq!(result.damage_raw, 12);
        assert_eq!(result.escaped_hp_raw, 300);
        assert_eq!(monsters.len(), 1);
        assert_eq!(monsters[0].id, 2);
        assert_eq!(monsters[0].move_on_route.route_index, 0);
        assert!(monsters[0].stage_progress_counted);
    }

    #[test]
    fn tower_cooldown_advance_saturates_in_core() {
        let template = TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: 0,
            default_attack_range_radius_raw: 0,
            default_damage_raw: 0,
            suit: None,
            rank: None,
            skill_templates: vec![],
            default_status_effects: vec![],
            used_cards: vec![],
        };
        let mut towers = vec![
            TowerState {
                id: Some(1),
                left_top: [0, 0],
                cooldown: 0,
                template: template.clone(),
                status_effects: vec![],
                skills: vec![],
                damage_multiplier_raw: crate::RATIO_SCALE,
                attack_range_radius_raw: WORLD_UNITS_PER_TILE,
                effective_shoot_interval: 1,
                on_hit_splashes: vec![],
                on_attack_splashes: vec![],
            },
            TowerState {
                id: Some(2),
                left_top: [1, 0],
                cooldown: 1,
                template: template.clone(),
                status_effects: vec![],
                skills: vec![],
                damage_multiplier_raw: crate::RATIO_SCALE,
                attack_range_radius_raw: WORLD_UNITS_PER_TILE,
                effective_shoot_interval: 1,
                on_hit_splashes: vec![],
                on_attack_splashes: vec![],
            },
            TowerState {
                id: Some(3),
                left_top: [2, 0],
                cooldown: 3,
                template,
                status_effects: vec![],
                skills: vec![],
                damage_multiplier_raw: crate::RATIO_SCALE,
                attack_range_radius_raw: WORLD_UNITS_PER_TILE,
                effective_shoot_interval: 1,
                on_hit_splashes: vec![],
                on_attack_splashes: vec![],
            },
        ];

        advance_tower_cooldowns(&mut towers);

        assert_eq!(
            towers
                .iter()
                .map(|tower| tower.cooldown)
                .collect::<Vec<_>>(),
            [0, 0, 2]
        );
    }

    #[test]
    fn tower_skill_activation_updates_last_used_at_in_core() {
        let template = TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: 0,
            default_attack_range_radius_raw: 0,
            default_damage_raw: 0,
            suit: None,
            rank: None,
            skill_templates: vec![],
            default_status_effects: vec![],
            used_cards: vec![],
        };
        let skill = TowerSkill {
            last_used_at: 0,
            template: TowerSkillTemplate {
                kind: TowerSkillKind::MoneyIncomeAdd { add: 1 },
                cooldown: 2,
                duration: 0,
            },
        };
        let mut towers = vec![TowerState {
            id: Some(7),
            left_top: [0, 0],
            cooldown: 0,
            template,
            status_effects: vec![],
            skills: vec![skill],
            damage_multiplier_raw: crate::RATIO_SCALE,
            attack_range_radius_raw: WORLD_UNITS_PER_TILE,
            effective_shoot_interval: 1,
            on_hit_splashes: vec![],
            on_attack_splashes: vec![],
        }];

        assert!(activate_tower_skills(&mut towers, 1).is_empty());
        let activated = activate_tower_skills(&mut towers, 2);

        assert_eq!(activated.len(), 1);
        assert_eq!(activated[0].tower_id, 7);
        assert_eq!(towers[0].skills[0].last_used_at, 2);
    }

    #[test]
    fn tower_skill_activation_applies_nearby_tower_effects_in_core() {
        let template = TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: 0,
            default_attack_range_radius_raw: 0,
            default_damage_raw: 0,
            suit: None,
            rank: None,
            skill_templates: vec![],
            default_status_effects: vec![],
            used_cards: vec![],
        };
        let skill = TowerSkill {
            last_used_at: 0,
            template: TowerSkillTemplate {
                kind: TowerSkillKind::NearbyTowerDamageAdd {
                    add_raw: 15,
                    range_radius_raw: WORLD_UNITS_PER_TILE,
                },
                cooldown: 0,
                duration: 10,
            },
        };
        let mut towers = vec![
            TowerState {
                id: Some(7),
                left_top: [0, 0],
                cooldown: 0,
                template: template.clone(),
                status_effects: vec![],
                skills: vec![skill],
                damage_multiplier_raw: crate::RATIO_SCALE,
                attack_range_radius_raw: WORLD_UNITS_PER_TILE,
                effective_shoot_interval: 1,
                on_hit_splashes: vec![],
                on_attack_splashes: vec![],
            },
            TowerState {
                id: Some(8),
                left_top: [1, 0],
                cooldown: 0,
                template,
                status_effects: vec![],
                skills: vec![],
                damage_multiplier_raw: crate::RATIO_SCALE,
                attack_range_radius_raw: WORLD_UNITS_PER_TILE,
                effective_shoot_interval: 1,
                on_hit_splashes: vec![],
                on_attack_splashes: vec![],
            },
        ];

        let activations = activate_tower_skills(&mut towers, 2);
        apply_tower_skill_activations(&mut towers, &mut [], &activations, 2);

        assert_eq!(towers[0].status_effects.len(), 1);
        assert_eq!(towers[1].status_effects.len(), 1);
        assert!(matches!(
            towers[1].status_effects[0],
            TowerStatusEffect {
                kind: TowerStatusEffectKind::DamageAdd { add_raw: 15 },
                end: TowerStatusEffectEnd::Time { end_at: 12 },
            }
        ));
    }

    #[test]
    fn expired_tower_statuses_return_damage_refresh_ids_in_core() {
        let template = TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: 0,
            default_attack_range_radius_raw: 0,
            default_damage_raw: 0,
            suit: None,
            rank: None,
            skill_templates: vec![],
            default_status_effects: vec![],
            used_cards: vec![],
        };
        let mut towers = vec![TowerState {
            id: Some(7),
            left_top: [0, 0],
            cooldown: 0,
            template,
            status_effects: vec![
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul { mul_raw: 500_000 },
                    end: TowerStatusEffectEnd::Time { end_at: 3 },
                },
                TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageAdd { add_raw: 2 },
                    end: TowerStatusEffectEnd::NeverEnd,
                },
            ],
            skills: vec![],
            damage_multiplier_raw: crate::RATIO_SCALE,
            attack_range_radius_raw: WORLD_UNITS_PER_TILE,
            effective_shoot_interval: 1,
            on_hit_splashes: vec![],
            on_attack_splashes: vec![],
        }];

        let refresh_ids = remove_expired_tower_statuses(&mut towers, 3);

        assert_eq!(refresh_ids, vec![7]);
        assert_eq!(towers[0].status_effects.len(), 1);
        assert!(matches!(
            towers[0].status_effects[0].end,
            TowerStatusEffectEnd::NeverEnd
        ));
    }

    #[test]
    fn expired_monster_statuses_are_removed_in_core() {
        let mut monsters = vec![MonsterState {
            id: 1,
            move_on_route: MoveOnRouteState {
                route: RouteState {
                    map_coords: vec![],
                    world_coords: vec![],
                    segment_lengths: vec![],
                    cumulative_lengths: vec![],
                },
                route_index: 0,
                route_progress_raw: 0,
                map_coord: [0, 0],
                velocity_raw: 0,
                movement_remainder: 0,
                motion_revision: 0,
            },
            kind: 0,
            hp_raw: 1,
            max_hp_raw: 1,
            stage_progress_counted: false,
            skills: vec![],
            status_effects: vec![
                MonsterStatusEffect {
                    kind: MonsterStatusEffectKind::Invincible,
                    end_at: 3,
                },
                MonsterStatusEffect {
                    kind: MonsterStatusEffectKind::ImmuneToSlow,
                    end_at: 5,
                },
            ],
            damage_raw: 0,
            reward: 0,
        }];

        remove_expired_monster_statuses(&mut monsters, 3);

        assert_eq!(monsters[0].status_effects.len(), 1);
        assert_eq!(monsters[0].status_effects[0].end_at, 5);
    }

    #[test]
    fn monster_skill_activation_updates_last_used_at_in_core() {
        let mut monsters = vec![MonsterState {
            id: 9,
            move_on_route: MoveOnRouteState {
                route: RouteState {
                    map_coords: vec![],
                    world_coords: vec![],
                    segment_lengths: vec![],
                    cumulative_lengths: vec![],
                },
                route_index: 0,
                route_progress_raw: 0,
                map_coord: [0, 0],
                velocity_raw: 0,
                movement_remainder: 0,
                motion_revision: 0,
            },
            kind: 0,
            hp_raw: 1,
            max_hp_raw: 1,
            stage_progress_counted: false,
            skills: vec![MonsterSkill {
                last_used_at: 0,
                template: MonsterSkillTemplate {
                    kind: MonsterSkillKind::Invincible,
                    target: MonsterSkillTarget::MySelf,
                    cooldown: 2,
                    duration: 1,
                },
            }],
            status_effects: vec![],
            damage_raw: 0,
            reward: 0,
        }];

        assert!(activate_monster_skills(&mut monsters, 1).is_empty());
        let activated = activate_monster_skills(&mut monsters, 2);

        assert_eq!(activated.len(), 1);
        assert_eq!(activated[0].monster_id, 9);
        assert_eq!(monsters[0].skills[0].last_used_at, 2);
    }

    #[test]
    fn monster_skill_activation_applies_status_and_heal_effects_in_core() {
        let movement = || MoveOnRouteState {
            route: RouteState {
                map_coords: vec![],
                world_coords: vec![],
                segment_lengths: vec![],
                cumulative_lengths: vec![],
            },
            route_index: 0,
            route_progress_raw: 0,
            map_coord: [0, 0],
            velocity_raw: 0,
            movement_remainder: 0,
            motion_revision: 0,
        };
        let mut monsters = vec![
            MonsterState {
                id: 9,
                move_on_route: movement(),
                kind: 0,
                hp_raw: 50,
                max_hp_raw: 100,
                stage_progress_counted: false,
                skills: vec![MonsterSkill {
                    last_used_at: 0,
                    template: MonsterSkillTemplate {
                        kind: MonsterSkillKind::HealByMaxHp { ratio_raw: 500_000 },
                        target: MonsterSkillTarget::AllMonsters,
                        cooldown: 0,
                        duration: 0,
                    },
                }],
                status_effects: vec![],
                damage_raw: 0,
                reward: 0,
            },
            MonsterState {
                id: 10,
                move_on_route: movement(),
                kind: 0,
                hp_raw: 25,
                max_hp_raw: 100,
                stage_progress_counted: false,
                skills: vec![],
                status_effects: vec![],
                damage_raw: 0,
                reward: 0,
            },
        ];

        let activations = activate_monster_skills(&mut monsters, 2);
        apply_monster_skill_activations(&mut monsters, &activations, 2);

        assert_eq!(monsters[0].hp_raw, 100);
        assert_eq!(monsters[1].hp_raw, 75);
    }

    #[test]
    fn expired_user_status_effects_are_removed_in_core() {
        let mut effects = vec![
            UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply_raw: 500_000,
                },
                end_at: 3,
            },
            UserStatusEffect {
                kind: UserStatusEffectKind::DamageReduction {
                    damage_multiply_raw: 750_000,
                },
                end_at: 5,
            },
        ];

        remove_expired_user_status_effects(&mut effects, 3);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].end_at, 5);
    }

    #[test]
    fn incoming_damage_multiplier_order_is_preserved_in_core() {
        let effects = vec![UserStatusEffect {
            kind: UserStatusEffectKind::DamageReduction {
                damage_multiply_raw: 500_000,
            },
            end_at: 60,
        }];

        let adjusted = adjust_incoming_damage(100_000, &effects, &[800_000], &[1_250_000]);

        assert_eq!(adjusted, 50_000);
    }
}
