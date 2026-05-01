use super::super::*;

#[test]
fn demolition_hammer_stage_start_effects_uses_removed_tower_count() {
    let mut state = UpgradeState::default();
    let mut upgrade = crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(2.0);
    if let Upgrade::DemolitionHammer(upgrade) = &mut upgrade {
        upgrade.removed_tower_count = 2;
    }
    state.upgrade(upgrade);

    let effects = state.stage_start_effects(1);

    assert_eq!(effects.damage_multiplier, 5.0);
    assert!(matches!(
        state.upgrades[0],
        Upgrade::DemolitionHammer(..)
    ));
    if let Upgrade::DemolitionHammer(upgrade) = state.upgrades[0] {
        assert_eq!(upgrade.removed_tower_count, 0);
    }
}

#[test]
fn demolition_hammer_uses_configured_damage_multiplier() {
    let mut state = UpgradeState::default();
    let mut upgrade = crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(1.25);
    if let Upgrade::DemolitionHammer(upgrade) = &mut upgrade {
        upgrade.removed_tower_count = 2;
    }
    state.upgrade(upgrade);

    let effects = state.stage_start_effects(1);

    assert!((effects.damage_multiplier - 3.5).abs() < f32::EPSILON);
}
