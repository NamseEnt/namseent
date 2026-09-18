mod flush;
mod four_of_a_kind;
mod full_house;
mod high_card;
mod one_pair;
mod royal_flush;
mod straight;
mod straight_flush;
mod template;
mod three_of_a_kind;
mod two_pair;

use super::get_highest_tower_template;
use crate::{CardState, GameConfigState, TowerTemplateState, UpgradeCollection, UpgradeKind};

const HIGH: u8 = 1;
const ONE_PAIR: u8 = 2;
const TWO_PAIR: u8 = 3;
const THREE_OF_A_KIND: u8 = 4;
const STRAIGHT: u8 = 5;
const FLUSH: u8 = 6;
const FULL_HOUSE: u8 = 7;
const FOUR_OF_A_KIND: u8 = 8;
const STRAIGHT_FLUSH: u8 = 9;
const ROYAL_FLUSH: u8 = 10;

fn config() -> GameConfigState {
    GameConfigState {
        player: crate::PlayerConfigState {
            max_hp_raw: 60_000,
            starting_gold: 100,
            starting_hp_raw: 60_000,
            base_dice_chance: 3,
            max_stages: 30,
            base_hand_slots: 5,
        },
        towers: crate::TowerConfigState {
            entries: (0..11)
                .map(|kind| crate::TowerConfigEntryState {
                    kind,
                    damage_raw: i64::from(kind),
                    range_raw: i64::from(kind),
                    cooldown_ms: 1_000,
                })
                .collect(),
        },
        monsters: crate::MonsterConfigState {
            stats: Vec::new(),
            stage_waves: Vec::new(),
        },
    }
}

fn cards(values: &[(u8, u8)]) -> Vec<CardState> {
    values
        .iter()
        .enumerate()
        .map(|(index, (suit, rank))| CardState {
            id: index + 1,
            suit: *suit,
            rank: *rank,
            polish_pct_raw: 0,
            engraving: None,
        })
        .collect()
}

fn upgrades(kinds: &[UpgradeKind]) -> UpgradeCollection {
    UpgradeCollection::from_entries(
        kinds
            .iter()
            .enumerate()
            .map(|(index, kind)| crate::generated_upgrade(*kind).with_id((index + 1) as u64))
            .collect(),
        0,
    )
}

fn evaluate(values: &[(u8, u8)], upgrade_kinds: &[UpgradeKind]) -> TowerTemplateState {
    get_highest_tower_template(&cards(values), &upgrades(upgrade_kinds), &config(), 0)
        .expect("template should be generated")
}
