use super::super::ItemUseEffect;

pub(super) fn always_can_use(_: &crate::CoreState) -> bool {
    true
}

pub(super) fn no_prepare_use(
    _: &crate::CoreState,
) -> Result<Option<crate::TowerTemplateState>, crate::CommandError> {
    Ok(None)
}

pub(super) fn apply_heal(
    core: &mut crate::CoreState,
    requested_raw: i64,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
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

pub(super) fn apply_shield(
    core: &mut crate::CoreState,
    shield_raw: i64,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
    core.shield_raw = core.shield_raw.saturating_add(shield_raw).max(0);
    Ok(vec![ItemUseEffect::GainShield {
        amount_raw: shield_raw,
    }])
}

pub(super) fn apply_heal_and_shield(
    core: &mut crate::CoreState,
    requested_raw: i64,
    shield_raw: i64,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
    let mut effects = apply_heal(core, requested_raw)?;
    effects.extend(apply_shield(core, shield_raw)?);
    Ok(effects)
}
