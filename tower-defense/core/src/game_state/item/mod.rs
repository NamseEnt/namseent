#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemEntry {
    pub(crate) id: u64,
    pub(crate) behavior: ItemBehaviorImpl,
    pub(crate) item: ItemRuntimeState,
}

impl ItemEntry {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn value(&self, index: usize) -> Option<i64> {
        match self.item {
            ItemRuntimeState::Bread(state) => {
                [state.heal_raw, state.shield_raw].get(index).copied()
            }
            ItemRuntimeState::Candy(state) => [state.heal_raw].get(index).copied(),
            ItemRuntimeState::Cannoli(state) => [state.heal_raw].get(index).copied(),
            ItemRuntimeState::Cookie(state) => [state.heal_raw].get(index).copied(),
            ItemRuntimeState::Donut(state) => [state.heal_raw].get(index).copied(),
            ItemRuntimeState::Gimbap(state) => {
                [state.heal_raw, state.shield_raw].get(index).copied()
            }
            ItemRuntimeState::LunchBox(state) => {
                [state.heal_raw, state.shield_raw].get(index).copied()
            }
            ItemRuntimeState::Milk(state) => [state.shield_raw].get(index).copied(),
            ItemRuntimeState::RiceBall(state) => {
                [state.heal_raw, state.shield_raw].get(index).copied()
            }
            ItemRuntimeState::LumpSugar(_) | ItemRuntimeState::RubberCone(_) => None,
        }
    }

    pub fn count(&self) -> Option<usize> {
        match self.item {
            ItemRuntimeState::LumpSugar(state) => Some(state.amount),
            ItemRuntimeState::RubberCone(state) => Some(state.count),
            _ => None,
        }
    }

    pub fn set_value(&mut self, index: usize, value: i64) -> bool {
        match &mut self.item {
            ItemRuntimeState::Bread(state) => match index {
                0 => state.heal_raw = value,
                1 => state.shield_raw = value,
                _ => return false,
            },
            ItemRuntimeState::Candy(state) => {
                if index == 0 {
                    state.heal_raw = value
                } else {
                    return false;
                }
            }
            ItemRuntimeState::Cannoli(state) => {
                if index == 0 {
                    state.heal_raw = value
                } else {
                    return false;
                }
            }
            ItemRuntimeState::Cookie(state) => {
                if index == 0 {
                    state.heal_raw = value
                } else {
                    return false;
                }
            }
            ItemRuntimeState::Donut(state) => {
                if index == 0 {
                    state.heal_raw = value
                } else {
                    return false;
                }
            }
            ItemRuntimeState::Gimbap(state) => match index {
                0 => state.heal_raw = value,
                1 => state.shield_raw = value,
                _ => return false,
            },
            ItemRuntimeState::LunchBox(state) => match index {
                0 => state.heal_raw = value,
                1 => state.shield_raw = value,
                _ => return false,
            },
            ItemRuntimeState::Milk(state) => {
                if index == 0 {
                    state.shield_raw = value
                } else {
                    return false;
                }
            }
            ItemRuntimeState::RiceBall(state) => match index {
                0 => state.heal_raw = value,
                1 => state.shield_raw = value,
                _ => return false,
            },
            ItemRuntimeState::LumpSugar(_) | ItemRuntimeState::RubberCone(_) => return false,
        }
        true
    }

    pub fn set_count(&mut self, value: usize) -> bool {
        match &mut self.item {
            ItemRuntimeState::LumpSugar(state) => state.amount = value,
            ItemRuntimeState::RubberCone(state) => state.count = value,
            _ => return false,
        }
        true
    }

    pub fn kind(&self) -> crate::ItemKind {
        self.behavior.kind()
    }

    #[allow(dead_code)]
    pub(crate) fn to_wire(&self) -> ItemWireEntry {
        self.to_raw()
    }

    #[allow(dead_code)]
    pub(crate) fn from_wire(item: ItemWireEntry) -> Result<Self, crate::CommandError> {
        Self::from_raw(item)
    }

    pub(crate) fn from_raw(item: ItemWireEntry) -> Result<Self, crate::CommandError> {
        let kind = crate::ItemKind::from_raw(item.kind)
            .ok_or(crate::CommandError::InvalidItemKind { raw: item.kind })?;
        let behavior = ItemBehaviorImpl::for_kind(kind);
        let state = codec::decode_runtime(&item)?;
        Ok(Self {
            id: item.id,
            behavior,
            item: state,
        })
    }

    pub(crate) fn to_raw(&self) -> ItemWireEntry {
        codec::encode_runtime(self.id, self.kind(), self.item)
    }
}

impl Serialize for ItemEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        codec::encode_item_entry(self, serializer)
    }
}

impl<'de> Deserialize<'de> for ItemEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        codec::decode_item_entry(deserializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ItemCollection {
    pub(crate) items: Vec<ItemEntry>,
}

impl ItemCollection {
    pub fn from_entries(items: Vec<ItemEntry>) -> Self {
        Self { items }
    }

    pub fn entries(&self) -> &[ItemEntry] {
        &self.items
    }

    pub fn entries_mut(&mut self) -> &mut Vec<ItemEntry> {
        &mut self.items
    }

    pub(crate) fn next_id(&self) -> u64 {
        self.items
            .iter()
            .map(|item| item.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ItemEntry> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn get(&self, index: usize) -> Option<&ItemEntry> {
        self.items.get(index)
    }
}

impl std::ops::Index<usize> for ItemCollection {
    type Output = ItemEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl std::ops::IndexMut<usize> for ItemCollection {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl FromIterator<ItemEntry> for ItemCollection {
    fn from_iter<T: IntoIterator<Item = ItemEntry>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

pub trait ItemGrantInput {
    fn into_runtime(self) -> Result<ItemEntry, crate::CommandError>;
}

impl ItemGrantInput for ItemEntry {
    fn into_runtime(self) -> Result<ItemEntry, crate::CommandError> {
        Ok(self)
    }
}

impl Serialize for ItemCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        codec::encode_item_collection(self, serializer)
    }
}

impl<'de> Deserialize<'de> for ItemCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        codec::decode_item_collection(deserializer)
    }
}

mod behaviors;
pub mod codec;

use rand::seq::SliceRandom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) use behaviors::ItemRuntimeState;
use behaviors::{ItemBehavior, ItemBehaviorImpl};
use codec::ItemWireEntry;

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
    pub item: ItemEntry,
    pub effects: Vec<ItemUseEffect>,
}

pub fn item_rarity(kind: crate::ItemKind) -> Option<crate::Rarity> {
    Some(ItemBehaviorImpl::for_kind(kind).rarity())
}

pub fn generated_item(kind: crate::ItemKind) -> Option<ItemEntry> {
    let behavior = ItemBehaviorImpl::for_kind(kind);
    Some(ItemEntry {
        id: 0,
        behavior,
        item: behavior.generated_state(),
    })
}

pub fn item_rarity_raw(raw: u8) -> Option<crate::Rarity> {
    crate::ItemKind::from_raw(raw).and_then(item_rarity)
}

pub fn generated_item_raw(raw: u8) -> Option<ItemEntry> {
    crate::ItemKind::from_raw(raw).and_then(generated_item)
}

pub fn generate_item_of_rarity_with_rng<R: rand::Rng + ?Sized>(
    rarity: crate::Rarity,
    rng: &mut R,
) -> Option<ItemEntry> {
    let candidates: Vec<_> = crate::ItemKind::ALL
        .iter()
        .map(|&kind| ItemBehaviorImpl::for_kind(kind))
        .filter(|behavior| behavior.rarity() == rarity)
        .collect();
    candidates.choose(rng).map(|behavior| ItemEntry {
        id: 0,
        behavior: *behavior,
        item: behavior.generated_state(),
    })
}

#[cfg(test)]
pub(crate) fn validate_item_codec(
    kind: crate::ItemKind,
    item: &ItemWireEntry,
) -> Result<(), crate::CommandError> {
    if item.kind != kind.raw() {
        return Err(crate::CommandError::Rejected);
    }
    codec::validate_kind(item)
}

pub(crate) fn can_use_item(core: &crate::CoreState, kind: crate::ItemKind) -> bool {
    ItemBehaviorImpl::for_kind(kind).can_use(core)
}

pub(crate) fn apply_inventory_item_use(
    core: &mut crate::CoreState,
    item_index: usize,
    kind: crate::ItemKind,
) -> Result<ItemUseOutput, crate::CommandError> {
    let item = core
        .items
        .items
        .get(item_index)
        .cloned()
        .ok_or(crate::CommandError::InvalidIndex)?;
    core.item_use_error_with_kind(kind)?;
    let prepared = item.behavior.prepare_use(core)?;
    let state = item.item;

    core.items.items.remove(item_index);
    core.progress.item_used = true;
    let effects = item.behavior.apply_use(core, state, prepared)?;
    Ok(ItemUseOutput { item, effects })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rarity;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn generated_states_are_valid_for_every_item_kind() {
        for &kind in crate::ItemKind::ALL {
            let behavior = ItemBehaviorImpl::for_kind(kind);
            let item = generated_item(kind)
                .expect("all catalog kinds generate")
                .to_wire();
            assert_eq!(item.id, 0);
            assert_eq!(item.kind, kind.raw());
            assert_eq!(behavior.kind(), kind);
            assert!(validate_item_codec(kind, &item).is_ok(), "kind {kind:?}");
            let round_trip = serde_json::from_str::<ItemWireEntry>(
                &serde_json::to_string(&item).expect("item serializes"),
            )
            .expect("item deserializes");
            assert_eq!(round_trip, item);
        }
    }

    #[test]
    fn generated_raw_shapes_match_the_item_contract() {
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
                .expect("catalog item")
                .to_wire();
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
                    crate::ItemKind::from_raw(item.kind().raw()).and_then(item_rarity)
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
    fn invalid_item_entries_are_rejected_without_partial_shapes() {
        let mut missing = generated_item(crate::ItemKind::Bread)
            .expect("bread")
            .to_wire();
        missing.signed_values.pop();
        assert_eq!(
            validate_item_codec(crate::ItemKind::Bread, &missing),
            Err(crate::CommandError::Rejected)
        );

        let mut extra = generated_item(crate::ItemKind::LumpSugar)
            .expect("lump sugar")
            .to_wire();
        extra.scalar_values.push(99);
        assert_eq!(
            validate_item_codec(crate::ItemKind::LumpSugar, &extra),
            Err(crate::CommandError::Rejected)
        );

        let mut negative = generated_item(crate::ItemKind::Candy)
            .expect("candy")
            .to_wire();
        negative.signed_values[0] = -1;
        assert_eq!(
            validate_item_codec(crate::ItemKind::Candy, &negative),
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
        let _state = crate::CoreState::new_initial(config, 7);
        let result = ItemEntry::from_raw(ItemWireEntry {
            id: 1,
            kind: u8::MAX,
            scalar_values: Vec::new(),
            signed_values: Vec::new(),
        });
        assert!(result.is_err());
    }

    #[test]
    fn applying_each_item_kind_uses_its_raw_codec_contract() {
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
            state.start_stage(1);
            state.hp_raw = 50_000;
            let item = generated_item(crate::ItemKind::from_raw(kind).expect("catalog item"))
                .expect("catalog item");
            state
                .grant_inventory_item(item)
                .expect("generated item can be granted");
            let output = state
                .apply_inventory_item_use(3)
                .expect("generated item can be used");
            assert_eq!(
                output.item.kind(),
                crate::ItemKind::from_raw(kind).expect("item kind")
            );
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
                    vec![ItemUseEffect::GainShield { amount_raw: 12_000 }]
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
