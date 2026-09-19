use super::normalize::{
    normalize_axis_ratio, normalize_damage_raw, normalize_polish_pct_raw, normalize_range_raw,
    normalize_rerolled_count, normalize_route_index, normalize_route_progress_raw, normalize_ticks,
    normalize_upgrade_bool, normalize_upgrade_ratio, normalize_upgrade_scalar,
};
use super::{EntityRow, EntitySet};
use crate::environment::{HandItemObservation, Observation};
use crate::ml::vocabulary::{engraving_key_id, rank_id, suit_id};
use serde::{Deserialize, Serialize};

pub const ENTITY_SET_COUNT: usize = 11;

pub const OWNED_CARDS: usize = 0;
pub const HAND_CARDS: usize = 1;
pub const HAND_TOWERS: usize = 2;
pub const DRAW_CARDS: usize = 3;
pub const DISCARD_CARDS: usize = 4;
pub const PLACED_TOWERS: usize = 5;
pub const MONSTERS: usize = 6;
pub const SHOP: usize = 7;
pub const INVENTORY: usize = 8;
pub const UPGRADES: usize = 9;
pub const ROUTE: usize = 10;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypedObservation {
    pub sets: [EntitySet; ENTITY_SET_COUNT],
}

impl Default for TypedObservation {
    fn default() -> Self {
        Self {
            sets: std::array::from_fn(|_| EntitySet::default()),
        }
    }
}

impl TypedObservation {
    pub fn from_observation(observation: &Observation) -> Self {
        let hand_cards = observation
            .hand
            .iter()
            .filter_map(|item| match &item.item {
                HandItemObservation::Card(card) => Some(EntityRow::new(
                    [
                        suit_id(&card.suit) as u32,
                        rank_id(&card.rank) as u32,
                        card.engraving
                            .as_deref()
                            .map_or(0, |value| engraving_key_id(value) as u32),
                        0,
                    ],
                    vec![
                        normalize_polish_pct_raw(card.polish_pct_raw),
                        item.selected as u8 as f32,
                    ],
                )),
                HandItemObservation::Tower(_) => None,
            })
            .collect();
        let hand_towers = observation
            .hand
            .iter()
            .filter_map(|item| match &item.item {
                HandItemObservation::Tower(tower) => Some(tower_row(tower)),
                HandItemObservation::Card(_) => None,
            })
            .collect();
        let owned_cards = observation
            .deck
            .all_cards
            .iter()
            .map(|card| {
                EntityRow::new(
                    [
                        suit_id(&card.suit) as u32,
                        rank_id(&card.rank) as u32,
                        card.engraving
                            .as_deref()
                            .map_or(0, |value| engraving_key_id(value) as u32),
                        0,
                    ],
                    vec![normalize_polish_pct_raw(card.polish_pct_raw)],
                )
            })
            .collect();
        let draw_cards = observation.deck.draw_cards.iter().map(card_row).collect();
        let discard_cards = observation
            .deck
            .discard_cards
            .iter()
            .map(card_row)
            .collect();
        let placed = observation
            .towers
            .iter()
            .map(|tower| {
                EntityRow::new(
                    [
                        tower.template.kind_id as u32,
                        tower
                            .template
                            .suit
                            .as_deref()
                            .map_or(0, |value| suit_id(value) as u32),
                        tower
                            .template
                            .rank
                            .as_deref()
                            .map_or(0, |value| rank_id(value) as u32),
                        0,
                    ],
                    vec![
                        normalize_axis_ratio(tower.left, observation.map_width),
                        normalize_axis_ratio(tower.top, observation.map_height),
                        normalize_damage_raw(tower.template.damage_raw),
                        normalize_ticks(tower.cooldown_ticks),
                        normalize_range_raw(tower.range_raw),
                        normalize_damage_raw(tower.attack_damage_raw),
                    ],
                )
            })
            .collect();
        let monsters = observation
            .monsters
            .iter()
            .map(|monster| {
                EntityRow::new(
                    [monster.kind_id as u32, 0, 0, 0],
                    vec![
                        normalize_route_index(monster.route_index),
                        normalize_route_progress_raw(monster.route_progress_raw),
                        monster.hp_raw.max(0) as f32 / monster.max_hp_raw.max(1) as f32,
                        monster.velocity_raw as f32 / 100_000.0,
                        normalize_damage_raw(monster.damage_raw),
                    ],
                )
            })
            .collect();
        let shop = observation
            .shop
            .iter()
            .map(|slot| {
                EntityRow::new(
                    [slot.kind_id as u32, slot.key_id as u32, 0, 0],
                    vec![slot.cost as f32 / 1_000.0, slot.purchased as u8 as f32],
                )
            })
            .collect();
        let inventory = observation
            .inventory
            .iter()
            .map(|item| {
                EntityRow::new(
                    [item.key_id as u32, 0, 0, 0],
                    vec![item.index as f32 / 20.0],
                )
            })
            .collect();
        let route = observation
            .route_coords
            .iter()
            .map(|coord| {
                EntityRow::new(
                    [0, 0, 0, 0],
                    vec![
                        normalize_axis_ratio(coord.x, observation.map_width),
                        normalize_axis_ratio(coord.y, observation.map_height),
                        normalize_axis_ratio(coord.index, observation.route_coords.len()),
                    ],
                )
            })
            .collect();
        let upgrades = EntitySet::new(
            observation
                .owned_upgrades
                .iter()
                .map(|upgrade| {
                    let mut numeric = vec![0.0; 6];
                    numeric[0] = upgrade.id as f32 / 1_000_000.0;
                    for (index, value) in upgrade.scalar_values.iter().take(2).enumerate() {
                        numeric[1 + index] = normalize_upgrade_scalar(*value);
                    }
                    for (index, value) in upgrade.ratio_values.iter().take(2).enumerate() {
                        numeric[3 + index] = normalize_upgrade_ratio(*value);
                    }
                    if let Some(value) = upgrade.bool_values.first() {
                        numeric[5] = normalize_upgrade_bool(*value);
                    }
                    EntityRow::new([upgrade.key_id as u32, 0, 0, 0], numeric)
                })
                .collect(),
        );
        Self {
            sets: [
                EntitySet::new(owned_cards),
                EntitySet::new(hand_cards),
                EntitySet::new(hand_towers),
                EntitySet::new(draw_cards),
                EntitySet::new(discard_cards),
                EntitySet::new(placed),
                EntitySet::new(monsters),
                EntitySet::new(shop),
                EntitySet::new(inventory),
                upgrades,
                EntitySet::new(route),
            ],
        }
    }
}

fn card_row(card: &crate::environment::CardObservation) -> EntityRow {
    EntityRow::new(
        [
            suit_id(&card.suit) as u32,
            rank_id(&card.rank) as u32,
            card.engraving
                .as_deref()
                .map_or(0, |value| engraving_key_id(value) as u32),
            0,
        ],
        vec![normalize_polish_pct_raw(card.polish_pct_raw)],
    )
}

fn tower_row(tower: &crate::environment::TowerTemplateObservation) -> EntityRow {
    EntityRow::new(
        [
            tower.kind_id as u32,
            tower
                .suit
                .as_deref()
                .map_or(0, |value| suit_id(value) as u32),
            tower
                .rank
                .as_deref()
                .map_or(0, |value| rank_id(value) as u32),
            tower.used_cards.len() as u32,
        ],
        vec![
            normalize_rerolled_count(tower.rerolled_count),
            normalize_damage_raw(tower.damage_raw),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::{
        CardObservation, DeckObservation, MonsterObservation, Observation, OwnedUpgradeObservation,
        RouteCoordObservation, StageModifiersObservation, TowerObservation,
        TowerTemplateObservation,
    };

    fn base_observation() -> Observation {
        Observation {
            observation_schema_version: 0,
            catalog_schema_version: 0,
            action_wire_schema_version: 0,
            environment_version: 0,
            action_schema_version: 0,
            decision_point: crate::environment::DecisionPoint::Terminal,
            item_window_stage: None,
            damage_trigger_tick: None,
            card_selection_purpose: None,
            selected_hand_slot_indices: vec![],
            card_selection_confirmable: false,
            stage: 0,
            sim_tick: 0,
            theme: None,
            hp_raw: 0,
            max_hp_raw: 1,
            shield_raw: 0,
            gold: 0,
            left_dice: 0,
            rerolled_count: 0,
            stage_progress_raw: 0,
            stage_total_hp_raw: 1,
            active_monster_count: 0,
            queued_monster_count: 0,
            hand: vec![],
            build_tower_candidates: vec![],
            extra_tower_card_templates: vec![],
            deck: DeckObservation {
                all_cards: vec![],
                draw_cards: vec![],
                discard_cards: vec![],
            },
            shop: vec![],
            inventory: vec![],
            item_capacity: 5,
            owned_upgrades: vec![],
            treasure_capacity: 5,
            discardable_treasure_ids: vec![],
            towers: vec![],
            tower_grid: vec![],
            map_width: 10,
            map_height: 20,
            route_coords: vec![],
            monsters: vec![],
            treasure_options: vec![],
            card_service: None,
            stage_modifiers: StageModifiersObservation {
                damage_multiplier_raw: 1000,
                damage_reduction_multiplier_raw: 1000,
                incoming_damage_multiplier_raw: 1000,
                gold_gain_multiplier_raw: 1000,
                enemy_health_multiplier_raw: 1000,
                enemy_speed_multiplier_raw: 1000,
                max_hand_slots_delta: 0,
                max_rerolls_delta: 0,
                reroll_health_cost: 0,
                item_use_disabled: false,
                purchases_disabled: false,
                free_shop: false,
            },
        }
    }

    fn tower_template(damage_raw: i64) -> TowerTemplateObservation {
        TowerTemplateObservation {
            kind: "single".to_owned(),
            kind_id: 1,
            suit: None,
            rank: None,
            rerolled_count: 0,
            damage_raw,
            effective_damage_raw: damage_raw,
            range_raw: 3_000_000,
            shoot_interval_ticks: 30,
            used_cards: Vec::new(),
            on_hit_splashes: Vec::new(),
            on_attack_splashes: Vec::new(),
        }
    }

    /// Card `polish_pct_raw` -> `owned_cards`/`hand_cards`/`draw_cards`/
    /// `discard_cards` numeric row, representative value from
    /// `docs/game-ai/03-observation-contract.md`'s normalization contract.
    #[test]
    fn card_polish_encodes_to_documented_normalization() {
        let mut observation = base_observation();
        observation.deck.all_cards.push(CardObservation {
            id: 1,
            suit: "spade".to_owned(),
            rank: "ace".to_owned(),
            polish_pct_raw: 2_500,
            engraving: None,
        });

        let typed = TypedObservation::from_observation(&observation);

        assert_eq!(typed.sets[OWNED_CARDS].rows[0].numeric[0], 2.5);
    }

    /// Test I: tower position/base damage/cooldown/range/current attack
    /// damage -> `placed_towers` numeric row. Uses distinct base
    /// (`template.damage_raw`) and current (`attack_damage_raw`) values so
    /// the two normalized slots are verified to be genuinely independent,
    /// not aliases of the same underlying value.
    #[test]
    fn placed_tower_encodes_position_damage_cooldown_range() {
        let mut observation = base_observation();
        observation.towers.push(TowerObservation {
            id: 1,
            left: 5,
            top: 10,
            template: tower_template(50_000),
            cooldown_ticks: 300,
            range_raw: 200_000,
            attack_damage_raw: 75_000,
            status_effects: Vec::new(),
            on_hit_splashes: Vec::new(),
            on_attack_splashes: Vec::new(),
        });

        let typed = TypedObservation::from_observation(&observation);
        let row = &typed.sets[PLACED_TOWERS].rows[0];

        assert_eq!(row.numeric[0], 5.0 / 10.0);
        assert_eq!(row.numeric[1], 10.0 / 20.0);
        assert_eq!(row.numeric[2], 5.0);
        assert_eq!(row.numeric[3], 0.5);
        assert_eq!(row.numeric[4], 2.0);
        assert_eq!(row.numeric[5], 7.5);
    }

    /// Monster route progress/damage -> `monsters` numeric row.
    #[test]
    fn monster_encodes_route_progress_and_damage() {
        let mut observation = base_observation();
        observation.monsters.push(MonsterObservation {
            id: 1,
            kind: "grunt".to_owned(),
            kind_id: 1,
            route_index: 50,
            route_progress_raw: 500,
            hp_raw: 50,
            max_hp_raw: 100,
            velocity_raw: 0,
            damage_raw: 5_000,
        });

        let typed = TypedObservation::from_observation(&observation);
        let row = &typed.sets[MONSTERS].rows[0];

        assert_eq!(row.numeric[0], 0.5);
        assert_eq!(row.numeric[1], 0.5);
        assert_eq!(row.numeric[2], 0.5);
        assert_eq!(row.numeric[4], 0.5);
    }

    /// Route coordinate x/y/index -> `route` numeric row.
    #[test]
    fn route_coord_encodes_position_and_progress() {
        let mut observation = base_observation();
        observation.route_coords.push(RouteCoordObservation {
            x: 5,
            y: 10,
            index: 0,
        });
        observation.route_coords.push(RouteCoordObservation {
            x: 6,
            y: 10,
            index: 1,
        });

        let typed = TypedObservation::from_observation(&observation);
        let row = &typed.sets[ROUTE].rows[1];

        assert_eq!(row.numeric[0], 6.0 / 10.0);
        assert_eq!(row.numeric[1], 10.0 / 20.0);
        assert_eq!(row.numeric[2], 1.0 / 2.0);
    }

    fn owned_upgrade(
        scalar_values: Vec<usize>,
        ratio_values: Vec<i64>,
        bool_values: Vec<bool>,
    ) -> OwnedUpgradeObservation {
        OwnedUpgradeObservation {
            id: 1,
            key: "cat".to_owned(),
            key_id: 7,
            scalar_values,
            ratio_values,
            bool_values,
        }
    }

    /// `OwnedUpgradeObservation::scalar_values` runtime parameter changes
    /// (e.g. `Cat`'s `gold_per_kill`) must change the encoded `upgrades` row.
    #[test]
    fn upgrade_scalar_runtime_change_is_reflected_in_encoding() {
        let mut observation = base_observation();
        observation
            .owned_upgrades
            .push(owned_upgrade(vec![5], vec![], vec![]));
        let before = TypedObservation::from_observation(&observation);

        observation.owned_upgrades[0].scalar_values = vec![50];
        let after = TypedObservation::from_observation(&observation);

        assert_eq!(before.sets[UPGRADES].rows[0].numeric[1], 5.0 / 1_000.0);
        assert_eq!(after.sets[UPGRADES].rows[0].numeric[1], 50.0 / 1_000.0);
        assert_ne!(
            before.sets[UPGRADES].rows[0].numeric[1],
            after.sets[UPGRADES].rows[0].numeric[1]
        );
    }

    /// `OwnedUpgradeObservation::ratio_values` runtime parameter changes
    /// (e.g. `Popcorn`'s active multiplier) must change the encoded
    /// `upgrades` row.
    #[test]
    fn upgrade_ratio_runtime_change_is_reflected_in_encoding() {
        let mut observation = base_observation();
        observation
            .owned_upgrades
            .push(owned_upgrade(vec![], vec![250_000], vec![]));
        let before = TypedObservation::from_observation(&observation);

        observation.owned_upgrades[0].ratio_values = vec![750_000];
        let after = TypedObservation::from_observation(&observation);

        assert_eq!(before.sets[UPGRADES].rows[0].numeric[3], 0.25);
        assert_eq!(after.sets[UPGRADES].rows[0].numeric[3], 0.75);
        assert_ne!(
            before.sets[UPGRADES].rows[0].numeric[3],
            after.sets[UPGRADES].rows[0].numeric[3]
        );
    }

    /// `OwnedUpgradeObservation::bool_values` runtime parameter changes
    /// (e.g. `Mirror`/`MembershipCard`'s `pending`) must change the encoded
    /// `upgrades` row.
    #[test]
    fn upgrade_bool_runtime_change_is_reflected_in_encoding() {
        let mut observation = base_observation();
        observation
            .owned_upgrades
            .push(owned_upgrade(vec![], vec![], vec![false]));
        let before = TypedObservation::from_observation(&observation);

        observation.owned_upgrades[0].bool_values = vec![true];
        let after = TypedObservation::from_observation(&observation);

        assert_eq!(before.sets[UPGRADES].rows[0].numeric[5], 0.0);
        assert_eq!(after.sets[UPGRADES].rows[0].numeric[5], 1.0);
        assert_ne!(
            before.sets[UPGRADES].rows[0].numeric[5],
            after.sets[UPGRADES].rows[0].numeric[5]
        );
    }
}
