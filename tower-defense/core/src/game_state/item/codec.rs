#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ItemWireEntry {
    pub(crate) id: u64,
    pub(crate) kind: u8,
    pub(crate) scalar_values: Vec<u64>,
    pub(crate) signed_values: Vec<i64>,
}

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn encode_item_entry<S>(entry: &super::ItemEntry, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    entry.to_raw().serialize(serializer)
}

pub fn decode_item_entry<'de, D>(deserializer: D) -> Result<super::ItemEntry, D::Error>
where
    D: Deserializer<'de>,
{
    let wire = ItemWireEntry::deserialize(deserializer)?;
    super::ItemEntry::from_raw(wire)
        .map_err(|_| serde::de::Error::custom("invalid item codec entry"))
}

pub fn encode_item_collection<S>(
    collection: &super::ItemCollection,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    collection.items.serialize(serializer)
}

pub fn decode_item_collection<'de, D>(deserializer: D) -> Result<super::ItemCollection, D::Error>
where
    D: Deserializer<'de>,
{
    let items = Vec::<super::ItemEntry>::deserialize(deserializer)?;
    Ok(super::ItemCollection::from_entries(items))
}

pub(super) fn generate_item(
    kind: crate::ItemKind,
    scalar_values: &[u64],
    signed_values: &[i64],
) -> ItemWireEntry {
    ItemWireEntry {
        id: 0,
        kind: kind.raw(),
        scalar_values: scalar_values.to_vec(),
        signed_values: signed_values.to_vec(),
    }
}

pub(super) fn encode_runtime(
    id: u64,
    kind: crate::ItemKind,
    state: super::behaviors::ItemRuntimeState,
) -> ItemWireEntry {
    let mut item = generate_item(kind, &[], &[]);
    item.id = id;
    match state {
        super::behaviors::ItemRuntimeState::Bread(state) => {
            item.signed_values = vec![state.heal_raw, state.shield_raw]
        }
        super::behaviors::ItemRuntimeState::Candy(state) => {
            item.signed_values = vec![state.heal_raw]
        }
        super::behaviors::ItemRuntimeState::Cannoli(state) => {
            item.signed_values = vec![state.heal_raw]
        }
        super::behaviors::ItemRuntimeState::Cookie(state) => {
            item.signed_values = vec![state.heal_raw]
        }
        super::behaviors::ItemRuntimeState::Donut(state) => {
            item.signed_values = vec![state.heal_raw]
        }
        super::behaviors::ItemRuntimeState::Gimbap(state) => {
            item.signed_values = vec![state.heal_raw, state.shield_raw]
        }
        super::behaviors::ItemRuntimeState::LumpSugar(state) => {
            item.scalar_values = vec![state.amount as u64]
        }
        super::behaviors::ItemRuntimeState::LunchBox(state) => {
            item.signed_values = vec![state.heal_raw, state.shield_raw]
        }
        super::behaviors::ItemRuntimeState::Milk(state) => {
            item.signed_values = vec![state.heal_raw]
        }
        super::behaviors::ItemRuntimeState::RiceBall(state) => {
            item.signed_values = vec![state.heal_raw, state.shield_raw]
        }
        super::behaviors::ItemRuntimeState::RubberCone(state) => {
            item.scalar_values = vec![state.count as u64]
        }
    }
    item
}

pub(super) fn decode_runtime(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    let kind = crate::ItemKind::from_raw(item.kind)
        .ok_or(crate::CommandError::InvalidItemKind { raw: item.kind })?;
    match kind {
        crate::ItemKind::Bread => decode_bread(item),
        crate::ItemKind::Candy => decode_candy(item),
        crate::ItemKind::Cannoli => decode_cannoli(item),
        crate::ItemKind::Cookie => decode_cookie(item),
        crate::ItemKind::Donut => decode_donut(item),
        crate::ItemKind::Gimbap => decode_gimbap(item),
        crate::ItemKind::LumpSugar => decode_lump_sugar(item),
        crate::ItemKind::LunchBox => decode_lunch_box(item),
        crate::ItemKind::Milk => decode_milk(item),
        crate::ItemKind::RiceBall => decode_rice_ball(item),
        crate::ItemKind::RubberCone => decode_rubber_cone(item),
    }
}

#[cfg(test)]
pub(super) fn validate_kind(item: &ItemWireEntry) -> Result<(), crate::CommandError> {
    decode_runtime(item).map(|_| ())
}

pub(super) fn validate_shape(
    item: &ItemWireEntry,
    scalar_count: usize,
    signed_count: usize,
) -> Result<(), crate::CommandError> {
    if item.scalar_values.len() != scalar_count || item.signed_values.len() != signed_count {
        return Err(crate::CommandError::Rejected);
    }
    if item.signed_values.iter().any(|value| *value < 0) {
        return Err(crate::CommandError::Rejected);
    }
    Ok(())
}

pub(super) fn decode_bread(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 2)?;
    Ok(super::behaviors::ItemRuntimeState::Bread(
        super::behaviors::bread::BreadItemState {
            heal_raw: item.signed_values[0],
            shield_raw: item.signed_values[1],
        },
    ))
}

pub(super) fn decode_candy(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 1)?;
    Ok(super::behaviors::ItemRuntimeState::Candy(
        super::behaviors::candy::CandyItemState {
            heal_raw: item.signed_values[0],
        },
    ))
}

pub(super) fn decode_cannoli(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 1)?;
    Ok(super::behaviors::ItemRuntimeState::Cannoli(
        super::behaviors::cannoli::CannoliItemState {
            heal_raw: item.signed_values[0],
        },
    ))
}

pub(super) fn decode_cookie(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 1)?;
    Ok(super::behaviors::ItemRuntimeState::Cookie(
        super::behaviors::cookie::CookieItemState {
            heal_raw: item.signed_values[0],
        },
    ))
}

pub(super) fn decode_donut(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 1)?;
    Ok(super::behaviors::ItemRuntimeState::Donut(
        super::behaviors::donut::DonutItemState {
            heal_raw: item.signed_values[0],
        },
    ))
}

pub(super) fn decode_gimbap(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 2)?;
    Ok(super::behaviors::ItemRuntimeState::Gimbap(
        super::behaviors::gimbap::GimbapItemState {
            heal_raw: item.signed_values[0],
            shield_raw: item.signed_values[1],
        },
    ))
}

pub(super) fn decode_lump_sugar(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 1, 0)?;
    Ok(super::behaviors::ItemRuntimeState::LumpSugar(
        super::behaviors::lump_sugar::LumpSugarItemState {
            amount: usize::try_from(item.scalar_values[0])
                .map_err(|_| crate::CommandError::Rejected)?,
        },
    ))
}

pub(super) fn decode_lunch_box(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 2)?;
    Ok(super::behaviors::ItemRuntimeState::LunchBox(
        super::behaviors::lunch_box::LunchBoxItemState {
            heal_raw: item.signed_values[0],
            shield_raw: item.signed_values[1],
        },
    ))
}

pub(super) fn decode_milk(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 1)?;
    Ok(super::behaviors::ItemRuntimeState::Milk(
        super::behaviors::milk::MilkItemState {
            heal_raw: item.signed_values[0],
        },
    ))
}

pub(super) fn decode_rice_ball(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 0, 2)?;
    Ok(super::behaviors::ItemRuntimeState::RiceBall(
        super::behaviors::rice_ball::RiceBallItemState {
            heal_raw: item.signed_values[0],
            shield_raw: item.signed_values[1],
        },
    ))
}

pub(super) fn decode_rubber_cone(
    item: &ItemWireEntry,
) -> Result<super::behaviors::ItemRuntimeState, crate::CommandError> {
    validate_shape(item, 1, 0)?;
    Ok(super::behaviors::ItemRuntimeState::RubberCone(
        super::behaviors::rubber_cone::RubberConeItemState {
            count: usize::try_from(item.scalar_values[0])
                .map_err(|_| crate::CommandError::Rejected)?,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_shape_validation_rejects_ambiguous_entries() {
        let bread = ItemWireEntry {
            id: 0,
            kind: crate::ItemKind::Bread.raw(),
            scalar_values: Vec::new(),
            signed_values: vec![6_000, 6_000],
        };
        assert_eq!(validate_shape(&bread, 0, 2), Ok(()));
    }

    #[test]
    fn decoders_reject_entries_with_ambiguous_shapes() {
        let malformed = ItemWireEntry {
            id: 0,
            kind: crate::ItemKind::Candy.raw(),
            scalar_values: vec![1],
            signed_values: vec![3_000],
        };

        assert_eq!(
            validate_shape(&malformed, 0, 1),
            Err(crate::CommandError::Rejected)
        );
    }
}
