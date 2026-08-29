#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemEntryState {
    pub id: u64,
    pub kind: u8,
    pub scalar_values: Vec<u64>,
    pub signed_values: Vec<i64>,
}

mod behaviors;
mod definition;

use rand::seq::SliceRandom;

use definition::item_definition;

pub const ITEM_KIND_COUNT: usize = crate::ItemKind::COUNT;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemUseEffect {
    Heal { requested_raw: i64, actual_raw: i64 },
    GainShield { amount_raw: i64 },
    GainRerolls { amount: usize },
    GrantTowerCards { tower_kind: u8, count: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemUseOutput {
    pub item: ItemEntryState,
    pub effects: Vec<ItemUseEffect>,
}

pub fn item_rarity(kind: crate::ItemKind) -> Option<crate::Rarity> {
    item_definition(kind).map(|definition| definition.rarity)
}

pub fn generated_item(kind: crate::ItemKind) -> Option<ItemEntryState> {
    item_definition(kind).map(|definition| (definition.generate)())
}

pub fn item_rarity_raw(raw: u8) -> Option<crate::Rarity> {
    crate::ItemKind::from_raw(raw).and_then(item_rarity)
}

pub fn generated_item_raw(raw: u8) -> Option<ItemEntryState> {
    crate::ItemKind::from_raw(raw).and_then(generated_item)
}

pub fn generate_item_of_rarity_with_rng<R: rand::Rng + ?Sized>(
    rarity: crate::Rarity,
    rng: &mut R,
) -> Option<ItemEntryState> {
    let candidates: Vec<_> = definition::ITEM_DEFINITIONS
        .iter()
        .filter(|definition| definition.rarity == rarity)
        .collect();
    candidates
        .choose(rng)
        .map(|definition| (definition.generate)())
}

pub fn validate_item_payload(
    kind: crate::ItemKind,
    item: &ItemEntryState,
) -> Result<(), crate::CommandError> {
    item_definition(kind)
        .ok_or(crate::CommandError::Rejected)
        .and_then(|definition| (definition.validate)(item))
}

pub fn validate_item_payload_raw(item: &ItemEntryState) -> Result<(), crate::CommandError> {
    let definition = definition::item_definition_raw(item.kind)
        .ok_or(crate::CommandError::InvalidItemKind { raw: item.kind })?;
    (definition.validate)(item)
}

pub(crate) fn can_use_item(core: &crate::CoreState, kind: crate::ItemKind) -> bool {
    item_definition(kind)
        .map(|definition| (definition.can_use)(core))
        .unwrap_or(false)
}

pub(crate) fn apply_inventory_item_use(
    core: &mut crate::CoreState,
    item_index: usize,
    kind: crate::ItemKind,
) -> Result<ItemUseOutput, crate::CommandError> {
    let item = core
        .items
        .get(item_index)
        .cloned()
        .ok_or(crate::CommandError::InvalidIndex)?;
    core.item_use_error_with_kind(kind, &item)?;
    let definition = item_definition(kind).ok_or(crate::CommandError::Rejected)?;
    let prepared = (definition.prepare_use)(core, &item)?;

    core.items.remove(item_index);
    core.progress.item_used = true;
    let effects = (definition.apply_use)(core, &item, prepared)?;
    Ok(ItemUseOutput { item, effects })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rarity;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generated_payloads_are_valid_for_every_item_kind() {
        for &kind in crate::ItemKind::ALL {
            let definition = item_definition(kind).expect("all catalog kinds are defined");
            let item = generated_item(kind).expect("all catalog kinds generate");
            assert_eq!(item.id, 0);
            assert_eq!(item.kind, kind.raw());
            assert_eq!(definition.kind.raw(), kind.raw());
            assert!(validate_item_payload(kind, &item).is_ok(), "kind {kind:?}");
            let round_trip = serde_json::from_str::<ItemEntryState>(
                &serde_json::to_string(&item).expect("item serializes"),
            )
            .expect("item deserializes");
            assert_eq!(round_trip, item);
        }
    }

    #[test]
    fn generated_payload_shapes_match_the_item_contract() {
        let expected = [
            (0, 2),
            (0, 1),
            (0, 1),
            (0, 1),
            (0, 1),
            (0, 2),
            (0, 2),
            (1, 0),
            (0, 1),
            (1, 0),
            (0, 2),
        ];
        for (kind, (scalar_count, signed_count)) in expected.into_iter().enumerate() {
            let item = generated_item(crate::ItemKind::from_raw(kind as u8).expect("catalog item"))
                .expect("catalog item");
            assert_eq!(item.scalar_values.len(), scalar_count);
            assert_eq!(item.signed_values.len(), signed_count);
        }
    }

    #[test]
    fn item_generation_is_seed_deterministic_and_respects_rarity() {
        for rarity in Rarity::ALL {
            let mut left = StdRng::seed_from_u64(0x5eed);
            let mut right = StdRng::seed_from_u64(0x5eed);
            let left = generate_item_of_rarity_with_rng(rarity, &mut left);
            let right = generate_item_of_rarity_with_rng(rarity, &mut right);
            assert_eq!(left, right);
            assert_eq!(
                left.as_ref().and_then(|item| {
                    crate::ItemKind::from_raw(item.kind).and_then(item_rarity)
                }),
                if rarity == Rarity::Legendary {
                    None
                } else {
                    Some(rarity)
                }
            );
        }
    }

    #[test]
    fn invalid_item_payloads_are_rejected_without_partial_shapes() {
        let mut missing = generated_item(crate::ItemKind::Bread).expect("bread");
        missing.signed_values.pop();
        assert_eq!(
            validate_item_payload(crate::ItemKind::Bread, &missing),
            Err(crate::CommandError::Rejected)
        );

        let mut extra = generated_item(crate::ItemKind::LumpSugar).expect("lump sugar");
        extra.scalar_values.push(99);
        assert_eq!(
            validate_item_payload(crate::ItemKind::LumpSugar, &extra),
            Err(crate::CommandError::Rejected)
        );

        let mut negative = generated_item(crate::ItemKind::Candy).expect("candy");
        negative.signed_values[0] = -1;
        assert_eq!(
            validate_item_payload(crate::ItemKind::Candy, &negative),
            Err(crate::CommandError::Rejected)
        );
    }

    #[test]
    fn unknown_raw_item_kind_is_rejected_at_the_state_boundary() {
        let config = crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 5,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: Vec::new(),
            },
            monsters: crate::MonsterConfigState {
                stats: Vec::new(),
                stage_waves: Vec::new(),
            },
        };
        let mut state = crate::CoreState::new_initial(config, 7);
        state.items.push(ItemEntryState {
            id: 1,
            kind: u8::MAX,
            scalar_values: Vec::new(),
            signed_values: Vec::new(),
        });

        let item_index = state.items.len() - 1;
        assert_eq!(
            state.use_inventory_item(item_index),
            Err(crate::CommandError::InvalidItemKind { raw: u8::MAX })
        );
        assert_eq!(
            validate_item_payload_raw(state.items.last().expect("item")),
            Err(crate::CommandError::InvalidItemKind { raw: u8::MAX })
        );
    }

    #[test]
    fn applying_each_item_kind_uses_its_raw_payload_contract() {
        let config = crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 5,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: vec![crate::TowerConfigEntryState {
                    kind: 0,
                    damage_raw: 1_000,
                    range_raw: 1_000,
                    cooldown_ms: 1_000,
                }],
            },
            monsters: crate::MonsterConfigState {
                stats: Vec::new(),
                stage_waves: Vec::new(),
            },
        };

        for kind in 0..ITEM_KIND_COUNT as u8 {
            let mut state = crate::CoreState::new_initial(config.clone(), 7);
            state.hp_raw = 50_000;
            let item = generated_item(crate::ItemKind::from_raw(kind).expect("catalog item"))
                .expect("catalog item");
            state
                .grant_inventory_item(item)
                .expect("generated item can be granted");
            let output = state
                .apply_inventory_item_use(3)
                .expect("generated item can be used");
            assert_eq!(output.item.kind, kind);
            assert_eq!(state.items().len(), 3);
            assert!(state.progress().item_used);
            match kind {
                0 => assert_eq!(
                    output.effects,
                    vec![
                        ItemUseEffect::Heal {
                            requested_raw: 6_000,
                            actual_raw: 6_000
                        },
                        ItemUseEffect::GainShield { amount_raw: 6_000 }
                    ]
                ),
                1 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::Heal {
                        requested_raw: 3_000,
                        actual_raw: 3_000
                    }]
                ),
                2 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::Heal {
                        requested_raw: 9_000,
                        actual_raw: 9_000
                    }]
                ),
                3 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::Heal {
                        requested_raw: 5_000,
                        actual_raw: 5_000
                    }]
                ),
                4 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::Heal {
                        requested_raw: 7_000,
                        actual_raw: 7_000
                    }]
                ),
                5 => assert_eq!(
                    output.effects,
                    vec![
                        ItemUseEffect::Heal {
                            requested_raw: 3_000,
                            actual_raw: 3_000
                        },
                        ItemUseEffect::GainShield { amount_raw: 3_000 }
                    ]
                ),
                6 => assert_eq!(
                    output.effects,
                    vec![
                        ItemUseEffect::Heal {
                            requested_raw: 12_000,
                            actual_raw: 10_000
                        },
                        ItemUseEffect::GainShield { amount_raw: 12_000 }
                    ]
                ),
                7 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::GainRerolls { amount: 1 }]
                ),
                8 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::Heal {
                        requested_raw: 12_000,
                        actual_raw: 10_000
                    }]
                ),
                9 => assert_eq!(
                    output.effects,
                    vec![ItemUseEffect::GrantTowerCards {
                        tower_kind: 0,
                        count: 4
                    }]
                ),
                10 => assert_eq!(
                    output.effects,
                    vec![
                        ItemUseEffect::Heal {
                            requested_raw: 9_000,
                            actual_raw: 9_000
                        },
                        ItemUseEffect::GainShield { amount_raw: 9_000 }
                    ]
                ),
                _ => unreachable!(),
            }
        }
    }
}
