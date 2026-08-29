use super::behaviors;

pub(crate) type ItemApplyUse = fn(
    &mut crate::CoreState,
    &super::ItemEntryState,
    Option<crate::TowerTemplateState>,
) -> Result<Vec<super::ItemUseEffect>, crate::CommandError>;

pub(crate) struct ItemDefinition {
    pub(crate) kind: crate::ItemKind,
    pub(crate) rarity: crate::Rarity,
    pub(crate) generate: fn() -> super::ItemEntryState,
    pub(crate) validate: fn(&super::ItemEntryState) -> Result<(), crate::CommandError>,
    pub(crate) can_use: fn(&crate::CoreState) -> bool,
    pub(crate) prepare_use: fn(
        &crate::CoreState,
        &super::ItemEntryState,
    )
        -> Result<Option<crate::TowerTemplateState>, crate::CommandError>,
    pub(crate) apply_use: ItemApplyUse,
}

pub(crate) static ITEM_DEFINITIONS: [ItemDefinition; crate::ItemKind::COUNT] = [
    behaviors::BREAD,
    behaviors::CANDY,
    behaviors::CANNOLI,
    behaviors::COOKIE,
    behaviors::DONUT,
    behaviors::RICE_BALL,
    behaviors::LUNCH_BOX,
    behaviors::LUMP_SUGAR,
    behaviors::MILK,
    behaviors::RUBBER_CONE,
    behaviors::GIMBAP,
];

pub(crate) fn item_definition(kind: crate::ItemKind) -> Option<&'static ItemDefinition> {
    let definition = &ITEM_DEFINITIONS[usize::from(kind.raw())];
    debug_assert_eq!(definition.kind, kind);
    Some(definition)
}

pub(crate) fn item_definition_raw(raw: u8) -> Option<&'static ItemDefinition> {
    crate::ItemKind::from_raw(raw).and_then(item_definition)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_exactly_one_definition_for_each_item_kind() {
        assert_eq!(ITEM_DEFINITIONS.len(), crate::ItemKind::ALL.len());
        for &kind in crate::ItemKind::ALL {
            let matches = ITEM_DEFINITIONS
                .iter()
                .filter(|definition| definition.kind == kind)
                .count();
            assert_eq!(matches, 1, "item kind {:?}", kind);
            assert_eq!(
                item_definition(kind).map(|definition| definition.kind),
                Some(kind)
            );
        }
    }
}
