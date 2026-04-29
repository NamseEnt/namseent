use super::super::*;
use namui::OneZero;

#[test]
fn demolition_hammer_stage_start_effects_uses_removed_tower_count() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::DemolitionHammer(DemolitionHammerUpgrade { damage_multiplier: 2.0, removed_tower_count: 2,
         }),
        value: OneZero::default(),
    });

    let effects = state.stage_start_effects(1);

    assert_eq!(effects.damage_multiplier, 5.0);
    assert!(matches!(
        state.upgrades[0].kind,
        UpgradeKind::DemolitionHammer(DemolitionHammerUpgrade { removed_tower_count: 0, .. })
    ));
}

#[test]
fn demolition_hammer_uses_configured_damage_multiplier() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::DemolitionHammer(DemolitionHammerUpgrade { damage_multiplier: 1.25, removed_tower_count: 2,
         }),
        value: OneZero::default(),
    });

    let effects = state.stage_start_effects(1);

    assert!((effects.damage_multiplier - 3.5).abs() < f32::EPSILON);
}
