pub mod card_service;
pub mod command;
pub mod config;
pub mod effect;
pub mod entity_id;
pub mod flow;
pub mod hand;
pub mod item;
pub mod monster;
pub mod monster_spawn;
pub mod observation;
pub mod replay;
pub mod reward;
pub mod rng;
pub mod session;
pub mod shop;
pub mod tick;
pub mod tower;
pub mod tower_selection;
pub mod upgrade;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CoreProgress {
    pub player_command_sequence: u64,
    pub stage: usize,
    pub gold: usize,
    pub left_dice: usize,
    pub rerolled_count: usize,
    pub left_quest_board_refresh_chance: usize,
    pub item_used: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerDamageStats {
    pub tower_id: u64,
    pub tower_kind: u8,
    pub rank: Option<u8>,
    pub suit: Option<u8>,
    pub total_damage_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameMetrics {
    pub total_gold_earned: usize,
    pub total_gold_spent: usize,
    pub current_consecutive_perfect_clears: usize,
    pub max_consecutive_perfect_clears: usize,
    pub tower_damage_stats: Vec<TowerDamageStats>,
    pub total_rerolled_count: usize,
    pub total_escaped_hp_raw: i64,
    pub total_player_damage_raw: i64,
    pub stage_damage: Vec<(usize, i64)>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CoreState {
    progress: CoreProgress,
    sim_tick: crate::SimTick,
    rng: crate::RngState,
    route: crate::RouteState,
    config: crate::GameConfigState,
    stage_modifiers: crate::StageModifiersState,
    upgrades: crate::UpgradeCollectionState,
    hand: crate::HandState,
    deck: crate::DeckState,
    items: Vec<crate::ItemEntryState>,
    monster_spawn: crate::MonsterSpawnState,
    in_flight_attacks: Vec<crate::InFlightAttackState>,
    user_status_effects: Vec<crate::UserStatusEffect>,
    next_entity_id: crate::EntityIdAllocator,
    metrics: GameMetrics,
    flow: crate::GameFlowState,
    hp_raw: i64,
    shield_raw: i64,
    monsters: Vec<crate::MonsterState>,
    towers: Vec<crate::TowerState>,
    #[serde(default)]
    player_commands: Vec<crate::game_state::command::RecordedPlayerCommand>,
    #[serde(default)]
    replay_checkpoints: Vec<crate::game_state::replay::ReplayCheckpoint>,
    #[serde(default)]
    pending_card_service_kind: Option<u8>,
    #[serde(skip, default)]
    card_service_selection: Option<crate::CardServiceSelectionState>,
    #[serde(skip, default)]
    events: crate::CoreEventQueue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreCombatOutput {
    pub monster_spawned: bool,
    pub damage_refresh_tower_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderSnapshot {
    pub sim_tick: crate::SimTick,
    pub monsters: Vec<RenderMonsterSnapshot>,
    pub spatial_attacks: Vec<RenderSpatialAttackSnapshot>,
    pub towers: Vec<RenderTowerSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderMonsterSnapshot {
    pub id: u64,
    pub position: [i64; 2],
    pub direction: [i64; 2],
    pub motion_revision: u64,
    pub kind: u8,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderSpatialAttackSnapshot {
    pub id: u64,
    pub position: [i64; 2],
    pub direction: [i64; 2],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderTowerSnapshot {
    pub id: u64,
    pub left_top: [usize; 2],
    pub attack_range_radius_raw: i64,
    pub on_attack_splash_radii_raw: Vec<i64>,
}

fn direction_for_route(state: &crate::MoveOnRouteState) -> [i64; 2] {
    let index = state
        .route_index
        .min(state.route.world_coords.len().saturating_sub(1));
    let start = state
        .route
        .world_coords
        .get(index)
        .copied()
        .unwrap_or([0, 0]);
    let end = state
        .route
        .world_coords
        .get(index.saturating_add(1))
        .copied()
        .unwrap_or(start);
    [
        end[0].saturating_sub(start[0]),
        end[1].saturating_sub(start[1]),
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceTowerOutput {
    pub tower: crate::TowerState,
}

/// Result of one fully recorded fixed simulation tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedTickOutput {
    pub sim_tick: crate::SimTick,
    pub events: Vec<crate::CoreEvent>,
    pub defense_end: Option<crate::DefenseEndOutputState>,
    pub state_hash: String,
    pub event_count: u64,
    pub event_digest: String,
}

/// Result of one fixed simulation tick without replay/observation metadata.
///
/// Unrecorded ticks apply the same authoritative gameplay transition as
/// [`RecordedTickOutput`],
/// but discard emitted events and do not compute an authoritative hash or event
/// digest. Callers that need replay or policy data must use [`RecordedTickOutput`]
/// instead. A final hash can be computed from the session after a bulk run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TickTransition {
    pub sim_tick: crate::SimTick,
    pub defense_end: Option<crate::DefenseEndOutputState>,
}

/// Result of one fixed simulation tick preserving events for presentation.
///
/// Presentation ticks preserve authoritative `CoreEvent` values for the app's
/// sound, particle, and UI bridges, but skip replay-only state hashes and
/// event digests. Replay and simulator policy paths must use
/// [`RecordedTickOutput`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TickEventsOutput {
    pub sim_tick: crate::SimTick,
    pub events: Vec<crate::CoreEvent>,
    pub defense_end: Option<crate::DefenseEndOutputState>,
}

#[deprecated(note = "use RecordedTickOutput")]
pub type TickOutput = RecordedTickOutput;

#[deprecated(note = "use TickTransition")]
pub type FastTickOutput = TickTransition;

#[deprecated(note = "use TickEventsOutput")]
pub type PresentationTickOutput = TickEventsOutput;

impl CoreState {
    /// Returns the immutable command and progression counters.
    pub fn progress(&self) -> &CoreProgress {
        &self.progress
    }

    pub fn sim_tick(&self) -> crate::SimTick {
        self.sim_tick
    }

    pub fn rng(&self) -> &crate::RngState {
        &self.rng
    }

    pub fn route(&self) -> &crate::RouteState {
        &self.route
    }

    pub fn config(&self) -> &crate::GameConfigState {
        &self.config
    }

    pub fn stage_modifiers(&self) -> &crate::StageModifiersState {
        &self.stage_modifiers
    }

    pub fn upgrades(&self) -> &crate::UpgradeCollectionState {
        &self.upgrades
    }

    pub fn hand(&self) -> &crate::HandState {
        &self.hand
    }

    pub fn deck(&self) -> &crate::DeckState {
        &self.deck
    }

    pub fn items(&self) -> &[crate::ItemEntryState] {
        &self.items
    }

    pub fn monster_spawn(&self) -> &crate::MonsterSpawnState {
        &self.monster_spawn
    }

    pub fn in_flight_attacks(&self) -> &[crate::InFlightAttackState] {
        &self.in_flight_attacks
    }

    pub fn user_status_effects(&self) -> &[crate::UserStatusEffect] {
        &self.user_status_effects
    }

    pub fn next_entity_id(&self) -> &crate::EntityIdAllocator {
        &self.next_entity_id
    }

    pub fn metrics(&self) -> &GameMetrics {
        &self.metrics
    }

    pub fn flow(&self) -> &crate::GameFlowState {
        &self.flow
    }

    pub fn hp_raw(&self) -> i64 {
        self.hp_raw
    }

    pub fn shield_raw(&self) -> i64 {
        self.shield_raw
    }

    pub fn monsters(&self) -> &[crate::MonsterState] {
        &self.monsters
    }

    pub fn towers(&self) -> &[crate::TowerState] {
        &self.towers
    }

    pub fn player_commands(&self) -> &[crate::game_state::command::RecordedPlayerCommand] {
        &self.player_commands
    }

    pub fn replay_checkpoints(&self) -> &[crate::game_state::replay::ReplayCheckpoint] {
        &self.replay_checkpoints
    }

    pub fn pending_card_service_kind(&self) -> Option<u8> {
        self.pending_card_service_kind
    }

    pub fn pending_card_service_kind_typed(&self) -> Option<crate::CardServiceKind> {
        self.pending_card_service_kind
            .and_then(crate::CardServiceKind::from_raw)
    }

    pub fn card_service_selection(&self) -> Option<&crate::CardServiceSelectionState> {
        self.card_service_selection.as_ref()
    }

    /// Creates the deterministic initial state for a game session.
    pub fn new_initial(config: crate::GameConfigState, seed: u64) -> Self {
        let route = crate::calculate_routes(&[], &crate::TRAVEL_POINTS, crate::MAP_SIZE)
            .expect("initial route must be valid");
        let deck = crate::DeckState {
            revision: 0,
            next_card_id: 52,
            all_cards: (0..13)
                .flat_map(|rank| {
                    (0..4).map(move |suit| CardState {
                        id: rank * 4 + suit,
                        suit: suit as u8,
                        rank: rank as u8,
                        polish_pct_raw: 0,
                        engraving: None,
                    })
                })
                .collect(),
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        };
        let mut state = Self {
            progress: CoreProgress {
                player_command_sequence: 0,
                stage: 1,
                gold: config.player.starting_gold,
                left_dice: config.player.base_dice_chance,
                rerolled_count: 0,
                left_quest_board_refresh_chance: 0,
                item_used: false,
            },
            sim_tick: crate::SimTick::ZERO,
            rng: crate::RngState::new(seed),
            route,
            config: config.clone(),
            stage_modifiers: crate::StageModifiersState {
                damage_multipliers_raw: Vec::new(),
                damage_reduction_multipliers_raw: Vec::new(),
                incoming_damage_multipliers_raw: Vec::new(),
                gold_gain_multipliers_raw: Vec::new(),
                enemy_health_multipliers_raw: Vec::new(),
                enemy_speed_multipliers_raw: Vec::new(),
                card_selection_hand_max_slots_bonus: 0,
                card_selection_hand_max_slots_penalty: 0,
                max_dice_rerolls_bonus: 0,
                max_dice_rerolls_penalty: 0,
                reroll_health_cost: 0,
                disable_item_and_upgrade_purchases: false,
                disable_item_use: false,
                free_shop_this_stage: false,
                disabled_ranks: Vec::new(),
                disabled_suits: Vec::new(),
                extra_tower_cards: Vec::new(),
                free_card_services: 0,
            },
            upgrades: crate::UpgradeCollectionState {
                upgrades: Vec::new(),
                revision: 0,
            },
            hand: crate::HandState {
                slots: Vec::new(),
                next_hand_slot_id: 1,
            },
            deck,
            items: vec![
                crate::ItemEntryState {
                    id: 1,
                    kind: 7,
                    scalar_values: vec![1],
                    signed_values: Vec::new(),
                },
                crate::ItemEntryState {
                    id: 2,
                    kind: 7,
                    scalar_values: vec![1],
                    signed_values: Vec::new(),
                },
                crate::ItemEntryState {
                    id: 3,
                    kind: 9,
                    scalar_values: vec![4],
                    signed_values: Vec::new(),
                },
            ],
            monster_spawn: crate::MonsterSpawnState {
                monster_queue: Vec::new(),
                next_spawn_tick: None,
                spawn_interval_ticks: 0,
            },
            in_flight_attacks: Vec::new(),
            user_status_effects: Vec::new(),
            next_entity_id: crate::EntityIdAllocator::default(),
            metrics: crate::GameMetrics {
                total_gold_earned: 0,
                total_gold_spent: 0,
                current_consecutive_perfect_clears: 0,
                max_consecutive_perfect_clears: 0,
                tower_damage_stats: Vec::new(),
                total_rerolled_count: 0,
                total_escaped_hp_raw: 0,
                total_player_damage_raw: 0,
                stage_damage: Vec::new(),
            },
            flow: crate::GameFlowState::Initializing,
            hp_raw: config.player.starting_hp_raw,
            shield_raw: 0,
            monsters: Vec::new(),
            towers: Vec::new(),
            player_commands: Vec::new(),
            replay_checkpoints: Vec::new(),
            pending_card_service_kind: None,
            card_service_selection: None,
            events: crate::CoreEventQueue::default(),
        };
        state.start_stage(1);
        state.events.events.clear();
        state
    }

    pub fn render_snapshot(&self) -> RenderSnapshot {
        let mut monsters = self
            .monsters
            .iter()
            .map(|monster| RenderMonsterSnapshot {
                id: monster.id,
                position: monster.move_on_route.map_coord,
                direction: direction_for_route(&monster.move_on_route),
                motion_revision: monster.move_on_route.motion_revision,
                kind: monster.kind,
                hp_raw: monster.hp_raw,
                max_hp_raw: monster.max_hp_raw,
            })
            .collect::<Vec<_>>();
        monsters.sort_by_key(|monster| monster.id);

        let mut spatial_attacks = self
            .in_flight_attacks
            .iter()
            .filter_map(|attack| match &attack.kind {
                crate::InFlightAttackKindState::Spatial(spatial) => {
                    Some(RenderSpatialAttackSnapshot {
                        id: attack.id,
                        position: spatial.position,
                        direction: spatial.velocity,
                    })
                }
                crate::InFlightAttackKindState::Timed(_)
                | crate::InFlightAttackKindState::Laser(_) => None,
            })
            .collect::<Vec<_>>();
        spatial_attacks.sort_by_key(|attack| attack.id);

        let mut towers = self
            .towers
            .iter()
            .filter_map(|tower| {
                tower.id.map(|id| RenderTowerSnapshot {
                    id,
                    left_top: tower.left_top,
                    attack_range_radius_raw: tower.attack_range_raw(),
                    on_attack_splash_radii_raw: tower
                        .on_attack_splashes
                        .iter()
                        .map(|splash| splash.radius_raw)
                        .collect(),
                })
            })
            .collect::<Vec<_>>();
        towers.sort_by_key(|tower| tower.id);

        RenderSnapshot {
            sim_tick: self.sim_tick,
            monsters,
            spatial_attacks,
            towers,
        }
    }

    pub fn drain_events(&mut self) -> std::vec::Drain<'_, crate::CoreEvent> {
        self.events.drain()
    }

    pub fn extend_events(&mut self, events: impl IntoIterator<Item = crate::CoreEvent>) {
        self.events.extend(events);
    }

    pub(crate) fn push_event(&mut self, event: crate::CoreEvent) {
        self.events.push(event);
    }

    pub fn validate_snapshot(&self) -> bool {
        if self.route.map_coords.is_empty()
            || self.route.world_coords.len() != self.route.map_coords.len()
            || self.route.segment_lengths.len() + 1 != self.route.world_coords.len()
            || self.route.cumulative_lengths.len() != self.route.world_coords.len()
        {
            return false;
        }
        if self
            .hand
            .slots
            .iter()
            .map(|slot| slot.id)
            .collect::<std::collections::HashSet<_>>()
            .len()
            != self.hand.slots.len()
        {
            return false;
        }
        if self.hand.slots.iter().any(|slot| slot.id == 0)
            || (self.hand.next_hand_slot_id != 0
                && self.hand.next_hand_slot_id
                    <= self
                        .hand
                        .slots
                        .iter()
                        .map(|slot| slot.id)
                        .max()
                        .unwrap_or(0))
        {
            return false;
        }
        if self
            .upgrades
            .upgrades
            .iter()
            .any(|upgrade| upgrade.upgrade_kind().is_err())
        {
            return false;
        }
        if self.towers.iter().any(|tower| {
            tower.template.kind > 10
                || tower.template.suit.is_some_and(|suit| suit > 3)
                || tower.template.rank.is_some_and(|rank| rank > 12)
        }) {
            return false;
        }
        let tower_ids = self
            .towers
            .iter()
            .filter_map(|tower| tower.id)
            .collect::<Vec<_>>();
        if tower_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != tower_ids.len()
        {
            return false;
        }
        let next_entity_id = self.next_entity_id.next_id();
        if next_entity_id == 0 {
            return false;
        }
        let mut entity_ids = std::collections::HashSet::new();
        let mut valid_entity_id = |id: u64| id != 0 && id < next_entity_id && entity_ids.insert(id);
        if self
            .monster_spawn
            .monster_queue
            .iter()
            .chain(self.monsters.iter())
            .any(|monster| !valid_entity_id(monster.id))
            || self
                .towers
                .iter()
                .filter_map(|tower| tower.id)
                .any(|tower_id| !valid_entity_id(tower_id))
            || self
                .in_flight_attacks
                .iter()
                .any(|attack| !valid_entity_id(attack.id))
        {
            return false;
        }
        if let Some(kind) = self.pending_card_service_kind {
            if crate::CardServiceSelectionState::new_raw(kind).is_none() {
                return false;
            }
        } else if self.card_service_selection.is_some() {
            return false;
        }
        true
    }

    pub(crate) fn start_shopping_flow(&mut self) {
        crate::game_state::shop::start_shopping_flow(self);
    }

    pub fn select_tower(
        &mut self,
        selected_slot_indices: &[usize],
    ) -> Result<(), crate::CommandError> {
        crate::game_state::tower_selection::select_tower_from_core(self, selected_slot_indices)
    }

    pub fn start_placing_tower_with_template(
        &mut self,
        initial_template: crate::TowerTemplateState,
    ) {
        crate::game_state::tower_selection::start_placing_tower_from_template(
            self,
            initial_template,
        );
    }

    pub fn place_tower(
        &mut self,
        hand_slot_index: usize,
        left: usize,
        top: usize,
    ) -> Result<PlaceTowerOutput, crate::CommandError> {
        if !matches!(self.flow, crate::GameFlowState::PlacingTower) {
            return Err(crate::CommandError::InvalidFlow);
        }
        let template = self
            .hand
            .slots
            .get(hand_slot_index)
            .ok_or(crate::CommandError::InvalidIndex)
            .and_then(|slot| match &slot.item {
                crate::HandItemState::Tower(template) => Ok(template.clone()),
                crate::HandItemState::Card(_) => Err(crate::CommandError::InvalidSelection),
            })?;
        self.place_tower_with_template(template, Some(hand_slot_index), left, top)
    }

    pub fn place_tower_with_template(
        &mut self,
        template: crate::TowerTemplateState,
        hand_slot_index: Option<usize>,
        left: usize,
        top: usize,
    ) -> Result<PlaceTowerOutput, crate::CommandError> {
        if let Some(hand_slot_index) = hand_slot_index {
            let slot = self
                .hand
                .slots
                .get(hand_slot_index)
                .ok_or(crate::CommandError::InvalidIndex)?;
            if !matches!(slot.item, crate::HandItemState::Tower(_)) {
                return Err(crate::CommandError::InvalidSelection);
            }
        }
        let right = left
            .checked_add(1)
            .ok_or(crate::CommandError::InvalidPlacement)?;
        let bottom = top
            .checked_add(1)
            .ok_or(crate::CommandError::InvalidPlacement)?;
        let occupied = crate::game_state::tower::tower_blockers(&self.towers);
        let new_coords = [[left, top], [right, top], [left, bottom], [right, bottom]];
        if new_coords.iter().any(|coord| {
            coord[0] >= crate::MAP_SIZE[0]
                || coord[1] >= crate::MAP_SIZE[1]
                || crate::TRAVEL_POINTS.contains(coord)
                || occupied.contains(coord)
        }) {
            return Err(crate::CommandError::InvalidPlacement);
        }
        let mut blockers = occupied;
        blockers.extend(new_coords);
        let route = crate::calculate_routes(&blockers, &crate::TRAVEL_POINTS, crate::MAP_SIZE)
            .ok_or(crate::CommandError::InvalidPlacement)?;
        let tower_id = self.next_entity_id.allocate_raw();
        let tower = crate::TowerState {
            id: Some(tower_id),
            left_top: [left, top],
            cooldown: 0,
            template: template.clone(),
            status_effects: template.default_status_effects.clone(),
            skills: template
                .skill_templates
                .iter()
                .cloned()
                .map(|template| crate::TowerSkill {
                    last_used_at: 0,
                    template,
                })
                .collect(),
            damage_multiplier_raw: crate::RATIO_SCALE,
            attack_range_radius_raw: template.default_attack_range_radius_raw,
            effective_shoot_interval: template.shoot_interval,
            on_hit_splashes: Vec::new(),
            on_attack_splashes: if template
                .used_cards
                .iter()
                .any(|card| card.engraving == Some(2))
            {
                vec![crate::DamageSplash {
                    radius_raw: 2 * crate::WORLD_UNITS_PER_TILE,
                    damage_pct_raw: 300_000,
                }]
            } else {
                Vec::new()
            },
        };
        self.towers.push(tower.clone());
        if let Some(hand_slot_index) = hand_slot_index {
            self.hand.slots.remove(hand_slot_index);
            for (index, slot) in self.hand.slots.iter_mut().enumerate() {
                slot.id = index + 1;
                slot.selected = index == 0;
            }
        }
        self.route = route;
        Ok(PlaceTowerOutput { tower })
    }

    pub fn can_place_tower(&self, hand_slot_index: usize, left: usize, top: usize) -> bool {
        let mut candidate = self.clone();
        candidate.place_tower(hand_slot_index, left, top).is_ok()
    }

    pub fn refresh_tower_damage_multipliers(&mut self) {
        let upgrades = self.upgrades.clone();
        for tower in &mut self.towers {
            tower.damage_multiplier_raw = crate::RATIO_SCALE
                .saturating_add(upgrades.tower_damage_bonus_raw(tower))
                .max(0);
        }
    }

    pub fn begin_card_service_selection(
        &mut self,
        service_kind: crate::CardServiceKind,
    ) -> Result<(), crate::CommandError> {
        let selection = crate::CardServiceSelectionState::new(service_kind)
            .ok_or(crate::CommandError::Rejected)?;
        self.pending_card_service_kind = Some(service_kind.raw());
        self.push_event(crate::CoreEvent::CardServiceSelectionRequested {
            service_kind: crate::CardServiceSelectionState::service_key(service_kind).to_string(),
            step_counts: selection.steps.iter().map(|step| step.count).collect(),
        });
        self.card_service_selection = None;
        Ok(())
    }

    pub fn begin_card_service_selection_raw(&mut self, raw: u8) -> Result<(), crate::CommandError> {
        let service_kind = crate::CardServiceKind::from_raw(raw)
            .ok_or(crate::CommandError::InvalidCardServiceKind { raw })?;
        self.begin_card_service_selection(service_kind)
    }

    pub fn clear_card_service_selection(&mut self) {
        self.pending_card_service_kind = None;
        self.card_service_selection = None;
    }

    pub fn validate_card_service_selection(
        &self,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError> {
        let service_kind = self
            .pending_card_service_kind
            .ok_or(crate::CommandError::InvalidFlow)?;
        let service_kind = crate::CardServiceKind::from_raw(service_kind)
            .ok_or(crate::CommandError::InvalidCardServiceKind { raw: service_kind })?;
        let selection = crate::CardServiceSelectionState::new(service_kind)
            .ok_or(crate::CommandError::InvalidFlow)?;
        selection.validate(service_kind, &self.deck, selected_card_ids)
    }

    pub fn clear_rate_raw(&self) -> i64 {
        const TOTAL_STAGES: i128 = 50;
        let stage_weight_raw = i128::from(crate::RATIO_SCALE) / TOTAL_STAGES;
        let previous_stages_progress = (self
            .progress
            .stage
            .saturating_sub(1)
            .min(TOTAL_STAGES as usize) as i128)
            .saturating_mul(stage_weight_raw);
        let (start_total_hp_raw, processed_hp_raw) = match &self.flow {
            crate::GameFlowState::Defense(flow) => (flow.start_total_hp_raw, flow.processed_hp_raw),
            _ => (self.max_hp_raw(), 0),
        };
        let current_stage_progress = if start_total_hp_raw > 0 {
            let stage_ratio_raw = divide_round_positive(
                i128::from(processed_hp_raw.max(0)).saturating_mul(i128::from(crate::RATIO_SCALE)),
                i128::from(start_total_hp_raw),
            );
            i128::from(crate::multiply_ratio_raw(
                stage_ratio_raw,
                stage_weight_raw as i64,
            ))
        } else {
            0
        };
        previous_stages_progress
            .saturating_add(current_stage_progress)
            .clamp(0, i128::from(crate::RATIO_SCALE)) as i64
    }

    pub fn can_purchase_shop_slot(&self, slot_index: usize) -> bool {
        let crate::GameFlowState::Shopping(shop) = &self.flow else {
            return false;
        };
        let Some(slot) = shop.slots.get(slot_index) else {
            return false;
        };
        if slot.purchased || self.stage_modifiers.disable_item_and_upgrade_purchases {
            return false;
        }

        let cost = if self.stage_modifiers.free_shop_this_stage {
            0
        } else {
            match &slot.slot {
                crate::ShopSlotState::Item { cost, .. }
                | crate::ShopSlotState::Upgrade { cost, .. }
                | crate::ShopSlotState::CardService { cost, .. } => *cost,
            }
        };
        self.progress.gold >= cost
            && match &slot.slot {
                crate::ShopSlotState::CardService { kind, .. } => {
                    crate::CardServiceKind::from_raw(*kind).is_some_and(|kind| {
                        crate::game_state::card_service::purchase_is_available(kind, &self.deck)
                    })
                }
                _ => true,
            }
    }

    pub fn purchase_shop_item(
        &mut self,
        slot_index: usize,
    ) -> Result<crate::ShopPurchaseOutput, crate::CommandError> {
        match &self.flow {
            crate::GameFlowState::Shopping(shop) => {
                let Some(slot) = shop.slots.get(slot_index) else {
                    return Err(crate::CommandError::InvalidIndex);
                };
                if let crate::ShopSlotState::CardService { kind, .. } = &slot.slot {
                    crate::CardServiceKind::from_raw(*kind)
                        .ok_or(crate::CommandError::InvalidCardServiceKind { raw: *kind })?;
                }
            }
            _ => return Err(crate::CommandError::InvalidFlow),
        }
        if !self.can_purchase_shop_slot(slot_index) {
            return Err(crate::CommandError::Rejected);
        }

        let (slot_id, slot, cost) = {
            let crate::GameFlowState::Shopping(shop) = &mut self.flow else {
                return Err(crate::CommandError::InvalidFlow);
            };
            let slot = shop
                .slots
                .get_mut(slot_index)
                .ok_or(crate::CommandError::InvalidIndex)?;
            let cost = if self.stage_modifiers.free_shop_this_stage {
                0
            } else {
                match &slot.slot {
                    crate::ShopSlotState::Item { cost, .. }
                    | crate::ShopSlotState::Upgrade { cost, .. }
                    | crate::ShopSlotState::CardService { cost, .. } => *cost,
                }
            };
            slot.purchased = true;
            (slot.id, slot.slot.clone(), cost)
        };
        self.progress.gold = self.progress.gold.saturating_sub(cost);
        self.metrics.total_gold_spent = self.metrics.total_gold_spent.saturating_add(cost);
        self.trigger_shop_purchase_upgrades(matches!(slot, crate::ShopSlotState::Item { .. }));
        Ok(crate::ShopPurchaseOutput {
            slot_id,
            slot,
            cost,
        })
    }

    pub fn grant_inventory_item(
        &mut self,
        item: crate::ItemEntryState,
    ) -> Result<(), crate::CommandError> {
        crate::game_state::item::validate_item_payload_raw(&item)?;
        let next_id = self
            .items
            .iter()
            .map(|item| item.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.items.push(crate::ItemEntryState {
            id: next_id,
            ..item
        });
        Ok(())
    }

    pub fn earn_gold(&mut self, amount: usize) {
        self.progress.gold = self.progress.gold.saturating_add(amount);
        self.metrics.total_gold_earned = self.metrics.total_gold_earned.saturating_add(amount);
        self.trigger_gold_earned_upgrades();
    }

    pub fn grant_tower_card(&mut self, template: crate::TowerTemplateState) {
        if matches!(self.flow, crate::GameFlowState::PlacingTower) {
            let id = self.hand.allocate_slot_id();
            self.hand.slots.push(crate::HandSlotState {
                id,
                item: crate::HandItemState::Tower(template),
                selected: false,
            });
            self.hand.sort_slots();
        } else {
            self.stage_modifiers
                .extra_tower_cards
                .push(crate::StageModifierTowerCardState {
                    kind: template.kind,
                    suit: template.suit,
                    rank: template.rank,
                });
        }
    }

    pub fn can_use_inventory_item(&self, item_index: usize) -> bool {
        self.item_use_error(item_index).is_ok()
    }

    pub fn item_use_error(&self, item_index: usize) -> Result<(), crate::CommandError> {
        let item = self
            .items
            .get(item_index)
            .ok_or(crate::CommandError::InvalidIndex)?;
        let kind = crate::ItemKind::from_raw(item.kind)
            .ok_or(crate::CommandError::InvalidItemKind { raw: item.kind })?;
        self.item_use_error_with_kind(kind, item)
    }

    fn item_use_error_with_kind(
        &self,
        kind: crate::ItemKind,
        item: &crate::ItemEntryState,
    ) -> Result<(), crate::CommandError> {
        if self.stage_modifiers.disable_item_use {
            return Err(crate::CommandError::Rejected);
        }
        if !crate::game_state::item::can_use_item(self, kind) {
            return Err(crate::CommandError::Rejected);
        }
        crate::game_state::item::validate_item_payload(kind, item)
    }

    pub fn use_inventory_item(
        &mut self,
        item_index: usize,
    ) -> Result<crate::ItemEntryState, crate::CommandError> {
        let item = self
            .items
            .get(item_index)
            .ok_or(crate::CommandError::InvalidIndex)?;
        let kind = crate::ItemKind::from_raw(item.kind)
            .ok_or(crate::CommandError::InvalidItemKind { raw: item.kind })?;
        Ok(crate::game_state::item::apply_inventory_item_use(self, item_index, kind)?.item)
    }

    pub fn apply_inventory_item_use(
        &mut self,
        item_index: usize,
    ) -> Result<crate::game_state::item::ItemUseOutput, crate::CommandError> {
        let item = self
            .items
            .get(item_index)
            .ok_or(crate::CommandError::InvalidIndex)?;
        let kind = crate::ItemKind::from_raw(item.kind)
            .ok_or(crate::CommandError::InvalidItemKind { raw: item.kind })?;
        crate::game_state::item::apply_inventory_item_use(self, item_index, kind)
    }

    fn rubber_cone_template(&self) -> Result<crate::TowerTemplateState, crate::CommandError> {
        let config = self
            .config
            .towers
            .entries
            .iter()
            .find(|entry| entry.kind == 0)
            .ok_or(crate::CommandError::Rejected)?;
        Ok(crate::TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: config
                .cooldown_ms
                .saturating_mul(crate::SIM_TICKS_PER_SECOND)
                .div_ceil(1_000),
            default_attack_range_radius_raw: config.range_raw,
            default_damage_raw: config.damage_raw,
            suit: None,
            rank: None,
            skill_templates: Vec::new(),
            default_status_effects: Vec::new(),
            used_cards: Vec::new(),
        })
    }

    pub fn reroll_cards(
        &mut self,
        selected_slot_indices: &[usize],
    ) -> Result<usize, crate::CommandError> {
        if !matches!(self.flow, crate::GameFlowState::SelectingTower) {
            return Err(crate::CommandError::InvalidFlow);
        }
        let active_slots: Vec<usize> = self.hand.slots.iter().map(|slot| slot.id).collect();
        let slot_ids = if selected_slot_indices.is_empty() {
            active_slots
        } else {
            selected_slot_indices
                .iter()
                .map(|index| {
                    self.hand
                        .slots
                        .get(*index)
                        .map(|slot| slot.id)
                        .ok_or(crate::CommandError::InvalidIndex)
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        if slot_ids.is_empty() {
            return Err(crate::CommandError::InvalidSelection);
        }
        let health_cost_raw = self
            .stage_modifiers
            .reroll_health_cost
            .min(i64::MAX as usize) as i64
            * 1_000;
        if self.progress.left_dice == 0 && self.hp_raw.saturating_sub(health_cost_raw) <= 1_000 {
            return Err(crate::CommandError::InvalidSelection);
        }
        let mut cards = Vec::with_capacity(slot_ids.len());
        for slot_id in &slot_ids {
            let Some(slot) = self.hand.slots.iter().find(|slot| slot.id == *slot_id) else {
                return Err(crate::CommandError::InvalidIndex);
            };
            let crate::HandItemState::Card(card) = &slot.item else {
                return Err(crate::CommandError::InvalidSelection);
            };
            cards.push(card.clone());
        }
        self.hand.slots.retain(|slot| !slot_ids.contains(&slot.id));
        self.deck.discard(cards);
        let mut rng = self.rng.next_rng(
            crate::deterministic_rng::domain::CARD_REROLL,
            &[
                self.progress.stage as u64,
                self.progress.rerolled_count as u64,
            ],
        );
        let drawn = self.deck.draw(&mut rng, slot_ids.len());
        let draw_count = drawn.len();
        for card in drawn {
            let id = self.hand.allocate_slot_id();
            self.hand.slots.push(crate::HandSlotState {
                id,
                item: crate::HandItemState::Card(card),
                selected: false,
            });
        }
        self.hand.sort_slots();
        self.progress.left_dice = self.progress.left_dice.saturating_sub(1);
        self.progress.rerolled_count = self.progress.rerolled_count.saturating_add(1);
        self.apply_player_damage_raw(health_cost_raw);
        Ok(draw_count)
    }

    pub fn apply_player_damage_raw(&mut self, damage_raw: i64) -> i64 {
        let hp_before = self.hp_raw;
        let mut damage_after_shield_raw = damage_raw.max(0);
        if self.shield_raw > 0 {
            let absorbed = damage_after_shield_raw.min(self.shield_raw);
            damage_after_shield_raw = damage_after_shield_raw.saturating_sub(absorbed);
            self.shield_raw = self.shield_raw.saturating_sub(absorbed).max(0);
        }
        self.hp_raw = self.hp_raw.saturating_sub(damage_after_shield_raw).max(0);
        let actual_damage_raw = hp_before.saturating_sub(self.hp_raw);
        if actual_damage_raw > 0 {
            self.metrics.total_player_damage_raw = self
                .metrics
                .total_player_damage_raw
                .saturating_add(actual_damage_raw);
            if let Some((_, stage_damage)) = self
                .metrics
                .stage_damage
                .iter_mut()
                .find(|(stage, _)| *stage == self.progress.stage)
            {
                *stage_damage = stage_damage.saturating_add(actual_damage_raw);
            } else {
                self.metrics
                    .stage_damage
                    .push((self.progress.stage, actual_damage_raw));
            }
        }
        if let crate::GameFlowState::Defense(defense_flow) = &mut self.flow {
            defense_flow.took_damage = true;
        }
        actual_damage_raw
    }

    pub fn start_selecting_tower(&mut self) -> bool {
        crate::game_state::flow::start_selecting_tower(&mut self.flow)
    }

    pub fn start_defense(&mut self) -> bool {
        if !matches!(self.flow, crate::GameFlowState::PlacingTower) {
            return false;
        }
        self.start_defense_unchecked();
        true
    }

    pub fn force_start_defense(&mut self) {
        self.start_defense_unchecked();
    }

    fn start_defense_unchecked(&mut self) {
        let start_total_hp_raw = crate::game_state::monster_spawn::calculate_stage_total_hp_raw(
            self.progress.stage,
            &self.config,
            &self.stage_modifiers.enemy_health_multipliers_raw,
        );
        self.hand.slots.clear();
        self.flow = crate::GameFlowState::Defense(crate::DefenseFlowState {
            start_total_hp_raw,
            processed_hp_raw: 0,
            took_damage: false,
        });
        crate::game_state::monster_spawn::start_spawn(self);
        self.push_event(crate::CoreEvent::DefenseStarted {
            stage: self.progress.stage,
        });
    }

    /// Remove the tower with `tower_id` and recompute the monster route.
    pub fn remove_tower(&mut self, tower_id: u64) -> Option<crate::RemoveTowerOutput> {
        let index = self
            .towers
            .iter()
            .position(|tower| tower.id == Some(tower_id))?;
        let removed = self.towers.remove(index);
        self.recalculate_route();
        Some(crate::RemoveTowerOutput {
            rerolled_count: removed.template.rerolled_count,
        })
    }

    fn recalculate_route(&mut self) {
        let blockers = crate::game_state::tower::tower_blockers(&self.towers);
        self.route = crate::calculate_routes(&blockers, &crate::TRAVEL_POINTS, crate::MAP_SIZE)
            .expect("route should exist after placing or removing a tower");
    }

    pub(crate) fn start_stage_setup(&mut self, stage: usize) -> usize {
        self.begin_stage(stage);
        self.hand.slots.clear();
        self.draw_hand()
    }

    pub fn start_stage(&mut self, stage: usize) -> usize {
        let card_count = self.start_stage_setup(stage);
        self.start_shopping_flow();
        self.trigger_stage_start_upgrades(stage);
        self.push_event(crate::CoreEvent::StageStarted { stage, card_count });
        card_count
    }

    pub fn start_treasure_selection(&mut self) -> bool {
        if !matches!(self.flow, crate::GameFlowState::TreasureSelection { .. }) {
            self.flow = crate::GameFlowState::TreasureSelection {
                options: crate::game_state::upgrade::generate_boss_reward_options(self),
                pending_selection: None,
            };
            return true;
        }
        false
    }

    pub fn select_treasure(&mut self, option_index: usize) -> Result<(), crate::CommandError> {
        let upgrade = match &self.flow {
            crate::GameFlowState::TreasureSelection { options, .. } => options
                .get(option_index)
                .cloned()
                .ok_or(crate::CommandError::InvalidIndex)?,
            _ => return Err(crate::CommandError::InvalidFlow),
        };
        let upgrade_state = upgrade.clone();
        let acquire = self.acquire_upgrade(upgrade)?;
        self.apply_upgrade_recovery(acquire.recovery);
        let stage = self.progress.stage;
        let card_count = self.start_stage_setup(stage);
        self.start_shopping_flow();
        self.trigger_stage_start_upgrades(stage);
        self.push_event(crate::CoreEvent::TreasureSelected {
            upgrade: upgrade_state,
        });
        self.push_event(crate::CoreEvent::StageStarted { stage, card_count });
        Ok(())
    }

    pub fn resolve_defense_end(&mut self) -> Option<crate::DefenseEndOutputState> {
        if !self.is_defense_complete() {
            return None;
        }
        let perfect_clear = match &self.flow {
            crate::GameFlowState::Defense(flow) => !flow.took_damage,
            _ => return None,
        };
        let gold = self.progress.gold;
        let item_count = self.items.len();
        let transition = if self.advance_stage_after_defense() {
            crate::DefenseEndTransitionState::GameOver
        } else if Self::is_boss_stage(self.progress.stage) {
            crate::DefenseEndTransitionState::TreasureSelection
        } else {
            crate::DefenseEndTransitionState::StartStage {
                stage: self.progress.stage,
            }
        };
        Some(crate::DefenseEndOutputState {
            perfect_clear,
            gold,
            item_count,
            card_count: 0,
            transition,
        })
    }

    pub fn apply_defense_end_transition(
        &mut self,
        transition: crate::DefenseEndTransitionState,
    ) -> usize {
        match transition {
            crate::DefenseEndTransitionState::GameOver => {
                self.clear_active_entities();
                self.flow = crate::GameFlowState::Result {
                    clear_rate_raw: self.clear_rate_raw(),
                };
                0
            }
            crate::DefenseEndTransitionState::TreasureSelection => {
                self.start_treasure_selection();
                0
            }
            crate::DefenseEndTransitionState::StartStage { stage } => {
                let card_count = self.start_stage_setup(stage);
                self.start_shopping_flow();
                self.trigger_stage_start_upgrades(stage);
                card_count
            }
        }
    }

    fn begin_stage(&mut self, stage: usize) {
        self.stage_modifiers.reset_stage_state();
        if !self.upgrades.upgrades.iter().any(|upgrade| {
            upgrade
                .upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::Spanner)
        }) {
            self.shield_raw = 0;
        }
        self.progress.item_used = false;
        self.metrics.total_rerolled_count = self
            .metrics
            .total_rerolled_count
            .saturating_add(self.progress.rerolled_count);
        self.progress.rerolled_count = 0;
        let mut rng = self.rng.next_rng(
            crate::deterministic_rng::domain::DECK_SHUFFLE,
            &[stage as u64],
        );
        self.deck.prepare_draw_pile(&mut rng);
        self.progress.left_dice = self.max_dice_chance();
        self.progress.stage = stage;
    }

    fn draw_hand(&mut self) -> usize {
        let max_slots = (self.config.player.base_hand_slots
            + self.stage_modifiers.card_selection_hand_max_slots_bonus)
            .saturating_sub(self.stage_modifiers.card_selection_hand_max_slots_penalty)
            .max(1);
        let mut rng = self.rng.next_rng(
            crate::deterministic_rng::domain::DECK_DRAW,
            &[self.progress.stage as u64],
        );
        let cards = self.deck.draw(&mut rng, max_slots);
        for card in cards {
            let id = self.hand.allocate_slot_id();
            self.hand.slots.push(crate::HandSlotState {
                id,
                item: crate::HandItemState::Card(card),
                selected: false,
            });
        }
        self.hand.sort_slots();
        max_slots
    }

    pub fn max_dice_chance(&self) -> usize {
        let dice_bonus = self
            .upgrades
            .upgrades
            .iter()
            .filter(|upgrade| {
                upgrade
                    .upgrade_kind()
                    .is_ok_and(|kind| kind == crate::UpgradeKind::DiceBundle)
            })
            .map(|upgrade| {
                upgrade
                    .scalar_values
                    .first()
                    .copied()
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or(0)
            })
            .sum::<usize>();
        (dice_bonus
            + self.config.player.base_dice_chance
            + self.stage_modifiers.max_dice_rerolls_bonus)
            .saturating_sub(self.stage_modifiers.max_dice_rerolls_penalty)
    }

    fn is_defense_complete(&self) -> bool {
        matches!(self.flow, crate::GameFlowState::Defense(_))
            && self.monster_spawn.monster_queue.is_empty()
            && self.monster_spawn.next_spawn_tick.is_none()
            && self.monsters.is_empty()
    }

    fn advance_stage_after_defense(&mut self) -> bool {
        self.progress.stage = self.progress.stage.saturating_add(1);
        if self.progress.stage > self.config.player.max_stages {
            self.progress.stage = self.progress.stage.saturating_sub(1);
            true
        } else {
            false
        }
    }

    fn clear_active_entities(&mut self) {
        self.monsters.clear();
        self.in_flight_attacks.clear();
    }

    fn is_boss_stage(stage: usize) -> bool {
        stage.is_multiple_of(5) || (46..=49).contains(&stage)
    }

    fn record_accepted_player_command_with_event_metadata(
        &mut self,
        command: crate::game_state::command::PlayerCommand,
        state_hash: String,
        event_count: u64,
        event_digest: String,
    ) {
        let sequence = self.progress.player_command_sequence;
        self.progress.player_command_sequence = sequence.wrapping_add(1);
        let completed_sim_tick = self.sim_tick.ticks();
        self.player_commands
            .push(crate::game_state::command::RecordedPlayerCommand {
                sequence,
                completed_sim_tick,
                command,
            });
        self.replay_checkpoints
            .push(crate::game_state::replay::ReplayCheckpoint {
                sequence,
                completed_sim_tick,
                state_hash,
                event_count,
                event_digest,
            });
    }

    pub(crate) fn record_accepted_command(
        &mut self,
        command: crate::game_state::command::PlayerCommand,
    ) -> crate::CommandReceipt {
        let sequence = self.progress.player_command_sequence;
        let completed_sim_tick = self.sim_tick.ticks();
        let state_hash = crate::authoritative_hash(self);
        let events: Vec<crate::CoreEvent> = self.drain_events().collect();
        let (event_count, event_digest) = crate::game_state::replay::event_metadata(&events);
        self.record_accepted_player_command_with_event_metadata(
            command.clone(),
            state_hash.clone(),
            event_count,
            event_digest.clone(),
        );
        crate::CommandReceipt {
            sequence,
            completed_sim_tick,
            command,
            state_hash,
            events,
            event_count,
            event_digest,
        }
    }

    pub(crate) fn advance_sim_tick(&mut self) -> crate::SimTick {
        self.sim_tick += crate::SimTickSpan::ONE;
        self.sim_tick
    }

    pub(crate) fn advance_and_step_sim_tick(&mut self) -> Option<crate::DefenseEndOutputState> {
        self.advance_sim_tick();
        self.step_sim_tick()
    }

    pub(crate) fn advance_tick(&mut self) -> RecordedTickOutput {
        let defense_end = self.advance_and_step_sim_tick();
        let events: Vec<crate::CoreEvent> = self.drain_events().collect();
        let state_hash = crate::authoritative_hash(self);
        let (event_count, event_digest) = crate::game_state::replay::event_metadata(&events);
        RecordedTickOutput {
            sim_tick: self.sim_tick,
            events,
            defense_end,
            state_hash,
            event_count,
            event_digest,
        }
    }

    pub(crate) fn advance_tick_unrecorded(&mut self) -> TickTransition {
        let defense_end = self.advance_and_step_sim_tick();
        self.events.events.clear();
        TickTransition {
            sim_tick: self.sim_tick,
            defense_end,
        }
    }

    pub(crate) fn advance_tick_with_events(&mut self) -> TickEventsOutput {
        let defense_end = self.advance_and_step_sim_tick();
        let events = self.drain_events().collect();
        TickEventsOutput {
            sim_tick: self.sim_tick,
            events,
            defense_end,
        }
    }

    /// Perform a full simulation tick in a single authoritative step.
    /// Returns the defense-end output if the defense phase ended, otherwise `None`.
    pub(crate) fn step_sim_tick(&mut self) -> Option<crate::DefenseEndOutputState> {
        // 1. Pre-combat: spawn monsters, advance tower cooldowns, expire effects,
        //    activate monster/tower skills, move monsters.
        self.advance_pre_combat();

        // 2. Advance in-flight attacks and resolve hits.
        //    Returns Vec<Vec<ResolvedAttack>> (one batch per attack kind).
        let attack_batches = self.advance_in_flight_attacks();
        for batch in &attack_batches {
            let (hits, sources) = resolved_attacks_to_damage_hits(batch);
            let deaths = self.apply_damage_hits(hits, &sources);
            if !deaths.is_empty() {
                self.trigger_monster_death_upgrades();
            }
        }

        // 3. Tower attacks.
        let tower_output = self.generate_tower_attacks();
        if !tower_output.area_damage_events.is_empty() {
            let area_damage_sources: Vec<Option<crate::AttackSourceState>> = tower_output
                .area_damage_sources
                .into_iter()
                .map(Some)
                .collect();
            let deaths = self
                .apply_area_damage_events(tower_output.area_damage_events, &area_damage_sources);
            if !deaths.is_empty() {
                self.trigger_monster_death_upgrades();
            }
        }

        // 4. Resolve base damage from escaped monsters.
        self.resolve_base_damage();
        if self.hp_raw == 0 && matches!(self.flow, crate::GameFlowState::Defense(_)) {
            let clear_rate_raw = self.clear_rate_raw();
            self.clear_active_entities();
            self.flow = crate::GameFlowState::Result { clear_rate_raw };
            self.push_event(crate::CoreEvent::GameFinished { victory: false });
            return None;
        }

        // 5. Check defense end.
        let completed_stage = self.progress.stage;
        let mut defense_end = self.resolve_defense_end()?;
        self.push_event(crate::CoreEvent::DefenseEnded {
            stage: completed_stage,
            perfect_clear: defense_end.perfect_clear,
            transition: defense_end.transition,
        });
        self.update_clear_metrics(defense_end.perfect_clear);
        self.trigger_stage_end_upgrades(
            defense_end.perfect_clear,
            defense_end.gold,
            defense_end.item_count,
        );
        defense_end.card_count = self.apply_defense_end_transition(defense_end.transition);
        match defense_end.transition {
            crate::DefenseEndTransitionState::GameOver => {
                self.push_event(crate::CoreEvent::GameFinished { victory: true });
            }
            crate::DefenseEndTransitionState::StartStage { stage } => {
                self.push_event(crate::CoreEvent::StageStarted {
                    stage,
                    card_count: defense_end.card_count,
                });
            }
            crate::DefenseEndTransitionState::TreasureSelection => {}
        }
        Some(defense_end)
    }

    pub fn update_clear_metrics(&mut self, perfect_clear: bool) {
        if perfect_clear {
            self.metrics.current_consecutive_perfect_clears = self
                .metrics
                .current_consecutive_perfect_clears
                .saturating_add(1);
            self.metrics.max_consecutive_perfect_clears = self
                .metrics
                .max_consecutive_perfect_clears
                .max(self.metrics.current_consecutive_perfect_clears);
        } else {
            self.metrics.current_consecutive_perfect_clears = 0;
        }
    }

    pub(crate) fn advance_monsters(&mut self, enemy_speed_multiplier_raw: i64) {
        crate::advance_monster_states(&mut self.monsters, enemy_speed_multiplier_raw);
    }

    pub(crate) fn enemy_speed_multiplier_raw(&self) -> i64 {
        crate::apply_ratio_product_raw(
            crate::RATIO_SCALE,
            &self.stage_modifiers.enemy_speed_multipliers_raw,
        )
    }

    pub(crate) fn advance_towers(&mut self) {
        crate::advance_tower_cooldowns(&mut self.towers);
    }

    pub(crate) fn expire_effects(&mut self) -> Vec<u64> {
        let damage_refresh_tower_ids =
            crate::remove_expired_tower_statuses(&mut self.towers, self.sim_tick.ticks());
        crate::remove_expired_monster_statuses(&mut self.monsters, self.sim_tick.ticks());
        crate::remove_expired_user_status_effects(
            &mut self.user_status_effects,
            self.sim_tick.ticks(),
        );
        damage_refresh_tower_ids
    }

    pub(crate) fn activate_skills(&mut self) {
        let monster_activations =
            crate::activate_monster_skills(&mut self.monsters, self.sim_tick.ticks());
        crate::apply_monster_skill_activations(
            &mut self.monsters,
            &monster_activations,
            self.sim_tick.ticks(),
        );

        let tower_activations =
            crate::activate_tower_skills(&mut self.towers, self.sim_tick.ticks());
        crate::apply_tower_skill_activations(
            &mut self.towers,
            &mut self.monsters,
            &tower_activations,
            self.sim_tick.ticks(),
        );
    }

    pub(crate) fn advance_pre_combat(&mut self) -> PreCombatOutput {
        let monster_spawned = crate::game_state::monster_spawn::spawn_due(self);
        self.advance_towers();
        let damage_refresh_tower_ids = self.expire_effects();
        self.activate_skills();
        self.advance_monsters(self.enemy_speed_multiplier_raw());
        PreCombatOutput {
            monster_spawned,
            damage_refresh_tower_ids,
        }
    }

    pub(crate) fn advance_in_flight_attacks(&mut self) -> Vec<Vec<crate::ResolvedAttack>> {
        crate::advance_in_flight_attacks_with_events(
            &mut self.in_flight_attacks,
            &self.monsters,
            self.sim_tick.ticks(),
            &mut self.events.events,
        )
    }

    pub(crate) fn generate_tower_attacks(&mut self) -> crate::TowerAttackOutput {
        let output = crate::generate_tower_attacks(
            &mut self.towers,
            &self.monsters,
            &mut self.in_flight_attacks,
            &mut self.next_entity_id,
            self.sim_tick.ticks(),
            &self.stage_modifiers.disabled_ranks,
            &self.stage_modifiers.disabled_suits,
        );
        self.events.events.extend(output.events.iter().cloned());
        output
    }

    pub fn apply_damage_hits(
        &mut self,
        hits: Vec<crate::DamageHit>,
        sources: &[Option<crate::AttackSourceState>],
    ) -> Vec<(crate::RemovedMonster, [i64; 2])> {
        let monster_centers = self
            .monsters
            .iter()
            .map(|monster| {
                [
                    monster.move_on_route.map_coord[0]
                        .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
                    monster.move_on_route.map_coord[1]
                        .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
                ]
            })
            .collect::<Vec<_>>();
        let monster_ids = self
            .monsters
            .iter()
            .map(|monster| monster.id)
            .collect::<Vec<_>>();
        let mut hits = crate::expand_on_hit_splashes(&monster_centers, hits);
        hits.sort_by_key(|hit| crate::damage_hit_sort_key(&monster_ids, hit.target_index));

        let mut deaths = Vec::new();
        for hit in hits {
            let damage = {
                let Some(monster) = self.monsters.get_mut(hit.target_index) else {
                    continue;
                };
                crate::apply_monster_damage(monster, hit.damage_raw)
            };
            if damage.applied_damage_raw > 0 {
                self.push_event(crate::CoreEvent::DamageApplied {
                    target_id: self.monsters[hit.target_index].id,
                    amount: damage.applied_damage_raw,
                    position: hit.at_xy,
                });
            }
            self.record_processed_monster_hp(damage.applied_damage_raw);
            if hit.damage_raw > 0
                && let Some(Some(source)) = sources.get(hit.source_index)
            {
                self.record_tower_damage(source, hit.damage_raw);
            }
            if !damage.dead {
                continue;
            }
            let Some(removed) = crate::remove_dead_monster(&mut self.monsters, hit.target_index)
            else {
                continue;
            };
            self.earn_monster_reward(removed.monster.reward);
            if removed.death.should_count_stage_progress {
                self.record_processed_monster_hp(removed.death.remaining_hp_raw);
            }
            self.push_event(crate::CoreEvent::MonsterDefeated {
                monster_id: removed.monster.id,
                position: hit.at_xy,
                monster_kind: removed.monster.kind,
                reward: removed.monster.reward,
                rotation_milliradians: 0,
            });
            deaths.push((removed, hit.at_xy));
        }
        deaths
    }

    pub(crate) fn apply_area_damage_events(
        &mut self,
        events: Vec<crate::AreaDamageEvent>,
        sources: &[Option<crate::AttackSourceState>],
    ) -> Vec<(crate::RemovedMonster, [i64; 2])> {
        let monster_centers = self
            .monsters
            .iter()
            .map(|monster| {
                [
                    monster.move_on_route.map_coord[0]
                        .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
                    monster.move_on_route.map_coord[1]
                        .saturating_add(crate::WORLD_UNITS_PER_TILE / 2),
                ]
            })
            .collect::<Vec<_>>();
        let hits = crate::expand_area_damage_events(&monster_centers, events);
        self.apply_damage_hits(hits, sources)
    }

    fn record_tower_damage(&mut self, source: &crate::AttackSourceState, damage_raw: i64) {
        if damage_raw <= 0 {
            return;
        }
        if let Some(entry) = self
            .metrics
            .tower_damage_stats
            .iter_mut()
            .find(|entry| entry.tower_id == source.tower_id)
        {
            entry.total_damage_raw = entry.total_damage_raw.saturating_add(damage_raw);
            return;
        }
        self.metrics
            .tower_damage_stats
            .push(crate::TowerDamageStats {
                tower_id: source.tower_id,
                tower_kind: source.tower_kind,
                rank: source.rank,
                suit: source.suit,
                total_damage_raw: damage_raw,
            });
    }

    fn earn_monster_reward(&mut self, reward: usize) {
        let earned = crate::apply_ratio_product_raw(
            reward.min(i64::MAX as usize) as i64,
            &self.stage_modifiers.gold_gain_multipliers_raw,
        ) as usize;
        self.progress.gold = self.progress.gold.saturating_add(earned);
        self.metrics.total_gold_earned = self.metrics.total_gold_earned.saturating_add(earned);
        self.trigger_gold_earned_upgrades();
    }

    fn record_processed_monster_hp(&mut self, amount_raw: i64) {
        if amount_raw <= 0 {
            return;
        }
        if let crate::GameFlowState::Defense(defense_flow) = &mut self.flow {
            defense_flow.processed_hp_raw =
                defense_flow.processed_hp_raw.saturating_add(amount_raw);
        }
    }

    pub(crate) fn resolve_base_damage(&mut self) -> Option<(i64, i64)> {
        let escape = crate::resolve_monster_escapes(&mut self.monsters);
        self.metrics.total_escaped_hp_raw = self
            .metrics
            .total_escaped_hp_raw
            .saturating_add(escape.escaped_hp_raw);
        self.record_processed_monster_hp(escape.escaped_hp_raw);

        let damage_raw = crate::adjust_incoming_damage(
            escape.damage_raw,
            &self.user_status_effects,
            &self.stage_modifiers.damage_reduction_multipliers_raw,
            &self.stage_modifiers.incoming_damage_multipliers_raw,
        );
        if damage_raw == 0 {
            return None;
        }

        let hp_before = self.hp_raw;
        let mut damage_after_shield_raw = damage_raw;
        if self.shield_raw > 0 {
            let absorbed = damage_raw.min(self.shield_raw);
            damage_after_shield_raw = damage_raw.saturating_sub(absorbed);
            self.shield_raw = self.shield_raw.saturating_sub(absorbed).max(0);
        }
        self.hp_raw = self.hp_raw.saturating_sub(damage_after_shield_raw).max(0);
        let actual_damage_raw = hp_before.saturating_sub(self.hp_raw);
        if actual_damage_raw > 0 {
            self.metrics.total_player_damage_raw = self
                .metrics
                .total_player_damage_raw
                .saturating_add(actual_damage_raw);
            if let Some((_, stage_damage)) = self
                .metrics
                .stage_damage
                .iter_mut()
                .find(|(stage, _)| *stage == self.progress.stage)
            {
                *stage_damage = stage_damage.saturating_add(actual_damage_raw);
            } else {
                self.metrics
                    .stage_damage
                    .push((self.progress.stage, actual_damage_raw));
            }
        }
        if let crate::GameFlowState::Defense(defense_flow) = &mut self.flow {
            defense_flow.took_damage = true;
        }
        self.push_event(crate::CoreEvent::BaseDamageApplied {
            amount: damage_raw,
            actual_amount: actual_damage_raw,
        });

        Some((damage_raw, actual_damage_raw))
    }
}

fn resolved_attacks_to_damage_hits(
    attacks: &[crate::ResolvedAttack],
) -> (Vec<crate::DamageHit>, Vec<Option<crate::AttackSourceState>>) {
    let mut sources: Vec<Option<crate::AttackSourceState>> = Vec::new();
    let mut hits: Vec<crate::DamageHit> = Vec::with_capacity(attacks.len());
    for resolved in attacks {
        let source_index = if let Some(source) = &resolved.attack.source_tower {
            let idx = sources.len();
            sources.push(Some(*source));
            idx
        } else {
            // No source tower — use a sentinel index that won't be looked up.
            // apply_damage_hits only accesses sources when damage_raw > 0,
            // and tower damage stats only record when source exists.
            usize::MAX
        };
        hits.push(crate::DamageHit {
            target_index: resolved.target_index,
            damage_raw: resolved.attack.damage_raw,
            at_xy: resolved.at_xy,
            source_index,
            splashes: resolved.attack.on_hit_splashes.clone(),
        });
    }
    (hits, sources)
}

fn divide_round_positive(numerator: i128, denominator: i128) -> i64 {
    if numerator <= 0 || denominator <= 0 {
        return 0;
    }
    numerator
        .saturating_add(denominator / 2)
        .saturating_div(denominator)
        .min(i128::from(i64::MAX)) as i64
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EntitySnapshots {
    pub monsters: Vec<crate::MonsterState>,
    pub towers: Vec<crate::TowerState>,
}

pub use crate::{
    CardState, DefenseFlowState, GameFlowState, HandItemState, HandSlotState, HandState,
    ItemEntryState, MonsterSpawnState, MonsterState, ShopPurchaseOutput, ShopSlotDataState,
    ShopSlotState, ShopState, TowerState, TowerTemplateState, UpgradeCacheState,
    UpgradeCollectionState, UpgradeEntryIdentityState, UpgradeEntryState,
};
