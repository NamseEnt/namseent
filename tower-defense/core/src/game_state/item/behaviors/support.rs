use super::super::{ItemEntryState, ItemUseEffect};

pub(super) fn generate_item(
    kind: crate::ItemKind,
    scalar_values: &[u64],
    signed_values: &[i64],
) -> ItemEntryState {
    ItemEntryState {
        id: 0,
        kind: kind.raw(),
        scalar_values: scalar_values.to_vec(),
        signed_values: signed_values.to_vec(),
    }
}

pub(super) fn validate_payload(
    item: &ItemEntryState,
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

pub(super) fn validate_two_signed_values(item: &ItemEntryState) -> Result<(), crate::CommandError> {
    validate_payload(item, 0, 2)
}

pub(super) fn validate_one_signed_value(item: &ItemEntryState) -> Result<(), crate::CommandError> {
    validate_payload(item, 0, 1)
}

pub(super) fn validate_one_scalar_value(item: &ItemEntryState) -> Result<(), crate::CommandError> {
    validate_payload(item, 1, 0)
}

pub(super) fn always_can_use(_: &crate::CoreState) -> bool {
    true
}

pub(super) fn no_prepare_use(
    _: &crate::CoreState,
    _: &ItemEntryState,
) -> Result<Option<crate::TowerTemplateState>, crate::CommandError> {
    Ok(None)
}

pub(super) fn signed(item: &ItemEntryState, index: usize) -> Result<i64, crate::CommandError> {
    item.signed_values
        .get(index)
        .copied()
        .ok_or(crate::CommandError::Rejected)
}

pub(super) fn scalar(item: &ItemEntryState, index: usize) -> Result<usize, crate::CommandError> {
    usize::try_from(
        *item
            .scalar_values
            .get(index)
            .ok_or(crate::CommandError::Rejected)?,
    )
    .map_err(|_| crate::CommandError::Rejected)
}

pub(super) fn apply_heal(
    core: &mut crate::CoreState,
    item: &ItemEntryState,
    _: Option<crate::TowerTemplateState>,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
    let requested_raw = signed(item, 0)?;
    let before = core.hp_raw;
    core.hp_raw = core
        .hp_raw
        .saturating_add(requested_raw)
        .min(core.max_hp_raw());
    Ok(vec![ItemUseEffect::Heal {
        requested_raw,
        actual_raw: core.hp_raw.saturating_sub(before),
    }])
}

pub(super) fn apply_heal_and_shield(
    core: &mut crate::CoreState,
    item: &ItemEntryState,
    _: Option<crate::TowerTemplateState>,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
    let requested_raw = signed(item, 0)?;
    let shield_raw = signed(item, 1)?;
    let before = core.hp_raw;
    core.hp_raw = core
        .hp_raw
        .saturating_add(requested_raw)
        .min(core.max_hp_raw());
    core.shield_raw = core.shield_raw.saturating_add(shield_raw).max(0);
    Ok(vec![
        ItemUseEffect::Heal {
            requested_raw,
            actual_raw: core.hp_raw.saturating_sub(before),
        },
        ItemUseEffect::GainShield {
            amount_raw: shield_raw,
        },
    ])
}
