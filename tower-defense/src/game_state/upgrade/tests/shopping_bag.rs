use super::super::*;
use namui::OneZero;

#[test]
fn shopping_bag_upgrade_activates_without_stacks() {
    let mut state = UpgradeState::default();
    state.upgrade(Upgrade {
        kind: UpgradeKind::ShoppingBag(ShoppingBagUpgrade { damage_multiplier: 1.5, stacks: 0,
         }),
        value: OneZero::default(),
    });

    assert!(state
        .upgrades
        .iter()
        .any(|u| matches!(u.kind, UpgradeKind::ShoppingBag(ShoppingBagUpgrade { stacks: 0, .. }))));
}
