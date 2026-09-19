use crate::{DecisionPoint, StageModifiersObservation};

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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerTemplateObservation {
    pub kind: String,
    pub kind_id: u16,
    pub suit: Option<String>,
    pub rank: Option<String>,
    pub rerolled_count: usize,
    pub damage_raw: i64,
    pub range_raw: i64,
    pub shoot_interval_ticks: u64,
    pub used_cards: Vec<CardObservation>,
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
                        HandItemObservation::Tower(tower_template_observation(tower))
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

        let towers = self
            .towers
            .iter()
            .filter_map(tower_observation)
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
        candidates.push(BuildTowerCandidateObservation {
            card_ids: canonical_card_ids,
            template: tower_template_observation(&template),
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
            tower_template_observation(&template)
        })
        .collect()
}

fn unordered_cards(cards: &[crate::CardState]) -> Vec<CardObservation> {
    let mut observations = cards.iter().map(card_observation).collect::<Vec<_>>();
    observations.sort_by_key(|card| card.id);
    observations
}

fn tower_template_observation(template: &crate::TowerTemplateState) -> TowerTemplateObservation {
    let (kind, kind_id) = tower_kind(template.kind);
    TowerTemplateObservation {
        kind: kind.to_string(),
        kind_id,
        suit: template.suit.map(suit_key).map(str::to_string),
        rank: template.rank.map(rank_key).map(str::to_string),
        rerolled_count: template.rerolled_count,
        damage_raw: template.default_damage_raw,
        range_raw: template.default_attack_range_radius_raw,
        shoot_interval_ticks: template.shoot_interval,
        used_cards: template.used_cards.iter().map(card_observation).collect(),
    }
}

fn tower_observation(tower: &crate::TowerState) -> Option<TowerObservation> {
    let id = tower.id?;
    TowerObservation {
        id,
        left: tower.left_top[0],
        top: tower.left_top[1],
        template: tower_template_observation(&tower.template),
        cooldown_ticks: tower.cooldown,
        range_raw: tower.attack_range_raw(),
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

fn suit_key(value: u8) -> &'static str {
    match value {
        0 => "spades",
        1 => "hearts",
        2 => "diamonds",
        3 => "clubs",
        _ => "unknown",
    }
}

fn rank_key(value: u8) -> &'static str {
    match value {
        0 => "two",
        1 => "three",
        2 => "four",
        3 => "five",
        4 => "six",
        5 => "seven",
        6 => "eight",
        7 => "nine",
        8 => "ten",
        9 => "jack",
        10 => "queen",
        11 => "king",
        12 => "ace",
        _ => "unknown",
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
    use super::tower_template_observation;

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
            let observation = tower_template_observation(&template);
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

        let template =
            crate::game_state::tower_selection::build_template(kind, None, None, Vec::new(), 0, &config);
        let observation = tower_template_observation(&template);
        assert_eq!(observation.range_raw, 7_777_777);
        assert_eq!(template.shoot_interval, 4_321u64.saturating_mul(60).div_ceil(1000));
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
}
