use crate::{
    DecisionPoint, StageModifiersObservation, TowerStatusEffectEnd, TowerStatusEffectKind,
};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardObservation {
    pub id: usize,
    pub suit: String,
    pub rank: String,
    pub polish_pct_raw: i64,
    pub engraving: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeckObservation {
    pub all_cards: Vec<CardObservation>,
    pub draw_cards: Vec<CardObservation>,
    pub discard_cards: Vec<CardObservation>,
}

/// A single splash damage effect: a radius and a percentage of the
/// triggering attack's damage applied to everything within that radius.
/// Semantic observation counterpart of `crate::DamageSplash`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DamageSplashObservation {
    pub radius_raw: i64,
    pub damage_pct_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerTemplateObservation {
    pub kind: String,
    pub kind_id: u16,
    pub suit: Option<String>,
    pub rank: Option<String>,
    pub rerolled_count: usize,
    pub damage_raw: i64,
    /// Deterministic damage this template would deal with every currently
    /// known, template-derivable modifier applied (card polish plus
    /// `UpgradeCollection::tower_upgrade_bonus_raw_for_template`). Excludes
    /// placement-trigger-only bonuses that only resolve once a tower ID
    /// exists (e.g. `NameTag`) and runtime-only status effects, neither of
    /// which exist before `PlaceTower` runs - see
    /// `docs/game-ai/03-observation-contract.md`.
    pub effective_damage_raw: i64,
    pub range_raw: i64,
    pub shoot_interval_ticks: u64,
    pub used_cards: Vec<CardObservation>,
    /// On-hit splash effects a tower placed from this template would carry,
    /// derived from `TowerTemplateState::derived_on_hit_splashes` - the same
    /// authoritative helper `place_tower_with_template` uses.
    pub on_hit_splashes: Vec<DamageSplashObservation>,
    /// On-attack splash effects a tower placed from this template would
    /// carry (e.g. Cactus engraving), derived from
    /// `TowerTemplateState::derived_on_attack_splashes` - the same
    /// authoritative helper `place_tower_with_template` uses.
    pub on_attack_splashes: Vec<DamageSplashObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TowerStatusEffectObservationKind {
    DamageMul { mul_raw: i64 },
    DamageAdd { add_raw: i64 },
}

/// Semantic observation counterpart of `crate::TowerStatusEffect`: the
/// absolute `end_at` tick is converted to a decision-relative
/// `remaining_ticks` so the policy doesn't need `sim_tick` to interpret it.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerStatusEffectObservation {
    pub kind: TowerStatusEffectObservationKind,
    pub remaining_ticks: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HandItemObservation {
    Card(CardObservation),
    Tower(TowerTemplateObservation),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HandObservation {
    pub index: usize,
    pub selected: bool,
    pub item: HandItemObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BuildTowerCandidateObservation {
    pub card_ids: Vec<usize>,
    pub template: TowerTemplateObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardServiceObservation {
    pub key: String,
    pub current_step: usize,
    pub step_count: usize,
    pub required_count: usize,
    pub selected_card_indices: Vec<usize>,
    pub candidate_card_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RouteCoordObservation {
    pub x: usize,
    pub y: usize,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShopSlotObservation {
    pub index: usize,
    pub purchased: bool,
    pub kind: String,
    pub kind_id: u16,
    pub key: String,
    pub key_id: u16,
    pub cost: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InventoryObservation {
    pub index: usize,
    pub key: String,
    pub key_id: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OwnedUpgradeObservation {
    pub id: u64,
    pub key: String,
    pub key_id: u16,
    #[serde(default)]
    pub scalar_values: Vec<usize>,
    #[serde(default)]
    pub ratio_values: Vec<i64>,
    #[serde(default)]
    pub bool_values: Vec<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerObservation {
    pub id: u64,
    pub left: usize,
    pub top: usize,
    pub template: TowerTemplateObservation,
    pub cooldown_ticks: u64,
    pub range_raw: i64,
    /// Current actual attack damage of this placed tower, i.e.
    /// `TowerState::attack_damage_raw()` at observation time - includes
    /// runtime-only status effects (`status_effects`), unlike
    /// `template.effective_damage_raw` which is a pre-placement preview.
    pub attack_damage_raw: i64,
    /// Full variable-cardinality set of currently active damage status
    /// effects on this tower, in a deterministic order independent of the
    /// runtime `Vec` order - see `docs/game-ai/03-observation-contract.md`.
    pub status_effects: Vec<TowerStatusEffectObservation>,
    /// Actual runtime on-hit splash effects this placed tower currently
    /// carries (`TowerState::on_hit_splashes`), as opposed to
    /// `template.on_hit_splashes`'s pre-placement preview.
    pub on_hit_splashes: Vec<DamageSplashObservation>,
    /// Actual runtime on-attack splash effects this placed tower currently
    /// carries (`TowerState::on_attack_splashes`), as opposed to
    /// `template.on_attack_splashes`'s pre-placement preview.
    pub on_attack_splashes: Vec<DamageSplashObservation>,
}

/// One run-length-encoded contiguous group of monsters of the same kind and
/// authoritative spawn stats, in the group's spawn order. See
/// `docs/game-ai/03-observation-contract.md` - "웨이브와 장기 상태" - for the
/// distinction between `stage_wave` (the current stage's full configured
/// composition) and `queued_wave` (actual remaining runtime spawn queue).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WaveGroupObservation {
    pub order_index: usize,
    pub kind: String,
    pub kind_id: u16,
    pub count: usize,
    pub max_hp_raw: i64,
    pub velocity_raw: i64,
    pub damage_raw: i64,
    pub reward: usize,
}

/// A contiguous run of not-yet-spawned monsters from the actual
/// `MonsterSpawnState.monster_queue`, in queue order. Deliberately excludes
/// per-monster entity IDs (see the hidden-information boundary in
/// `docs/game-ai/03-observation-contract.md`) but preserves exact queue
/// ordering and composition: two same-kind runs separated by a different
/// kind are never merged into one group.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueuedMonsterGroupObservation {
    pub order_index: usize,
    pub kind: String,
    pub kind_id: u16,
    pub count: usize,
    pub max_hp_raw: i64,
    pub velocity_raw: i64,
    pub damage_raw: i64,
    pub reward: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterObservation {
    pub id: u64,
    pub kind: String,
    pub kind_id: u16,
    pub route_index: usize,
    pub route_progress_raw: i64,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
    pub velocity_raw: i64,
    pub damage_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Observation {
    #[serde(default)]
    pub observation_schema_version: u32,
    #[serde(default)]
    pub catalog_schema_version: u32,
    #[serde(default)]
    pub action_wire_schema_version: u32,
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub decision_point: DecisionPoint,
    pub item_window_stage: Option<usize>,
    pub damage_trigger_tick: Option<u64>,
    pub card_selection_purpose: Option<String>,
    pub selected_hand_slot_indices: Vec<usize>,
    pub card_selection_confirmable: bool,
    pub stage: usize,
    pub sim_tick: u64,
    pub theme: Option<String>,
    pub hp_raw: i64,
    pub max_hp_raw: i64,
    pub shield_raw: i64,
    pub gold: usize,
    pub left_dice: usize,
    pub rerolled_count: usize,
    pub stage_progress_raw: i64,
    pub stage_total_hp_raw: i64,
    pub active_monster_count: usize,
    pub queued_monster_count: usize,
    /// Current stage's full configured wave composition (`config.monsters.
    /// stage_waves` entries for `stage`), in config entry order - including
    /// groups already spawned. Available in every decision point, including
    /// Shopping/CardSelection/TowerPlacement, since it is deterministic
    /// public configuration, not hidden RNG. See `queued_wave` for the
    /// actual remaining runtime spawn queue.
    #[serde(default)]
    pub stage_wave: Vec<WaveGroupObservation>,
    /// Actual remaining runtime spawn queue
    /// (`MonsterSpawnState.monster_queue`), run-length encoded in exact
    /// queue order with no monster entity IDs. Empty before defense starts
    /// or once the queue is exhausted. `queued_monster_count` always equals
    /// the sum of these groups' `count`.
    #[serde(default)]
    pub queued_wave: Vec<QueuedMonsterGroupObservation>,
    /// Ticks between spawns for the current defense, mirroring
    /// `MonsterSpawnState.spawn_interval_ticks`.
    #[serde(default)]
    pub spawn_interval_ticks: u64,
    /// Ticks remaining until the next spawn (`next_spawn_tick - sim_tick`),
    /// or `None` if defense hasn't started or no future spawn is scheduled.
    #[serde(default)]
    pub next_spawn_in_ticks: Option<u64>,
    pub hand: Vec<HandObservation>,
    #[serde(default)]
    pub build_tower_candidates: Vec<BuildTowerCandidateObservation>,
    /// Preview of the tower template each pending `stage_modifiers`
    /// `extra_tower_cards` entry will resolve to once `BuildTower`
    /// selects a card subset - these towers don't depend on which cards
    /// are selected (`start_placing_tower_from_template` builds them with
    /// no `used_cards`), so this can be computed ahead of that selection.
    /// Index `i` corresponds to `AgentAction::BuildTower`'s
    /// `hand_slot_index == i + 1` (`hand_slot_index == 0` is always the
    /// selected card subset's own template, in `build_tower_candidates`).
    #[serde(default)]
    pub extra_tower_card_templates: Vec<TowerTemplateObservation>,
    pub deck: DeckObservation,
    pub shop: Vec<ShopSlotObservation>,
    pub inventory: Vec<InventoryObservation>,
    pub item_capacity: usize,
    pub owned_upgrades: Vec<OwnedUpgradeObservation>,
    pub treasure_capacity: usize,
    pub discardable_treasure_ids: Vec<u64>,
    pub towers: Vec<TowerObservation>,
    pub tower_grid: Vec<Option<u64>>,
    pub map_width: usize,
    pub map_height: usize,
    pub route_coords: Vec<RouteCoordObservation>,
    pub monsters: Vec<MonsterObservation>,
    pub treasure_options: Vec<String>,
    pub card_service: Option<CardServiceObservation>,
    pub stage_modifiers: StageModifiersObservation,
}

impl crate::CoreState {
    pub fn observation(
        &self,
        environment_version: u32,
        action_schema_version: u32,
        map_width: usize,
        map_height: usize,
    ) -> Observation {
        let (stage_progress_raw, stage_total_hp_raw) = match &self.flow {
            crate::GameFlowState::Defense(flow) => (
                flow.processed_hp_raw,
                crate::game_state::monster_spawn::calculate_stage_total_hp_raw(
                    self.progress.stage,
                    &self.config,
                    &self.stage_modifiers.enemy_health_multipliers_raw,
                ),
            ),
            _ => (
                0,
                crate::game_state::monster_spawn::calculate_stage_total_hp_raw(
                    self.progress.stage,
                    &self.config,
                    &self.stage_modifiers.enemy_health_multipliers_raw,
                ),
            ),
        };

        let hand = self
            .hand
            .slots
            .iter()
            .enumerate()
            .map(|(index, slot)| HandObservation {
                index,
                selected: slot.selected,
                item: match &slot.item {
                    crate::HandItemState::Card(card) => {
                        HandItemObservation::Card(card_observation(card))
                    }
                    crate::HandItemState::Tower(tower) => {
                        let upgrade_bonus_raw =
                            self.upgrades.tower_upgrade_bonus_raw_for_template(tower);
                        HandItemObservation::Tower(tower_template_observation(
                            tower,
                            upgrade_bonus_raw,
                        ))
                    }
                },
            })
            .collect();
        let build_tower_candidates = build_tower_candidates(self);
        let extra_tower_card_templates = extra_tower_card_templates(self);

        let (shop, treasure_options) = match &self.flow {
            crate::GameFlowState::Shopping(flow) => (
                flow.slots
                    .iter()
                    .enumerate()
                    .map(|(index, slot)| shop_slot_observation(self, index, slot))
                    .collect(),
                Vec::new(),
            ),
            crate::GameFlowState::TreasureSelection { options, .. } => (
                Vec::new(),
                options
                    .iter()
                    .map(|upgrade| upgrade_key(upgrade.kind().raw()).0.to_string())
                    .collect(),
            ),
            _ => (Vec::new(), Vec::new()),
        };

        let sim_tick = self.sim_tick.ticks();
        let towers = self
            .towers
            .iter()
            .filter_map(|tower| tower_observation(self, tower, sim_tick))
            .collect::<Vec<_>>();
        let mut tower_grid = vec![None; map_width.saturating_mul(map_height)];
        for tower in &towers {
            for row in tower.top..tower.top.saturating_add(2) {
                for column in tower.left..tower.left.saturating_add(2) {
                    if row < map_height && column < map_width {
                        tower_grid[row * map_width + column] = Some(tower.id);
                    }
                }
            }
        }

        let mut monsters = self
            .monsters
            .iter()
            .map(|monster| {
                let (kind, kind_id) = monster_kind(monster.kind);
                MonsterObservation {
                    id: monster.id,
                    kind: kind.to_string(),
                    kind_id,
                    route_index: monster.move_on_route.route_index,
                    route_progress_raw: monster.move_on_route.route_progress_raw,
                    hp_raw: monster.hp_raw,
                    max_hp_raw: monster.max_hp_raw,
                    velocity_raw: monster.move_on_route.velocity_raw,
                    damage_raw: monster.damage_raw,
                }
            })
            .collect::<Vec<_>>();
        monsters.sort_by_key(|monster| monster.id);

        Observation {
            observation_schema_version: crate::OBSERVATION_SCHEMA_VERSION,
            catalog_schema_version: crate::CATALOG_SCHEMA_VERSION,
            action_wire_schema_version: crate::ACTION_WIRE_SCHEMA_VERSION,
            environment_version,
            action_schema_version,
            decision_point: decision_point_from_flow(&self.flow),
            item_window_stage: None,
            damage_trigger_tick: None,
            card_selection_purpose: None,
            selected_hand_slot_indices: Vec::new(),
            card_selection_confirmable: false,
            stage: self.progress.stage,
            sim_tick: self.sim_tick.ticks(),
            theme: None,
            hp_raw: self.hp_raw,
            max_hp_raw: self.max_hp_raw(),
            shield_raw: self.shield_raw,
            gold: self.progress.gold,
            left_dice: self.progress.left_dice,
            rerolled_count: self.progress.rerolled_count,
            stage_progress_raw,
            stage_total_hp_raw,
            active_monster_count: self.monsters.len(),
            queued_monster_count: self.monster_spawn.monster_queue.len(),
            stage_wave: stage_wave_observation(self),
            queued_wave: queued_wave_observation(self),
            spawn_interval_ticks: self.monster_spawn.spawn_interval_ticks,
            next_spawn_in_ticks: self
                .monster_spawn
                .next_spawn_tick
                .map(|next_spawn_tick| next_spawn_tick.saturating_sub(sim_tick)),
            hand,
            build_tower_candidates,
            extra_tower_card_templates,
            deck: DeckObservation {
                all_cards: self.deck.all_cards.iter().map(card_observation).collect(),
                draw_cards: unordered_cards(&self.deck.draw_pile),
                discard_cards: unordered_cards(&self.deck.discard_pile),
            },
            shop,
            inventory: self
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let (key, key_id) = item_key(item.kind().raw());
                    InventoryObservation {
                        index,
                        key: key.to_string(),
                        key_id,
                    }
                })
                .collect(),
            item_capacity: self.item_capacity(),
            owned_upgrades: self
                .upgrades
                .upgrades
                .iter()
                .map(|upgrade| {
                    let (key, key_id) = upgrade_key(upgrade.kind().raw());
                    OwnedUpgradeObservation {
                        id: upgrade.id,
                        key: key.to_string(),
                        key_id,
                        scalar_values: (0..4)
                            .filter_map(|index| upgrade.scalar_value(index))
                            .collect(),
                        ratio_values: (0..4)
                            .filter_map(|index| upgrade.ratio_value(index))
                            .collect(),
                        bool_values: (0..4)
                            .filter_map(|index| upgrade.bool_value(index))
                            .collect(),
                    }
                })
                .collect(),
            treasure_capacity: self.treasure_capacity(),
            discardable_treasure_ids: self
                .upgrades
                .upgrades
                .iter()
                .filter(|upgrade| self.can_discard_treasure(upgrade.id))
                .map(|upgrade| upgrade.id)
                .collect(),
            towers,
            tower_grid,
            map_width,
            map_height,
            route_coords: self
                .route
                .map_coords
                .iter()
                .enumerate()
                .map(|(index, [x, y])| RouteCoordObservation {
                    x: *x,
                    y: *y,
                    index,
                })
                .collect(),
            monsters,
            treasure_options,
            card_service: None,
            stage_modifiers: stage_modifiers_observation(&self.stage_modifiers),
        }
    }

    pub fn max_hp_raw(&self) -> i64 {
        self.config
            .player
            .max_hp_raw
            .saturating_add(self.upgrades.cache_state().max_hp_plus_raw)
    }
}

fn decision_point_from_flow(flow: &crate::GameFlowState) -> crate::DecisionPoint {
    match flow {
        crate::GameFlowState::Shopping(_) => crate::DecisionPoint::Shop,
        crate::GameFlowState::SelectingTower => crate::DecisionPoint::CardSelection,
        crate::GameFlowState::PlacingTower => crate::DecisionPoint::TowerPlacement,
        crate::GameFlowState::Defense(_) => crate::DecisionPoint::Defense,
        crate::GameFlowState::TreasureSelection { .. } => crate::DecisionPoint::TreasureSelection,
        crate::GameFlowState::Result { .. } | crate::GameFlowState::Initializing => {
            crate::DecisionPoint::Terminal
        }
    }
}

fn card_observation(card: &crate::CardState) -> CardObservation {
    CardObservation {
        id: card.id,
        suit: suit_key(card.suit).to_string(),
        rank: rank_key(card.rank).to_string(),
        polish_pct_raw: card.polish_pct_raw,
        engraving: card.engraving.map(engraving_key).map(str::to_string),
    }
}

fn build_tower_candidates(state: &crate::CoreState) -> Vec<BuildTowerCandidateObservation> {
    if !matches!(
        state.flow(),
        crate::GameFlowState::Shopping(_) | crate::GameFlowState::SelectingTower
    ) {
        return Vec::new();
    }
    let card_slots = state
        .hand
        .slots
        .iter()
        .enumerate()
        .filter_map(|(index, slot)| match &slot.item {
            crate::HandItemState::Card(card) => Some((index, card.id)),
            crate::HandItemState::Tower(_) => None,
        })
        .collect::<Vec<_>>();
    if card_slots.is_empty() {
        return Vec::new();
    }
    let subset_count = 1usize << card_slots.len();
    let mut candidates = Vec::with_capacity(subset_count.saturating_sub(1));
    for subset_mask in 1..subset_count {
        let selected_slots = card_slots
            .iter()
            .enumerate()
            .filter_map(|(offset, (slot_index, card_id))| {
                (subset_mask & (1usize << offset) != 0).then_some((*slot_index, *card_id))
            })
            .collect::<Vec<_>>();
        let canonical_card_ids: Vec<usize> = if subset_mask + 1 == subset_count {
            Vec::new()
        } else {
            selected_slots.iter().map(|(_, card_id)| *card_id).collect()
        };
        let source_slots = if canonical_card_ids.is_empty() {
            card_slots.clone()
        } else {
            selected_slots.clone()
        };
        let cards = source_slots
            .iter()
            .filter_map(
                |(slot_index, _)| match &state.hand.slots[*slot_index].item {
                    crate::HandItemState::Card(card) => Some(card.clone()),
                    crate::HandItemState::Tower(_) => None,
                },
            )
            .collect::<Vec<_>>();
        let Some(template) = crate::game_state::tower_selection::select_tower_build_template(
            &cards,
            state.upgrades(),
            state.config(),
            state.progress.rerolled_count,
        ) else {
            continue;
        };
        let upgrade_bonus_raw = state
            .upgrades()
            .tower_upgrade_bonus_raw_for_template(&template);
        candidates.push(BuildTowerCandidateObservation {
            card_ids: canonical_card_ids,
            template: tower_template_observation(&template, upgrade_bonus_raw),
        });
    }
    candidates
}

/// Preview templates for `stage_modifiers.extra_tower_cards`, matching
/// exactly how `tower_selection::start_placing_tower_from_template` builds
/// them (same `build_template` call, no `used_cards`) - so this can be
/// computed before a card subset is even selected. Gated on the same flow
/// window `build_tower_candidates` uses: `extra_tower_cards` remains in
/// `stage_modifiers` (and is meaningful as a preview) only while still
/// Shopping/SelectingTower; once `BuildTower` runs, it's drained into
/// `hand.slots` by `start_placing_tower_from_template`.
fn extra_tower_card_templates(state: &crate::CoreState) -> Vec<TowerTemplateObservation> {
    if !matches!(
        state.flow(),
        crate::GameFlowState::Shopping(_) | crate::GameFlowState::SelectingTower
    ) {
        return Vec::new();
    }
    state
        .stage_modifiers()
        .extra_tower_cards
        .iter()
        .map(|extra| {
            let template = crate::game_state::tower_selection::build_template(
                extra.kind,
                extra.suit,
                extra.rank,
                Vec::new(),
                state.progress.rerolled_count,
                state.config(),
            );
            let upgrade_bonus_raw = state
                .upgrades()
                .tower_upgrade_bonus_raw_for_template(&template);
            tower_template_observation(&template, upgrade_bonus_raw)
        })
        .collect()
}

/// Current stage's full configured wave composition, preserving
/// `config.monsters.stage_waves` entry order exactly - see
/// `docs/game-ai/03-observation-contract.md`. Available regardless of flow
/// state (Shopping/CardSelection/TowerPlacement/Defense) since this is
/// deterministic public configuration, not hidden RNG.
fn stage_wave_observation(state: &crate::CoreState) -> Vec<WaveGroupObservation> {
    let Some(wave) = state
        .config
        .monsters
        .stage_waves
        .iter()
        .find(|wave| wave.stage == state.progress.stage)
    else {
        return Vec::new();
    };
    wave.entries
        .iter()
        .enumerate()
        .map(|(order_index, entry)| {
            let profile = crate::game_state::monster_spawn::monster_spawn_profile(
                entry.kind,
                &state.config,
                &state.stage_modifiers,
            );
            let (kind, kind_id) = monster_kind(entry.kind);
            WaveGroupObservation {
                order_index,
                kind: kind.to_string(),
                kind_id,
                count: entry.count,
                max_hp_raw: profile.max_hp_raw,
                velocity_raw: profile.velocity_raw,
                damage_raw: profile.damage_raw,
                reward: profile.reward,
            }
        })
        .collect()
}

/// Actual remaining runtime spawn queue, run-length encoded in exact queue
/// order. Consecutive queue entries only merge into one group when both the
/// kind and every authoritative spawn stat match - a same-kind run
/// interrupted by a different kind (or, in principle, a differently-stated
/// same-kind monster) never merges with an earlier run. No monster entity
/// IDs are exposed. See `docs/game-ai/03-observation-contract.md`.
fn queued_wave_observation(state: &crate::CoreState) -> Vec<QueuedMonsterGroupObservation> {
    let mut groups: Vec<QueuedMonsterGroupObservation> = Vec::new();
    for monster in &state.monster_spawn.monster_queue {
        let (kind, kind_id) = monster_kind(monster.kind);
        let velocity_raw = monster.move_on_route.velocity_raw;
        if let Some(last) = groups.last_mut()
            && last.kind_id == kind_id
            && last.max_hp_raw == monster.max_hp_raw
            && last.velocity_raw == velocity_raw
            && last.damage_raw == monster.damage_raw
            && last.reward == monster.reward
        {
            last.count += 1;
            continue;
        }
        groups.push(QueuedMonsterGroupObservation {
            order_index: groups.len(),
            kind: kind.to_string(),
            kind_id,
            count: 1,
            max_hp_raw: monster.max_hp_raw,
            velocity_raw,
            damage_raw: monster.damage_raw,
            reward: monster.reward,
        });
    }
    groups
}

fn unordered_cards(cards: &[crate::CardState]) -> Vec<CardObservation> {
    let mut observations = cards.iter().map(card_observation).collect::<Vec<_>>();
    observations.sort_by_key(|card| card.id);
    observations
}

fn damage_splash_observation(splash: &crate::DamageSplash) -> DamageSplashObservation {
    DamageSplashObservation {
        radius_raw: splash.radius_raw,
        damage_pct_raw: splash.damage_pct_raw,
    }
}

fn damage_splashes_observation(splashes: &[crate::DamageSplash]) -> Vec<DamageSplashObservation> {
    splashes.iter().map(damage_splash_observation).collect()
}

fn tower_template_observation(
    template: &crate::TowerTemplateState,
    upgrade_bonus_raw: i64,
) -> TowerTemplateObservation {
    let (kind, kind_id) = tower_kind(template.kind);
    TowerTemplateObservation {
        kind: kind.to_string(),
        kind_id,
        suit: template
            .suit
            .and_then(crate::Suit::from_raw)
            .map(suit_key)
            .map(str::to_string),
        rank: template
            .rank
            .and_then(crate::Rank::from_raw)
            .map(rank_key)
            .map(str::to_string),
        rerolled_count: template.rerolled_count,
        damage_raw: template.default_damage_raw,
        effective_damage_raw: template.effective_damage_raw(upgrade_bonus_raw),
        range_raw: template.default_attack_range_radius_raw,
        shoot_interval_ticks: template.shoot_interval,
        used_cards: template.used_cards.iter().map(card_observation).collect(),
        on_hit_splashes: damage_splashes_observation(&template.derived_on_hit_splashes()),
        on_attack_splashes: damage_splashes_observation(&template.derived_on_attack_splashes()),
    }
}

/// Deterministic total order for `TowerStatusEffectObservation`, independent
/// of the runtime `Vec` order: (kind tag, raw value, has-expiry tag,
/// remaining ticks). See `docs/game-ai/03-observation-contract.md`.
fn status_effect_sort_key(effect: &TowerStatusEffectObservation) -> (u8, i64, u8, u64) {
    let (kind_tag, value_raw) = match effect.kind {
        TowerStatusEffectObservationKind::DamageAdd { add_raw } => (0u8, add_raw),
        TowerStatusEffectObservationKind::DamageMul { mul_raw } => (1u8, mul_raw),
    };
    match effect.remaining_ticks {
        Some(remaining_ticks) => (kind_tag, value_raw, 0, remaining_ticks),
        None => (kind_tag, value_raw, 1, 0),
    }
}

fn tower_status_effect_observation(
    effect: &crate::TowerStatusEffect,
    sim_tick: u64,
) -> TowerStatusEffectObservation {
    let kind = match effect.kind {
        TowerStatusEffectKind::DamageMul { mul_raw } => {
            TowerStatusEffectObservationKind::DamageMul { mul_raw }
        }
        TowerStatusEffectKind::DamageAdd { add_raw } => {
            TowerStatusEffectObservationKind::DamageAdd { add_raw }
        }
    };
    let remaining_ticks = match effect.end {
        TowerStatusEffectEnd::Time { end_at } => Some(end_at.saturating_sub(sim_tick)),
        TowerStatusEffectEnd::NeverEnd => None,
    };
    TowerStatusEffectObservation {
        kind,
        remaining_ticks,
    }
}

fn tower_observation(
    state: &crate::CoreState,
    tower: &crate::TowerState,
    sim_tick: u64,
) -> Option<TowerObservation> {
    let id = tower.id?;
    let upgrade_bonus_raw = state.upgrades().tower_upgrade_bonus_raw(tower);
    let mut status_effects = tower
        .status_effects
        .iter()
        .map(|effect| tower_status_effect_observation(effect, sim_tick))
        .collect::<Vec<_>>();
    status_effects.sort_by_key(status_effect_sort_key);
    TowerObservation {
        id,
        left: tower.left_top[0],
        top: tower.left_top[1],
        template: tower_template_observation(&tower.template, upgrade_bonus_raw),
        cooldown_ticks: tower.cooldown,
        range_raw: tower.attack_range_raw(),
        attack_damage_raw: tower.attack_damage_raw(),
        status_effects,
        on_hit_splashes: damage_splashes_observation(&tower.on_hit_splashes),
        on_attack_splashes: damage_splashes_observation(&tower.on_attack_splashes),
    }
    .into()
}

fn shop_slot_observation(
    state: &crate::CoreState,
    index: usize,
    slot: &crate::ShopSlotDataState,
) -> ShopSlotObservation {
    let (kind, kind_id, key, key_id, cost) = match &slot.slot {
        crate::ShopSlotState::Item { item, cost } => {
            let (key, key_id) = item_key(item.kind().raw());
            ("item", 1, key, key_id, *cost)
        }
        crate::ShopSlotState::Upgrade { upgrade, cost } => {
            let (key, key_id) = upgrade_key(upgrade.kind().raw());
            ("upgrade", 2, key, key_id, *cost)
        }
        crate::ShopSlotState::CardService { kind, cost } => {
            let (key, key_id) = card_service_key(*kind);
            ("card_service", 3, key, key_id, *cost)
        }
    };
    ShopSlotObservation {
        index,
        purchased: slot.purchased,
        kind: kind.to_string(),
        kind_id,
        key: key.to_string(),
        key_id,
        cost: if state.stage_modifiers.free_shop_this_stage {
            0
        } else {
            cost
        },
    }
}

fn stage_modifiers_observation(
    modifiers: &crate::StageModifiersState,
) -> StageModifiersObservation {
    let combined = |factors: &[i64]| crate::apply_ratio_product_raw(crate::RATIO_SCALE, factors);
    StageModifiersObservation {
        damage_multiplier_raw: combined(&modifiers.damage_multipliers_raw),
        damage_reduction_multiplier_raw: combined(&modifiers.damage_reduction_multipliers_raw),
        incoming_damage_multiplier_raw: combined(&modifiers.incoming_damage_multipliers_raw),
        gold_gain_multiplier_raw: combined(&modifiers.gold_gain_multipliers_raw),
        enemy_health_multiplier_raw: combined(&modifiers.enemy_health_multipliers_raw),
        enemy_speed_multiplier_raw: combined(&modifiers.enemy_speed_multipliers_raw),
        max_hand_slots_delta: modifiers.card_selection_hand_max_slots_bonus as isize
            - modifiers.card_selection_hand_max_slots_penalty as isize,
        max_rerolls_delta: modifiers.max_dice_rerolls_bonus as isize
            - modifiers.max_dice_rerolls_penalty as isize,
        reroll_health_cost: modifiers.reroll_health_cost,
        item_use_disabled: modifiers.disable_item_use,
        purchases_disabled: modifiers.disable_item_and_upgrade_purchases,
        free_shop: modifiers.free_shop_this_stage,
    }
}

fn suit_key(value: crate::Suit) -> &'static str {
    match value {
        crate::Suit::Spades => "spades",
        crate::Suit::Hearts => "hearts",
        crate::Suit::Diamonds => "diamonds",
        crate::Suit::Clubs => "clubs",
    }
}

fn rank_key(value: crate::Rank) -> &'static str {
    match value {
        crate::Rank::Two => "two",
        crate::Rank::Three => "three",
        crate::Rank::Four => "four",
        crate::Rank::Five => "five",
        crate::Rank::Six => "six",
        crate::Rank::Seven => "seven",
        crate::Rank::Eight => "eight",
        crate::Rank::Nine => "nine",
        crate::Rank::Ten => "ten",
        crate::Rank::Jack => "jack",
        crate::Rank::Queen => "queen",
        crate::Rank::King => "king",
        crate::Rank::Ace => "ace",
    }
}

fn engraving_key(value: u8) -> &'static str {
    match value {
        0 => "magnet",
        1 => "overcharge",
        2 => "cactus",
        3 => "spinning_top",
        _ => "unknown",
    }
}

fn tower_kind(value: u8) -> (&'static str, u16) {
    const NAMES: [&str; 11] = [
        "RubberCone",
        "High",
        "OnePair",
        "TwoPair",
        "ThreeOfAKind",
        "Straight",
        "Flush",
        "FullHouse",
        "FourOfAKind",
        "StraightFlush",
        "RoyalFlush",
    ];
    NAMES
        .get(value as usize)
        .copied()
        .map(|name| (name, value as u16 + 1))
        .unwrap_or(("Unknown", 0))
}

fn monster_kind(value: u8) -> (&'static str, u16) {
    const NAMES: [&str; 64] = [
        "Mob01", "Mob02", "Mob03", "Mob04", "Mob05", "Mob06", "Mob07", "Mob08", "Mob09", "Mob10",
        "Mob11", "Mob12", "Mob13", "Mob14", "Mob15", "Mob16", "Mob17", "Mob18", "Mob19", "Mob20",
        "Mob21", "Mob22", "Mob23", "Mob24", "Mob25", "Mob26", "Mob27", "Mob28", "Mob29", "Mob30",
        "Mob31", "Mob32", "Mob33", "Mob34", "Mob35", "Mob36", "Mob37", "Mob38", "Mob39", "Mob40",
        "Mob41", "Mob42", "Mob43", "Mob44", "Mob45", "Mob46", "Mob47", "Mob48", "Mob49", "Mob50",
        "Boss01", "Boss02", "Boss03", "Boss04", "Boss05", "Boss06", "Boss07", "Boss08", "Boss09",
        "Boss10", "Boss11", "Boss12", "Boss13", "Boss14",
    ];
    NAMES
        .get(value as usize)
        .copied()
        .map(|name| (name, value as u16 + 1))
        .unwrap_or(("Unknown", 0))
}

fn item_key(value: u8) -> (&'static str, u16) {
    const NAMES: [&str; 11] = [
        "bread",
        "candy",
        "cannoli",
        "cookie",
        "donut",
        "rice_ball",
        "lunch_box",
        "lump_sugar",
        "milk",
        "rubber_cone",
        "gimbap",
    ];
    NAMES
        .get(value as usize)
        .copied()
        .map(|name| (name, value as u16 + 1))
        .unwrap_or(("unknown", 0))
}

fn upgrade_key(value: u8) -> (&'static str, u16) {
    const NAMES: [&str; 37] = [
        "apple",
        "banana",
        "carrot",
        "cat",
        "backpack",
        "dice_bundle",
        "energy_drink",
        "perfect_pottery",
        "four_leaf_clover",
        "rabbit",
        "black_white",
        "trophy",
        "crock",
        "cup_noodles",
        "french_fries",
        "hamburger",
        "pizza",
        "demolition_hammer",
        "metronome",
        "tape",
        "name_tag",
        "shopping_bag",
        "resolution",
        "mirror",
        "ice_cream",
        "spanner",
        "pea",
        "slot_machine",
        "piggy_bank",
        "camera",
        "gift_box",
        "fang",
        "popcorn",
        "membership_card",
        "broken_pottery",
        "strawberry",
        "watermelon",
    ];
    NAMES
        .get(value as usize)
        .copied()
        .map(|name| (name, value as u16 + 1))
        .unwrap_or(("unknown", 0))
}

fn card_service_key(value: u8) -> (&'static str, u16) {
    const NAMES: [&str; 16] = [
        "long_sword",
        "staff",
        "mace",
        "club_sword",
        "brush",
        "fountain_pen",
        "tricycle",
        "eraser",
        "magic_wand",
        "pliers",
        "screwdriver",
        "copier",
        "magnet",
        "cactus",
        "spinning_top",
        "battery",
    ];
    let Some(name) = NAMES.get(value as usize).copied() else {
        return ("unknown", 0);
    };
    let id = match name {
        "magnet" => 13,
        "battery" => 14,
        "cactus" => 15,
        "spinning_top" => 16,
        _ => value as u16 + 1,
    };
    (name, id)
}

#[cfg(test)]
mod tests {
    use super::{
        TowerStatusEffectObservation, TowerStatusEffectObservationKind, tower_template_observation,
    };

    /// A `GameConfig` whose stage-1 wave is order-sensitive: kind A (Mob01,
    /// `kind = 0`) x3, kind B (Mob02, `kind = 1`) x2, kind A x1 again - two
    /// non-contiguous same-kind runs. Used by the wave-observation tests
    /// (Part 9 A/C/F of the observation contract wave-visibility work).
    fn config_with_order_sensitive_stage_one_wave() -> crate::GameConfigState {
        let mut config = crate::GameConfig::default_config();
        config.monsters.stage_waves.retain(|wave| wave.stage != 1);
        config.monsters.stage_waves.push(crate::StageWaveState {
            stage: 1,
            entries: vec![
                crate::StageWaveEntryState { kind: 0, count: 3 },
                crate::StageWaveEntryState { kind: 1, count: 2 },
                crate::StageWaveEntryState { kind: 0, count: 1 },
            ],
        });
        config
    }

    #[test]
    fn stage_wave_preserves_order_sensitive_composition_without_kind_aggregation() {
        let config = config_with_order_sensitive_stage_one_wave();
        let state = crate::CoreState::new_initial(config, 1);
        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);

        assert_eq!(
            observation.stage_wave.len(),
            3,
            "A/B/A must stay 3 groups, not aggregate by kind"
        );
        assert_eq!(
            observation
                .stage_wave
                .iter()
                .map(|group| (group.order_index, group.kind_id, group.count))
                .collect::<Vec<_>>(),
            vec![(0, 1, 3), (1, 2, 2), (2, 1, 1)],
            "kind_id 1 = Mob01 (A), kind_id 2 = Mob02 (B); order must be A x3, B x2, A x1"
        );
    }

    #[test]
    fn stage_wave_is_visible_during_shopping() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.flow = crate::GameFlowState::Shopping(crate::ShopState { slots: Vec::new() });

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(observation.decision_point, crate::DecisionPoint::Shop);
        assert_eq!(observation.stage_wave.len(), 3);
    }

    #[test]
    fn stage_wave_stats_match_authoritative_spawn_profile_with_health_modifier() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.stage_modifiers.enemy_health_multipliers_raw = vec![2_000_000];
        state.flow = crate::GameFlowState::PlacingTower;
        state.force_start_defense();

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        // Stage wave stats (computed by the shared authoritative helper)
        // must match the stats of the monsters `start_spawn()` actually
        // queued, group by group.
        let mut queue_offset = 0usize;
        for group in &observation.stage_wave {
            for _ in 0..group.count {
                let queued = &state.monster_spawn().monster_queue[queue_offset];
                assert_eq!(queued.max_hp_raw, group.max_hp_raw);
                assert_eq!(queued.move_on_route.velocity_raw, group.velocity_raw);
                assert_eq!(queued.damage_raw, group.damage_raw);
                assert_eq!(queued.reward, group.reward);
                queue_offset += 1;
            }
        }
        assert_eq!(queue_offset, state.monster_spawn().monster_queue.len());
    }

    #[test]
    fn queued_wave_matches_actual_spawn_queue_right_after_defense_starts() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.flow = crate::GameFlowState::PlacingTower;
        state.force_start_defense();

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(
            observation
                .queued_wave
                .iter()
                .map(|group| (group.order_index, group.kind_id, group.count))
                .collect::<Vec<_>>(),
            vec![(0, 1, 3), (1, 2, 2), (2, 1, 1)],
            "queued_wave must not merge the two non-contiguous kind-A runs"
        );
        assert_eq!(
            observation.queued_monster_count,
            observation
                .queued_wave
                .iter()
                .map(|group| group.count)
                .sum::<usize>()
        );
    }

    #[test]
    fn queue_progress_decrements_count_and_drops_spawned_monster() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.flow = crate::GameFlowState::PlacingTower;
        state.force_start_defense();

        crate::game_state::monster_spawn::spawn_due(&mut state);

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(observation.active_monster_count, 1);
        assert_eq!(
            observation
                .queued_wave
                .iter()
                .map(|group| (group.kind_id, group.count))
                .collect::<Vec<_>>(),
            vec![(1, 2), (2, 2), (1, 1)],
            "one kind-A monster spawned out of the leading A x3 group"
        );
        assert_eq!(
            observation.queued_monster_count,
            observation
                .queued_wave
                .iter()
                .map(|group| group.count)
                .sum::<usize>()
        );
    }

    #[test]
    fn spawn_timing_reflects_authoritative_next_spawn_tick_and_decreases() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.flow = crate::GameFlowState::PlacingTower;
        state.force_start_defense();

        // `start_spawn()` schedules the first spawn for the current tick, so
        // the first spawn is already due; consume it to get a genuinely
        // future `next_spawn_tick` to assert against.
        crate::game_state::monster_spawn::spawn_due(&mut state);

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(
            observation.spawn_interval_ticks,
            state.monster_spawn().spawn_interval_ticks
        );
        let expected_first = state
            .monster_spawn()
            .next_spawn_tick
            .map(|tick| tick.saturating_sub(state.sim_tick().ticks()));
        assert_eq!(observation.next_spawn_in_ticks, expected_first);
        assert_eq!(
            observation.next_spawn_in_ticks,
            Some(state.monster_spawn().spawn_interval_ticks)
        );

        state.sim_tick = crate::SimTick::from_ticks(state.sim_tick().ticks() + 1);
        let later_observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let expected_later = state
            .monster_spawn()
            .next_spawn_tick
            .map(|tick| tick.saturating_sub(state.sim_tick().ticks()));
        assert_eq!(later_observation.next_spawn_in_ticks, expected_later);
        assert!(
            later_observation.next_spawn_in_ticks.unwrap()
                < observation.next_spawn_in_ticks.unwrap()
        );

        // Drain the queue: no future spawn should remain. Each spawn only
        // becomes due once `sim_tick` reaches `next_spawn_tick`, so advance
        // to it before each attempt instead of looping at a fixed tick.
        while !state.monster_spawn().monster_queue.is_empty() {
            if let Some(next_spawn_tick) = state.monster_spawn().next_spawn_tick {
                state.sim_tick = crate::SimTick::from_ticks(next_spawn_tick);
            }
            crate::game_state::monster_spawn::spawn_due(&mut state);
        }
        if let Some(next_spawn_tick) = state.monster_spawn().next_spawn_tick {
            state.sim_tick = crate::SimTick::from_ticks(next_spawn_tick);
        }
        crate::game_state::monster_spawn::spawn_due(&mut state);
        let drained_observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(drained_observation.next_spawn_in_ticks, None);
    }

    #[test]
    fn wave_observations_are_deterministic_for_the_same_state() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.flow = crate::GameFlowState::PlacingTower;
        state.force_start_defense();

        let a = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let b = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(a.stage_wave, b.stage_wave);
        assert_eq!(a.queued_wave, b.queued_wave);
        assert_eq!(a.next_spawn_in_ticks, b.next_spawn_in_ticks);
    }

    #[test]
    fn queued_wave_serialization_never_leaks_a_monster_entity_id_field() {
        let config = config_with_order_sensitive_stage_one_wave();
        let mut state = crate::CoreState::new_initial(config, 1);
        state.flow = crate::GameFlowState::PlacingTower;
        state.force_start_defense();

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let json =
            serde_json::to_value(&observation.queued_wave).expect("queued_wave should serialize");
        let array = json
            .as_array()
            .expect("queued_wave should serialize as an array");
        for entry in array {
            let object = entry
                .as_object()
                .expect("group should serialize as an object");
            assert!(
                !object.contains_key("id"),
                "queued wave group must not expose a monster entity id"
            );
        }
    }

    fn card(id: usize, suit: u8, rank: u8) -> crate::CardState {
        crate::CardState {
            id,
            suit,
            rank,
            polish_pct_raw: 0,
            engraving: None,
        }
    }

    /// A tower config with a distinct, non-collapsing `range_raw`/
    /// `cooldown_ms` per kind, used to catch the historical bug where
    /// `TowerTemplateObservation` dropped range/cooldown and downstream
    /// code re-derived them from a kind-string lookup that defaulted every
    /// unmatched (PascalCase) kind string to the same `4_000_000` value.
    fn distinct_tower_config() -> crate::GameConfigState {
        let mut config = crate::GameConfig::default_config();
        config.towers.entries = (0u8..=10)
            .map(|kind| crate::TowerConfigEntryState {
                kind,
                damage_raw: 1_000 + i64::from(kind) * 100,
                range_raw: 1_000_000 + i64::from(kind) * 500_000,
                cooldown_ms: 500 + u64::from(kind) * 50,
            })
            .collect();
        config
    }

    /// Representative tower kinds (`tower_kind()`'s numeric ids): High,
    /// OnePair, TwoPair, ThreeOfAKind, Straight, FullHouse, StraightFlush,
    /// RoyalFlush.
    const REPRESENTATIVE_KINDS: [u8; 8] = [1, 2, 3, 4, 5, 7, 9, 10];

    #[test]
    fn tower_template_observation_exposes_authoritative_range_and_cooldown() {
        let config = distinct_tower_config();
        let mut ranges = Vec::new();
        for kind in REPRESENTATIVE_KINDS {
            let template = crate::game_state::tower_selection::build_template(
                kind,
                None,
                None,
                Vec::new(),
                0,
                &config,
            );
            let observation = tower_template_observation(&template, 0);
            assert_eq!(
                observation.range_raw, template.default_attack_range_radius_raw,
                "kind {kind} range_raw should mirror the authoritative template"
            );
            assert_eq!(
                observation.shoot_interval_ticks, template.shoot_interval,
                "kind {kind} shoot_interval_ticks should mirror the authoritative template"
            );
            ranges.push(observation.range_raw);
        }
        assert!(
            ranges.iter().any(|&range| range != ranges[0]),
            "distinct tower kinds must not collapse to the same range_raw: {ranges:?}"
        );
    }

    #[test]
    fn tower_template_observation_reflects_custom_config_range_and_cooldown() {
        let mut config = distinct_tower_config();
        let kind = 9u8; // StraightFlush
        let entry = config
            .towers
            .entries
            .iter_mut()
            .find(|entry| entry.kind == kind)
            .expect("kind should exist in config");
        entry.range_raw = 7_777_777;
        entry.cooldown_ms = 4_321;

        let template = crate::game_state::tower_selection::build_template(
            kind,
            None,
            None,
            Vec::new(),
            0,
            &config,
        );
        let observation = tower_template_observation(&template, 0);
        assert_eq!(observation.range_raw, 7_777_777);
        assert_eq!(
            template.shoot_interval,
            4_321u64.saturating_mul(60).div_ceil(1000)
        );
        assert_eq!(observation.shoot_interval_ticks, template.shoot_interval);
    }

    #[test]
    fn build_tower_candidate_and_placed_tower_share_authoritative_range_and_cooldown() {
        let config = distinct_tower_config();
        let mut state = crate::CoreState::new_initial(config, 11);
        // Force a straight flush: five same-suit consecutive-rank cards.
        let cards = vec![
            card(1, 0, 4),
            card(2, 0, 5),
            card(3, 0, 6),
            card(4, 0, 7),
            card(5, 0, 8),
        ];
        state.hand.slots.clear();
        for c in cards {
            let id = state.hand.allocate_slot_id();
            state.hand.slots.push(crate::HandSlotState {
                id,
                item: crate::HandItemState::Card(c),
                selected: false,
            });
        }
        state.flow = crate::GameFlowState::SelectingTower;

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let full_hand_candidate = observation
            .build_tower_candidates
            .iter()
            .find(|candidate| candidate.card_ids.is_empty())
            .expect("full-hand candidate should exist");
        assert_eq!(full_hand_candidate.template.kind, "StraightFlush");
        let preview_range = full_hand_candidate.template.range_raw;
        let preview_cooldown = full_hand_candidate.template.shoot_interval_ticks;

        let placed_template = crate::game_state::tower_selection::build_template(
            9,
            Some(0),
            Some(8),
            Vec::new(),
            0,
            state.config(),
        );
        state
            .place_tower_with_template(placed_template, None, 0, 0)
            .expect("tower should be placeable");
        let after_placement = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let placed_tower = after_placement
            .towers
            .first()
            .expect("tower should be placed");
        assert_eq!(placed_tower.range_raw, preview_range);
        assert_eq!(placed_tower.template.shoot_interval_ticks, preview_cooldown);
    }

    #[test]
    fn owned_upgrade_observation_exposes_runtime_parameters() {
        let mut upgrade = crate::generated_upgrade(crate::UpgradeKind::Backpack);
        assert!(upgrade.set_scalar_value(0, 3));
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 7);
        state
            .acquire_upgrade(upgrade)
            .expect("upgrade should be acquired");

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(observation.owned_upgrades.len(), 1);
        assert_eq!(observation.owned_upgrades[0].scalar_values, vec![3]);
        assert!(observation.owned_upgrades[0].ratio_values.is_empty());
    }

    fn cactus_card(id: usize, suit: u8, rank: u8) -> crate::CardState {
        crate::CardState {
            id,
            suit,
            rank,
            polish_pct_raw: 0,
            engraving: Some(2),
        }
    }

    /// Test A: a resulting template built from a Cactus-engraved card
    /// exposes exactly one on-attack splash with the authoritative
    /// radius/damage percentage; a non-Cactus template exposes none.
    #[test]
    fn cactus_template_preview_exposes_derived_on_attack_splash() {
        let config = crate::GameConfig::default_config();
        let cactus_template = crate::game_state::tower_selection::build_template(
            1,
            None,
            None,
            vec![cactus_card(1, 0, 4)],
            0,
            &config,
        );
        let observation = tower_template_observation(&cactus_template, 0);
        assert_eq!(observation.on_attack_splashes.len(), 1);
        assert_eq!(
            observation.on_attack_splashes[0].radius_raw,
            2 * crate::WORLD_UNITS_PER_TILE
        );
        assert_eq!(observation.on_attack_splashes[0].damage_pct_raw, 300_000);
        assert!(observation.on_hit_splashes.is_empty());

        let plain_template = crate::game_state::tower_selection::build_template(
            1,
            None,
            None,
            vec![card(2, 0, 5)],
            0,
            &config,
        );
        let plain_observation = tower_template_observation(&plain_template, 0);
        assert!(plain_observation.on_attack_splashes.is_empty());
        assert!(plain_observation.on_hit_splashes.is_empty());
    }

    /// Test B: the template preview's derived on-attack splash and the
    /// actually placed tower's runtime on-attack splash must match exactly -
    /// both go through `TowerTemplateState::derived_on_attack_splashes`.
    #[test]
    fn cactus_preview_and_placed_splash_match() {
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 3);
        let template = crate::game_state::tower_selection::build_template(
            1,
            None,
            None,
            vec![cactus_card(1, 0, 4)],
            0,
            state.config(),
        );
        let preview = tower_template_observation(&template, 0);

        state
            .place_tower_with_template(template, None, 0, 0)
            .expect("tower should be placeable");
        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let placed = observation.towers.first().expect("tower should be placed");

        assert_eq!(placed.on_attack_splashes, preview.on_attack_splashes);
        assert_eq!(
            placed.template.on_attack_splashes,
            preview.on_attack_splashes
        );
    }

    /// Test C: with no active status effects, `TowerObservation.
    /// attack_damage_raw` must equal `TowerState::attack_damage_raw()`.
    #[test]
    fn placed_tower_attack_damage_matches_authoritative_calculation_with_no_status() {
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        let template = crate::game_state::tower_selection::build_template(
            1,
            None,
            None,
            Vec::new(),
            0,
            state.config(),
        );
        state
            .place_tower_with_template(template, None, 0, 0)
            .expect("tower should be placeable");
        let expected = state.towers[0].attack_damage_raw();

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let placed = observation.towers.first().expect("tower should be placed");
        assert_eq!(placed.attack_damage_raw, expected);
        assert!(placed.status_effects.is_empty());
    }

    fn place_plain_tower(state: &mut crate::CoreState) {
        let template = crate::game_state::tower_selection::build_template(
            1,
            None,
            None,
            Vec::new(),
            0,
            state.config(),
        );
        state
            .place_tower_with_template(template, None, 0, 0)
            .expect("tower should be placeable");
    }

    /// Test D: an active `DamageAdd` status changes `attack_damage_raw` and
    /// is reflected in `status_effects` with a distinctive value.
    #[test]
    fn active_damage_add_status_is_reflected_in_observation() {
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        place_plain_tower(&mut state);
        let damage_before = state.towers[0].attack_damage_raw();
        state.towers[0]
            .status_effects
            .push(crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageAdd { add_raw: 4_242 },
                end: crate::TowerStatusEffectEnd::Time { end_at: 100 },
            });
        state.sim_tick = crate::SimTick::from_ticks(40);

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let placed = observation.towers.first().expect("tower should be placed");
        assert_eq!(placed.attack_damage_raw, damage_before + 4_242);
        assert_eq!(placed.status_effects.len(), 1);
        assert_eq!(
            placed.status_effects[0].kind,
            TowerStatusEffectObservationKind::DamageAdd { add_raw: 4_242 }
        );
        assert_eq!(placed.status_effects[0].remaining_ticks, Some(60));
    }

    /// Test E: an active `DamageMul` status changes `attack_damage_raw` and
    /// is reflected in `status_effects`.
    #[test]
    fn active_damage_mul_status_is_reflected_in_observation() {
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        place_plain_tower(&mut state);
        let damage_before = state.towers[0].attack_damage_raw();
        state.towers[0]
            .status_effects
            .push(crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageMul { mul_raw: 2_000_000 },
                end: crate::TowerStatusEffectEnd::NeverEnd,
            });

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let placed = observation.towers.first().expect("tower should be placed");
        assert_ne!(placed.attack_damage_raw, damage_before);
        assert_eq!(placed.status_effects.len(), 1);
        assert_eq!(
            placed.status_effects[0].kind,
            TowerStatusEffectObservationKind::DamageMul { mul_raw: 2_000_000 }
        );
        assert_eq!(placed.status_effects[0].remaining_ticks, None);
    }

    /// Test F: `remaining_ticks` for a `Time`-bounded status decreases as
    /// `sim_tick` advances, and `NeverEnd` always reports `None`.
    #[test]
    fn status_remaining_ticks_decreases_with_sim_tick() {
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        place_plain_tower(&mut state);
        state.towers[0]
            .status_effects
            .push(crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageAdd { add_raw: 10 },
                end: crate::TowerStatusEffectEnd::Time { end_at: 100 },
            });

        state.sim_tick = crate::SimTick::from_ticks(10);
        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let remaining_at_10 = observation.towers[0].status_effects[0].remaining_ticks;

        state.sim_tick = crate::SimTick::from_ticks(60);
        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let remaining_at_60 = observation.towers[0].status_effects[0].remaining_ticks;

        assert_eq!(remaining_at_10, Some(90));
        assert_eq!(remaining_at_60, Some(40));
        assert!(remaining_at_60 < remaining_at_10);
    }

    /// Test G: multiple distinct status effects (different kinds and
    /// expirations) on the same tower must all survive into the
    /// observation - no aggregation into a single scalar.
    #[test]
    fn multiple_status_effects_are_all_preserved() {
        let mut state = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        place_plain_tower(&mut state);
        state.towers[0].status_effects = vec![
            crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageAdd { add_raw: 11 },
                end: crate::TowerStatusEffectEnd::Time { end_at: 50 },
            },
            crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageAdd { add_raw: 22 },
                end: crate::TowerStatusEffectEnd::NeverEnd,
            },
            crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageMul { mul_raw: 333_333 },
                end: crate::TowerStatusEffectEnd::Time { end_at: 80 },
            },
        ];
        state.sim_tick = crate::SimTick::from_ticks(0);

        let observation = state.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let statuses = &observation.towers[0].status_effects;
        assert_eq!(statuses.len(), 3);
        assert!(statuses.contains(&TowerStatusEffectObservation {
            kind: TowerStatusEffectObservationKind::DamageAdd { add_raw: 11 },
            remaining_ticks: Some(50),
        }));
        assert!(statuses.contains(&TowerStatusEffectObservation {
            kind: TowerStatusEffectObservationKind::DamageAdd { add_raw: 22 },
            remaining_ticks: None,
        }));
        assert!(statuses.contains(&TowerStatusEffectObservation {
            kind: TowerStatusEffectObservationKind::DamageMul { mul_raw: 333_333 },
            remaining_ticks: Some(80),
        }));
    }

    /// Test H: the same status set in a different runtime `Vec` order must
    /// produce the same deterministic observation ordering.
    #[test]
    fn status_effect_observation_order_is_deterministic() {
        let mut state_a = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        place_plain_tower(&mut state_a);
        let mut state_b = crate::CoreState::new_initial(crate::GameConfig::default_config(), 5);
        place_plain_tower(&mut state_b);

        let effects = [
            crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageAdd { add_raw: 11 },
                end: crate::TowerStatusEffectEnd::Time { end_at: 50 },
            },
            crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageMul { mul_raw: 333_333 },
                end: crate::TowerStatusEffectEnd::NeverEnd,
            },
            crate::TowerStatusEffect {
                kind: crate::TowerStatusEffectKind::DamageAdd { add_raw: 22 },
                end: crate::TowerStatusEffectEnd::NeverEnd,
            },
        ];
        state_a.towers[0].status_effects = effects.to_vec();
        let mut reversed = effects.to_vec();
        reversed.reverse();
        state_b.towers[0].status_effects = reversed;

        let observation_a = state_a.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        let observation_b = state_b.observation(1, 1, crate::MAP_SIZE[0], crate::MAP_SIZE[1]);
        assert_eq!(
            observation_a.towers[0].status_effects,
            observation_b.towers[0].status_effects
        );
    }
}
