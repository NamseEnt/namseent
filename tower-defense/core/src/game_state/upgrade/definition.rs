pub(crate) struct UpgradeDefinition {
    pub(crate) kind: crate::UpgradeKind,
    pub(crate) generate_payload: fn(&mut crate::UpgradeEntryState),
    pub(crate) rarity: crate::Rarity,
    pub(crate) cache:
        fn(&crate::UpgradeEntryState) -> crate::game_state::upgrade::UpgradeCacheContribution,
    pub(crate) acquire: fn(&mut crate::CoreState, crate::UpgradeEntryState) -> usize,
    pub(crate) recovery: fn() -> super::UpgradeAcquireRecovery,
    pub(crate) tower_bonus: fn(&crate::UpgradeEntryState, &crate::TowerState) -> i64,
    pub(crate) tower_bonus_for_template:
        fn(&crate::UpgradeEntryState, &crate::TowerTemplateState) -> i64,
    pub(crate) current_and_max: fn(&crate::CoreState) -> Option<(usize, usize)>,
    pub(crate) triggers: UpgradeTriggerDefinition,
}
#[derive(Clone, Copy)]
pub(crate) struct UpgradeTriggerDefinition {
    pub(crate) monster_death: fn(&crate::CoreState, usize, &mut usize, &mut i64),
    pub(crate) gold_earned: fn(&mut crate::CoreState, usize) -> bool,
    pub(crate) card_rerolled: fn(&mut crate::CoreState, usize) -> bool,
    pub(crate) shop_purchase: fn(&mut crate::CoreState, usize, bool) -> bool,
    pub(crate) tower_placed:
        fn(&mut crate::CoreState, usize, u64, bool, &crate::TowerTemplateState, &mut usize) -> bool,
    pub(crate) tower_removed: fn(&mut crate::CoreState, usize, usize),
    pub(crate) stage_start: fn(&mut crate::CoreState, usize, usize) -> bool,
    pub(crate) stage_end: fn(&mut crate::CoreState, usize, bool, usize, usize) -> (bool, usize),
}

pub(crate) static UPGRADE_DEFINITIONS: [UpgradeDefinition; crate::UpgradeKind::COUNT] = [
    super::behaviors::APPLE,
    super::behaviors::BANANA,
    super::behaviors::CARROT,
    super::behaviors::CAT,
    super::behaviors::BACKPACK,
    super::behaviors::DICE_BUNDLE,
    super::behaviors::ENERGY_DRINK,
    super::behaviors::PERFECT_POTTERY,
    super::behaviors::FOUR_LEAF_CLOVER,
    super::behaviors::RABBIT,
    super::behaviors::BLACK_WHITE,
    super::behaviors::TROPHY,
    super::behaviors::CROCK,
    super::behaviors::CUP_NOODLES,
    super::behaviors::FRENCH_FRIES,
    super::behaviors::HAMBURGER,
    super::behaviors::PIZZA,
    super::behaviors::DEMOLITION_HAMMER,
    super::behaviors::METRONOME,
    super::behaviors::TAPE,
    super::behaviors::NAME_TAG,
    super::behaviors::SHOPPING_BAG,
    super::behaviors::RESOLUTION,
    super::behaviors::MIRROR,
    super::behaviors::ICE_CREAM,
    super::behaviors::SPANNER,
    super::behaviors::PEA,
    super::behaviors::SLOT_MACHINE,
    super::behaviors::PIGGY_BANK,
    super::behaviors::CAMERA,
    super::behaviors::GIFT_BOX,
    super::behaviors::FANG,
    super::behaviors::POPCORN,
    super::behaviors::MEMBERSHIP_CARD,
    super::behaviors::BROKEN_POTTERY,
    super::behaviors::STRAWBERRY,
    super::behaviors::WATERMELON,
];

pub(crate) fn definition(kind: crate::UpgradeKind) -> &'static UpgradeDefinition {
    let definition = UPGRADE_DEFINITIONS
        .get(usize::from(kind.raw()))
        .expect("every canonical upgrade kind must have a registry definition");
    debug_assert_eq!(definition.kind, kind);
    definition
}
