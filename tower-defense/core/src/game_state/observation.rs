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
    pub deck: DeckObservation,
    pub shop: Vec<ShopSlotObservation>,
    pub inventory: Vec<InventoryObservation>,
    pub owned_upgrades: Vec<OwnedUpgradeObservation>,
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
                    }
                })
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
