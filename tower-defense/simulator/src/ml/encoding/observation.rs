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
                        card.polish_pct_raw as f32 / 1_000.0,
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
                    vec![card.polish_pct_raw as f32 / 1_000.0],
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
                        tower.left as f32 / observation.map_width.max(1) as f32,
                        tower.top as f32 / observation.map_height.max(1) as f32,
                        tower.template.damage_raw as f32 / 10_000.0,
                        tower.cooldown_ticks as f32 / 600.0,
                        tower.range_raw as f32 / 100_000.0,
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
                        monster.route_index as f32 / 100.0,
                        monster.route_progress_raw as f32 / 1_000.0,
                        monster.hp_raw.max(0) as f32 / monster.max_hp_raw.max(1) as f32,
                        monster.velocity_raw as f32 / 100_000.0,
                        monster.damage_raw as f32 / 10_000.0,
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
                        coord.x as f32 / observation.map_width.max(1) as f32,
                        coord.y as f32 / observation.map_height.max(1) as f32,
                        coord.index as f32 / observation.route_coords.len().max(1) as f32,
                    ],
                )
            })
            .collect();
        let upgrades = EntitySet::new(
            observation
                .owned_upgrades
                .iter()
                .map(|upgrade| {
                    EntityRow::new(
                        [upgrade.key_id as u32, 0, 0, 0],
                        vec![upgrade.id as f32 / 1_000_000.0],
                    )
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
        vec![card.polish_pct_raw as f32 / 1_000.0],
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
            tower.rerolled_count as f32 / 20.0,
            tower.damage_raw as f32 / 10_000.0,
        ],
    )
}
