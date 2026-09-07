use super::super::ItemUseEffect;
use super::{
    ItemRuntimeState, bread::BreadItemState, candy::CandyItemState, cannoli::CannoliItemState,
    cookie::CookieItemState, donut::DonutItemState, gimbap::GimbapItemState,
    lunch_box::LunchBoxItemState, milk::MilkItemState, rice_ball::RiceBallItemState,
};

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
    state: ItemRuntimeState,
    _: Option<crate::TowerTemplateState>,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
    let requested_raw = match state {
        ItemRuntimeState::Candy(CandyItemState { heal_raw })
        | ItemRuntimeState::Cannoli(CannoliItemState { heal_raw })
        | ItemRuntimeState::Cookie(CookieItemState { heal_raw })
        | ItemRuntimeState::Donut(DonutItemState { heal_raw })
        | ItemRuntimeState::Milk(MilkItemState { heal_raw }) => heal_raw,
        _ => return Err(crate::CommandError::Rejected),
    };
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
    state: ItemRuntimeState,
    _: Option<crate::TowerTemplateState>,
) -> Result<Vec<ItemUseEffect>, crate::CommandError> {
    let (requested_raw, shield_raw) = match state {
        ItemRuntimeState::Bread(BreadItemState {
            heal_raw,
            shield_raw,
        })
        | ItemRuntimeState::Gimbap(GimbapItemState {
            heal_raw,
            shield_raw,
        })
        | ItemRuntimeState::LunchBox(LunchBoxItemState {
            heal_raw,
            shield_raw,
        })
        | ItemRuntimeState::RiceBall(RiceBallItemState {
            heal_raw,
            shield_raw,
        }) => (heal_raw, shield_raw),
        _ => return Err(crate::CommandError::Rejected),
    };
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
